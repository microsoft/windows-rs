#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(&self, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR, strqname: &::windows_core::BSTR, strtype: &::windows_core::BSTR, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addAttributeFromIndex(&self, varatts: &super::super::super::System::Variant::VARIANT, nindex: i32) -> ::windows_core::Result<()>;
    fn clear(&self) -> ::windows_core::Result<()>;
    fn removeAttribute(&self, nindex: i32) -> ::windows_core::Result<()>;
    fn setAttribute(&self, nindex: i32, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR, strqname: &::windows_core::BSTR, strtype: &::windows_core::BSTR, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setAttributes(&self, varatts: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setLocalName(&self, nindex: i32, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setQName(&self, nindex: i32, strqname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setType(&self, nindex: i32, strtype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setURI(&self, nindex: i32, struri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setValue(&self, nindex: i32, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXAttributes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>() -> IMXAttributes_Vtbl {
        unsafe extern "system" fn addAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addAttribute(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Variant::VARIANT, nindex: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addAttributeFromIndex(::core::mem::transmute(&varatts), ::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.clear().into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute_copy(&nindex)).into()
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn setAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttributes(::core::mem::transmute(&varatts)).into()
        }
        unsafe extern "system" fn setLocalName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strlocalname)).into()
        }
        unsafe extern "system" fn setQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strqname)).into()
        }
        unsafe extern "system" fn setType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strtype)).into()
        }
        unsafe extern "system" fn setURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri)).into()
        }
        unsafe extern "system" fn setValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXAttributes as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMXNamespaceManager_Impl: Sized {
    fn putAllowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getAllowOverride(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn pushContext(&self) -> ::windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn popContext(&self) -> ::windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &::windows_core::PCWSTR, namespaceuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::Result<()>;
    fn getPrefix(&self, pwsznamespaceuri: &::windows_core::PCWSTR, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::Result<()>;
    fn getURI(&self, pwchprefix: &::windows_core::PCWSTR, pcontextnode: ::core::option::Option<&IXMLDOMNode>, pwchuri: ::windows_core::PWSTR, pcchuri: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IMXNamespaceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>() -> IMXNamespaceManager_Vtbl {
        unsafe extern "system" fn putAllowOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putAllowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn getAllowOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAllowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushNodeContext(::windows_core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::windows_core::PCWSTR, namespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getDeclaredPrefix(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: ::windows_core::PCWSTR, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getPrefix(::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into()
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, pcontextnode: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PWSTR, pcchuri: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getURI(::core::mem::transmute(&pwchprefix), ::windows_core::from_raw_borrowed(&pcontextnode), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXNamespaceManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXNamespacePrefixes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXNamespacePrefixes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>() -> IMXNamespacePrefixes_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXReaderControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn abort(&self) -> ::windows_core::Result<()>;
    fn resume(&self) -> ::windows_core::Result<()>;
    fn suspend(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXReaderControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>() -> IMXReaderControl_Vtbl {
        unsafe extern "system" fn abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resume().into()
        }
        unsafe extern "system" fn suspend<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXReaderControl as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXSchemaDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(&self, oschemaelement: ::core::option::Option<&ISchemaElement>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXSchemaDeclHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>() -> IMXSchemaDeclHandler_Vtbl {
        unsafe extern "system" fn schemaElementDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oschemaelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.schemaElementDecl(::windows_core::from_raw_borrowed(&oschemaelement)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            schemaElementDecl: schemaElementDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXWriter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(&self, vardestination: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn output(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Setencoding(&self, strencoding: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn encoding(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetbyteOrderMark(&self, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn byteOrderMark(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setindent(&self, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn indent(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setstandalone(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn standalone(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetomitXMLDeclaration(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn omitXMLDeclaration(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setversion(&self, strversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdisableOutputEscaping(&self, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn disableOutputEscaping(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn flush(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXWriter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>() -> IMXWriter_Vtbl {
        unsafe extern "system" fn Setoutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setoutput(::core::mem::transmute(&vardestination)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.output() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vardestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setencoding(::core::mem::transmute(&strencoding)).into()
        }
        unsafe extern "system" fn encoding<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strencoding: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.encoding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbyteOrderMark(::core::mem::transmute_copy(&fwritebyteordermark)).into()
        }
        unsafe extern "system" fn byteOrderMark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.byteOrderMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fwritebyteordermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setindent(::core::mem::transmute_copy(&findentmode)).into()
        }
        unsafe extern "system" fn indent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findentmode: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.indent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(findentmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setstandalone(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn standalone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.standalone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetomitXMLDeclaration(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.omitXMLDeclaration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setversion(::core::mem::transmute(&strversion)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdisableOutputEscaping(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.disableOutputEscaping() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXWriter as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &::windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(&self, strname: &::windows_core::BSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn entityResolver(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_entityResolver(&self, oresolver: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn contentHandler(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_contentHandler(&self, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn dtdHandler(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_dtdHandler(&self, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn errorHandler(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_errorHandler(&self, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>() -> IMXXMLFilter_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_entityResolver(::windows_core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_contentHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_dtdHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_errorHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMXXMLFilter as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXAttributes_Impl: Sized {
    fn getLength(&self) -> ::windows_core::Result<i32>;
    fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows_core::Result<()>;
    fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows_core::Result<()>;
    fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::Result<()>;
    fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::Result<()>;
    fn getIndexFromName(&self, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32) -> ::windows_core::Result<i32>;
    fn getIndexFromQName(&self, pwchqname: &::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::Result<i32>;
    fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getTypeFromName(&self, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getTypeFromQName(&self, pwchqname: &::windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
    fn getValueFromName(&self, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
    fn getValueFromQName(&self, pwchqname: &::windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXAttributes {}
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>() -> ISAXAttributes_Vtbl {
        unsafe extern "system" fn getLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getURI(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)).into()
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getLocalName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)).into()
        }
        unsafe extern "system" fn getQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getQName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getName(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into()
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getType(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getTypeFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getTypeFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into()
        }
        unsafe extern "system" fn getValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValue(::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValueFromName(::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.getValueFromQName(::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXAttributes as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXContentHandler_Impl: Sized {
    fn putDocumentLocator(&self, plocator: ::core::option::Option<&ISAXLocator>) -> ::windows_core::Result<()>;
    fn startDocument(&self) -> ::windows_core::Result<()>;
    fn endDocument(&self) -> ::windows_core::Result<()>;
    fn startPrefixMapping(&self, pwchprefix: &::windows_core::PCWSTR, cchprefix: i32, pwchuri: &::windows_core::PCWSTR, cchuri: i32) -> ::windows_core::Result<()>;
    fn endPrefixMapping(&self, pwchprefix: &::windows_core::PCWSTR, cchprefix: i32) -> ::windows_core::Result<()>;
    fn startElement(&self, pwchnamespaceuri: &::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, pwchqname: &::windows_core::PCWSTR, cchqname: i32, pattributes: ::core::option::Option<&ISAXAttributes>) -> ::windows_core::Result<()>;
    fn endElement(&self, pwchnamespaceuri: &::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, pwchqname: &::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::Result<()>;
    fn characters(&self, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
    fn ignorableWhitespace(&self, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
    fn processingInstruction(&self, pwchtarget: &::windows_core::PCWSTR, cchtarget: i32, pwchdata: &::windows_core::PCWSTR, cchdata: i32) -> ::windows_core::Result<()>;
    fn skippedEntity(&self, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXContentHandler {}
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>() -> ISAXContentHandler_Vtbl {
        unsafe extern "system" fn putDocumentLocator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putDocumentLocator(::windows_core::from_raw_borrowed(&plocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, cchprefix: i32, pwchuri: ::windows_core::PCWSTR, cchuri: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix), ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, cchprefix: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endPrefixMapping(::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pwchqname: ::windows_core::PCWSTR, cchqname: i32, pattributes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::windows_core::from_raw_borrowed(&pattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pwchqname: ::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endElement(::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.characters(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWhitespace(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchtarget: ::windows_core::PCWSTR, cchtarget: i32, pwchdata: ::windows_core::PCWSTR, cchdata: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.processingInstruction(::core::mem::transmute(&pwchtarget), ::core::mem::transmute_copy(&cchtarget), ::core::mem::transmute(&pwchdata), ::core::mem::transmute_copy(&cchdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.skippedEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXContentHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXDTDHandler_Impl: Sized {
    fn notationDecl(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
    fn unparsedEntityDecl(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: &::windows_core::PCWSTR, cchnotationname: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXDTDHandler {}
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>() -> ISAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.notationDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: ::windows_core::PCWSTR, cchnotationname: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.unparsedEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid), ::core::mem::transmute(&pwchnotationname), ::core::mem::transmute_copy(&cchnotationname)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXDTDHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXDeclHandler_Impl: Sized {
    fn elementDecl(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchmodel: &::windows_core::PCWSTR, cchmodel: i32) -> ::windows_core::Result<()>;
    fn attributeDecl(&self, pwchelementname: &::windows_core::PCWSTR, cchelementname: i32, pwchattributename: &::windows_core::PCWSTR, cchattributename: i32, pwchtype: &::windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: &::windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: &::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::Result<()>;
    fn internalEntityDecl(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchvalue: &::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::Result<()>;
    fn externalEntityDecl(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXDeclHandler {}
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>() -> ISAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchmodel: ::windows_core::PCWSTR, cchmodel: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.elementDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchmodel), ::core::mem::transmute_copy(&cchmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchelementname: ::windows_core::PCWSTR, cchelementname: i32, pwchattributename: ::windows_core::PCWSTR, cchattributename: i32, pwchtype: ::windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: ::windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: ::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeDecl(::core::mem::transmute(&pwchelementname), ::core::mem::transmute_copy(&cchelementname), ::core::mem::transmute(&pwchattributename), ::core::mem::transmute_copy(&cchattributename), ::core::mem::transmute(&pwchtype), ::core::mem::transmute_copy(&cchtype), ::core::mem::transmute(&pwchvaluedefault), ::core::mem::transmute_copy(&cchvaluedefault), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchvalue: ::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.internalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.externalEntityDecl(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXDeclHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXEntityResolver_Impl: Sized {
    fn resolveEntity(&self, pwchpublicid: &::windows_core::PCWSTR, pwchsystemid: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISAXEntityResolver {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>() -> ISAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchpublicid: ::windows_core::PCWSTR, pwchsystemid: ::windows_core::PCWSTR, pvarinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.resolveEntity(::core::mem::transmute(&pwchpublicid), ::core::mem::transmute(&pwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXEntityResolver as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXErrorHandler_Impl: Sized {
    fn error(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn fatalError(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ignorableWarning(&self, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXErrorHandler {}
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>() -> ISAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.error(::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fatalError(::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWarning(::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXErrorHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXLexicalHandler_Impl: Sized {
    fn startDTD(&self, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
    fn endDTD(&self) -> ::windows_core::Result<()>;
    fn startEntity(&self, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
    fn endEntity(&self, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
    fn startCDATA(&self) -> ::windows_core::Result<()>;
    fn endCDATA(&self) -> ::windows_core::Result<()>;
    fn comment(&self, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISAXLexicalHandler {}
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>() -> ISAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDTD(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endEntity(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.comment(::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait ISAXLocator_Impl: Sized {
    fn getColumnNumber(&self) -> ::windows_core::Result<i32>;
    fn getLineNumber(&self) -> ::windows_core::Result<i32>;
    fn getPublicId(&self) -> ::windows_core::Result<*mut u16>;
    fn getSystemId(&self) -> ::windows_core::Result<*mut u16>;
}
impl ::windows_core::RuntimeName for ISAXLocator {}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>() -> ISAXLocator_Vtbl {
        unsafe extern "system" fn getColumnNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getColumnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getPublicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchpublicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsystemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getColumnNumber: getColumnNumber::<Identity, Impl, OFFSET>,
            getLineNumber: getLineNumber::<Identity, Impl, OFFSET>,
            getPublicId: getPublicId::<Identity, Impl, OFFSET>,
            getSystemId: getSystemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXLocator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXXMLFilter_Impl: Sized + ISAXXMLReader_Impl {
    fn getParent(&self) -> ::windows_core::Result<ISAXXMLReader>;
    fn putParent(&self, preader: ::core::option::Option<&ISAXXMLReader>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISAXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>() -> ISAXXMLFilter_Vtbl {
        unsafe extern "system" fn getParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putParent(::windows_core::from_raw_borrowed(&preader)).into()
        }
        Self {
            base__: ISAXXMLReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            getParent: getParent::<Identity, Impl, OFFSET>,
            putParent: putParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXXMLFilter as ::windows_core::ComInterface>::IID || iid == &<ISAXXMLReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXXMLReader_Impl: Sized {
    fn getFeature(&self, pwchname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, pwchname: &::windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(&self, pwchname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(&self, pwchname: &::windows_core::PCWSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getEntityResolver(&self) -> ::windows_core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&self, presolver: ::core::option::Option<&ISAXEntityResolver>) -> ::windows_core::Result<()>;
    fn getContentHandler(&self) -> ::windows_core::Result<ISAXContentHandler>;
    fn putContentHandler(&self, phandler: ::core::option::Option<&ISAXContentHandler>) -> ::windows_core::Result<()>;
    fn getDTDHandler(&self) -> ::windows_core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&self, phandler: ::core::option::Option<&ISAXDTDHandler>) -> ::windows_core::Result<()>;
    fn getErrorHandler(&self) -> ::windows_core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&self, phandler: ::core::option::Option<&ISAXErrorHandler>) -> ::windows_core::Result<()>;
    fn getBaseURL(&self) -> ::windows_core::Result<*mut u16>;
    fn putBaseURL(&self, pwchbaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn getSecureBaseURL(&self) -> ::windows_core::Result<*mut u16>;
    fn putSecureBaseURL(&self, pwchsecurebaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn parseURL(&self, pwchurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISAXXMLReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>() -> ISAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, pvfvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&vfvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, pvarvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&pwchname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn getEntityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getEntityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putEntityResolver(::windows_core::from_raw_borrowed(&presolver)).into()
        }
        unsafe extern "system" fn getContentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getContentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putContentHandler(::windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getDTDHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDTDHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putDTDHandler(::windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getErrorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getErrorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putErrorHandler(::windows_core::from_raw_borrowed(&phandler)).into()
        }
        unsafe extern "system" fn getBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchbaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putBaseURL(::core::mem::transmute(&pwchbaseurl)).into()
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSecureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putSecureBaseURL(::core::mem::transmute(&pwchsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwchurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parseURL(::core::mem::transmute(&pwchurl)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISAXXMLReader as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchema_Impl: Sized + ISchemaItem_Impl {
    fn targetNamespace(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn types(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn elements(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn attributes(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn attributeGroups(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn modelGroups(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn notations(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn schemaLocations(&self) -> ::windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchema {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchema_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>() -> ISchema_Vtbl {
        unsafe extern "system" fn targetNamespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.targetNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, types: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.types() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(types, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.elements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributeGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modelgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.modelGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modelgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schemalocations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schemaLocations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemalocations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchema as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAny_Impl: Sized + ISchemaParticle_Impl {
    fn namespaces(&self) -> ::windows_core::Result<ISchemaStringCollection>;
    fn processContents(&self) -> ::windows_core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaAny {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>() -> ISchemaAny_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.processContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(processcontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaAny as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAttribute_Impl: Sized + ISchemaItem_Impl {
    fn r#type(&self) -> ::windows_core::Result<ISchemaType>;
    fn scope(&self) -> ::windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fixedValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn r#use(&self) -> ::windows_core::Result<SCHEMAUSE>;
    fn isReference(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>() -> ISchemaAttribute_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#use() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#use, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaAttribute as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAttributeGroup_Impl: Sized + ISchemaItem_Impl {
    fn anyAttribute(&self) -> ::windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaAttributeGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>() -> ISchemaAttributeGroup_Vtbl {
        unsafe extern "system" fn anyAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaComplexType_Impl: Sized + ISchemaType_Impl {
    fn isAbstract(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn anyAttribute(&self) -> ::windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn contentType(&self) -> ::windows_core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&self) -> ::windows_core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&self) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaComplexType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>() -> ISchemaComplexType_Vtbl {
        unsafe extern "system" fn isAbstract<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.anyAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentmodel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentmodel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.prohibitedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibited, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaComplexType as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID || iid == &<ISchemaType as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaElement_Impl: Sized + ISchemaParticle_Impl {
    fn r#type(&self) -> ::windows_core::Result<ISchemaType>;
    fn scope(&self) -> ::windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fixedValue(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn isNillable(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn identityConstraints(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&self) -> ::windows_core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&self) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&self) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn isReference(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>() -> ISchemaElement_Vtbl {
        unsafe extern "system" fn r#type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.defaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fixedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nillable: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isNillable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nillable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, constraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.identityConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(constraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.substitutionGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.substitutionGroupExclusions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exclusions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.disallowedSubstitutions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isAbstract() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaElement as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaIdentityConstraint_Impl: Sized + ISchemaItem_Impl {
    fn selector(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fields(&self) -> ::windows_core::Result<ISchemaStringCollection>;
    fn referencedKey(&self) -> ::windows_core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaIdentityConstraint {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>() -> ISchemaIdentityConstraint_Vtbl {
        unsafe extern "system" fn selector<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selector: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fields: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fields() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fields, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.referencedKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaItem_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn namespaceURI(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn schema(&self) -> ::windows_core::Result<ISchema>;
    fn id(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn itemType(&self) -> ::windows_core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&self) -> ::windows_core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&self, annotationsink: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>() -> ISchemaItem_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaceURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schema() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.unhandledAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.writeAnnotation(::windows_core::from_raw_borrowed(&annotationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaItem as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaItemCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows_core::Result<ISchemaItem>;
    fn itemByName(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<ISchemaItem>;
    fn itemByQName(&self, name: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<ISchemaItem>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaItemCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>() -> ISchemaItemCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemByName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.itemByQName(::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaItemCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaModelGroup_Impl: Sized + ISchemaParticle_Impl {
    fn particles(&self) -> ::windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaModelGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>() -> ISchemaModelGroup_Vtbl {
        unsafe extern "system" fn particles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, particles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.particles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(particles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ISchemaParticle_Vtbl::new::<Identity, Impl, OFFSET>(), particles: particles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaModelGroup as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID || iid == &<ISchemaParticle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaNotation_Impl: Sized + ISchemaItem_Impl {
    fn systemIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn publicIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaNotation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>() -> ISchemaNotation_Vtbl {
        unsafe extern "system" fn systemIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaNotation as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaParticle_Impl: Sized + ISchemaItem_Impl {
    fn minOccurs(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn maxOccurs(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaParticle {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>() -> ISchemaParticle_Vtbl {
        unsafe extern "system" fn minOccurs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minoccurs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxOccurs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxoccurs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaParticle as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaStringCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaStringCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>() -> ISchemaStringCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaStringCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaType_Impl: Sized + ISchemaItem_Impl {
    fn baseTypes(&self) -> ::windows_core::Result<ISchemaItemCollection>;
    fn r#final(&self) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&self) -> ::windows_core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&self) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn minExclusive(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn minInclusive(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn maxExclusive(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn maxInclusive(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn totalDigits(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn fractionDigits(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn length(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn minLength(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn maxLength(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn enumeration(&self) -> ::windows_core::Result<ISchemaStringCollection>;
    fn whitespace(&self) -> ::windows_core::Result<SCHEMAWHITESPACE>;
    fn patterns(&self) -> ::windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISchemaType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>() -> ISchemaType_Vtbl {
        unsafe extern "system" fn baseTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetypes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(basetypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#final() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#final, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.variety() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variety, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.derivedBy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedby, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, valid: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.isValid(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minexclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minexclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mininclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mininclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxexclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxInclusive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxinclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.totalDigits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(totaldigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fractionDigits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fractiondigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.minLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.maxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumeration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.enumeration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumeration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.whitespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(whitespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterns: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.patterns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patterns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISchemaType as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISchemaItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IServerXMLHTTPRequest_Impl: Sized + IXMLHTTPRequest_Impl {
    fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::Result<()>;
    fn waitForResponse(&self, timeoutinseconds: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IServerXMLHTTPRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn setTimeouts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setTimeouts(::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn waitForResponse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinseconds: super::super::super::System::Variant::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.waitForResponse(::core::mem::transmute(&timeoutinseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getOption(::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLHTTPRequest as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IServerXMLHTTPRequest2_Impl: Sized + IServerXMLHTTPRequest_Impl {
    fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::super::super::System::Variant::VARIANT, varbypasslist: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setProxyCredentials(&self, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IServerXMLHTTPRequest2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IServerXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn setProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::super::super::System::Variant::VARIANT, varbypasslist: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProxy(::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&varproxyserver), ::core::mem::transmute(&varbypasslist)).into()
        }
        unsafe extern "system" fn setProxyCredentials<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLHTTPRequest as ::windows_core::ComInterface>::IID || iid == &<IServerXMLHTTPRequest as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBMXNamespaceManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(&self, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn allowOverride(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn pushContext(&self) -> ::windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn popContext(&self) -> ::windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getDeclaredPrefixes(&self) -> ::windows_core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&self, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IMXNamespacePrefixes>;
    fn getURI(&self, prefix: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn getURIFromNode(&self, strprefix: &::windows_core::BSTR, contextnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBMXNamespaceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>() -> IVBMXNamespaceManager_Vtbl {
        unsafe extern "system" fn SetallowOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetallowOverride(::core::mem::transmute_copy(&foverride)).into()
        }
        unsafe extern "system" fn allowOverride<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allowOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn pushContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushContext().into()
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.pushNodeContext(::windows_core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into()
        }
        unsafe extern "system" fn popContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.popContext().into()
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.declarePrefix(::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDeclaredPrefixes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getPrefixes(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, uri: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURI(::core::mem::transmute(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, contextnode: *mut ::core::ffi::c_void, uri: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURIFromNode(::core::mem::transmute(&strprefix), ::windows_core::from_raw_borrowed(&contextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn length(&self) -> ::windows_core::Result<i32>;
    fn getURI(&self, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getLocalName(&self, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getQName(&self, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getIndexFromName(&self, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn getIndexFromQName(&self, strqname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn getType(&self, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getTypeFromName(&self, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getTypeFromQName(&self, strqname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValue(&self, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValueFromName(&self, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValueFromQName(&self, strqname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXAttributes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>() -> IVBSAXAttributes_Vtbl {
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getURI(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(struri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getLocalName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strlocalname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getQName(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strqname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getIndexFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getType(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getTypeFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getTypeFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValue(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValueFromName(::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getValueFromQName(::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXAttributes as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXContentHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(&self, olocator: ::core::option::Option<&IVBSAXLocator>) -> ::windows_core::Result<()>;
    fn startDocument(&self) -> ::windows_core::Result<()>;
    fn endDocument(&self) -> ::windows_core::Result<()>;
    fn startPrefixMapping(&self, strprefix: *mut ::windows_core::BSTR, struri: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endPrefixMapping(&self, strprefix: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startElement(&self, strnamespaceuri: *mut ::windows_core::BSTR, strlocalname: *mut ::windows_core::BSTR, strqname: *mut ::windows_core::BSTR, oattributes: ::core::option::Option<&IVBSAXAttributes>) -> ::windows_core::Result<()>;
    fn endElement(&self, strnamespaceuri: *mut ::windows_core::BSTR, strlocalname: *mut ::windows_core::BSTR, strqname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn characters(&self, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ignorableWhitespace(&self, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn processingInstruction(&self, strtarget: *mut ::windows_core::BSTR, strdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn skippedEntity(&self, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXContentHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>() -> IVBSAXContentHandler_Vtbl {
        unsafe extern "system" fn putref_documentLocator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_documentLocator(::windows_core::from_raw_borrowed(&olocator)).into()
        }
        unsafe extern "system" fn startDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDocument().into()
        }
        unsafe extern "system" fn endDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDocument().into()
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, struri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startPrefixMapping(::core::mem::transmute_copy(&strprefix), ::core::mem::transmute_copy(&struri)).into()
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endPrefixMapping(::core::mem::transmute_copy(&strprefix)).into()
        }
        unsafe extern "system" fn startElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, oattributes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::windows_core::from_raw_borrowed(&oattributes)).into()
        }
        unsafe extern "system" fn endElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endElement(::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname)).into()
        }
        unsafe extern "system" fn characters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.characters(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWhitespace(::core::mem::transmute_copy(&strchars)).into()
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strtarget: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.processingInstruction(::core::mem::transmute_copy(&strtarget), ::core::mem::transmute_copy(&strdata)).into()
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXDTDHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(&self, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn unparsedEntityDecl(&self, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR, strnotationname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXDTDHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>() -> IVBSAXDTDHandler_Vtbl {
        unsafe extern "system" fn notationDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.notationDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strnotationname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXDeclHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(&self, strname: *mut ::windows_core::BSTR, strmodel: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn attributeDecl(&self, strelementname: *mut ::windows_core::BSTR, strattributename: *mut ::windows_core::BSTR, strtype: *mut ::windows_core::BSTR, strvaluedefault: *mut ::windows_core::BSTR, strvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn internalEntityDecl(&self, strname: *mut ::windows_core::BSTR, strvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn externalEntityDecl(&self, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXDeclHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>() -> IVBSAXDeclHandler_Vtbl {
        unsafe extern "system" fn elementDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strmodel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.elementDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strmodel)).into()
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strelementname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strattributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvaluedefault: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.attributeDecl(::core::mem::transmute_copy(&strelementname), ::core::mem::transmute_copy(&strattributename), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvaluedefault), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.internalEntityDecl(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXEntityResolver_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(&self, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR, varinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXEntityResolver {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>() -> IVBSAXEntityResolver_Vtbl {
        unsafe extern "system" fn resolveEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, varinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.resolveEntity(::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&varinput)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            resolveEntity: resolveEntity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXErrorHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn error(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
    fn fatalError(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
    fn ignorableWarning(&self, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXErrorHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>() -> IVBSAXErrorHandler_Vtbl {
        unsafe extern "system" fn error<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.error(::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn fatalError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.fatalError(::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ignorableWarning(::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXLexicalHandler_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(&self, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endDTD(&self) -> ::windows_core::Result<()>;
    fn startEntity(&self, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endEntity(&self, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startCDATA(&self) -> ::windows_core::Result<()>;
    fn endCDATA(&self) -> ::windows_core::Result<()>;
    fn comment(&self, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXLexicalHandler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>() -> IVBSAXLexicalHandler_Vtbl {
        unsafe extern "system" fn startDTD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startDTD(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into()
        }
        unsafe extern "system" fn endDTD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endDTD().into()
        }
        unsafe extern "system" fn startEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn endEntity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endEntity(::core::mem::transmute_copy(&strname)).into()
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.startCDATA().into()
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.endCDATA().into()
        }
        unsafe extern "system" fn comment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXLocator_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(&self) -> ::windows_core::Result<i32>;
    fn lineNumber(&self) -> ::windows_core::Result<i32>;
    fn publicId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn systemId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>() -> IVBSAXLocator_Vtbl {
        unsafe extern "system" fn columnNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.columnNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ncolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lineNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strpublicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsystemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXLocator as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXXMLFilter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn parent(&self) -> ::windows_core::Result<IVBSAXXMLReader>;
    fn putref_parent(&self, oreader: ::core::option::Option<&IVBSAXXMLReader>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXXMLFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>() -> IVBSAXXMLFilter_Vtbl {
        unsafe extern "system" fn parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oreader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_parent(::windows_core::from_raw_borrowed(&oreader)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            parent: parent::<Identity, Impl, OFFSET>,
            putref_parent: putref_parent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXXMLReader_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(&self, strname: &::windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(&self, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(&self, strname: &::windows_core::BSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn entityResolver(&self) -> ::windows_core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&self, oresolver: ::core::option::Option<&IVBSAXEntityResolver>) -> ::windows_core::Result<()>;
    fn contentHandler(&self) -> ::windows_core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&self, ohandler: ::core::option::Option<&IVBSAXContentHandler>) -> ::windows_core::Result<()>;
    fn dtdHandler(&self) -> ::windows_core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&self, ohandler: ::core::option::Option<&IVBSAXDTDHandler>) -> ::windows_core::Result<()>;
    fn errorHandler(&self) -> ::windows_core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&self, ohandler: ::core::option::Option<&IVBSAXErrorHandler>) -> ::windows_core::Result<()>;
    fn baseURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetbaseURL(&self, strbaseurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn secureBaseURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetsecureBaseURL(&self, strsecurebaseurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parse(&self, varinput: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn parseURL(&self, strurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IVBSAXXMLReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>() -> IVBSAXXMLReader_Vtbl {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getFeature(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putFeature(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putProperty(::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entityResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_entityResolver(::windows_core::from_raw_borrowed(&oresolver)).into()
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.contentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_contentHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_dtdHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorHandler() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_errorHandler(::windows_core::from_raw_borrowed(&ohandler)).into()
        }
        unsafe extern "system" fn baseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strbaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetbaseURL(::core::mem::transmute(&strbaseurl)).into()
        }
        unsafe extern "system" fn secureBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.secureBaseURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetsecureBaseURL(::core::mem::transmute(&strsecurebaseurl)).into()
        }
        unsafe extern "system" fn parse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.parse(::core::mem::transmute(&varinput)).into()
        }
        unsafe extern "system" fn parseURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn value(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>() -> IXMLAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, n: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(n, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(v, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLAttribute as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMAttribute_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn value(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Setvalue(&self, attributevalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMAttribute {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>() -> IXMLDOMAttribute_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setvalue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributevalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMCDATASection_Impl: Sized + IXMLDOMText_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMCDATASection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMCDATASection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCDATASection_Impl, const OFFSET: isize>() -> IXMLDOMCDATASection_Vtbl {
        Self { base__: IXMLDOMText_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMText as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMCharacterData_Impl: Sized + IXMLDOMNode_Impl {
    fn data(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setdata(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn substringData(&self, offset: i32, count: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn appendData(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn insertData(&self, offset: i32, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn deleteData(&self, offset: i32, count: i32) -> ::windows_core::Result<()>;
    fn replaceData(&self, offset: i32, count: i32, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMCharacterData {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>() -> IXMLDOMCharacterData_Vtbl {
        unsafe extern "system" fn data<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setdata(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datalength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substringData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.substringData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.appendData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn insertData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.insertData(::core::mem::transmute_copy(&offset), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn deleteData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.deleteData(::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn replaceData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMCharacterData as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMComment_Impl: Sized + IXMLDOMCharacterData_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMComment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMComment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMComment_Impl, const OFFSET: isize>() -> IXMLDOMComment_Vtbl {
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMComment as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument_Impl: Sized + IXMLDOMNode_Impl {
    fn doctype(&self) -> ::windows_core::Result<IXMLDOMDocumentType>;
    fn implementation(&self) -> ::windows_core::Result<IXMLDOMImplementation>;
    fn documentElement(&self) -> ::windows_core::Result<IXMLDOMElement>;
    fn putref_documentElement(&self, domelement: ::core::option::Option<&IXMLDOMElement>) -> ::windows_core::Result<()>;
    fn createElement(&self, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&self) -> ::windows_core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMText>;
    fn createComment(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMComment>;
    fn createCDATASection(&self, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&self, target: &::windows_core::BSTR, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&self, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn createNode(&self, r#type: &super::super::super::System::Variant::VARIANT, name: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeFromID(&self, idstring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn load(&self, xmlsource: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
    fn parseError(&self) -> ::windows_core::Result<IXMLDOMParseError>;
    fn url(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn r#async(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(&self, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn abort(&self) -> ::windows_core::Result<()>;
    fn loadXML(&self, bstrxml: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn save(&self, destination: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn validateOnParse(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetvalidateOnParse(&self, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn resolveExternals(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetresolveExternals(&self, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn preserveWhiteSpace(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetpreserveWhiteSpace(&self, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setonreadystatechange(&self, readystatechangesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Setondataavailable(&self, ondataavailablesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Setontransformnode(&self, ontransformnodesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>() -> IXMLDOMDocument_Vtbl {
        unsafe extern "system" fn doctype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documenttype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#impl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.implementation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#impl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.documentElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(domelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_documentElement(::windows_core::from_raw_borrowed(&domelement)).into()
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, docfrag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createDocumentFragment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docfrag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, text: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createTextNode(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, comment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createComment(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, cdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createCDATASection(::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows_core::BSTR>, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, pi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createProcessingInstruction(::core::mem::transmute(&target), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, attribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, entityref: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createEntityReference(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entityref, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::super::super::System::Variant::VARIANT, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createNode(::core::mem::transmute(&r#type), ::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeFromID(::core::mem::transmute(&idstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlsource: super::super::super::System::Variant::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.load(::core::mem::transmute(&xmlsource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parseError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parseError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(urlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#async() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setasync(::core::mem::transmute_copy(&isasync)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn loadXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.loadXML(::core::mem::transmute(&bstrxml)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destination: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.save(::core::mem::transmute(&destination)).into()
        }
        unsafe extern "system" fn validateOnParse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validateOnParse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isvalidating, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetvalidateOnParse(::core::mem::transmute_copy(&isvalidating)).into()
        }
        unsafe extern "system" fn resolveExternals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.resolveExternals() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isresolving, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetresolveExternals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetresolveExternals(::core::mem::transmute_copy(&isresolving)).into()
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.preserveWhiteSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispreserving, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetpreserveWhiteSpace(::core::mem::transmute_copy(&ispreserving)).into()
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, readystatechangesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setonreadystatechange(::core::mem::transmute(&readystatechangesink)).into()
        }
        unsafe extern "system" fn Setondataavailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ondataavailablesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setondataavailable(::core::mem::transmute(&ondataavailablesink)).into()
        }
        unsafe extern "system" fn Setontransformnode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ontransformnodesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument2_Impl: Sized + IXMLDOMDocument_Impl {
    fn namespaces(&self) -> ::windows_core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putref_schemas(&self, othercollection: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn validate(&self) -> ::windows_core::Result<IXMLDOMParseError>;
    fn setProperty(&self, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getProperty(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMDocument2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>() -> IXMLDOMDocument2_Vtbl {
        unsafe extern "system" fn namespaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespacecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.schemas() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(othercollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_schemas(::core::mem::transmute(&othercollection)).into()
        }
        unsafe extern "system" fn validate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMDocument as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument3_Impl: Sized + IXMLDOMDocument2_Impl {
    fn validateNode(&self, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMParseError>;
    fn importNode(&self, node: ::core::option::Option<&IXMLDOMNode>, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMDocument3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>() -> IXMLDOMDocument3_Vtbl {
        unsafe extern "system" fn validateNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validateNode(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, clone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.importNode(::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMDocument as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMDocument2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocumentFragment_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMDocumentFragment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMDocumentFragment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentFragment_Impl, const OFFSET: isize>() -> IXMLDOMDocumentFragment_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocumentType_Impl: Sized + IXMLDOMNode_Impl {
    fn name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn entities(&self) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&self) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMDocumentType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>() -> IXMLDOMDocumentType_Vtbl {
        unsafe extern "system" fn name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rootname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rootname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn entities<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.entities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entitymap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notationmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notationmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMElement_Impl: Sized + IXMLDOMNode_Impl {
    fn tagName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setAttribute(&self, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getAttributeNode(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&self, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&self, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&self, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn normalize(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>() -> IXMLDOMElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.tagName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tagname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn getAttributeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttributeNode(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.setAttributeNode(::windows_core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeAttributeNode(::windows_core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getElementsByTagName(::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMElement as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMEntity_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn systemId(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn notationName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMEntity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>() -> IXMLDOMEntity_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(systemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notationName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.notationName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
            notationName: notationName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntity as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMEntityReference_Impl: Sized + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMEntityReference {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMEntityReference_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMEntityReference_Impl, const OFFSET: isize>() -> IXMLDOMEntityReference_Vtbl {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMImplementation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(&self, feature: &::windows_core::BSTR, version: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMImplementation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>() -> IXMLDOMImplementation_Vtbl {
        unsafe extern "system" fn hasFeature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>, version: ::std::mem::MaybeUninit<::windows_core::BSTR>, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasFeature(::core::mem::transmute(&feature), ::core::mem::transmute(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), hasFeature: hasFeature::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNamedNodeMap_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn setNamedItem(&self, newitem: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeNamedItem(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn get_item(&self, index: i32) -> ::windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn getQualifiedItem(&self, basename: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&self, basename: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn nextNode(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMNamedNodeMap {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>() -> IXMLDOMNamedNodeMap_Vtbl {
        unsafe extern "system" fn getNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newitem: *mut ::core::ffi::c_void, nameitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.setNamedItem(::windows_core::from_raw_borrowed(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeNamedItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQualifiedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeQualifiedItem(::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNode_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn nodeValue(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetnodeValue(&self, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn nodeType(&self) -> ::windows_core::Result<DOMNodeType>;
    fn parentNode(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn childNodes(&self) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn firstChild(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn lastChild(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn previousSibling(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn nextSibling(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn attributes(&self) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(&self, newchild: ::core::option::Option<&IXMLDOMNode>, refchild: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLDOMNode>;
    fn replaceChild(&self, newchild: ::core::option::Option<&IXMLDOMNode>, oldchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeChild(&self, childnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn appendChild(&self, newchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn hasChildNodes(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn ownerDocument(&self) -> ::windows_core::Result<IXMLDOMDocument>;
    fn cloneNode(&self, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeTypeString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn text(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(&self, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn specified(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn definition(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetnodeTypedValue(&self, typedvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn dataType(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetdataType(&self, datatypename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn xml(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn transformNode(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn selectNodes(&self, querystring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&self, querystring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn parsed(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn namespaceURI(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn prefix(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn baseName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn transformNodeToObject(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>, outputobject: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMNode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>() -> IXMLDOMNode_Vtbl {
        unsafe extern "system" fn nodeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnodeValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn nodeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parentNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parentNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.childNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.firstChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.lastChild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previoussibling: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.previousSibling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previoussibling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextsibling: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextSibling() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextsibling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributemap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributemap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, refchild: super::super::super::System::Variant::VARIANT, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.insertBefore(::windows_core::from_raw_borrowed(&newchild), ::core::mem::transmute(&refchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void, outoldchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.replaceChild(::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outoldchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, oldchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeChild(::windows_core::from_raw_borrowed(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oldchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.appendChild(::windows_core::from_raw_borrowed(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.hasChildNodes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(haschild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ownerDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmldomdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, cloneroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.cloneNode(::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cloneroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeTypeString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn specified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.specified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn definition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, definitionnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.definition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(definitionnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nodeTypedValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typedvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetnodeTypedValue(::core::mem::transmute(&typedvalue)).into()
        }
        unsafe extern "system" fn dataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dataType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datatypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdataType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatypename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetdataType(::core::mem::transmute(&datatypename)).into()
        }
        unsafe extern "system" fn xml<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.xml() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.transformNode(::windows_core::from_raw_borrowed(&stylesheet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectNodes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selectNodes(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.selectSingleNode(::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parsed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isparsed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.namespaceURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefixstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.prefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.baseName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNodeToObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, outputobject: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.transformNodeToObject(::windows_core::from_raw_borrowed(&stylesheet), ::core::mem::transmute(&outputobject)).into()
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNodeList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn nextNode(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMNodeList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>() -> IXMLDOMNodeList_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.nextNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNotation_Impl: Sized + IXMLDOMNode_Impl {
    fn publicId(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn systemId(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMNotation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>() -> IXMLDOMNotation_Vtbl {
        unsafe extern "system" fn publicId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.publicId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.systemId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(systemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMNotation as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseError_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self) -> ::windows_core::Result<i32>;
    fn url(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn reason(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn srcText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn line(&self) -> ::windows_core::Result<i32>;
    fn linepos(&self) -> ::windows_core::Result<i32>;
    fn filepos(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMParseError {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>() -> IXMLDOMParseError_Vtbl {
        unsafe extern "system" fn errorCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.url() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(urlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reason<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reasonstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.reason() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reasonstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn srcText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.srcText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourcestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn line<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.line() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn linepos<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.linepos() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn filepos<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.filepos() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseError2_Impl: Sized + IXMLDOMParseError_Impl {
    fn errorXPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn allErrors(&self) -> ::windows_core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn errorParametersCount(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMParseError2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>() -> IXMLDOMParseError2_Vtbl {
        unsafe extern "system" fn errorXPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorXPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpathexpr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allErrors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allerrors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorParameters(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.errorParametersCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMParseError as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseErrorCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(&self, index: i32) -> ::windows_core::Result<IXMLDOMParseError2>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn next(&self) -> ::windows_core::Result<IXMLDOMParseError2>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMParseErrorCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>() -> IXMLDOMParseErrorCollection_Vtbl {
        unsafe extern "system" fn get_item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMProcessingInstruction_Impl: Sized + IXMLDOMNode_Impl {
    fn target(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn data(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setdata(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMProcessingInstruction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>() -> IXMLDOMProcessingInstruction_Vtbl {
        unsafe extern "system" fn target<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.target() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn data<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSchemaCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn add(&self, namespaceuri: &::windows_core::BSTR, var: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn get(&self, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn remove(&self, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn get_namespaceURI(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn addCollection(&self, othercollection: ::core::option::Option<&IXMLDOMSchemaCollection>) -> ::windows_core::Result<()>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMSchemaCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection_Vtbl {
        unsafe extern "system" fn add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, var: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.add(::core::mem::transmute(&namespaceuri), ::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn get<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, schemanode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemanode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.remove(::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_namespaceURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_namespaceURI(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othercollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addCollection(::windows_core::from_raw_borrowed(&othercollection)).into()
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSchemaCollection2_Impl: Sized + IXMLDOMSchemaCollection_Impl {
    fn validate(&self) -> ::windows_core::Result<()>;
    fn SetvalidateOnLoad(&self, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn validateOnLoad(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getSchema(&self, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<ISchema>;
    fn getDeclaration(&self, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMSchemaCollection2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>() -> IXMLDOMSchemaCollection2_Vtbl {
        unsafe extern "system" fn validate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.validate().into()
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetvalidateOnLoad(::core::mem::transmute_copy(&validateonload)).into()
        }
        unsafe extern "system" fn validateOnLoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validateonload: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.validateOnLoad() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validateonload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, schema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getSchema(::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getDeclaration(::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMSchemaCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSelection_Impl: Sized + IXMLDOMNodeList_Impl {
    fn expr(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setexpr(&self, expression: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn context(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn putref_context(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<()>;
    fn peekNode(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn matches(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeNext(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeAll(&self) -> ::windows_core::Result<()>;
    fn clone(&self) -> ::windows_core::Result<IXMLDOMSelection>;
    fn getProperty(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setProperty(&self, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMSelection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>() -> IXMLDOMSelection_Vtbl {
        unsafe extern "system" fn expr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.expr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expression, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expression: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setexpr(::core::mem::transmute(&expression)).into()
        }
        unsafe extern "system" fn context<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.context() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_context(::windows_core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn peekNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.peekNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.matches(::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.removeNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAll().into()
        }
        unsafe extern "system" fn clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMSelection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNodeList as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMText_Impl: Sized + IXMLDOMCharacterData_Impl {
    fn splitText(&self, offset: i32) -> ::windows_core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDOMText {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>() -> IXMLDOMText_Vtbl {
        unsafe extern "system" fn splitText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.splitText(::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(righthandtextnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, Impl, OFFSET>(), splitText: splitText::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDOMText as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMCharacterData as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDSOControl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> ::windows_core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&self, ppdoc: ::core::option::Option<&IXMLDOMDocument>) -> ::windows_core::Result<()>;
    fn JavaDSOCompatible(&self) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetJavaDSOCompatible(&self, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDSOControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>() -> IXMLDSOControl_Vtbl {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.XMLDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXMLDocument(::windows_core::from_raw_borrowed(&ppdoc)).into()
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JavaDSOCompatible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fjavadsocompatible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJavaDSOCompatible(::core::mem::transmute_copy(&fjavadsocompatible)).into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDSOControl as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDocument_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows_core::Result<IXMLElement>;
    fn fileSize(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileModifiedDate(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn URL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetURL(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn mimeType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
    fn charset(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setcharset(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn doctype(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn dtdURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Variant::VARIANT, var1: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>() -> IXMLDocument_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.root() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.charset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Variant::VARIANT, var1: super::super::super::System::Variant::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDocument as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDocument2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn root(&self) -> ::windows_core::Result<IXMLElement2>;
    fn fileSize(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileModifiedDate(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn URL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetURL(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn mimeType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
    fn charset(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setcharset(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn doctype(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn dtdURL(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn createElement(&self, vtype: &super::super::super::System::Variant::VARIANT, var1: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLElement2>;
    fn r#async(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(&self, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLDocument2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>() -> IXMLDocument2_Vtbl {
        unsafe extern "system" fn root<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.root() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileModifiedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.fileUpdatedDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.URL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURL(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn mimeType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.mimeType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.charset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setcharset(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.doctype() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.dtdURL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Variant::VARIANT, var1: super::super::super::System::Variant::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createElement(::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pf: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#async() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLDocument2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElement_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettagName(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parent(&self) -> ::windows_core::Result<IXMLElement>;
    fn setAttribute(&self, strpropertyname: &::windows_core::BSTR, propertyvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn children(&self) -> ::windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows_core::Result<i32>;
    fn text(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addChild(&self, pchildelem: ::core::option::Option<&IXMLElement>, lindex: i32, lreserved: i32) -> ::windows_core::Result<()>;
    fn removeChild(&self, pchildelem: ::core::option::Option<&IXMLElement>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>() -> IXMLElement_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.tagName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addChild(::windows_core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeChild(::windows_core::from_raw_borrowed(&pchildelem)).into()
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLElement as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElement2_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettagName(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parent(&self) -> ::windows_core::Result<IXMLElement2>;
    fn setAttribute(&self, strpropertyname: &::windows_core::BSTR, propertyvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn children(&self) -> ::windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> ::windows_core::Result<i32>;
    fn text(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(&self, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addChild(&self, pchildelem: ::core::option::Option<&IXMLElement2>, lindex: i32, lreserved: i32) -> ::windows_core::Result<()>;
    fn removeChild(&self, pchildelem: ::core::option::Option<&IXMLElement2>) -> ::windows_core::Result<()>;
    fn attributes(&self) -> ::windows_core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLElement2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>() -> IXMLElement2_Vtbl {
        unsafe extern "system" fn tagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.tagName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SettagName(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAttribute(::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAttribute(::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeAttribute(::core::mem::transmute(&strpropertyname)).into()
        }
        unsafe extern "system" fn children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.r#type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Settext(::core::mem::transmute(&p)).into()
        }
        unsafe extern "system" fn addChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addChild(::windows_core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into()
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.removeChild(::windows_core::from_raw_borrowed(&pchildelem)).into()
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLElement2 as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElementCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(&self, v: i32) -> ::windows_core::Result<()>;
    fn length(&self) -> ::windows_core::Result<i32>;
    fn _newEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn item(&self, var1: &super::super::super::System::Variant::VARIANT, var2: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLElementCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>() -> IXMLElementCollection_Vtbl {
        unsafe extern "system" fn Setlength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setlength(::core::mem::transmute_copy(&v)).into()
        }
        unsafe extern "system" fn length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._newEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var1: super::super::super::System::Variant::VARIANT, var2: super::super::super::System::Variant::VARIANT, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.item(::core::mem::transmute(&var1), ::core::mem::transmute(&var2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLElementCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"implement\"`*"]
pub trait IXMLError_Impl: Sized {
    fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXMLError {}
impl IXMLError_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>() -> IXMLError_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorInfo(::core::mem::transmute_copy(&perrorreturn)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLError as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLHTTPRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn open(&self, bstrmethod: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, varasync: &super::super::super::System::Variant::VARIANT, bstruser: &super::super::super::System::Variant::VARIANT, bstrpassword: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getAllResponseHeaders(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn send(&self, varbody: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn abort(&self) -> ::windows_core::Result<()>;
    fn status(&self) -> ::windows_core::Result<i32>;
    fn statusText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn responseXML(&self) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn responseBody(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn responseStream(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: ::core::option::Option<&super::super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXMLHTTPRequest {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>() -> IXMLHTTPRequest_Vtbl {
        unsafe extern "system" fn open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethod: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, varasync: super::super::super::System::Variant::VARIANT, bstruser: super::super::super::System::Variant::VARIANT, bstrpassword: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.open(::core::mem::transmute(&bstrmethod), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&varasync), ::core::mem::transmute(&bstruser), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn setRequestHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setRequestHeader(::core::mem::transmute(&bstrheader), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn getResponseHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getResponseHeader(::core::mem::transmute(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.send(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.abort().into()
        }
        unsafe extern "system" fn status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.statusText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbody: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseXML() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseBody() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.responseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystatesink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setonreadystatechange(::windows_core::from_raw_borrowed(&preadystatesink)).into()
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2_Impl: Sized {
    fn Open(&self, pwszmethod: &::windows_core::PCWSTR, pwszurl: &::windows_core::PCWSTR, pstatuscallback: ::core::option::Option<&IXMLHTTPRequest2Callback>, pwszusername: &::windows_core::PCWSTR, pwszpassword: &::windows_core::PCWSTR, pwszproxyusername: &::windows_core::PCWSTR, pwszproxypassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Send(&self, pbody: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> ::windows_core::Result<u32>;
    fn SetCustomResponseStream(&self, psequentialstream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows_core::Result<()>;
    fn SetRequestHeader(&self, pwszheader: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAllResponseHeaders(&self) -> ::windows_core::Result<*mut u16>;
    fn GetCookie(&self, pwszurl: &::windows_core::PCWSTR, pwszname: &::windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows_core::Result<()>;
    fn GetResponseHeader(&self, pwszheader: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXMLHTTPRequest2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmethod: ::windows_core::PCWSTR, pwszurl: ::windows_core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pwszusername: ::windows_core::PCWSTR, pwszpassword: ::windows_core::PCWSTR, pwszproxyusername: ::windows_core::PCWSTR, pwszproxypassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszmethod), ::core::mem::transmute(&pwszurl), ::windows_core::from_raw_borrowed(&pstatuscallback), ::core::mem::transmute(&pwszusername), ::core::mem::transmute(&pwszpassword), ::core::mem::transmute(&pwszproxyusername), ::core::mem::transmute(&pwszproxypassword)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut ::core::ffi::c_void, cbbody: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows_core::from_raw_borrowed(&pbody), ::core::mem::transmute_copy(&cbbody)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn SetCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetCookie(::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookiestate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psequentialstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustomResponseStream(::windows_core::from_raw_borrowed(&psequentialstream)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&eproperty), ::core::mem::transmute_copy(&ullvalue)).into()
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestHeader(::core::mem::transmute(&pwszheader), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllResponseHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pwszname: ::windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCookie(::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)).into()
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows_core::PCWSTR, ppwszvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResponseHeader(::core::mem::transmute(&pwszheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2Callback_Impl: Sized {
    fn OnRedirect(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, pwszredirecturl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnHeadersAvailable(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDataAvailable(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn OnResponseReceived(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn OnError(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IXMLHTTPRequest2Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest2Callback_Vtbl {
        unsafe extern "system" fn OnRedirect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, pwszredirecturl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRedirect(::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute(&pwszredirecturl)).into()
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwstatus: u32, pwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHeadersAvailable(::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute(&pwszstatus)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataAvailable(::windows_core::from_raw_borrowed(&pxhr), ::windows_core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnResponseReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResponseReceived(::windows_core::from_raw_borrowed(&pxhr), ::windows_core::from_raw_borrowed(&presponsestream)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, Impl, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3_Impl: Sized + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXMLHTTPRequest3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3_Vtbl {
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientCertificate(::core::mem::transmute_copy(&cbclientcertificatehash), ::core::mem::transmute_copy(&pbclientcertificatehash), ::core::mem::transmute(&pwszpin)).into()
        }
        Self { base__: IXMLHTTPRequest2_Vtbl::new::<Identity, Impl, OFFSET>(), SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as ::windows_core::ComInterface>::IID || iid == &<IXMLHTTPRequest2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3Callback_Impl: Sized + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows_core::Result<()>;
    fn OnClientCertificateRequested(&self, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IXMLHTTPRequest3Callback {}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>() -> IXMLHTTPRequest3Callback_Vtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnServerCertificateReceived(::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwcertificateerrors), ::core::mem::transmute_copy(&cservercertificatechain), ::core::mem::transmute_copy(&rgservercertificatechain)).into()
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClientCertificateRequested(::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&cissuerlist), ::core::mem::transmute_copy(&rgpwszissuerlist)).into()
        }
        Self {
            base__: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, Impl, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as ::windows_core::ComInterface>::IID || iid == &<IXMLHTTPRequest2Callback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXSLProcessor_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(&self, var: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn input(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn ownerTemplate(&self) -> ::windows_core::Result<IXSLTemplate>;
    fn setStartMode(&self, mode: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startMode(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn startModeURI(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setoutput(&self, output: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn output(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn transform(&self) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(&self) -> ::windows_core::Result<()>;
    fn readyState(&self) -> ::windows_core::Result<i32>;
    fn addParameter(&self, basename: &::windows_core::BSTR, parameter: &super::super::super::System::Variant::VARIANT, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addObject(&self, obj: ::core::option::Option<&super::super::super::System::Com::IDispatch>, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn stylesheet(&self) -> ::windows_core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXSLProcessor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>() -> IXSLProcessor_Vtbl {
        unsafe extern "system" fn Setinput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setinput(::core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn input<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.input() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ownerTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setStartMode(::core::mem::transmute(&mode), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn startMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.startMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.startModeURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Setoutput(::core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn output<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.output() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdone: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.transform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.reset().into()
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.readyState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preadystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, parameter: super::super::super::System::Variant::VARIANT, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addParameter(::core::mem::transmute(&basename), ::core::mem::transmute(&parameter), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn addObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.addObject(::windows_core::from_raw_borrowed(&obj), ::core::mem::transmute(&namespaceuri)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXSLProcessor as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXSLTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(&self, stylesheet: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<()>;
    fn stylesheet(&self) -> ::windows_core::Result<IXMLDOMNode>;
    fn createProcessor(&self) -> ::windows_core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXSLTemplate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>() -> IXSLTemplate_Vtbl {
        unsafe extern "system" fn putref_stylesheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_stylesheet(::windows_core::from_raw_borrowed(&stylesheet)).into()
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.stylesheet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.createProcessor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprocessor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXSLTemplate as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXTLRuntime_Impl: Sized + IXMLDOMNode_Impl {
    fn uniqueID(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn depth(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn childNumber(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn ancestorChildNumber(&self, bstrnodename: &::windows_core::BSTR, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn absoluteChildNumber(&self, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn formatIndex(&self, lindex: i32, bstrformat: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatNumber(&self, dblnumber: f64, bstrformat: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatDate(&self, vardate: &super::super::super::System::Variant::VARIANT, bstrformat: &::windows_core::BSTR, vardestlocale: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatTime(&self, vartime: &super::super::super::System::Variant::VARIANT, bstrformat: &::windows_core::BSTR, vardestlocale: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IXTLRuntime {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>() -> IXTLRuntime_Vtbl {
        unsafe extern "system" fn uniqueID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.uniqueID(::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn depth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pdepth: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.depth(::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.childNumber(::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnodename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ancestorChildNumber(::core::mem::transmute(&bstrnodename), ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.absoluteChildNumber(::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.formatIndex(::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.formatNumber(::core::mem::transmute_copy(&dblnumber), ::core::mem::transmute(&bstrformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardate: super::super::super::System::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, vardestlocale: super::super::super::System::Variant::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.formatDate(::core::mem::transmute(&vardate), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartime: super::super::super::System::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, vardestlocale: super::super::super::System::Variant::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.formatTime(::core::mem::transmute(&vartime), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXTLRuntime as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IXMLDOMNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait XMLDOMDocumentEvents_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for XMLDOMDocumentEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl XMLDOMDocumentEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: XMLDOMDocumentEvents_Impl, const OFFSET: isize>() -> XMLDOMDocumentEvents_Vtbl {
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
