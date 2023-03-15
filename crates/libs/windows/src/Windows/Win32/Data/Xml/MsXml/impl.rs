#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(&self, struri: &::windows::core::BSTR, strlocalname: &::windows::core::BSTR, strqname: &::windows::core::BSTR, strtype: &::windows::core::BSTR, strvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn addAttributeFromIndex(&self, varatts: &super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::Result<()>;
    fn clear(&self) -> ::windows::core::Result<()>;
    fn removeAttribute(&self, nindex: i32) -> ::windows::core::Result<()>;
    fn setAttribute(&self, nindex: i32, struri: &::windows::core::BSTR, strlocalname: &::windows::core::BSTR, strqname: &::windows::core::BSTR, strtype: &::windows::core::BSTR, strvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setAttributes(&self, varatts: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setLocalName(&self, nindex: i32, strlocalname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setQName(&self, nindex: i32, strqname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setType(&self, nindex: i32, strtype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setURI(&self, nindex: i32, struri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn setValue(&self, nindex: i32, strvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXAttributes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>() -> IMXAttributes_Vtbl {
        unsafe extern "system" fn addAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addAttribute(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Com::VARIANT, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addAttributeFromIndex(::core::mem::transmute(&varatts), ::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.clear().into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn setAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttributes(::core::mem::transmute(&varatts)).into()
        }
        unsafe extern "system" fn setLocalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strlocalname)).into()
        }
        unsafe extern "system" fn setQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strqname)).into()
        }
        unsafe extern "system" fn setType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strtype)).into()
        }
        unsafe extern "system" fn setURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri)).into()
        }
        unsafe extern "system" fn setValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMXAttributes as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMXNamespaceManager_Impl: Sized {
    fn putAllowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getAllowOverride(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn pushContext(&self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&self, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn popContext(&self) -> ::windows::core::Result<()>;
    fn declarePrefix(&self, prefix: &::windows::core::PCWSTR, namespaceuri: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getPrefix(&self, pwsznamespaceuri: &::windows::core::PCWSTR, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::Result<()>;
    fn getURI(&self, pwchprefix: &::windows::core::PCWSTR, pcontextnode: ::core::option::Option<&IXMLDOMNode>, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IMXNamespaceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>() -> IMXNamespaceManager_Vtbl {
        unsafe extern "system" fn putAllowOverride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putAllowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn getAllowOverride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAllowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushNodeContext(::windows::core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::windows::core::PCWSTR, namespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getDeclaredPrefix(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getPrefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: ::windows::core::PCWSTR, nindex: i32, pwchprefix: ::windows::core::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getPrefix(::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, pcontextnode: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getURI(::core::mem::transmute(&pwchprefix), ::windows::core::from_raw_borrowed(&pcontextnode), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMXNamespaceManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXNamespacePrefixes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXNamespacePrefixes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>() -> IMXNamespacePrefixes_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXReaderControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn abort(&self) -> ::windows::core::Result<()>;
    fn resume(&self) -> ::windows::core::Result<()>;
    fn suspend(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXReaderControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>() -> IMXReaderControl_Vtbl {
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resume().into()
        }
        unsafe extern "system" fn suspend<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.suspend().into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            abort: abort::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            suspend: suspend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXReaderControl as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXSchemaDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(&self, oschemaelement: ::core::option::Option<&ISchemaElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXSchemaDeclHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>() -> IMXSchemaDeclHandler_Vtbl {
        unsafe extern "system" fn schemaElementDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oschemaelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.schemaElementDecl(::windows::core::from_raw_borrowed(&oschemaelement)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            schemaElementDecl: schemaElementDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXWriter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(&self, vardestination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn Setencoding(&self, strencoding: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn encoding(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetbyteOrderMark(&self, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn byteOrderMark(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setindent(&self, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn indent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setstandalone(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn standalone(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetomitXMLDeclaration(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn omitXMLDeclaration(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setversion(&self, strversion: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetdisableOutputEscaping(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn disableOutputEscaping(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn flush(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXWriter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>() -> IMXWriter_Vtbl {
        unsafe extern "system" fn Setoutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setoutput(::core::mem::transmute(&vardestination)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.output() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vardestination, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setencoding(::core::mem::transmute(&strencoding)).into()
        }
        unsafe extern "system" fn encoding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.encoding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strencoding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbyteOrderMark(::core::mem::transmute_copy(&fwritebyteordermark)).into()
        }
        unsafe extern "system" fn byteOrderMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.byteOrderMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fwritebyteordermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setindent(::core::mem::transmute_copy(&findentmode)).into()
        }
        unsafe extern "system" fn indent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.indent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(findentmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setstandalone(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn standalone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.standalone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetomitXMLDeclaration(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.omitXMLDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setversion(::core::mem::transmute(&strversion)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdisableOutputEscaping(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.disableOutputEscaping() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.flush().into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMXWriter as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &::windows::core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getProperty(&self, strname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, strname: &::windows::core::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_entityResolver(&self, oresolver: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn contentHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_contentHandler(&self, ohandler: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn dtdHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_dtdHandler(&self, ohandler: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn errorHandler(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn putref_errorHandler(&self, ohandler: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>() -> IMXXMLFilter_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_entityResolver(::windows::core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_contentHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_dtdHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_errorHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMXXMLFilter as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
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
impl ::windows::core::RuntimeName for ISAXAttributes {}
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>() -> ISAXAttributes_Vtbl {
        unsafe extern "system" fn getLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)).into()
        }
        unsafe extern "system" fn getQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getTypeFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getTypeFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows::core::PCWSTR, cchuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValueFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows::core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValueFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISAXAttributes as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXContentHandler_Impl: Sized {
    fn putDocumentLocator(&self, plocator: ::core::option::Option<&ISAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&self) -> ::windows::core::Result<()>;
    fn endDocument(&self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&self, pwchprefix: &::windows::core::PCWSTR, cchprefix: i32, pwchuri: &::windows::core::PCWSTR, cchuri: i32) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&self, pwchprefix: &::windows::core::PCWSTR, cchprefix: i32) -> ::windows::core::Result<()>;
    fn startElement(&self, pwchnamespaceuri: &::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, pwchqname: &::windows::core::PCWSTR, cchqname: i32, pattributes: ::core::option::Option<&ISAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&self, pwchnamespaceuri: &::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows::core::PCWSTR, cchlocalname: i32, pwchqname: &::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::Result<()>;
    fn characters(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
    fn processingInstruction(&self, pwchtarget: &::windows::core::PCWSTR, cchtarget: i32, pwchdata: &::windows::core::PCWSTR, cchdata: i32) -> ::windows::core::Result<()>;
    fn skippedEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISAXContentHandler {}
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>() -> ISAXContentHandler_Vtbl {
        unsafe extern "system" fn putDocumentLocator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putDocumentLocator(::windows::core::from_raw_borrowed(&plocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32, pwchuri: ::windows::core::PCWSTR, cchuri: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix), ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows::core::PCWSTR, cchprefix: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32, pattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::windows::core::from_raw_borrowed(&pattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows::core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows::core::PCWSTR, cchlocalname: i32, pwchqname: ::windows::core::PCWSTR, cchqname: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.characters(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWhitespace(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchtarget: ::windows::core::PCWSTR, cchtarget: i32, pwchdata: ::windows::core::PCWSTR, cchdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.processingInstruction(::core::mem::transmute(&pwchtarget), ::core::mem::transmute_copy(&cchtarget), ::core::mem::transmute(&pwchdata), ::core::mem::transmute_copy(&cchdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.skippedEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISAXContentHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXDTDHandler_Impl: Sized {
    fn notationDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32, pwchnotationname: &::windows::core::PCWSTR, cchnotationname: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISAXDTDHandler {}
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>() -> ISAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.notationDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32, pwchnotationname: ::windows::core::PCWSTR, cchnotationname: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.unparsedEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid), ::core::mem::transmute(&pwchnotationname), ::core::mem::transmute_copy(&cchnotationname)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDTDHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXDeclHandler_Impl: Sized {
    fn elementDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchmodel: &::windows::core::PCWSTR, cchmodel: i32) -> ::windows::core::Result<()>;
    fn attributeDecl(&self, pwchelementname: &::windows::core::PCWSTR, cchelementname: i32, pwchattributename: &::windows::core::PCWSTR, cchattributename: i32, pwchtype: &::windows::core::PCWSTR, cchtype: i32, pwchvaluedefault: &::windows::core::PCWSTR, cchvaluedefault: i32, pwchvalue: &::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchvalue: &::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISAXDeclHandler {}
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>() -> ISAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchmodel: ::windows::core::PCWSTR, cchmodel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.elementDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchmodel), ::core::mem::transmute_copy(&cchmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchelementname: ::windows::core::PCWSTR, cchelementname: i32, pwchattributename: ::windows::core::PCWSTR, cchattributename: i32, pwchtype: ::windows::core::PCWSTR, cchtype: i32, pwchvaluedefault: ::windows::core::PCWSTR, cchvaluedefault: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeDecl(::core::mem::transmute(&pwchelementname), ::core::mem::transmute_copy(&cchelementname), ::core::mem::transmute(&pwchattributename), ::core::mem::transmute_copy(&cchattributename), ::core::mem::transmute(&pwchtype), ::core::mem::transmute_copy(&cchtype), ::core::mem::transmute(&pwchvaluedefault), ::core::mem::transmute_copy(&cchvaluedefault), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchvalue: ::windows::core::PCWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.internalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.externalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXDeclHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXEntityResolver_Impl: Sized {
    fn resolveEntity(&self, pwchpublicid: &::windows::core::PCWSTR, pwchsystemid: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISAXEntityResolver {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>() -> ISAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchpublicid: ::windows::core::PCWSTR, pwchsystemid: ::windows::core::PCWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.resolveEntity(::core::mem::transmute(&pwchpublicid), ::core::mem::transmute(&pwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarinput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXEntityResolver as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXErrorHandler_Impl: Sized {
    fn error(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn fatalError(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn ignorableWarning(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISAXErrorHandler {}
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>() -> ISAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.error(::windows::core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fatalError(::windows::core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows::core::PCWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWarning(::windows::core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXErrorHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXLexicalHandler_Impl: Sized {
    fn startDTD(&self, pwchname: &::windows::core::PCWSTR, cchname: i32, pwchpublicid: &::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::Result<()>;
    fn endDTD(&self) -> ::windows::core::Result<()>;
    fn startEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn endEntity(&self, pwchname: &::windows::core::PCWSTR, cchname: i32) -> ::windows::core::Result<()>;
    fn startCDATA(&self) -> ::windows::core::Result<()>;
    fn endCDATA(&self) -> ::windows::core::Result<()>;
    fn comment(&self, pwchchars: &::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISAXLexicalHandler {}
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>() -> ISAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32, pwchpublicid: ::windows::core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows::core::PCWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDTD(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows::core::PCWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.comment(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISAXLexicalHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXLocator_Impl: Sized {
    fn getColumnNumber(&self) -> ::windows::core::Result<i32>;
    fn getLineNumber(&self) -> ::windows::core::Result<i32>;
    fn getPublicId(&self) -> ::windows::core::Result<*mut u16>;
    fn getSystemId(&self) -> ::windows::core::Result<*mut u16>;
}
impl ::windows::core::RuntimeName for ISAXLocator {}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>() -> ISAXLocator_Vtbl {
        unsafe extern "system" fn getColumnNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getColumnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncolumn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getPublicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchpublicid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsystemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getColumnNumber: getColumnNumber::<Identity, Impl, OFFSET>,
            getLineNumber: getLineNumber::<Identity, Impl, OFFSET>,
            getPublicId: getPublicId::<Identity, Impl, OFFSET>,
            getSystemId: getSystemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXLocator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLFilter_Impl: Sized + ISAXXMLReader_Impl {
    fn getParent(&self) -> ::windows::core::Result<ISAXXMLReader>;
    fn putParent(&self, preader: ::core::option::Option<&ISAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISAXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>() -> ISAXXMLFilter_Vtbl {
        unsafe extern "system" fn getParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putParent(::windows::core::from_raw_borrowed(&preader)).into()
        }
        Self {
            base__: ISAXXMLReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            getParent: getParent::<Identity, Impl, OFFSET>,
            putParent: putParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISAXXMLFilter as ::windows::core::ComInterface>::IID || iid == &<ISAXXMLReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISAXXMLReader_Impl: Sized {
    fn getFeature(&self, pwchname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, pwchname: &::windows::core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getProperty(&self, pwchname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, pwchname: &::windows::core::PCWSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getEntityResolver(&self) -> ::windows::core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&self, presolver: ::core::option::Option<&ISAXEntityResolver>) -> ::windows::core::Result<()>;
    fn getContentHandler(&self) -> ::windows::core::Result<ISAXContentHandler>;
    fn putContentHandler(&self, phandler: ::core::option::Option<&ISAXContentHandler>) -> ::windows::core::Result<()>;
    fn getDTDHandler(&self) -> ::windows::core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&self, phandler: ::core::option::Option<&ISAXDTDHandler>) -> ::windows::core::Result<()>;
    fn getErrorHandler(&self) -> ::windows::core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&self, phandler: ::core::option::Option<&ISAXErrorHandler>) -> ::windows::core::Result<()>;
    fn getBaseURL(&self) -> ::windows::core::Result<*mut u16>;
    fn putBaseURL(&self, pwchbaseurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn getSecureBaseURL(&self) -> ::windows::core::Result<*mut u16>;
    fn putSecureBaseURL(&self, pwchsecurebaseurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&self, pwchurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISAXXMLReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>() -> ISAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvfvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&vfvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows::core::PCWSTR, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&pwchname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn getEntityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEntityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresolver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putEntityResolver(::windows::core::from_raw_borrowed(&presolver)).into()
        }
        unsafe extern "system" fn getContentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getContentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putContentHandler(::windows::core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getDTDHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDTDHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putDTDHandler(::windows::core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getErrorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getErrorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putErrorHandler(::windows::core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchbaseurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putBaseURL(::core::mem::transmute(&pwchbaseurl)).into()
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSecureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putSecureBaseURL(::core::mem::transmute(&pwchsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parseURL(::core::mem::transmute(&pwchurl)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<ISAXXMLReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchema_Impl: Sized + ISchemaItem_Impl {
    fn targetNamespace(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn version(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn types(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn elements(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn attributeGroups(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn modelGroups(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn notations(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn schemaLocations(&self) -> ::windows::core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchema {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchema_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>() -> ISchema_Vtbl {
        unsafe extern "system" fn targetNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, types: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.types() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(types, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.elements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elements, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributeGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.modelGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modelgroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schemalocations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schemaLocations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemalocations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISchema as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAny_Impl: Sized + ISchemaParticle_Impl {
    fn namespaces(&self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn processContents(&self) -> ::windows::core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaAny {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>() -> ISchemaAny_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.processContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(processcontents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            processContents: processContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAny as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttribute_Impl: Sized + ISchemaItem_Impl {
    fn r#type(&self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fixedValue(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn r#use(&self) -> ::windows::core::Result<SCHEMAUSE>;
    fn isReference(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>() -> ISchemaAttribute_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#use() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#use, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            r#use: r#use::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttribute as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaAttributeGroup_Impl: Sized + ISchemaItem_Impl {
    fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaAttributeGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>() -> ISchemaAttributeGroup_Vtbl {
        unsafe extern "system" fn anyAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaComplexType_Impl: Sized + ISchemaType_Impl {
    fn isAbstract(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn anyAttribute(&self) -> ::windows::core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn contentType(&self) -> ::windows::core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&self) -> ::windows::core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaComplexType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>() -> ISchemaComplexType_Vtbl {
        unsafe extern "system" fn isAbstract<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentmodel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentmodel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.prohibitedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibited, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaType_Vtbl::new::<Identity, Impl, OFFSET>(),
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            contentType: contentType::<Identity, Impl, OFFSET>,
            contentModel: contentModel::<Identity, Impl, OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaComplexType as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID || iid == &<ISchemaType as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaElement_Impl: Sized + ISchemaParticle_Impl {
    fn r#type(&self) -> ::windows::core::Result<ISchemaType>;
    fn scope(&self) -> ::windows::core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fixedValue(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn isNillable(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn identityConstraints(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&self) -> ::windows::core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn isReference(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>() -> ISchemaElement_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nillable: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isNillable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nillable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.identityConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(constraints, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.substitutionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.substitutionGroupExclusions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exclusions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.disallowedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disallowed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISchemaElement as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaIdentityConstraint_Impl: Sized + ISchemaItem_Impl {
    fn selector(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fields(&self) -> ::windows::core::Result<ISchemaStringCollection>;
    fn referencedKey(&self) -> ::windows::core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaIdentityConstraint {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>() -> ISchemaIdentityConstraint_Vtbl {
        unsafe extern "system" fn selector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fields: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fields() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fields, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.referencedKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            selector: selector::<Identity, Impl, OFFSET>,
            fields: fields::<Identity, Impl, OFFSET>,
            referencedKey: referencedKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItem_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn namespaceURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn schema(&self) -> ::windows::core::Result<ISchema>;
    fn id(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn itemType(&self) -> ::windows::core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&self) -> ::windows::core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&self, annotationsink: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>() -> ISchemaItem_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaceURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schema() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unhandledAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.writeAnnotation(::windows::core::from_raw_borrowed(&annotationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iswritten, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISchemaItem as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaItemCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows::core::Result<ISchemaItem>;
    fn itemByName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn itemByQName(&self, name: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<ISchemaItem>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaItemCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>() -> ISchemaItemCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemByQName(::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            itemByName: itemByName::<Identity, Impl, OFFSET>,
            itemByQName: itemByQName::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaItemCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaModelGroup_Impl: Sized + ISchemaParticle_Impl {
    fn particles(&self) -> ::windows::core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaModelGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>() -> ISchemaModelGroup_Vtbl {
        unsafe extern "system" fn particles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, particles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.particles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(particles, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(), particles: particles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaModelGroup as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaNotation_Impl: Sized + ISchemaItem_Impl {
    fn systemIdentifier(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn publicIdentifier(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaNotation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>() -> ISchemaNotation_Vtbl {
        unsafe extern "system" fn systemIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            systemIdentifier: systemIdentifier::<Identity, Impl, OFFSET>,
            publicIdentifier: publicIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaNotation as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaParticle_Impl: Sized + ISchemaItem_Impl {
    fn minOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn maxOccurs(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaParticle {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>() -> ISchemaParticle_Vtbl {
        unsafe extern "system" fn minOccurs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minoccurs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxoccurs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            minOccurs: minOccurs::<Identity, Impl, OFFSET>,
            maxOccurs: maxOccurs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaParticle as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaStringCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISchemaStringCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>() -> ISchemaStringCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchemaStringCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchemaType_Impl: Sized + ISchemaItem_Impl {
    fn baseTypes(&self) -> ::windows::core::Result<ISchemaItemCollection>;
    fn r#final(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&self) -> ::windows::core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&self) -> ::windows::core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn minExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn minInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn maxExclusive(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn maxInclusive(&self) -> ::windows::core::Result<::windows::core::BSTR>;
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
impl ::windows::core::RuntimeName for ISchemaType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>() -> ISchemaType_Vtbl {
        unsafe extern "system" fn baseTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetypes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(basetypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#final() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#final, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.variety() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variety, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.derivedBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedby, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, valid: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isValid(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minexclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minexclusive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mininclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mininclusive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxexclusive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxinclusive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.totalDigits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(totaldigits, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fractiondigits, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumeration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.enumeration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumeration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.whitespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(whitespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterns: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.patterns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patterns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISchemaType as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<ISchemaItem as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest_Impl: Sized + IXMLHTTPRequest_Impl {
    fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::Result<()>;
    fn waitForResponse(&self, timeoutinseconds: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IServerXMLHTTPRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn setTimeouts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setTimeouts(::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn waitForResponse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinseconds: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.waitForResponse(::core::mem::transmute(&timeoutinseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setOption(::core::mem::transmute_copy(&option), ::core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setTimeouts: setTimeouts::<Identity, Impl, OFFSET>,
            waitForResponse: waitForResponse::<Identity, Impl, OFFSET>,
            getOption: getOption::<Identity, Impl, OFFSET>,
            setOption: setOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IServerXMLHTTPRequest2_Impl: Sized + IServerXMLHTTPRequest_Impl {
    fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::super::super::System::Com::VARIANT, varbypasslist: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setProxyCredentials(&self, bstrusername: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IServerXMLHTTPRequest2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn setProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::super::super::System::Com::VARIANT, varbypasslist: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxy(::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&varproxyserver), ::core::mem::transmute(&varbypasslist)).into()
        }
        unsafe extern "system" fn setProxyCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxyCredentials(::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into()
        }
        Self {
            base__: IServerXMLHTTPRequest_Vtbl::new::<Identity, Impl, OFFSET>(),
            setProxy: setProxy::<Identity, Impl, OFFSET>,
            setProxyCredentials: setProxyCredentials::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLHTTPRequest as ::windows::core::ComInterface>::IID || iid == &<IServerXMLHTTPRequest as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBMXNamespaceManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn allowOverride(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn pushContext(&self) -> ::windows::core::Result<()>;
    fn pushNodeContext(&self, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn popContext(&self) -> ::windows::core::Result<()>;
    fn declarePrefix(&self, prefix: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getDeclaredPrefixes(&self) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&self, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IMXNamespacePrefixes>;
    fn getURI(&self, prefix: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn getURIFromNode(&self, strprefix: &::windows::core::BSTR, contextnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBMXNamespaceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>() -> IVBMXNamespaceManager_Vtbl {
        unsafe extern "system" fn SetallowOverride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetallowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn allowOverride<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushNodeContext(::windows::core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDeclaredPrefixes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getPrefixes(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURI(::core::mem::transmute(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: ::std::mem::MaybeUninit<::windows::core::BSTR>, contextnode: *mut ::core::ffi::c_void, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURIFromNode(::core::mem::transmute(&strprefix), ::windows::core::from_raw_borrowed(&contextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IVBMXNamespaceManager as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn length(&self) -> ::windows::core::Result<i32>;
    fn getURI(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getLocalName(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getQName(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getIndexFromName(&self, struri: &::windows::core::BSTR, strlocalname: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
    fn getIndexFromQName(&self, strqname: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
    fn getType(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getTypeFromName(&self, struri: &::windows::core::BSTR, strlocalname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getTypeFromQName(&self, strqname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getValue(&self, nindex: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getValueFromName(&self, struri: &::windows::core::BSTR, strlocalname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getValueFromQName(&self, strqname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXAttributes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>() -> IVBSAXAttributes_Vtbl {
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURI(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(struri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLocalName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strlocalname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getQName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strqname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getType(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getTypeFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getTypeFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValue(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValueFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValueFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IVBSAXAttributes as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXContentHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(&self, olocator: ::core::option::Option<&IVBSAXLocator>) -> ::windows::core::Result<()>;
    fn startDocument(&self) -> ::windows::core::Result<()>;
    fn endDocument(&self) -> ::windows::core::Result<()>;
    fn startPrefixMapping(&self, strprefix: *mut ::windows::core::BSTR, struri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn endPrefixMapping(&self, strprefix: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn startElement(&self, strnamespaceuri: *mut ::windows::core::BSTR, strlocalname: *mut ::windows::core::BSTR, strqname: *mut ::windows::core::BSTR, oattributes: ::core::option::Option<&IVBSAXAttributes>) -> ::windows::core::Result<()>;
    fn endElement(&self, strnamespaceuri: *mut ::windows::core::BSTR, strlocalname: *mut ::windows::core::BSTR, strqname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn characters(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ignorableWhitespace(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn processingInstruction(&self, strtarget: *mut ::windows::core::BSTR, strdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn skippedEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXContentHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>() -> IVBSAXContentHandler_Vtbl {
        unsafe extern "system" fn putref_documentLocator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_documentLocator(::windows::core::from_raw_borrowed(&olocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, struri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startPrefixMapping(::core::mem::transmute_copy(&strprefix), ::core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endPrefixMapping(::core::mem::transmute_copy(&strprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, oattributes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::windows::core::from_raw_borrowed(&oattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.characters(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWhitespace(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtarget: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strdata: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.processingInstruction(::core::mem::transmute_copy(&strtarget), ::core::mem::transmute_copy(&strdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.skippedEntity(::core::mem::transmute_copy(&strname)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IVBSAXContentHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDTDHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn unparsedEntityDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR, strnotationname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXDTDHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>() -> IVBSAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.notationDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strnotationname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.unparsedEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&strnotationname)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(&self, strname: *mut ::windows::core::BSTR, strmodel: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn attributeDecl(&self, strelementname: *mut ::windows::core::BSTR, strattributename: *mut ::windows::core::BSTR, strtype: *mut ::windows::core::BSTR, strvaluedefault: *mut ::windows::core::BSTR, strvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn internalEntityDecl(&self, strname: *mut ::windows::core::BSTR, strvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn externalEntityDecl(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXDeclHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>() -> IVBSAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strmodel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.elementDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strelementname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strattributename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvaluedefault: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeDecl(::core::mem::transmute_copy(&strelementname), ::core::mem::transmute_copy(&strattributename), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvaluedefault), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.internalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.externalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXEntityResolver_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(&self, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXEntityResolver {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>() -> IVBSAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resolveEntity(::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&varinput)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            resolveEntity: resolveEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXErrorHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn error(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn fatalError(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
    fn ignorableWarning(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows::core::BSTR, nerrorcode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXErrorHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>() -> IVBSAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.error(::windows::core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fatalError(::windows::core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWarning(::windows::core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLexicalHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(&self, strname: *mut ::windows::core::BSTR, strpublicid: *mut ::windows::core::BSTR, strsystemid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn endDTD(&self) -> ::windows::core::Result<()>;
    fn startEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn endEntity(&self, strname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn startCDATA(&self) -> ::windows::core::Result<()>;
    fn endCDATA(&self) -> ::windows::core::Result<()>;
    fn comment(&self, strchars: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXLexicalHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>() -> IVBSAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDTD(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.comment(::core::mem::transmute_copy(&strchars)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IVBSAXLexicalHandler as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXLocator_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(&self) -> ::windows::core::Result<i32>;
    fn lineNumber(&self) -> ::windows::core::Result<i32>;
    fn publicId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn systemId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>() -> IVBSAXLocator_Vtbl {
        unsafe extern "system" fn columnNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.columnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ncolumn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strpublicid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsystemid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsystemid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            columnNumber: columnNumber::<Identity, Impl, OFFSET>,
            lineNumber: lineNumber::<Identity, Impl, OFFSET>,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXLocator as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn parent(&self) -> ::windows::core::Result<IVBSAXXMLReader>;
    fn putref_parent(&self, oreader: ::core::option::Option<&IVBSAXXMLReader>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>() -> IVBSAXXMLFilter_Vtbl {
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oreader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_parent(::windows::core::from_raw_borrowed(&oreader)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            parent: parent::<Identity, Impl, OFFSET>,
            putref_parent: putref_parent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IVBSAXXMLReader_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &::windows::core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn getProperty(&self, strname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putProperty(&self, strname: &::windows::core::BSTR, varvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn entityResolver(&self) -> ::windows::core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&self, oresolver: ::core::option::Option<&IVBSAXEntityResolver>) -> ::windows::core::Result<()>;
    fn contentHandler(&self) -> ::windows::core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&self, ohandler: ::core::option::Option<&IVBSAXContentHandler>) -> ::windows::core::Result<()>;
    fn dtdHandler(&self) -> ::windows::core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&self, ohandler: ::core::option::Option<&IVBSAXDTDHandler>) -> ::windows::core::Result<()>;
    fn errorHandler(&self) -> ::windows::core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&self, ohandler: ::core::option::Option<&IVBSAXErrorHandler>) -> ::windows::core::Result<()>;
    fn baseURL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetbaseURL(&self, strbaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn secureBaseURL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetsecureBaseURL(&self, strsecurebaseurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn parseURL(&self, strurl: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IVBSAXXMLReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>() -> IVBSAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_entityResolver(::windows::core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_contentHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_dtdHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_errorHandler(::windows::core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn baseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strbaseurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbaseURL(::core::mem::transmute(&strbaseurl)).into()
        }
        unsafe extern "system" fn secureBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.secureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetsecureBaseURL(::core::mem::transmute(&strsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strurl: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parseURL(::core::mem::transmute(&strurl)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IVBSAXXMLReader as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn value(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>() -> IXMLAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, n: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(n, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(v, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLAttribute as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMAttribute_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self, attributename: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn value(&self, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setvalue(&self, attributevalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>() -> IXMLDOMAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.name(::core::mem::transmute_copy(&attributename)).into()
        }
        unsafe extern "system" fn value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.value(::core::mem::transmute_copy(&attributevalue)).into()
        }
        unsafe extern "system" fn Setvalue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setvalue(::core::mem::transmute(&attributevalue)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
            Setvalue: Setvalue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCDATASection_Impl: Sized + IXMLDOMText_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMCDATASection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCDATASection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCDATASection_Impl, const OFFSET: isize>() -> IXMLDOMCDATASection_Vtbl {
        Self { base__: IXMLDOMText_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMText as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMCharacterData_Impl: Sized + IXMLDOMNode_Impl {
    fn data(&self, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn length(&self, datalength: *mut i32) -> ::windows::core::Result<()>;
    fn substringData(&self, offset: i32, count: i32, data: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn appendData(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn insertData(&self, offset: i32, data: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn deleteData(&self, offset: i32, count: i32) -> ::windows::core::Result<()>;
    fn replaceData(&self, offset: i32, count: i32, data: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMCharacterData {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>() -> IXMLDOMCharacterData_Vtbl {
        unsafe extern "system" fn data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.data(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Setdata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setdata(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.length(::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn substringData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.substringData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn appendData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.appendData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn insertData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.insertData(::core::mem::transmute_copy(&offset), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn deleteData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deleteData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn replaceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.replaceData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute(&data)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMCharacterData as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMComment_Impl: Sized + IXMLDOMCharacterData_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMComment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMComment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMComment_Impl, const OFFSET: isize>() -> IXMLDOMComment_Vtbl {
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMComment as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument_Impl: Sized + IXMLDOMNode_Impl {
    fn doctype(&self) -> ::windows::core::Result<IXMLDOMDocumentType>;
    fn implementation(&self) -> ::windows::core::Result<IXMLDOMImplementation>;
    fn documentElement(&self) -> ::windows::core::Result<IXMLDOMElement>;
    fn putref_documentElement(&self, domelement: ::core::option::Option<&IXMLDOMElement>) -> ::windows::core::Result<()>;
    fn createElement(&self, tagname: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&self) -> ::windows::core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMText>;
    fn createComment(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMComment>;
    fn createCDATASection(&self, data: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&self, target: &::windows::core::BSTR, data: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&self, tagname: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn createNode(&self, r#type: &super::super::super::System::Com::VARIANT, name: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeFromID(&self, idstring: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn load(&self, xmlsource: &super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn readyState(&self, value: *mut i32) -> ::windows::core::Result<()>;
    fn parseError(&self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn r#async(&self, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setasync(&self, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn abort(&self) -> ::windows::core::Result<()>;
    fn loadXML(&self, bstrxml: &::windows::core::BSTR, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn save(&self, destination: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validateOnParse(&self, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetvalidateOnParse(&self, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn resolveExternals(&self, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetresolveExternals(&self, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn preserveWhiteSpace(&self, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn SetpreserveWhiteSpace(&self, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Setonreadystatechange(&self, readystatechangesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setondataavailable(&self, ondataavailablesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Setontransformnode(&self, ontransformnodesink: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>() -> IXMLDOMDocument_Vtbl {
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#impl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.implementation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#impl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.documentElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(domelement, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_documentElement(::windows::core::from_raw_borrowed(&domelement)).into()
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, docfrag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createDocumentFragment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docfrag, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, text: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createTextNode(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, comment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createComment(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, cdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createCDATASection(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows::core::BSTR>, data: ::std::mem::MaybeUninit<::windows::core::BSTR>, pi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createProcessingInstruction(::core::mem::transmute(&target), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, attribute: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attribute, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, entityref: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createEntityReference(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entityref, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::super::super::System::Com::VARIANT, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createNode(::core::mem::transmute(&r#type), ::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idstring: ::std::mem::MaybeUninit<::windows::core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeFromID(::core::mem::transmute(&idstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlsource: super::super::super::System::Com::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.load(::core::mem::transmute(&xmlsource), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.readyState(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn parseError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parseError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn r#async<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.r#async(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn Setasync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setasync(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn loadXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.loadXML(::core::mem::transmute(&bstrxml), ::core::mem::transmute_copy(&issuccessful)).into()
        }
        unsafe extern "system" fn save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.save(::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn validateOnParse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.validateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetvalidateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn resolveExternals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn SetresolveExternals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetresolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.preserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetpreserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystatechangesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setonreadystatechange(::core::mem::transmute(&readystatechangesink)).into()
        }
        unsafe extern "system" fn Setondataavailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ondataavailablesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setondataavailable(::core::mem::transmute(&ondataavailablesink)).into()
        }
        unsafe extern "system" fn Setontransformnode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ontransformnodesink: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setontransformnode(::core::mem::transmute(&ontransformnodesink)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMDocument as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument2_Impl: Sized + IXMLDOMDocument_Impl {
    fn namespaces(&self) -> ::windows::core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn putref_schemas(&self, othercollection: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn validate(&self) -> ::windows::core::Result<IXMLDOMParseError>;
    fn setProperty(&self, name: &::windows::core::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMDocument2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>() -> IXMLDOMDocument2_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespacecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schemas() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(othercollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_schemas(::core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            schemas: schemas::<Identity, Impl, OFFSET>,
            putref_schemas: putref_schemas::<Identity, Impl, OFFSET>,
            validate: validate::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMDocument as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocument3_Impl: Sized + IXMLDOMDocument2_Impl {
    fn validateNode(&self, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMParseError>;
    fn importNode(&self, node: ::core::option::Option<&IXMLDOMNode>, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMDocument3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>() -> IXMLDOMDocument3_Vtbl {
        unsafe extern "system" fn validateNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validateNode(::windows::core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, clone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.importNode(::windows::core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMDocument2_Vtbl::new::<Identity, Impl, OFFSET>(),
            validateNode: validateNode::<Identity, Impl, OFFSET>,
            importNode: importNode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMDocument as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMDocument2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentFragment_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMDocumentFragment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentFragment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentFragment_Impl, const OFFSET: isize>() -> IXMLDOMDocumentFragment_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMDocumentType_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self, rootname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn entities(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&self) -> ::windows::core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMDocumentType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>() -> IXMLDOMDocumentType_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.name(::core::mem::transmute_copy(&rootname)).into()
        }
        unsafe extern "system" fn entities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitymap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entitymap, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notationmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notationmap, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            name: name::<Identity, Impl, OFFSET>,
            entities: entities::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMElement_Impl: Sized + IXMLDOMNode_Impl {
    fn tagName(&self, tagname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getAttribute(&self, name: &::windows::core::BSTR, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setAttribute(&self, name: &::windows::core::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn removeAttribute(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getAttributeNode(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&self, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&self, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows::core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&self, tagname: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn normalize(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>() -> IXMLDOMElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.tagName(::core::mem::transmute_copy(&tagname)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getAttribute(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn getAttributeNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttributeNode(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.setAttributeNode(::windows::core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeAttributeNode(::windows::core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.normalize().into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMElement as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntity_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn notationName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMEntity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>() -> IXMLDOMEntity_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        unsafe extern "system" fn notationName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.notationName(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
            notationName: notationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntity as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMEntityReference_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMEntityReference {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMEntityReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntityReference_Impl, const OFFSET: isize>() -> IXMLDOMEntityReference_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMImplementation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(&self, feature: &::windows::core::BSTR, version: &::windows::core::BSTR, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMImplementation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>() -> IXMLDOMImplementation_Vtbl {
        unsafe extern "system" fn hasFeature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows::core::BSTR>, version: ::std::mem::MaybeUninit<::windows::core::BSTR>, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hasFeature(::core::mem::transmute(&feature), ::core::mem::transmute(&version), ::core::mem::transmute_copy(&hasfeature)).into()
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), hasFeature: hasFeature::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNamedNodeMap_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn setNamedItem(&self, newitem: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNamedItem(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn getQualifiedItem(&self, basename: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&self, basename: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMNamedNodeMap {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>() -> IXMLDOMNamedNodeMap_Vtbl {
        unsafe extern "system" fn getNamedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newitem: *mut ::core::ffi::c_void, nameitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.setNamedItem(::windows::core::from_raw_borrowed(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn getQualifiedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            getNamedItem: getNamedItem::<Identity, Impl, OFFSET>,
            setNamedItem: setNamedItem::<Identity, Impl, OFFSET>,
            removeNamedItem: removeNamedItem::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            getQualifiedItem: getQualifiedItem::<Identity, Impl, OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNode_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn insertBefore(&self, newchild: ::core::option::Option<&IXMLDOMNode>, refchild: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLDOMNode>;
    fn replaceChild(&self, newchild: ::core::option::Option<&IXMLDOMNode>, oldchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeChild(&self, childnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn appendChild(&self, newchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn hasChildNodes(&self, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ownerDocument(&self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn cloneNode(&self, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypeString(&self, nodetype: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn text(&self, text: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Settext(&self, text: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn specified(&self, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn definition(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&self, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetnodeTypedValue(&self, typedvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn dataType(&self, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetdataType(&self, datatypename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn xml(&self, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn transformNode(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>, xmlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn selectNodes(&self, querystring: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&self, querystring: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn parsed(&self, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn namespaceURI(&self, namespaceuri: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn prefix(&self, prefixstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn baseName(&self, namestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn transformNodeToObject(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>, outputobject: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMNode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>() -> IXMLDOMNode_Vtbl {
        unsafe extern "system" fn nodeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.nodeName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn nodeValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.nodeValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetnodeValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnodeValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn nodeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.nodeType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn parentNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parentNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.childNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.firstChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lastChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previoussibling: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.previousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previoussibling, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextsibling: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextsibling, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributemap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributemap, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, refchild: super::super::super::System::Com::VARIANT, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.insertBefore(::windows::core::from_raw_borrowed(&newchild), ::core::mem::transmute(&refchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void, outoldchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.replaceChild(::windows::core::from_raw_borrowed(&newchild), ::windows::core::from_raw_borrowed(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outoldchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, oldchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeChild(::windows::core::from_raw_borrowed(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oldchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.appendChild(::windows::core::from_raw_borrowed(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.hasChildNodes(::core::mem::transmute_copy(&haschild)).into()
        }
        unsafe extern "system" fn ownerDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ownerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmldomdocument, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, cloneroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.cloneNode(::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cloneroot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.nodeTypeString(::core::mem::transmute_copy(&nodetype)).into()
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.text(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn specified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.specified(::core::mem::transmute_copy(&isspecified)).into()
        }
        unsafe extern "system" fn definition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definitionnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.definition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(definitionnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.nodeTypedValue(::core::mem::transmute_copy(&typedvalue)).into()
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnodeTypedValue(::core::mem::transmute(&typedvalue)).into()
        }
        unsafe extern "system" fn dataType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.dataType(::core::mem::transmute_copy(&datatypename)).into()
        }
        unsafe extern "system" fn SetdataType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdataType(::core::mem::transmute(&datatypename)).into()
        }
        unsafe extern "system" fn xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.xml(::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn transformNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.transformNode(::windows::core::from_raw_borrowed(&stylesheet), ::core::mem::transmute_copy(&xmlstring)).into()
        }
        unsafe extern "system" fn selectNodes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selectNodes(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows::core::BSTR>, resultnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selectSingleNode(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parsed(::core::mem::transmute_copy(&isparsed)).into()
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.namespaceURI(::core::mem::transmute_copy(&namespaceuri)).into()
        }
        unsafe extern "system" fn prefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.prefix(::core::mem::transmute_copy(&prefixstring)).into()
        }
        unsafe extern "system" fn baseName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.baseName(::core::mem::transmute_copy(&namestring)).into()
        }
        unsafe extern "system" fn transformNodeToObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, outputobject: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.transformNodeToObject(::windows::core::from_raw_borrowed(&stylesheet), ::core::mem::transmute(&outputobject)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNodeList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMNode>;
    fn length(&self, listlength: *mut i32) -> ::windows::core::Result<()>;
    fn nextNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMNodeList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>() -> IXMLDOMNodeList_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.length(::core::mem::transmute_copy(&listlength)).into()
        }
        unsafe extern "system" fn nextNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMNotation_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn systemId(&self, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMNotation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>() -> IXMLDOMNotation_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.publicId(::core::mem::transmute_copy(&publicid)).into()
        }
        unsafe extern "system" fn systemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.systemId(::core::mem::transmute_copy(&systemid)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMNotation as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self, errorcode: *mut i32) -> ::windows::core::Result<()>;
    fn url(&self, urlstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn reason(&self, reasonstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn srcText(&self, sourcestring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn line(&self, linenumber: *mut i32) -> ::windows::core::Result<()>;
    fn linepos(&self, lineposition: *mut i32) -> ::windows::core::Result<()>;
    fn filepos(&self, fileposition: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMParseError {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>() -> IXMLDOMParseError_Vtbl {
        unsafe extern "system" fn errorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.errorCode(::core::mem::transmute_copy(&errorcode)).into()
        }
        unsafe extern "system" fn url<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.url(::core::mem::transmute_copy(&urlstring)).into()
        }
        unsafe extern "system" fn reason<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reasonstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reason(::core::mem::transmute_copy(&reasonstring)).into()
        }
        unsafe extern "system" fn srcText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.srcText(::core::mem::transmute_copy(&sourcestring)).into()
        }
        unsafe extern "system" fn line<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.line(::core::mem::transmute_copy(&linenumber)).into()
        }
        unsafe extern "system" fn linepos<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.linepos(::core::mem::transmute_copy(&lineposition)).into()
        }
        unsafe extern "system" fn filepos<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.filepos(::core::mem::transmute_copy(&fileposition)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMParseError as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseError2_Impl: Sized + IXMLDOMParseError_Impl {
    fn errorXPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn allErrors(&self) -> ::windows::core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn errorParametersCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMParseError2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>() -> IXMLDOMParseError2_Vtbl {
        unsafe extern "system" fn errorXPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorXPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpathexpr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allerrors: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allErrors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allerrors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorParameters(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorParametersCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMParseError_Vtbl::new::<Identity, Impl, OFFSET>(),
            errorXPath: errorXPath::<Identity, Impl, OFFSET>,
            allErrors: allErrors::<Identity, Impl, OFFSET>,
            errorParameters: errorParameters::<Identity, Impl, OFFSET>,
            errorParametersCount: errorParametersCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMParseError as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMParseErrorCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn next(&self) -> ::windows::core::Result<IXMLDOMParseError2>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMParseErrorCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>() -> IXMLDOMParseErrorCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMProcessingInstruction_Impl: Sized + IXMLDOMNode_Impl {
    fn target(&self, name: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn data(&self, value: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Setdata(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMProcessingInstruction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>() -> IXMLDOMProcessingInstruction_Vtbl {
        unsafe extern "system" fn target<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.target(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.data(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Setdata<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setdata(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            target: target::<Identity, Impl, OFFSET>,
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn add(&self, namespaceuri: &::windows::core::BSTR, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn get(&self, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<IXMLDOMNode>;
    fn remove(&self, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn get_namespaceURI(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn addCollection(&self, othercollection: ::core::option::Option<&IXMLDOMSchemaCollection>) -> ::windows::core::Result<()>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMSchemaCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, var: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.add(::core::mem::transmute(&namespaceuri), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, schemanode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemanode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remove(::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_namespaceURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_namespaceURI(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addCollection(::windows::core::from_raw_borrowed(&othercollection)).into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            add: add::<Identity, Impl, OFFSET>,
            get: get::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            get_namespaceURI: get_namespaceURI::<Identity, Impl, OFFSET>,
            addCollection: addCollection::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSchemaCollection2_Impl: Sized + IXMLDOMSchemaCollection_Impl {
    fn validate(&self) -> ::windows::core::Result<()>;
    fn SetvalidateOnLoad(&self, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn validateOnLoad(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getSchema(&self, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<ISchema>;
    fn getDeclaration(&self, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMSchemaCollection2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection2_Vtbl {
        unsafe extern "system" fn validate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.validate().into()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetvalidateOnLoad(::core::mem::transmute_copy(&validateonload)).into()
        }
        unsafe extern "system" fn validateOnLoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validateOnLoad() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validateonload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, schema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSchema(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDeclaration(::windows::core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMSchemaCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            validate: validate::<Identity, Impl, OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Identity, Impl, OFFSET>,
            validateOnLoad: validateOnLoad::<Identity, Impl, OFFSET>,
            getSchema: getSchema::<Identity, Impl, OFFSET>,
            getDeclaration: getDeclaration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMSchemaCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMSelection_Impl: Sized + IXMLDOMNodeList_Impl {
    fn expr(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Setexpr(&self, expression: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn context(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn putref_context(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn peekNode(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn matches(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeNext(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn removeAll(&self) -> ::windows::core::Result<()>;
    fn clone(&self) -> ::windows::core::Result<IXMLDOMSelection>;
    fn getProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn setProperty(&self, name: &::windows::core::BSTR, value: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMSelection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>() -> IXMLDOMSelection_Vtbl {
        unsafe extern "system" fn expr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.expr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expression, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setexpr(::core::mem::transmute(&expression)).into()
        }
        unsafe extern "system" fn context<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.context() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_context(::windows::core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn peekNode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.peekNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.matches(::windows::core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAll().into()
        }
        unsafe extern "system" fn clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        Self {
            base__: IXMLDOMNodeList_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDOMSelection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNodeList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDOMText_Impl: Sized + IXMLDOMCharacterData_Impl {
    fn splitText(&self, offset: i32) -> ::windows::core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDOMText {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>() -> IXMLDOMText_Vtbl {
        unsafe extern "system" fn splitText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.splitText(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(righthandtextnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>(), splitText: splitText::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDOMText as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDSOControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> ::windows::core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&self, ppdoc: ::core::option::Option<&IXMLDOMDocument>) -> ::windows::core::Result<()>;
    fn JavaDSOCompatible(&self, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetJavaDSOCompatible(&self, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn readyState(&self, state: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDSOControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>() -> IXMLDSOControl_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdoc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXMLDocument(::windows::core::from_raw_borrowed(&ppdoc)).into()
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.JavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.readyState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            SetXMLDocument: SetXMLDocument::<Identity, Impl, OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Identity, Impl, OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLDSOControl as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows::core::Result<IXMLElement>;
    fn fileSize(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fileModifiedDate(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetURL(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn charset(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Setcharset(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn doctype(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn dtdURL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>() -> IXMLDocument_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.root() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.charset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDocument as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows::core::Result<IXMLElement2>;
    fn fileSize(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fileModifiedDate(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn URL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetURL(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn mimeType(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn charset(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Setcharset(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn version(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn doctype(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn dtdURL(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Com::VARIANT, var1: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<IXMLElement2>;
    fn r#async(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(&self, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLDocument2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>() -> IXMLDocument2_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.root() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.charset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Com::VARIANT, var1: super::super::super::System::Com::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#async() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setasync(::core::mem::transmute_copy(&f)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLDocument2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SettagName(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn parent(&self) -> ::windows::core::Result<IXMLElement>;
    fn setAttribute(&self, strpropertyname: &::windows::core::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&self, strpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn children(&self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows::core::Result<i32>;
    fn text(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Settext(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&self, pchildelem: ::core::option::Option<&IXMLElement>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&self, pchildelem: ::core::option::Option<&IXMLElement>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>() -> IXMLElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.tagName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addChild(::windows::core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeChild(::windows::core::from_raw_borrowed(&pchildelem)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLElement as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElement2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SettagName(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn parent(&self) -> ::windows::core::Result<IXMLElement2>;
    fn setAttribute(&self, strpropertyname: &::windows::core::BSTR, propertyvalue: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn getAttribute(&self, strpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn children(&self) -> ::windows::core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows::core::Result<i32>;
    fn text(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Settext(&self, p: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn addChild(&self, pchildelem: ::core::option::Option<&IXMLElement2>, lindex: i32, lreserved: i32) -> ::windows::core::Result<()>;
    fn removeChild(&self, pchildelem: ::core::option::Option<&IXMLElement2>) -> ::windows::core::Result<()>;
    fn attributes(&self) -> ::windows::core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLElement2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>() -> IXMLElement2_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.tagName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addChild(::windows::core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeChild(::windows::core::from_raw_borrowed(&pchildelem)).into()
        }
        unsafe extern "system" fn attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLElement2 as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLElementCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(&self, v: i32) -> ::windows::core::Result<()>;
    fn length(&self) -> ::windows::core::Result<i32>;
    fn _newEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn item(&self, var1: &super::super::super::System::Com::VARIANT, var2: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLElementCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>() -> IXMLElementCollection_Vtbl {
        unsafe extern "system" fn Setlength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setlength(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var1: super::super::super::System::Com::VARIANT, var2: super::super::super::System::Com::VARIANT, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.item(::core::mem::transmute(&var1), ::core::mem::transmute(&var2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Setlength: Setlength::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLElementCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait IXMLError_Impl: Sized {
    fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXMLError {}
impl IXMLError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>() -> IXMLError_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorInfo(::core::mem::transmute_copy(&perrorreturn)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLError as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&self, bstrmethod: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, varasync: &super::super::super::System::Com::VARIANT, bstruser: &super::super::super::System::Com::VARIANT, bstrpassword: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn getAllResponseHeaders(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn send(&self, varbody: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn abort(&self) -> ::windows::core::Result<()>;
    fn status(&self) -> ::windows::core::Result<i32>;
    fn statusText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn responseXML(&self) -> ::windows::core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn responseBody(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn responseStream(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: ::core::option::Option<&super::super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXMLHTTPRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>() -> IXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, varasync: super::super::super::System::Com::VARIANT, bstruser: super::super::super::System::Com::VARIANT, bstrpassword: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.open(::core::mem::transmute(&bstrmethod), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&varasync), ::core::mem::transmute(&bstruser), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setRequestHeader(::core::mem::transmute(&bstrheader), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getResponseHeader(::core::mem::transmute(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.send(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.statusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setonreadystatechange(::windows::core::from_raw_borrowed(&preadystatesink)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXMLHTTPRequest as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2_Impl: Sized {
    fn Open(&self, pwszmethod: &::windows::core::PCWSTR, pwszurl: &::windows::core::PCWSTR, pstatuscallback: ::core::option::Option<&IXMLHTTPRequest2Callback>, pwszusername: &::windows::core::PCWSTR, pwszpassword: &::windows::core::PCWSTR, pwszproxyusername: &::windows::core::PCWSTR, pwszproxypassword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Send(&self, pbody: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> ::windows::core::Result<u32>;
    fn SetCustomResponseStream(&self, psequentialstream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::Result<()>;
    fn SetRequestHeader(&self, pwszheader: &::windows::core::PCWSTR, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetAllResponseHeaders(&self) -> ::windows::core::Result<*mut u16>;
    fn GetCookie(&self, pwszurl: &::windows::core::PCWSTR, pwszname: &::windows::core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::Result<()>;
    fn GetResponseHeader(&self, pwszheader: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXMLHTTPRequest2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmethod: ::windows::core::PCWSTR, pwszurl: ::windows::core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pwszusername: ::windows::core::PCWSTR, pwszpassword: ::windows::core::PCWSTR, pwszproxyusername: ::windows::core::PCWSTR, pwszproxypassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszmethod), ::core::mem::transmute(&pwszurl), ::windows::core::from_raw_borrowed(&pstatuscallback), ::core::mem::transmute(&pwszusername), ::core::mem::transmute(&pwszpassword), ::core::mem::transmute(&pwszproxyusername), ::core::mem::transmute(&pwszproxypassword)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut ::core::ffi::c_void, cbbody: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows::core::from_raw_borrowed(&pbody), ::core::mem::transmute_copy(&cbbody)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn SetCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetCookie(::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookiestate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psequentialstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustomResponseStream(::windows::core::from_raw_borrowed(&psequentialstream)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&eproperty), ::core::mem::transmute_copy(&ullvalue)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestHeader(::core::mem::transmute(&pwszheader), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwszname: ::windows::core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCookie(::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows::core::PCWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResponseHeader(::core::mem::transmute(&pwszheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IXMLHTTPRequest2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2Callback_Impl: Sized {
    fn OnRedirect(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, pwszredirecturl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnHeadersAvailable(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnDataAvailable(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnResponseReceived(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows::core::Result<()>;
    fn OnError(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLHTTPRequest2Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2Callback_Vtbl {
        unsafe extern "system" fn OnRedirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, pwszredirecturl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRedirect(::windows::core::from_raw_borrowed(&pxhr), ::core::mem::transmute(&pwszredirecturl)).into()
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwstatus: u32, pwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHeadersAvailable(::windows::core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute(&pwszstatus)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataAvailable(::windows::core::from_raw_borrowed(&pxhr), ::windows::core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnResponseReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResponseReceived(::windows::core::from_raw_borrowed(&pxhr), ::windows::core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::windows::core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, Impl, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3_Impl: Sized + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IXMLHTTPRequest3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3_Vtbl {
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificate(::core::mem::transmute_copy(&cbclientcertificatehash), ::core::mem::transmute_copy(&pbclientcertificatehash), ::core::mem::transmute(&pwszpin)).into()
        }
        Self { base__: IXMLHTTPRequest2_Vtbl::new::<Identity, Impl, OFFSET>(), SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as ::windows::core::ComInterface>::IID || iid == &<IXMLHTTPRequest2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3Callback_Impl: Sized + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::Result<()>;
    fn OnClientCertificateRequested(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLHTTPRequest3Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3Callback_Vtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnServerCertificateReceived(::windows::core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwcertificateerrors), ::core::mem::transmute_copy(&cservercertificatechain), ::core::mem::transmute_copy(&rgservercertificatechain)).into()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClientCertificateRequested(::windows::core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&cissuerlist), ::core::mem::transmute_copy(&rgpwszissuerlist)).into()
        }
        Self {
            base__: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, Impl, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as ::windows::core::ComInterface>::IID || iid == &<IXMLHTTPRequest2Callback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLProcessor_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(&self, var: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn input(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn ownerTemplate(&self) -> ::windows::core::Result<IXSLTemplate>;
    fn setStartMode(&self, mode: &::windows::core::BSTR, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn startMode(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn startModeURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Setoutput(&self, output: &super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn output(&self) -> ::windows::core::Result<super::super::super::System::Com::VARIANT>;
    fn transform(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows::core::Result<()>;
    fn readyState(&self) -> ::windows::core::Result<i32>;
    fn addParameter(&self, basename: &::windows::core::BSTR, parameter: &super::super::super::System::Com::VARIANT, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn addObject(&self, obj: ::core::option::Option<&super::super::super::System::Com::IDispatch>, namespaceuri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXSLProcessor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>() -> IXSLProcessor_Vtbl {
        unsafe extern "system" fn Setinput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setinput(::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn input<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.input() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ownerTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setStartMode(::core::mem::transmute(&mode), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn startMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.startMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.startModeURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setoutput(::core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.output() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdone: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.transform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preadystate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows::core::BSTR>, parameter: super::super::super::System::Com::VARIANT, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addParameter(::core::mem::transmute(&basename), ::core::mem::transmute(&parameter), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn addObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addObject(::windows::core::from_raw_borrowed(&obj), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXSLProcessor as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXSLTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>) -> ::windows::core::Result<()>;
    fn stylesheet(&self) -> ::windows::core::Result<IXMLDOMNode>;
    fn createProcessor(&self) -> ::windows::core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXSLTemplate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>() -> IXSLTemplate_Vtbl {
        unsafe extern "system" fn putref_stylesheet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_stylesheet(::windows::core::from_raw_borrowed(&stylesheet)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprocessor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_stylesheet: putref_stylesheet::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
            createProcessor: createProcessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXSLTemplate as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IXTLRuntime_Impl: Sized + IXMLDOMNode_Impl {
    fn uniqueID(&self, pnode: ::core::option::Option<&IXMLDOMNode>, pid: *mut i32) -> ::windows::core::Result<()>;
    fn depth(&self, pnode: ::core::option::Option<&IXMLDOMNode>, pdepth: *mut i32) -> ::windows::core::Result<()>;
    fn childNumber(&self, pnode: ::core::option::Option<&IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn ancestorChildNumber(&self, bstrnodename: &::windows::core::BSTR, pnode: ::core::option::Option<&IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn absoluteChildNumber(&self, pnode: ::core::option::Option<&IXMLDOMNode>, pnumber: *mut i32) -> ::windows::core::Result<()>;
    fn formatIndex(&self, lindex: i32, bstrformat: &::windows::core::BSTR, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn formatNumber(&self, dblnumber: f64, bstrformat: &::windows::core::BSTR, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn formatDate(&self, vardate: &super::super::super::System::Com::VARIANT, bstrformat: &::windows::core::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn formatTime(&self, vartime: &super::super::super::System::Com::VARIANT, bstrformat: &::windows::core::BSTR, vardestlocale: &super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IXTLRuntime {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>() -> IXTLRuntime_Vtbl {
        unsafe extern "system" fn uniqueID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.uniqueID(::windows::core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn depth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pdepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.depth(::windows::core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&pdepth)).into()
        }
        unsafe extern "system" fn childNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.childNumber(::windows::core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnodename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ancestorChildNumber(::core::mem::transmute(&bstrnodename), ::windows::core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.absoluteChildNumber(::windows::core::from_raw_borrowed(&pnode), ::core::mem::transmute_copy(&pnumber)).into()
        }
        unsafe extern "system" fn formatIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.formatIndex(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.formatNumber(::core::mem::transmute_copy(&dblnumber), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardate: super::super::super::System::Com::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.formatDate(::core::mem::transmute(&vardate), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        unsafe extern "system" fn formatTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartime: super::super::super::System::Com::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows::core::BSTR>, vardestlocale: super::super::super::System::Com::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.formatTime(::core::mem::transmute(&vartime), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale), ::core::mem::transmute_copy(&pbstrformattedstring)).into()
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IXTLRuntime as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait XMLDOMDocumentEvents_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for XMLDOMDocumentEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl XMLDOMDocumentEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: XMLDOMDocumentEvents_Impl, const OFFSET: isize>() -> XMLDOMDocumentEvents_Vtbl {
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as ::windows::core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
