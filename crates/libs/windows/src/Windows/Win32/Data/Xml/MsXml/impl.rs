#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(&self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR, strqname: &super::super::super::Foundation::BSTR, strtype: &super::super::super::Foundation::BSTR, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addAttributeFromIndex(&self, varatts: &super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::Result<()>;
    fn clear(&self) -> ::windows::core::Result<()>;
    fn removeAttribute(&self, nindex: i32) -> ::windows::core::Result<()>;
    fn setAttribute(&self, nindex: i32, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR, strqname: &super::super::super::Foundation::BSTR, strtype: &super::super::super::Foundation::BSTR, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setAttributes(&self, varatts: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setLocalName(&self, nindex: i32, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setQName(&self, nindex: i32, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setType(&self, nindex: i32, strtype: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setURI(&self, nindex: i32, struri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn setValue(&self, nindex: i32, strvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>() -> IMXAttributes_Vtbl {
        unsafe extern "system" fn addAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addAttribute(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addAttributeFromIndex(::core::mem::transmute(&varatts), ::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn clear<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).clear().into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setAttribute(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn setAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setAttributes(::core::mem::transmute(&varatts)).into()
        }
        unsafe extern "system" fn setLocalName<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strlocalname)).into()
        }
        unsafe extern "system" fn setQName<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strqname)).into()
        }
        unsafe extern "system" fn setType<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strtype)).into()
        }
        unsafe extern "system" fn setURI<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri)).into()
        }
        unsafe extern "system" fn setValue<Identity: ::windows::core::IUnknownImpl, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strvalue)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            addAttribute: addAttribute::<Identity, Impl, OFFSET>,
            addAttributeFromIndex: addAttributeFromIndex::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            setAttributes: setAttributes::<Identity, Impl, OFFSET>,
            setLocalName: setLocalName::<Identity, Impl, OFFSET>,
            setQName: setQName::<Identity, Impl, OFFSET>,
            setType: setType::<Identity, Impl, OFFSET>,
            setURI: setURI::<Identity, Impl, OFFSET>,
            setValue: setValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXAttributes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXNamespaceManager_Impl: Sized {
    fn putAllowOverride(&self, foverride: i16) -> ::windows::core::Result<()>;
    fn getAllowOverride(&self) -> ::windows::core::Result<i16>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn pushContext(&self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&self, contextnode: &::core::option::Option<IXMLDOMNode>, fdeep: i16) -> ::windows::core::Result<()>;
    fn popContext(&self) -> ::windows::core::Result<()>;
    fn declarePrefix(&self, prefix: &::windows::core::PCWSTR, namespaceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getPrefix(&self, pwsznamespaceuri: &::windows::core::PCWSTR, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getURI(&self, pwchprefix: &::windows::core::PCWSTR, pcontextnode: &::core::option::Option<IXMLDOMNode>, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>() -> IMXNamespaceManager_Vtbl {
        unsafe extern "system" fn putAllowOverride<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putAllowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn getAllowOverride<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAllowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *foverride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pushNodeContext(::core::mem::transmute(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::windows::core::PCWSTR, namespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getDeclaredPrefix(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: ::windows::core::PCWSTR, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getPrefix(::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, pcontextnode: ::windows::core::RawPtr, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getURI(::core::mem::transmute(&pwchprefix), ::core::mem::transmute(&pcontextnode), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            putAllowOverride: putAllowOverride::<Identity, Impl, OFFSET>,
            getAllowOverride: getAllowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefix: getDeclaredPrefix::<Identity, Impl, OFFSET>,
            getPrefix: getPrefix::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespaceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXNamespacePrefixes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>() -> IMXNamespacePrefixes_Vtbl {
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *prefix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            item: item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXReaderControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn abort(&self) -> ::windows::core::Result<()>;
    fn resume(&self) -> ::windows::core::Result<()>;
    fn suspend(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControl_Impl, const OFFSET: isize>() -> IMXReaderControl_Vtbl {
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).resume().into()
        }
        unsafe extern "system" fn suspend<Identity: ::windows::core::IUnknownImpl, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).suspend().into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            abort: abort::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            suspend: suspend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXReaderControl as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXSchemaDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(&self, oschemaelement: &::core::option::Option<ISchemaElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>() -> IMXSchemaDeclHandler_Vtbl {
        unsafe extern "system" fn schemaElementDecl<Identity: ::windows::core::IUnknownImpl, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oschemaelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).schemaElementDecl(::core::mem::transmute(&oschemaelement)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            schemaElementDecl: schemaElementDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXWriter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(&self, vardestination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Setencoding(&self, strencoding: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn encoding(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetbyteOrderMark(&self, fwritebyteordermark: i16) -> ::windows::core::Result<()>;
    fn byteOrderMark(&self) -> ::windows::core::Result<i16>;
    fn Setindent(&self, findentmode: i16) -> ::windows::core::Result<()>;
    fn indent(&self) -> ::windows::core::Result<i16>;
    fn Setstandalone(&self, fvalue: i16) -> ::windows::core::Result<()>;
    fn standalone(&self) -> ::windows::core::Result<i16>;
    fn SetomitXMLDeclaration(&self, fvalue: i16) -> ::windows::core::Result<()>;
    fn omitXMLDeclaration(&self) -> ::windows::core::Result<i16>;
    fn Setversion(&self, strversion: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetdisableOutputEscaping(&self, fvalue: i16) -> ::windows::core::Result<()>;
    fn disableOutputEscaping(&self) -> ::windows::core::Result<i16>;
    fn flush(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>() -> IMXWriter_Vtbl {
        unsafe extern "system" fn Setoutput<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setoutput(::core::mem::transmute(&vardestination)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).output() {
                ::core::result::Result::Ok(ok__) => {
                    *vardestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setencoding(::core::mem::transmute(&strencoding)).into()
        }
        unsafe extern "system" fn encoding<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).encoding() {
                ::core::result::Result::Ok(ok__) => {
                    *strencoding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetbyteOrderMark(::core::mem::transmute_copy(&fwritebyteordermark)).into()
        }
        unsafe extern "system" fn byteOrderMark<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).byteOrderMark() {
                ::core::result::Result::Ok(ok__) => {
                    *fwritebyteordermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setindent(::core::mem::transmute_copy(&findentmode)).into()
        }
        unsafe extern "system" fn indent<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).indent() {
                ::core::result::Result::Ok(ok__) => {
                    *findentmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setstandalone(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn standalone<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).standalone() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetomitXMLDeclaration(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).omitXMLDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setversion(::core::mem::transmute(&strversion)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *strversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetdisableOutputEscaping(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).disableOutputEscaping() {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Identity: ::windows::core::IUnknownImpl, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).flush().into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            Setencoding: Setencoding::<Identity, Impl, OFFSET>,
            encoding: encoding::<Identity, Impl, OFFSET>,
            SetbyteOrderMark: SetbyteOrderMark::<Identity, Impl, OFFSET>,
            byteOrderMark: byteOrderMark::<Identity, Impl, OFFSET>,
            Setindent: Setindent::<Identity, Impl, OFFSET>,
            indent: indent::<Identity, Impl, OFFSET>,
            Setstandalone: Setstandalone::<Identity, Impl, OFFSET>,
            standalone: standalone::<Identity, Impl, OFFSET>,
            SetomitXMLDeclaration: SetomitXMLDeclaration::<Identity, Impl, OFFSET>,
            omitXMLDeclaration: omitXMLDeclaration::<Identity, Impl, OFFSET>,
            Setversion: Setversion::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            SetdisableOutputEscaping: SetdisableOutputEscaping::<Identity, Impl, OFFSET>,
            disableOutputEscaping: disableOutputEscaping::<Identity, Impl, OFFSET>,
            flush: flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXWriter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&self, strname: &super::super::super::Foundation::BSTR, fvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, strname: &super::super::super::Foundation::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_entityResolver(&self, oresolver: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn contentHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_contentHandler(&self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn dtdHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_dtdHandler(&self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn errorHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_errorHandler(&self, ohandler: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>() -> IMXXMLFilter_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *oresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_entityResolver(::core::mem::transmute(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_contentHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_dtdHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows::core::IUnknownImpl, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_errorHandler(::core::mem::transmute(&ohandler)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXXMLFilter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ISAXAttributes_Impl: Sized {
    fn getLength(&self) -> ::windows::core::Result<i32>;
    fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::Result<()>;
    fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::Result<()>;
    fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()>;
    fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::Result<()>;
    fn getIndexFromName(&self, pwchuri: &::windows::core::PCWSTR, cchuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32) -> ::windows::core::Result<i32>;
    fn getIndexFromQName(&self, pwchqname: &::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::Result<i32>;
    fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getTypeFromName(&self, pwchuri: &::windows::core::PCWSTR, cchuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getTypeFromQName(&self, pwchqname: &::windows::core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::Result<()>;
    fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
    fn getValueFromName(&self, pwchuri: &::windows::core::PCWSTR, cchuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
    fn getValueFromQName(&self, pwchqname: &::windows::core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::Result<()>;
}
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>() -> ISAXAttributes_Vtbl {
        unsafe extern "system" fn getLength<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pnlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)).into()
        }
        unsafe extern "system" fn getQName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getIndexFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getIndexFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getTypeFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getTypeFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getValue<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getValueFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows::core::IUnknownImpl, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getValueFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getLength: getLength::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getName: getName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXAttributes as ::windows::core::Interface>::IID
    }
}
pub trait ISAXContentHandler_Impl: Sized {
    fn putDocumentLocator(&self, plocator: &::core::option::Option<ISAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&self) -> ::windows::core::Result<()>;
    fn endDocument(&self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&self, pwchprefix: &::windows::core::PCWSTR, cchprefix: i32, pwchuri: &::windows::core::PCWSTR, cchuri: i32) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&self, pwchprefix: &::windows::core::PCWSTR, cchprefix: i32) -> ::windows::core::Result<()>;
    fn startElement(&self, pwchnamespaceuri: &::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, pwchqname: &::windows::core::PCWSTR, cchqname: i32, pattributes: &::core::option::Option<ISAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&self, pwchnamespaceuri: &::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, pwchqname: &::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::Result<()>;
    fn characters(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn processingInstruction(&self, pwchtarget: &::windows::core::PCWSTR, cchtarget: i32, pwchdata: &::windows::core::PCWSTR, cchdata: i32) -> ::windows::core::Result<()>;
    fn skippedEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
}
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>() -> ISAXContentHandler_Vtbl {
        unsafe extern "system" fn putDocumentLocator<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putDocumentLocator(::core::mem::transmute(&plocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32, pwchuri: ::windows::core::PCWSTR, cchuri: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix), ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute(&pattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).characters(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ignorableWhitespace(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchtarget: ::windows::core::PCWSTR, cchtarget: i32, pwchdata: ::windows::core::PCWSTR, cchdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).processingInstruction(::core::mem::transmute(&pwchtarget), ::core::mem::transmute_copy(&cchtarget), ::core::mem::transmute(&pwchdata), ::core::mem::transmute_copy(&cchdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).skippedEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            putDocumentLocator: putDocumentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXContentHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXDTDHandler_Impl: Sized {
    fn notationDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32, pwchnotationname: &::windows::core::PCWSTR, cchnotationname: i32) -> ::windows::core::Result<()>;
}
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>() -> ISAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).notationDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32, pwchnotationname: ::windows::core::PCWSTR, cchnotationname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).unparsedEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid), ::core::mem::transmute(&pwchnotationname), ::core::mem::transmute_copy(&cchnotationname)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDTDHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXDeclHandler_Impl: Sized {
    fn elementDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchmodel: &::windows::core::PCWSTR, cchmodel: i32) -> ::windows::core::Result<()>;
    fn attributeDecl(&self, pwchelementname: &::windows::core::PCWSTR, cchelementname: i32, pwchattributename: &::windows::core::PCWSTR, cchattributename: i32, pwchtype: &::windows::core::PCWSTR, cchtype: i32, pwchvaluedefault: &::windows::core::PCWSTR, cchvaluedefault: i32, pwchvalue: &::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchvalue: &::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
}
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>() -> ISAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchmodel: ::windows::core::PCWSTR, cchmodel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).elementDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchmodel), ::core::mem::transmute_copy(&cchmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchelementname: ::windows::core::PCWSTR, cchelementname: i32, pwchattributename: ::windows::core::PCWSTR, cchattributename: i32, pwchtype: ::windows::core::PCWSTR, cchtype: i32, pwchvaluedefault: ::windows::core::PCWSTR, cchvaluedefault: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).attributeDecl(::core::mem::transmute(&pwchelementname), ::core::mem::transmute_copy(&cchelementname), ::core::mem::transmute(&pwchattributename), ::core::mem::transmute_copy(&cchattributename), ::core::mem::transmute(&pwchtype), ::core::mem::transmute_copy(&cchtype), ::core::mem::transmute(&pwchvaluedefault), ::core::mem::transmute_copy(&cchvaluedefault), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).internalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).externalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDeclHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXEntityResolver_Impl: Sized {
    fn resolveEntity(&self, pwchpublicid: &::windows::core::PCWSTR, pwchsystemid: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>() -> ISAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchpublicid: ::windows::core::PCWSTR, pwchsystemid: ::windows::core::PCWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).resolveEntity(::core::mem::transmute(&pwchpublicid), ::core::mem::transmute(&pwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXEntityResolver as ::windows::core::Interface>::IID
    }
}
pub trait ISAXErrorHandler_Impl: Sized {
    fn error(&self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn fatalError(&self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ignorableWarning(&self, plocator: &::core::option::Option<ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>() -> ISAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).error(::core::mem::transmute(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fatalError(::core::mem::transmute(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows::core::IUnknownImpl, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ignorableWarning(::core::mem::transmute(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXErrorHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXLexicalHandler_Impl: Sized {
    fn startDTD(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn endDTD(&self) -> ::windows::core::Result<()>;
    fn startEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn endEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn startCDATA(&self) -> ::windows::core::Result<()>;
    fn endCDATA(&self) -> ::windows::core::Result<()>;
    fn comment(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
}
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>() -> ISAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startDTD(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).comment(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as ::windows::core::Interface>::IID
    }
}
pub trait ISAXLocator_Impl: Sized {
    fn getColumnNumber(&self) -> ::windows::core::Result<i32>;
    fn getLineNumber(&self) -> ::windows::core::Result<i32>;
    fn getPublicId(&self) -> ::windows::core::Result<*mut u16>;
    fn getSystemId(&self) -> ::windows::core::Result<*mut u16>;
}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const OFFSET: isize>() -> ISAXLocator_Vtbl {
        unsafe extern "system" fn getColumnNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getColumnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pncolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pnline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getPublicId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchpublicid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Identity: ::windows::core::IUnknownImpl, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchsystemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getColumnNumber: getColumnNumber::<Identity, Impl, OFFSET>,
            getLineNumber: getLineNumber::<Identity, Impl, OFFSET>,
            getPublicId: getPublicId::<Identity, Impl, OFFSET>,
            getSystemId: getSystemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLFilter_Impl: Sized + ISAXXMLReader_Impl {
    fn getParent(&self) -> ::windows::core::Result<ISAXXMLReader>;
    fn putParent(&self, preader: &::core::option::Option<ISAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>() -> ISAXXMLFilter_Vtbl {
        unsafe extern "system" fn getParent<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putParent(::core::mem::transmute(&preader)).into()
        }
        Self {
            base: ISAXXMLReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            getParent: getParent::<Identity, Impl, OFFSET>,
            putParent: putParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLFilter as ::windows::core::Interface>::IID || iid == &<ISAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLReader_Impl: Sized {
    fn getFeature(&self, pwchname: &::windows::core::PCWSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&self, pwchname: &::windows::core::PCWSTR, vfvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&self, pwchname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, pwchname: &::windows::core::PCWSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getEntityResolver(&self) -> ::windows::core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&self, presolver: &::core::option::Option<ISAXEntityResolver>) -> ::windows::core::Result<()>;
    fn getContentHandler(&self) -> ::windows::core::Result<ISAXContentHandler>;
    fn putContentHandler(&self, phandler: &::core::option::Option<ISAXContentHandler>) -> ::windows::core::Result<()>;
    fn getDTDHandler(&self) -> ::windows::core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&self, phandler: &::core::option::Option<ISAXDTDHandler>) -> ::windows::core::Result<()>;
    fn getErrorHandler(&self) -> ::windows::core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&self, phandler: &::core::option::Option<ISAXErrorHandler>) -> ::windows::core::Result<()>;
    fn getBaseURL(&self) -> ::windows::core::Result<*mut u16>;
    fn putBaseURL(&self, pwchbaseurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn getSecureBaseURL(&self) -> ::windows::core::Result<*mut u16>;
    fn putSecureBaseURL(&self, pwchsecurebaseurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&self, pwchurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>() -> ISAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvfvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getFeature(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, vfvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putFeature(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&vfvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getProperty(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putProperty(::core::mem::transmute(&pwchname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn getEntityResolver<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getEntityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putEntityResolver(::core::mem::transmute(&presolver)).into()
        }
        unsafe extern "system" fn getContentHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getContentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putContentHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getDTDHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDTDHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putDTDHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getErrorHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getErrorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *pphandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putErrorHandler(::core::mem::transmute(&phandler)).into()
        }
        unsafe extern "system" fn getBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchbaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putBaseURL(::core::mem::transmute(&pwchbaseurl)).into()
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getSecureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwchsecurebaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putSecureBaseURL(::core::mem::transmute(&pwchsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows::core::IUnknownImpl, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).parseURL(::core::mem::transmute(&pwchurl)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            getEntityResolver: getEntityResolver::<Identity, Impl, OFFSET>,
            putEntityResolver: putEntityResolver::<Identity, Impl, OFFSET>,
            getContentHandler: getContentHandler::<Identity, Impl, OFFSET>,
            putContentHandler: putContentHandler::<Identity, Impl, OFFSET>,
            getDTDHandler: getDTDHandler::<Identity, Impl, OFFSET>,
            putDTDHandler: putDTDHandler::<Identity, Impl, OFFSET>,
            getErrorHandler: getErrorHandler::<Identity, Impl, OFFSET>,
            putErrorHandler: putErrorHandler::<Identity, Impl, OFFSET>,
            getBaseURL: getBaseURL::<Identity, Impl, OFFSET>,
            putBaseURL: putBaseURL::<Identity, Impl, OFFSET>,
            getSecureBaseURL: getSecureBaseURL::<Identity, Impl, OFFSET>,
            putSecureBaseURL: putSecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchema_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn targetNamespace(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn version(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn types(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn elements(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributeGroups(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn modelGroups(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn notations(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn schemaLocations(&self) -> ::windows::core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchema_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>() -> ISchema_Vtbl {
        unsafe extern "system" fn targetNamespace<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).targetNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *targetnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, types: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).types() {
                ::core::result::Result::Ok(ok__) => {
                    *types = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).elements() {
                ::core::result::Result::Ok(ok__) => {
                    *elements = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributeGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *attributegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).modelGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *modelgroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).notations() {
                ::core::result::Result::Ok(ok__) => {
                    *notations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Identity: ::windows::core::IUnknownImpl, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schemalocations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).schemaLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *schemalocations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            targetNamespace: targetNamespace::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            types: types::<Identity, Impl, OFFSET>,
            elements: elements::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            attributeGroups: attributeGroups::<Identity, Impl, OFFSET>,
            modelGroups: modelGroups::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
            schemaLocations: schemaLocations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchema as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAny_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn namespaces(&self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn processContents(&self) -> ::windows::core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAny_Impl, const OFFSET: isize>() -> ISchemaAny_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).processContents() {
                ::core::result::Result::Ok(ok__) => {
                    *processcontents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            processContents: processContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAny as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn r#type(&self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fixedValue(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn r#use(&self) -> ::windows::core::Result<SCHEMAUSE>;
    fn isReference(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>() -> ISchemaAttribute_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *defaultvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *fixedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#use() {
                ::core::result::Result::Ok(ok__) => {
                    *r#use = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isReference() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            r#use: r#use::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttribute as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttributeGroup_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>() -> ISchemaAttributeGroup_Vtbl {
        unsafe extern "system" fn anyAttribute<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    *anyattribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaComplexType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaType_Impl {
    fn isAbstract(&self) -> ::windows::core::Result<i16>;
    fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn contentType(&self) -> ::windows::core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&self) -> ::windows::core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>() -> ISchemaComplexType_Vtbl {
        unsafe extern "system" fn isAbstract<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    *r#abstract = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    *anyattribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).contentType() {
                ::core::result::Result::Ok(ok__) => {
                    *contenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).contentModel() {
                ::core::result::Result::Ok(ok__) => {
                    *contentmodel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).prohibitedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    *prohibited = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaType_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            contentType: contentType::<Identity, Impl, OFFSET>,
            contentModel: contentModel::<Identity, Impl, OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaComplexType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaType as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn r#type(&self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fixedValue(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn isNillable(&self) -> ::windows::core::Result<i16>;
    fn identityConstraints(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&self) -> ::windows::core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&self) -> ::windows::core::Result<i16>;
    fn isReference(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>() -> ISchemaElement_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).scope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *defaultvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *fixedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nillable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isNillable() {
                ::core::result::Result::Ok(ok__) => {
                    *nillable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).identityConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *constraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).substitutionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).substitutionGroupExclusions() {
                ::core::result::Result::Ok(ok__) => {
                    *exclusions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).disallowedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    *disallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    *r#abstract = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isReference() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            isNillable: isNillable::<Identity, Impl, OFFSET>,
            identityConstraints: identityConstraints::<Identity, Impl, OFFSET>,
            substitutionGroup: substitutionGroup::<Identity, Impl, OFFSET>,
            substitutionGroupExclusions: substitutionGroupExclusions::<Identity, Impl, OFFSET>,
            disallowedSubstitutions: disallowedSubstitutions::<Identity, Impl, OFFSET>,
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaIdentityConstraint_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn selector(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fields(&self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn referencedKey(&self) -> ::windows::core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>() -> ISchemaIdentityConstraint_Vtbl {
        unsafe extern "system" fn selector<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).selector() {
                ::core::result::Result::Ok(ok__) => {
                    *selector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fields() {
                ::core::result::Result::Ok(ok__) => {
                    *fields = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).referencedKey() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            selector: selector::<Identity, Impl, OFFSET>,
            fields: fields::<Identity, Impl, OFFSET>,
            referencedKey: referencedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItem_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn namespaceURI(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn schema(&self) -> ::windows::core::Result<ISchema>;
    fn id(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&self, annotationsink: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>() -> ISchemaItem_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).namespaceURI() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).schema() {
                ::core::result::Result::Ok(ok__) => {
                    *schema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).itemType() {
                ::core::result::Result::Ok(ok__) => {
                    *itemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).unhandledAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).writeAnnotation(::core::mem::transmute(&annotationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *iswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            schema: schema::<Identity, Impl, OFFSET>,
            id: id::<Identity, Impl, OFFSET>,
            itemType: itemType::<Identity, Impl, OFFSET>,
            unhandledAttributes: unhandledAttributes::<Identity, Impl, OFFSET>,
            writeAnnotation: writeAnnotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItemCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&self, index: i32) -> ::windows::core::Result<ISchemaItem>;
    fn itemByName(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn itemByQName(&self, name: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>() -> ISchemaItemCollection_Vtbl {
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).itemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).itemByQName(::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            item: item::<Identity, Impl, OFFSET>,
            itemByName: itemByName::<Identity, Impl, OFFSET>,
            itemByQName: itemByQName::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItemCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaModelGroup_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl + ISchemaParticle_Impl {
    fn particles(&self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>() -> ISchemaModelGroup_Vtbl {
        unsafe extern "system" fn particles<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, particles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).particles() {
                ::core::result::Result::Ok(ok__) => {
                    *particles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(), particles: particles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaModelGroup as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID || iid == &<ISchemaParticle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaNotation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn systemIdentifier(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn publicIdentifier(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaNotation_Impl, const OFFSET: isize>() -> ISchemaNotation_Vtbl {
        unsafe extern "system" fn systemIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).systemIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).publicIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            systemIdentifier: systemIdentifier::<Identity, Impl, OFFSET>,
            publicIdentifier: publicIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaNotation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaParticle_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaParticle_Impl, const OFFSET: isize>() -> ISchemaParticle_Vtbl {
        unsafe extern "system" fn minOccurs<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).minOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    *minoccurs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).maxOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    *maxoccurs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            minOccurs: minOccurs::<Identity, Impl, OFFSET>,
            maxOccurs: maxOccurs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaParticle as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaStringCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>() -> ISchemaStringCollection_Vtbl {
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *bstr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            item: item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaStringCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + ISchemaItem_Impl {
    fn baseTypes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn r#final(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&self) -> ::windows::core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn minExclusive(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn minInclusive(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn maxExclusive(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn maxInclusive(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn totalDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn fractionDigits(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn length(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn minLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn maxLength(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn enumeration(&self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn whitespace(&self) -> ::windows::core::Result<SCHEMAWHITESPACE>;
    fn patterns(&self) -> ::windows::core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>() -> ISchemaType_Vtbl {
        unsafe extern "system" fn baseTypes<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).baseTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *basetypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#final() {
                ::core::result::Result::Ok(ok__) => {
                    *r#final = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).variety() {
                ::core::result::Result::Ok(ok__) => {
                    *variety = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).derivedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *derivedby = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).isValid(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *valid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).minExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *minexclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mininclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).minInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *mininclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).maxExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *maxexclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).maxInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *maxinclusive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).totalDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *totaldigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *fractiondigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).minLength() {
                ::core::result::Result::Ok(ok__) => {
                    *minlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).maxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *maxlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumeration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).enumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *enumeration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).whitespace() {
                ::core::result::Result::Ok(ok__) => {
                    *whitespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Identity: ::windows::core::IUnknownImpl, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).patterns() {
                ::core::result::Result::Ok(ok__) => {
                    *patterns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            baseTypes: baseTypes::<Identity, Impl, OFFSET>,
            r#final: r#final::<Identity, Impl, OFFSET>,
            variety: variety::<Identity, Impl, OFFSET>,
            derivedBy: derivedBy::<Identity, Impl, OFFSET>,
            isValid: isValid::<Identity, Impl, OFFSET>,
            minExclusive: minExclusive::<Identity, Impl, OFFSET>,
            minInclusive: minInclusive::<Identity, Impl, OFFSET>,
            maxExclusive: maxExclusive::<Identity, Impl, OFFSET>,
            maxInclusive: maxInclusive::<Identity, Impl, OFFSET>,
            totalDigits: totalDigits::<Identity, Impl, OFFSET>,
            fractionDigits: fractionDigits::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            minLength: minLength::<Identity, Impl, OFFSET>,
            maxLength: maxLength::<Identity, Impl, OFFSET>,
            enumeration: enumeration::<Identity, Impl, OFFSET>,
            whitespace: whitespace::<Identity, Impl, OFFSET>,
            patterns: patterns::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISchemaItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLHTTPRequest_Impl {
    fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::Result<()>;
    fn waitForResponse(&self, timeoutinseconds: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn setTimeouts<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setTimeouts(::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn waitForResponse<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinseconds: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).waitForResponse(::core::mem::transmute(&timeoutinseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *issuccessful = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setOption(::core::mem::transmute_copy(&option), ::core::mem::transmute(&value)).into()
        }
        Self {
            base: IXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setTimeouts: setTimeouts::<Identity, Impl, OFFSET>,
            waitForResponse: waitForResponse::<Identity, Impl, OFFSET>,
            getOption: getOption::<Identity, Impl, OFFSET>,
            setOption: setOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLHTTPRequest_Impl + IServerXMLHTTPRequest_Impl {
    fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::super::super::System::Com::VARIANT, varbypasslist: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setProxyCredentials(&self, bstrusername: &super::super::super::Foundation::BSTR, bstrpassword: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn setProxy<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, varbypasslist: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxy(::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&varproxyserver), ::core::mem::transmute(&varbypasslist)).into()
        }
        unsafe extern "system" fn setProxyCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProxyCredentials(::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into()
        }
        Self {
            base: IServerXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setProxy: setProxy::<Identity, Impl, OFFSET>,
            setProxyCredentials: setProxyCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<IServerXMLHTTPRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBMXNamespaceManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(&self, foverride: i16) -> ::windows::core::Result<()>;
    fn allowOverride(&self) -> ::windows::core::Result<i16>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn pushContext(&self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&self, contextnode: &::core::option::Option<IXMLDOMNode>, fdeep: i16) -> ::windows::core::Result<()>;
    fn popContext(&self) -> ::windows::core::Result<()>;
    fn declarePrefix(&self, prefix: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefixes(&self) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getURI(&self, prefix: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn getURIFromNode(&self, strprefix: &super::super::super::Foundation::BSTR, contextnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>() -> IVBMXNamespaceManager_Vtbl {
        unsafe extern "system" fn SetallowOverride<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetallowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn allowOverride<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).allowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *foverride = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).pushNodeContext(::core::mem::transmute(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDeclaredPrefixes() {
                ::core::result::Result::Ok(ok__) => {
                    *prefixes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getPrefixes(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *prefixes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getURI(::core::mem::transmute(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Identity: ::windows::core::IUnknownImpl, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, contextnode: ::windows::core::RawPtr, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getURIFromNode(::core::mem::transmute(&strprefix), ::core::mem::transmute(&contextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetallowOverride: SetallowOverride::<Identity, Impl, OFFSET>,
            allowOverride: allowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefixes: getDeclaredPrefixes::<Identity, Impl, OFFSET>,
            getPrefixes: getPrefixes::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getURIFromNode: getURIFromNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn length(&self) -> ::windows::core::Result<i32>;
    fn getURI(&self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getLocalName(&self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getQName(&self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getIndexFromName(&self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn getIndexFromQName(&self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn getType(&self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getTypeFromName(&self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getTypeFromQName(&self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValue(&self, nindex: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValueFromName(&self, struri: &super::super::super::Foundation::BSTR, strlocalname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getValueFromQName(&self, strqname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>() -> IVBSAXAttributes_Vtbl {
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *nlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getURI(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *struri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getLocalName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strlocalname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getQName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strqname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getIndexFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *nindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getIndexFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *nindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getType(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getTypeFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getTypeFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getValue(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getValueFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getValueFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            length: length::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXAttributes as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXContentHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(&self, olocator: &::core::option::Option<IVBSAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&self) -> ::windows::core::Result<()>;
    fn endDocument(&self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&self, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&self, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startElement(&self, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: &::core::option::Option<IVBSAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&self, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn characters(&self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn processingInstruction(&self, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn skippedEntity(&self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>() -> IVBSAXContentHandler_Vtbl {
        unsafe extern "system" fn putref_documentLocator<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_documentLocator(::core::mem::transmute(&olocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startPrefixMapping(::core::mem::transmute_copy(&strprefix), ::core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endPrefixMapping(::core::mem::transmute_copy(&strprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::core::mem::transmute(&oattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).characters(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ignorableWhitespace(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).processingInstruction(::core::mem::transmute_copy(&strtarget), ::core::mem::transmute_copy(&strdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).skippedEntity(::core::mem::transmute_copy(&strname)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_documentLocator: putref_documentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDTDHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(&self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>() -> IVBSAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).notationDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).unparsedEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&strnotationname)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(&self, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn attributeDecl(&self, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&self, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>() -> IVBSAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).elementDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).attributeDecl(::core::mem::transmute_copy(&strelementname), ::core::mem::transmute_copy(&strattributename), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvaluedefault), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).internalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).externalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXEntityResolver_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(&self, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>() -> IVBSAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).resolveEntity(::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&varinput)).into()
        }
        Self { base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXErrorHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn error(&self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn fatalError(&self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn ignorableWarning(&self, olocator: &::core::option::Option<IVBSAXLocator>, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>() -> IVBSAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).error(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).fatalError(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ignorableWarning(::core::mem::transmute(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLexicalHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(&self, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endDTD(&self) -> ::windows::core::Result<()>;
    fn startEntity(&self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn endEntity(&self, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startCDATA(&self) -> ::windows::core::Result<()>;
    fn endCDATA(&self) -> ::windows::core::Result<()>;
    fn comment(&self, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>() -> IVBSAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startDTD(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).comment(::core::mem::transmute_copy(&strchars)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLocator_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(&self) -> ::windows::core::Result<i32>;
    fn lineNumber(&self) -> ::windows::core::Result<i32>;
    fn publicId(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn systemId(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const OFFSET: isize>() -> IVBSAXLocator_Vtbl {
        unsafe extern "system" fn columnNumber<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).columnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *ncolumn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *nline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).publicId() {
                ::core::result::Result::Ok(ok__) => {
                    *strpublicid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).systemId() {
                ::core::result::Result::Ok(ok__) => {
                    *strsystemid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            columnNumber: columnNumber::<Identity, Impl, OFFSET>,
            lineNumber: lineNumber::<Identity, Impl, OFFSET>,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLocator as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn parent(&self) -> ::windows::core::Result<IVBSAXXMLReader>;
    fn putref_parent(&self, oreader: &::core::option::Option<IVBSAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>() -> IVBSAXXMLFilter_Vtbl {
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *oreader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_parent(::core::mem::transmute(&oreader)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            parent: parent::<Identity, Impl, OFFSET>,
            putref_parent: putref_parent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLReader_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn putFeature(&self, strname: &super::super::super::Foundation::BSTR, fvalue: i16) -> ::windows::core::Result<()>;
    fn getProperty(&self, strname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, strname: &super::super::super::Foundation::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&self) -> ::windows::core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&self, oresolver: &::core::option::Option<IVBSAXEntityResolver>) -> ::windows::core::Result<()>;
    fn contentHandler(&self) -> ::windows::core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&self, ohandler: &::core::option::Option<IVBSAXContentHandler>) -> ::windows::core::Result<()>;
    fn dtdHandler(&self) -> ::windows::core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&self, ohandler: &::core::option::Option<IVBSAXDTDHandler>) -> ::windows::core::Result<()>;
    fn errorHandler(&self) -> ::windows::core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&self, ohandler: &::core::option::Option<IVBSAXErrorHandler>) -> ::windows::core::Result<()>;
    fn baseURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetbaseURL(&self, strbaseurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn secureBaseURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetsecureBaseURL(&self, strsecurebaseurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&self, strurl: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>() -> IVBSAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *fvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *oresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_entityResolver(::core::mem::transmute(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_contentHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_dtdHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ohandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_errorHandler(::core::mem::transmute(&ohandler)).into()
        }
        unsafe extern "system" fn baseURL<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).baseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *strbaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetbaseURL(::core::mem::transmute(&strbaseurl)).into()
        }
        unsafe extern "system" fn secureBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).secureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    *strsecurebaseurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetsecureBaseURL(::core::mem::transmute(&strsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows::core::IUnknownImpl, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).parseURL(::core::mem::transmute(&strurl)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
            baseURL: baseURL::<Identity, Impl, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, Impl, OFFSET>,
            secureBaseURL: secureBaseURL::<Identity, Impl, OFFSET>,
            SetsecureBaseURL: SetsecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn value(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLAttribute_Impl, const OFFSET: isize>() -> IXMLAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, n: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).name() {
                ::core::result::Result::Ok(ok__) => {
                    *n = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: ::windows::core::IUnknownImpl, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).value() {
                ::core::result::Result::Ok(ok__) => {
                    *v = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLAttribute as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn name(&self, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn value(&self, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setvalue(&self, attributevalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>() -> IXMLDOMAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&attributename)).into()
        }
        unsafe extern "system" fn value<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).value(::core::mem::transmute_copy(&attributevalue)).into()
        }
        unsafe extern "system" fn Setvalue<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setvalue(::core::mem::transmute(&attributevalue)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
            Setvalue: Setvalue::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCDATASection_Impl, const OFFSET: isize>() -> IXMLDOMCDATASection_Vtbl {
        Self { base: IXMLDOMText_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID || iid == &<IXMLDOMText as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCharacterData_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn data(&self, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()>;
    fn substringData(&self, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn appendData(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn insertData(&self, offset: i32, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()>;
    fn replaceData(&self, offset: i32, count: i32, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>() -> IXMLDOMCharacterData_Vtbl {
        unsafe extern "system" fn data<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).data(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Setdata<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setdata(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn substringData<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).substringData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn appendData<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).appendData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn insertData<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).insertData(::core::mem::transmute_copy(&offset), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn deleteData<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).deleteData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn replaceData<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).replaceData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute(&data)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            substringData: substringData::<Identity, Impl, OFFSET>,
            appendData: appendData::<Identity, Impl, OFFSET>,
            insertData: insertData::<Identity, Impl, OFFSET>,
            deleteData: deleteData::<Identity, Impl, OFFSET>,
            replaceData: replaceData::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMComment_Impl, const OFFSET: isize>() -> IXMLDOMComment_Vtbl {
        Self { base: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMComment as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn doctype(&self) -> ::windows::core::Result<IXMLDOMDocumentType>;
    fn implementation(&self) -> ::windows::core::Result<IXMLDOMImplementation>;
    fn documentElement(&self) -> ::windows::core::Result<IXMLDOMElement>;
    fn putref_documentElement(&self, domelement: &::core::option::Option<IXMLDOMElement>) -> ::windows::core::Result<()>;
    fn createElement(&self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&self) -> ::windows::core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMText>;
    fn createComment(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMComment>;
    fn createCDATASection(&self, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&self, target: &super::super::super::Foundation::BSTR, data: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn createNode(&self, r#type: &super::super::super::System::Com::VARIANT, name: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeFromID(&self, idstring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn load(&self, xmlsource: &super::super::super::System::Com::VARIANT, issuccessful: *mut i16) -> ::windows::core::Result<()>;
    fn readyState(&self, value: *mut i32) -> ::windows::core::Result<()>;
    fn parseError(&self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn url(&self, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn r#async(&self, isasync: *mut i16) -> ::windows::core::Result<()>;
    fn Setasync(&self, isasync: i16) -> ::windows::core::Result<()>;
    fn abort(&self) -> ::windows::core::Result<()>;
    fn loadXML(&self, bstrxml: &super::super::super::Foundation::BSTR, issuccessful: *mut i16) -> ::windows::core::Result<()>;
    fn save(&self, destination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validateOnParse(&self, isvalidating: *mut i16) -> ::windows::core::Result<()>;
    fn SetvalidateOnParse(&self, isvalidating: i16) -> ::windows::core::Result<()>;
    fn resolveExternals(&self, isresolving: *mut i16) -> ::windows::core::Result<()>;
    fn SetresolveExternals(&self, isresolving: i16) -> ::windows::core::Result<()>;
    fn preserveWhiteSpace(&self, ispreserving: *mut i16) -> ::windows::core::Result<()>;
    fn SetpreserveWhiteSpace(&self, ispreserving: i16) -> ::windows::core::Result<()>;
    fn Setonreadystatechange(&self, readystatechangesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setondataavailable(&self, ondataavailablesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setontransformnode(&self, ontransformnodesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>() -> IXMLDOMDocument_Vtbl {
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *documenttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#impl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).implementation() {
                ::core::result::Result::Ok(ok__) => {
                    *r#impl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).documentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *domelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_documentElement(::core::mem::transmute(&domelement)).into()
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createElement(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, docfrag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createDocumentFragment() {
                ::core::result::Result::Ok(ok__) => {
                    *docfrag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, text: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createTextNode(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, comment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createComment(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *comment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, cdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createCDATASection(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *cdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createProcessingInstruction(::core::mem::transmute(&target), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *pi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *attribute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, entityref: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createEntityReference(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *entityref = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createNode(::core::mem::transmute(&r#type), ::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).nodeFromID(::core::mem::transmute(&idstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *node = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlsource: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).load(::core::mem::transmute(&xmlsource), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).readyState(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn parseError<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).parseError() {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn r#async<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).r#async(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn Setasync<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setasync(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn loadXML<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).loadXML(::core::mem::transmute(&bstrxml), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn save<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).save(::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn validateOnParse<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).validateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetvalidateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn resolveExternals<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).resolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn SetresolveExternals<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetresolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).preserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetpreserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystatechangesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute(&readystatechangesink)).into()
        }
        unsafe extern "system" fn Setondataavailable<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ondataavailablesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setondataavailable(::core::mem::transmute(&ondataavailablesink)).into()
        }
        unsafe extern "system" fn Setontransformnode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ontransformnodesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setontransformnode(::core::mem::transmute(&ontransformnodesink)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            doctype: doctype::<Identity, Impl, OFFSET>,
            implementation: implementation::<Identity, Impl, OFFSET>,
            documentElement: documentElement::<Identity, Impl, OFFSET>,
            putref_documentElement: putref_documentElement::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            createDocumentFragment: createDocumentFragment::<Identity, Impl, OFFSET>,
            createTextNode: createTextNode::<Identity, Impl, OFFSET>,
            createComment: createComment::<Identity, Impl, OFFSET>,
            createCDATASection: createCDATASection::<Identity, Impl, OFFSET>,
            createProcessingInstruction: createProcessingInstruction::<Identity, Impl, OFFSET>,
            createAttribute: createAttribute::<Identity, Impl, OFFSET>,
            createEntityReference: createEntityReference::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            createNode: createNode::<Identity, Impl, OFFSET>,
            nodeFromID: nodeFromID::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            parseError: parseError::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            loadXML: loadXML::<Identity, Impl, OFFSET>,
            save: save::<Identity, Impl, OFFSET>,
            validateOnParse: validateOnParse::<Identity, Impl, OFFSET>,
            SetvalidateOnParse: SetvalidateOnParse::<Identity, Impl, OFFSET>,
            resolveExternals: resolveExternals::<Identity, Impl, OFFSET>,
            SetresolveExternals: SetresolveExternals::<Identity, Impl, OFFSET>,
            preserveWhiteSpace: preserveWhiteSpace::<Identity, Impl, OFFSET>,
            SetpreserveWhiteSpace: SetpreserveWhiteSpace::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
            Setondataavailable: Setondataavailable::<Identity, Impl, OFFSET>,
            Setontransformnode: Setontransformnode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMDocument_Impl {
    fn namespaces(&self) -> ::windows::core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putref_schemas(&self, othercollection: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validate(&self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn setProperty(&self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getProperty(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>() -> IXMLDOMDocument2_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *namespacecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).schemas() {
                ::core::result::Result::Ok(ok__) => {
                    *othercollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_schemas(::core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn validate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).validate() {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            schemas: schemas::<Identity, Impl, OFFSET>,
            putref_schemas: putref_schemas::<Identity, Impl, OFFSET>,
            validate: validate::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument3_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMDocument_Impl + IXMLDOMDocument2_Impl {
    fn validateNode(&self, node: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMParseError>;
    fn importNode(&self, node: &::core::option::Option<IXMLDOMNode>, deep: i16) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>() -> IXMLDOMDocument3_Vtbl {
        unsafe extern "system" fn validateNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).validateNode(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, deep: i16, clone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).importNode(::core::mem::transmute(&node), ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    *clone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMDocument2_Vtbl::new::<Identity, Impl, OFFSET>(),
            validateNode: validateNode::<Identity, Impl, OFFSET>,
            importNode: importNode::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentFragment_Impl, const OFFSET: isize>() -> IXMLDOMDocumentFragment_Vtbl {
        Self { base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentType_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn name(&self, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn entities(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>() -> IXMLDOMDocumentType_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).name(::core::mem::transmute_copy(&rootname)).into()
        }
        unsafe extern "system" fn entities<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).entities() {
                ::core::result::Result::Ok(ok__) => {
                    *entitymap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notationmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).notations() {
                ::core::result::Result::Ok(ok__) => {
                    *notationmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            entities: entities::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn tagName(&self, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttribute(&self, name: &super::super::super::Foundation::BSTR, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setAttribute(&self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn removeAttribute(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeNode(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&self, domattribute: &::core::option::Option<IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&self, domattribute: &::core::option::Option<IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&self, tagname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn normalize(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>() -> IXMLDOMElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).tagName(::core::mem::transmute_copy(&tagname)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).getAttribute(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setAttribute(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn getAttributeNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAttributeNode(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).setAttributeNode(::core::mem::transmute(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).removeAttributeNode(::core::mem::transmute(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributenode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).normalize().into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            getAttributeNode: getAttributeNode::<Identity, Impl, OFFSET>,
            setAttributeNode: setAttributeNode::<Identity, Impl, OFFSET>,
            removeAttributeNode: removeAttributeNode::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            normalize: normalize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntity_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn notationName(&self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>() -> IXMLDOMEntity_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        unsafe extern "system" fn notationName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).notationName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
            notationName: notationName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMEntityReference_Impl, const OFFSET: isize>() -> IXMLDOMEntityReference_Vtbl {
        Self { base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMImplementation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(&self, feature: &super::super::super::Foundation::BSTR, version: &super::super::super::Foundation::BSTR, hasfeature: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>() -> IXMLDOMImplementation_Vtbl {
        unsafe extern "system" fn hasFeature<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, version: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, hasfeature: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).hasFeature(::core::mem::transmute(&feature), ::core::mem::transmute(&version), ::core::mem::transmute_copy(&hasfeature)).into()
        }
        Self { base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), hasFeature: hasFeature::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNamedNodeMap_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn setNamedItem(&self, newitem: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNamedItem(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn getQualifiedItem(&self, basename: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&self, basename: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>() -> IXMLDOMNamedNodeMap_Vtbl {
        unsafe extern "system" fn getNamedItem<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newitem: ::windows::core::RawPtr, nameitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).setNamedItem(::core::mem::transmute(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).removeNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *nameditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *listitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn getQualifiedItem<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *qualifieditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).removeQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *qualifieditem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    *nextitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getNamedItem: getNamedItem::<Identity, Impl, OFFSET>,
            setNamedItem: setNamedItem::<Identity, Impl, OFFSET>,
            removeNamedItem: removeNamedItem::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            getQualifiedItem: getQualifiedItem::<Identity, Impl, OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNode_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(&self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn nodeValue(&self, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetnodeValue(&self, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn nodeType(&self, r#type: *mut DOMNodeType) -> ::windows::core::Result<()>;
    fn parentNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn childNodes(&self) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn firstChild(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn lastChild(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn previousSibling(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn nextSibling(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn attributes(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(&self, newchild: &::core::option::Option<IXMLDOMNode>, refchild: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>;
    fn replaceChild(&self, newchild: &::core::option::Option<IXMLDOMNode>, oldchild: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeChild(&self, childnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn appendChild(&self, newchild: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn hasChildNodes(&self, haschild: *mut i16) -> ::windows::core::Result<()>;
    fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn cloneNode(&self, deep: i16) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypeString(&self, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn text(&self, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Settext(&self, text: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn specified(&self, isspecified: *mut i16) -> ::windows::core::Result<()>;
    fn definition(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetnodeTypedValue(&self, typedvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetdataType(&self, datatypename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn xml(&self, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn transformNode(&self, stylesheet: &::core::option::Option<IXMLDOMNode>, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn selectNodes(&self, querystring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&self, querystring: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn parsed(&self, isparsed: *mut i16) -> ::windows::core::Result<()>;
    fn namespaceURI(&self, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn prefix(&self, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn baseName(&self, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn transformNodeToObject(&self, stylesheet: &::core::option::Option<IXMLDOMNode>, outputobject: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>() -> IXMLDOMNode_Vtbl {
        unsafe extern "system" fn nodeName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).nodeName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn nodeValue<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).nodeValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetnodeValue<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetnodeValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn nodeType<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).nodeType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn parentNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).parentNode() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).childNodes() {
                ::core::result::Result::Ok(ok__) => {
                    *childlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).firstChild() {
                ::core::result::Result::Ok(ok__) => {
                    *firstchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lastChild() {
                ::core::result::Result::Ok(ok__) => {
                    *lastchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previoussibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).previousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *previoussibling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextsibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).nextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    *nextsibling = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributemap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributemap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, refchild: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).insertBefore(::core::mem::transmute(&newchild), ::core::mem::transmute(&refchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outnewchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr, outoldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).replaceChild(::core::mem::transmute(&newchild), ::core::mem::transmute(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outoldchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, oldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).removeChild(::core::mem::transmute(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *oldchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).appendChild(::core::mem::transmute(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *outnewchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haschild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).hasChildNodes(::core::mem::transmute_copy(&haschild)).into()
        }
        unsafe extern "system" fn ownerDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ownerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *xmldomdocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: i16, cloneroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).cloneNode(::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    *cloneroot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).nodeTypeString(::core::mem::transmute_copy(&nodetype)).into()
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).text(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Settext(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn specified<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).specified(::core::mem::transmute_copy(&isspecified)).into()
        }
        unsafe extern "system" fn definition<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definitionnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).definition() {
                ::core::result::Result::Ok(ok__) => {
                    *definitionnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).nodeTypedValue(::core::mem::transmute_copy(&typedvalue)).into()
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetnodeTypedValue(::core::mem::transmute(&typedvalue)).into()
        }
        unsafe extern "system" fn dataType<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).dataType(::core::mem::transmute_copy(&datatypename)).into()
        }
        unsafe extern "system" fn SetdataType<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetdataType(::core::mem::transmute(&datatypename)).into()
        }
        unsafe extern "system" fn xml<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).xml(::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn transformNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).transformNode(::core::mem::transmute(&stylesheet), ::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn selectNodes<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).selectNodes(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).selectSingleNode(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isparsed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).parsed(::core::mem::transmute_copy(&isparsed)).into()
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).namespaceURI(::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn prefix<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).prefix(::core::mem::transmute_copy(&prefixstring)).into()
        }
        unsafe extern "system" fn baseName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).baseName(::core::mem::transmute_copy(&namestring)).into()
        }
        unsafe extern "system" fn transformNodeToObject<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, outputobject: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).transformNodeToObject(::core::mem::transmute(&stylesheet), ::core::mem::transmute(&outputobject)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            nodeName: nodeName::<Identity, Impl, OFFSET>,
            nodeValue: nodeValue::<Identity, Impl, OFFSET>,
            SetnodeValue: SetnodeValue::<Identity, Impl, OFFSET>,
            nodeType: nodeType::<Identity, Impl, OFFSET>,
            parentNode: parentNode::<Identity, Impl, OFFSET>,
            childNodes: childNodes::<Identity, Impl, OFFSET>,
            firstChild: firstChild::<Identity, Impl, OFFSET>,
            lastChild: lastChild::<Identity, Impl, OFFSET>,
            previousSibling: previousSibling::<Identity, Impl, OFFSET>,
            nextSibling: nextSibling::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            insertBefore: insertBefore::<Identity, Impl, OFFSET>,
            replaceChild: replaceChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            appendChild: appendChild::<Identity, Impl, OFFSET>,
            hasChildNodes: hasChildNodes::<Identity, Impl, OFFSET>,
            ownerDocument: ownerDocument::<Identity, Impl, OFFSET>,
            cloneNode: cloneNode::<Identity, Impl, OFFSET>,
            nodeTypeString: nodeTypeString::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            specified: specified::<Identity, Impl, OFFSET>,
            definition: definition::<Identity, Impl, OFFSET>,
            nodeTypedValue: nodeTypedValue::<Identity, Impl, OFFSET>,
            SetnodeTypedValue: SetnodeTypedValue::<Identity, Impl, OFFSET>,
            dataType: dataType::<Identity, Impl, OFFSET>,
            SetdataType: SetdataType::<Identity, Impl, OFFSET>,
            xml: xml::<Identity, Impl, OFFSET>,
            transformNode: transformNode::<Identity, Impl, OFFSET>,
            selectNodes: selectNodes::<Identity, Impl, OFFSET>,
            selectSingleNode: selectSingleNode::<Identity, Impl, OFFSET>,
            parsed: parsed::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            prefix: prefix::<Identity, Impl, OFFSET>,
            baseName: baseName::<Identity, Impl, OFFSET>,
            transformNodeToObject: transformNodeToObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNodeList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>() -> IXMLDOMNodeList_Vtbl {
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *listitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn nextNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    *nextitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            item: item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNotation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>() -> IXMLDOMNotation_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNotation as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self, errorcode: *mut i32) -> ::windows::core::Result<()>;
    fn url(&self, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn reason(&self, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn srcText(&self, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn line(&self, linenumber: *mut i32) -> ::windows::core::Result<()>;
    fn linepos(&self, lineposition: *mut i32) -> ::windows::core::Result<()>;
    fn filepos(&self, fileposition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>() -> IXMLDOMParseError_Vtbl {
        unsafe extern "system" fn errorCode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).errorCode(::core::mem::transmute_copy(&errorcode)).into()
        }
        unsafe extern "system" fn url<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn reason<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reason(::core::mem::transmute_copy(&reasonstring)).into()
        }
        unsafe extern "system" fn srcText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).srcText(::core::mem::transmute_copy(&sourcestring)).into()
        }
        unsafe extern "system" fn line<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).line(::core::mem::transmute_copy(&linenumber)).into()
        }
        unsafe extern "system" fn linepos<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).linepos(::core::mem::transmute_copy(&lineposition)).into()
        }
        unsafe extern "system" fn filepos<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).filepos(::core::mem::transmute_copy(&fileposition)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            reason: reason::<Identity, Impl, OFFSET>,
            srcText: srcText::<Identity, Impl, OFFSET>,
            line: line::<Identity, Impl, OFFSET>,
            linepos: linepos::<Identity, Impl, OFFSET>,
            filepos: filepos::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMParseError_Impl {
    fn errorXPath(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn allErrors(&self) -> ::windows::core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn errorParametersCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>() -> IXMLDOMParseError2_Vtbl {
        unsafe extern "system" fn errorXPath<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).errorXPath() {
                ::core::result::Result::Ok(ok__) => {
                    *xpathexpr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).allErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *allerrors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).errorParameters(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *param1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).errorParametersCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMParseError_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorXPath: errorXPath::<Identity, Impl, OFFSET>,
            allErrors: allErrors::<Identity, Impl, OFFSET>,
            errorParameters: errorParameters::<Identity, Impl, OFFSET>,
            errorParametersCount: errorParametersCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMParseError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseErrorCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn item(&self, index: i32) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn next(&self) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>() -> IXMLDOMParseErrorCollection_Vtbl {
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).next() {
                ::core::result::Result::Ok(ok__) => {
                    *error = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            item: item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMProcessingInstruction_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn target(&self, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn data(&self, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&self, value: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>() -> IXMLDOMProcessingInstruction_Vtbl {
        unsafe extern "system" fn target<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).target(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn data<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).data(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Setdata<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setdata(::core::mem::transmute(&value)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            target: target::<Identity, Impl, OFFSET>,
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn add(&self, namespaceuri: &super::super::super::Foundation::BSTR, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn get(&self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn remove(&self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn namespaceURI(&self, index: i32) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn addCollection(&self, othercollection: &::core::option::Option<IXMLDOMSchemaCollection>) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).add(::core::mem::transmute(&namespaceuri), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn get<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schemanode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).get(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *schemanode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).remove(::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).namespaceURI(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addCollection(::core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            add: add::<Identity, Impl, OFFSET>,
            get: get::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            addCollection: addCollection::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMSchemaCollection_Impl {
    fn validate(&self) -> ::windows::core::Result<()>;
    fn SetvalidateOnLoad(&self, validateonload: i16) -> ::windows::core::Result<()>;
    fn validateOnLoad(&self) -> ::windows::core::Result<i16>;
    fn getSchema(&self, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<ISchema>;
    fn getDeclaration(&self, node: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection2_Vtbl {
        unsafe extern "system" fn validate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).validate().into()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetvalidateOnLoad(::core::mem::transmute_copy(&validateonload)).into()
        }
        unsafe extern "system" fn validateOnLoad<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).validateOnLoad() {
                ::core::result::Result::Ok(ok__) => {
                    *validateonload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getSchema(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *schema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getDeclaration(::core::mem::transmute(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IXMLDOMSchemaCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            validate: validate::<Identity, Impl, OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Identity, Impl, OFFSET>,
            validateOnLoad: validateOnLoad::<Identity, Impl, OFFSET>,
            getSchema: getSchema::<Identity, Impl, OFFSET>,
            getDeclaration: getDeclaration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMSchemaCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSelection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNodeList_Impl {
    fn expr(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setexpr(&self, expression: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn context(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn putref_context(&self, pnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn peekNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn matches(&self, pnode: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNext(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeAll(&self) -> ::windows::core::Result<()>;
    fn clone(&self) -> ::windows::core::Result<IXMLDOMSelection>;
    fn getProperty(&self, name: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setProperty(&self, name: &super::super::super::Foundation::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>() -> IXMLDOMSelection_Vtbl {
        unsafe extern "system" fn expr<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).expr() {
                ::core::result::Result::Ok(ok__) => {
                    *expression = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setexpr(::core::mem::transmute(&expression)).into()
        }
        unsafe extern "system" fn context<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).context() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_context(::core::mem::transmute(&pnode)).into()
        }
        unsafe extern "system" fn peekNode<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).peekNode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).matches(::core::mem::transmute(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).removeNext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeAll().into()
        }
        unsafe extern "system" fn clone<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        Self {
            base: IXMLDOMNodeList_Vtbl::new::<Identity, Impl, OFFSET>(),
            expr: expr::<Identity, Impl, OFFSET>,
            Setexpr: Setexpr::<Identity, Impl, OFFSET>,
            context: context::<Identity, Impl, OFFSET>,
            putref_context: putref_context::<Identity, Impl, OFFSET>,
            peekNode: peekNode::<Identity, Impl, OFFSET>,
            matches: matches::<Identity, Impl, OFFSET>,
            removeNext: removeNext::<Identity, Impl, OFFSET>,
            removeAll: removeAll::<Identity, Impl, OFFSET>,
            clone: clone::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSelection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNodeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMText_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl + IXMLDOMCharacterData_Impl {
    fn splitText(&self, offset: i32) -> ::windows::core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMText_Impl, const OFFSET: isize>() -> IXMLDOMText_Vtbl {
        unsafe extern "system" fn splitText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).splitText(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *righthandtextnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>(), splitText: splitText::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMText as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IXMLDOMNode as ::windows::core::Interface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDSOControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&self, ppdoc: &::core::option::Option<IXMLDOMDocument>) -> ::windows::core::Result<()>;
    fn JavaDSOCompatible(&self, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetJavaDSOCompatible(&self, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn readyState(&self, state: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>() -> IXMLDSOControl_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdoc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetXMLDocument(::core::mem::transmute(&ppdoc)).into()
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).JavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).readyState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            SetXMLDocument: SetXMLDocument::<Identity, Impl, OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Identity, Impl, OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDSOControl as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows::core::Result<IXMLElement>;
    fn fileSize(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileModifiedDate(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn URL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn charset(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setcharset(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn doctype(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn dtdURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>() -> IXMLDocument_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).root() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).URL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *pl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).charset() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows::core::Result<IXMLElement2>;
    fn fileSize(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileModifiedDate(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn URL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetURL(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn charset(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setcharset(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn doctype(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn dtdURL(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement2>;
    fn r#async(&self) -> ::windows::core::Result<i16>;
    fn Setasync(&self, f: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>() -> IXMLDocument2_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).root() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).URL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *pl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).charset() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).version() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).doctype() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppelem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#async() {
                ::core::result::Result::Ok(ok__) => {
                    *pf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: ::windows::core::IUnknownImpl, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, f: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setasync(::core::mem::transmute_copy(&f)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDocument2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SettagName(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parent(&self) -> ::windows::core::Result<IXMLElement>;
    fn setAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn children(&self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows::core::Result<i32>;
    fn text(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Settext(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&self, pchildelem: &::core::option::Option<IXMLElement>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&self, pchildelem: &::core::option::Option<IXMLElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>() -> IXMLElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).tagName() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).children() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *pltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).text() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addChild(::core::mem::transmute(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeChild(::core::mem::transmute(&pchildelem)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SettagName(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn parent(&self) -> ::windows::core::Result<IXMLElement2>;
    fn setAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn children(&self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows::core::Result<i32>;
    fn text(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Settext(&self, p: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&self, pchildelem: &::core::option::Option<IXMLElement2>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&self, pchildelem: &::core::option::Option<IXMLElement2>) -> ::windows::core::Result<()>;
    fn attributes(&self) -> ::windows::core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>() -> IXMLElement2_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).tagName() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).parent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).children() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).r#type() {
                ::core::result::Result::Ok(ok__) => {
                    *pltype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).text() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addChild(::core::mem::transmute(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).removeChild(::core::mem::transmute(&pchildelem)).into()
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElement2 as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElementCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(&self, v: i32) -> ::windows::core::Result<()>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn item(&self, var1: &super::super::super::System::Com::VARIANT, var2: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const OFFSET: isize>() -> IXMLElementCollection_Vtbl {
        unsafe extern "system" fn Setlength<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setlength(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).length() {
                ::core::result::Result::Ok(ok__) => {
                    *p = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).item(::core::mem::transmute(&var1), ::core::mem::transmute(&var2)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setlength: Setlength::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElementCollection as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IXMLError_Impl: Sized {
    fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IXMLError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLError_Impl, const OFFSET: isize>() -> IXMLError_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IXMLError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetErrorInfo(::core::mem::transmute_copy(&perrorreturn)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&self, bstrmethod: &super::super::super::Foundation::BSTR, bstrurl: &super::super::super::Foundation::BSTR, varasync: &super::super::super::System::Com::VARIANT, bstruser: &super::super::super::System::Com::VARIANT, bstrpassword: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &super::super::super::Foundation::BSTR, bstrvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getAllResponseHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn send(&self, varbody: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn abort(&self) -> ::windows::core::Result<()>;
    fn status(&self) -> ::windows::core::Result<i32>;
    fn statusText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: &::core::option::Option<super::super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>() -> IXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn open<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).open(::core::mem::transmute(&bstrmethod), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&varasync), ::core::mem::transmute(&bstruser), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setRequestHeader(::core::mem::transmute(&bstrheader), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getResponseHeader(::core::mem::transmute(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).send(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).status() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).statusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute(&preadystatesink)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            open: open::<Identity, Impl, OFFSET>,
            setRequestHeader: setRequestHeader::<Identity, Impl, OFFSET>,
            getResponseHeader: getResponseHeader::<Identity, Impl, OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Identity, Impl, OFFSET>,
            send: send::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            statusText: statusText::<Identity, Impl, OFFSET>,
            responseXML: responseXML::<Identity, Impl, OFFSET>,
            responseText: responseText::<Identity, Impl, OFFSET>,
            responseBody: responseBody::<Identity, Impl, OFFSET>,
            responseStream: responseStream::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2_Impl: Sized {
    fn Open(&self, pwszmethod: &::windows::core::PCWSTR, pwszurl: &::windows::core::PCWSTR, pstatuscallback: &::core::option::Option<IXMLHTTPRequest2Callback>, pwszusername: &::windows::core::PCWSTR, pwszpassword: &::windows::core::PCWSTR, pwszproxyusername: &::windows::core::PCWSTR, pwszproxypassword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Send(&self, pbody: &::core::option::Option<super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> ::windows::core::Result<u32>;
    fn SetCustomResponseStream(&self, psequentialstream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::Result<()>;
    fn SetRequestHeader(&self, pwszheader: &::windows::core::PCWSTR, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetAllResponseHeaders(&self) -> ::windows::core::Result<*mut u16>;
    fn GetCookie(&self, pwszurl: &::windows::core::PCWSTR, pwszname: &::windows::core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::Result<()>;
    fn GetResponseHeader(&self, pwszheader: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmethod: ::windows::core::PCWSTR, pwszurl: ::windows::core::PCWSTR, pstatuscallback: ::windows::core::RawPtr, pwszusername: ::windows::core::PCWSTR, pwszpassword: ::windows::core::PCWSTR, pwszproxyusername: ::windows::core::PCWSTR, pwszproxypassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&pwszmethod), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pstatuscallback), ::core::mem::transmute(&pwszusername), ::core::mem::transmute(&pwszpassword), ::core::mem::transmute(&pwszproxyusername), ::core::mem::transmute(&pwszproxypassword)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: ::windows::core::RawPtr, cbbody: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Send(::core::mem::transmute(&pbody), ::core::mem::transmute_copy(&cbbody)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn SetCookie<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetCookie(::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookiestate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psequentialstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustomResponseStream(::core::mem::transmute(&psequentialstream)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&eproperty), ::core::mem::transmute_copy(&ullvalue)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRequestHeader(::core::mem::transmute(&pwszheader), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwszname: ::windows::core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCookie(::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResponseHeader(::core::mem::transmute(&pwszheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            SetCookie: SetCookie::<Identity, Impl, OFFSET>,
            SetCustomResponseStream: SetCustomResponseStream::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2Callback_Impl: Sized {
    fn OnRedirect(&self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, pwszredirecturl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnHeadersAvailable(&self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnDataAvailable(&self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, presponsestream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnResponseReceived(&self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, presponsestream: &::core::option::Option<super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnError(&self, pxhr: &::core::option::Option<IXMLHTTPRequest2>, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2Callback_Vtbl {
        unsafe extern "system" fn OnRedirect<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, pwszredirecturl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnRedirect(::core::mem::transmute(&pxhr), ::core::mem::transmute(&pwszredirecturl)).into()
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwstatus: u32, pwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHeadersAvailable(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute(&pwszstatus)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDataAvailable(::core::mem::transmute(&pxhr), ::core::mem::transmute(&presponsestream)).into()
        }
        unsafe extern "system" fn OnResponseReceived<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnResponseReceived(::core::mem::transmute(&pxhr), ::core::mem::transmute(&presponsestream)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnError(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, Impl, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3_Impl: Sized + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3_Vtbl {
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientCertificate(::core::mem::transmute_copy(&cbclientcertificatehash), ::core::mem::transmute_copy(&pbclientcertificatehash), ::core::mem::transmute(&pwszpin)).into()
        }
        Self { base: IXMLHTTPRequest2_Vtbl::new::<Identity, Impl, OFFSET>(), SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3Callback_Impl: Sized + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&self, pxhr: &::core::option::Option<IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::Result<()>;
    fn OnClientCertificateRequested(&self, pxhr: &::core::option::Option<IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3Callback_Vtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnServerCertificateReceived(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&dwcertificateerrors), ::core::mem::transmute_copy(&cservercertificatechain), ::core::mem::transmute_copy(&rgservercertificatechain)).into()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnClientCertificateRequested(::core::mem::transmute(&pxhr), ::core::mem::transmute_copy(&cissuerlist), ::core::mem::transmute_copy(&rgpwszissuerlist)).into()
        }
        Self {
            base: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, Impl, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as ::windows::core::Interface>::IID || iid == &<IXMLHTTPRequest2Callback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHttpRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&self, bstrmethod: &super::super::super::Foundation::BSTR, bstrurl: &super::super::super::Foundation::BSTR, varasync: &super::super::super::System::Com::VARIANT, bstruser: &super::super::super::System::Com::VARIANT, bstrpassword: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &super::super::super::Foundation::BSTR, bstrvalue: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn getAllResponseHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn send(&self, varbody: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn abort(&self) -> ::windows::core::Result<()>;
    fn status(&self) -> ::windows::core::Result<i32>;
    fn statusText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: &::core::option::Option<super::super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHttpRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>() -> IXMLHttpRequest_Vtbl {
        unsafe extern "system" fn open<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).open(::core::mem::transmute(&bstrmethod), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&varasync), ::core::mem::transmute(&bstruser), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setRequestHeader(::core::mem::transmute(&bstrheader), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getResponseHeader(::core::mem::transmute(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).send(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).abort().into()
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).status() {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).statusText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows::core::IUnknownImpl, Impl: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setonreadystatechange(::core::mem::transmute(&preadystatesink)).into()
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            open: open::<Identity, Impl, OFFSET>,
            setRequestHeader: setRequestHeader::<Identity, Impl, OFFSET>,
            getResponseHeader: getResponseHeader::<Identity, Impl, OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Identity, Impl, OFFSET>,
            send: send::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            statusText: statusText::<Identity, Impl, OFFSET>,
            responseXML: responseXML::<Identity, Impl, OFFSET>,
            responseText: responseText::<Identity, Impl, OFFSET>,
            responseBody: responseBody::<Identity, Impl, OFFSET>,
            responseStream: responseStream::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHttpRequest as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLProcessor_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(&self, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn input(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn ownerTemplate(&self) -> ::windows::core::Result<IXSLTemplate>;
    fn setStartMode(&self, mode: &super::super::super::Foundation::BSTR, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn startMode(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn startModeURI(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn Setoutput(&self, output: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn transform(&self) -> ::windows::core::Result<i16>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn addParameter(&self, basename: &super::super::super::Foundation::BSTR, parameter: &super::super::super::System::Com::VARIANT, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn addObject(&self, obj: &::core::option::Option<super::super::super::System::Com::IDispatch>, namespaceuri: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>() -> IXSLProcessor_Vtbl {
        unsafe extern "system" fn Setinput<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setinput(::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn input<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).input() {
                ::core::result::Result::Ok(ok__) => {
                    *pvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ownerTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *pptemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).setStartMode(::core::mem::transmute(&mode), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn startMode<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).startMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).startModeURI() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Setoutput(::core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).output() {
                ::core::result::Result::Ok(ok__) => {
                    *poutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdone: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).transform() {
                ::core::result::Result::Ok(ok__) => {
                    *pdone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).reset().into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).readyState() {
                ::core::result::Result::Ok(ok__) => {
                    *preadystate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, parameter: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addParameter(::core::mem::transmute(&basename), ::core::mem::transmute(&parameter), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn addObject<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).addObject(::core::mem::transmute(&obj), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows::core::IUnknownImpl, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setinput: Setinput::<Identity, Impl, OFFSET>,
            input: input::<Identity, Impl, OFFSET>,
            ownerTemplate: ownerTemplate::<Identity, Impl, OFFSET>,
            setStartMode: setStartMode::<Identity, Impl, OFFSET>,
            startMode: startMode::<Identity, Impl, OFFSET>,
            startModeURI: startModeURI::<Identity, Impl, OFFSET>,
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            transform: transform::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            addParameter: addParameter::<Identity, Impl, OFFSET>,
            addObject: addObject::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLProcessor as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(&self, stylesheet: &::core::option::Option<IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn createProcessor(&self) -> ::windows::core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplate_Impl, const OFFSET: isize>() -> IXSLTemplate_Vtbl {
        unsafe extern "system" fn putref_stylesheet<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_stylesheet(::core::mem::transmute(&stylesheet)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    *stylesheet = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Identity: ::windows::core::IUnknownImpl, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).createProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprocessor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_stylesheet: putref_stylesheet::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
            createProcessor: createProcessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLTemplate as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXTLRuntime_Impl: Sized + super::super::super::System::Com::IDispatch_Impl + IXMLDOMNode_Impl {
    fn uniqueID(&self, pnode: &::core::option::Option<IXMLDOMNode>, pid: *mut i32) -> ::windows::core::Result<()>;
    fn depth(&self, pnode: &::core::option::Option<IXMLDOMNode>, pdepth: *mut i32) -> ::windows::core::Result<()>;
    fn childNumber(&self, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn ancestorChildNumber(&self, bstrnodename: &super::super::super::Foundation::BSTR, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn absoluteChildNumber(&self, pnode: &::core::option::Option<IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn formatIndex(&self, lindex: i32, bstrformat: &super::super::super::Foundation::BSTR, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatNumber(&self, dblnumber: f64, bstrformat: &super::super::super::Foundation::BSTR, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatDate(&self, vardate: &super::super::super::System::Com::VARIANT, bstrformat: &super::super::super::Foundation::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn formatTime(&self, vartime: &super::super::super::System::Com::VARIANT, bstrformat: &super::super::super::Foundation::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>() -> IXTLRuntime_Vtbl {
        unsafe extern "system" fn uniqueID<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).uniqueID(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn depth<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pdepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).depth(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pdepth)).into()
        }
        unsafe extern "system" fn childNumber<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).childNumber(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnodename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ancestorChildNumber(::core::mem::transmute(&bstrnodename), ::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).absoluteChildNumber(::core::mem::transmute(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn formatIndex<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).formatIndex(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatNumber<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).formatNumber(::core::mem::transmute_copy(&dblnumber), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatDate<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardate: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).formatDate(::core::mem::transmute(&vardate), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatTime<Identity: ::windows::core::IUnknownImpl, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartime: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).formatTime(::core::mem::transmute(&vartime), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        Self {
            base: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            uniqueID: uniqueID::<Identity, Impl, OFFSET>,
            depth: depth::<Identity, Impl, OFFSET>,
            childNumber: childNumber::<Identity, Impl, OFFSET>,
            ancestorChildNumber: ancestorChildNumber::<Identity, Impl, OFFSET>,
            absoluteChildNumber: absoluteChildNumber::<Identity, Impl, OFFSET>,
            formatIndex: formatIndex::<Identity, Impl, OFFSET>,
            formatNumber: formatNumber::<Identity, Impl, OFFSET>,
            formatDate: formatDate::<Identity, Impl, OFFSET>,
            formatTime: formatTime::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: XMLDOMDocumentEvents_Impl, const OFFSET: isize>() -> XMLDOMDocumentEvents_Vtbl {
        Self { base: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as ::windows::core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
