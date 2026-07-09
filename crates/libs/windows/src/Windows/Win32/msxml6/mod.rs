pub const DOMDocument60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a05_f192_11d4_a65f_0040963251e5);
pub const E_XML_BUFFERTOOSMALL: u32 = 3222069798;
pub const E_XML_INVALID: u32 = 3222069797;
pub const E_XML_NODTD: u32 = 3222069796;
pub const E_XML_NOTWF: u32 = 3222069795;
pub const FreeThreadedDOMDocument60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a06_f192_11d4_a65f_0040963251e5);
pub const FreeThreadedXMLHTTP60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a09_f192_11d4_a65f_0040963251e5);
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXAttributes, IMXAttributes_Vtbl, 0xf10d27cc_3ec0_415c_8ed8_77ab1c5e7262);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXAttributes {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXAttributes, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXAttributes {
    pub unsafe fn addAttribute(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).addAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(struri), core::mem::transmute_copy(strlocalname), core::mem::transmute_copy(strqname), core::mem::transmute_copy(strtype), core::mem::transmute_copy(strvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn addAttributeFromIndex(&self, varatts: &super::oaidl::VARIANT, nindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).addAttributeFromIndex)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varatts), nindex) }
    }
    pub unsafe fn clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn removeAttribute(&self, nindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), nindex) }
    }
    pub unsafe fn setAttribute(&self, nindex: i32, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(struri), core::mem::transmute_copy(strlocalname), core::mem::transmute_copy(strqname), core::mem::transmute_copy(strtype), core::mem::transmute_copy(strvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setAttributes(&self, varatts: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setAttributes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varatts)) }
    }
    pub unsafe fn setLocalName(&self, nindex: i32, strlocalname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setLocalName)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(strlocalname)) }
    }
    pub unsafe fn setQName(&self, nindex: i32, strqname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setQName)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(strqname)) }
    }
    pub unsafe fn setType(&self, nindex: i32, strtype: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setType)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(strtype)) }
    }
    pub unsafe fn setURI(&self, nindex: i32, struri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setURI)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(struri)) }
    }
    pub unsafe fn setValue(&self, nindex: i32, strvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setValue)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute_copy(strvalue)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXAttributes_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub addAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub addAttributeFromIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    addAttributeFromIndex: usize,
    pub clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setAttributes: usize,
    pub setLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setQName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setURI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXAttributes_Impl: super::oaidl::IDispatch_Impl {
    fn addAttribute(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addAttributeFromIndex(&self, varatts: &super::oaidl::VARIANT, nindex: i32) -> windows_core::Result<()>;
    fn clear(&self) -> windows_core::Result<()>;
    fn removeAttribute(&self, nindex: i32) -> windows_core::Result<()>;
    fn setAttribute(&self, nindex: i32, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR, strqname: &windows_core::BSTR, strtype: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setAttributes(&self, varatts: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn setLocalName(&self, nindex: i32, strlocalname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setQName(&self, nindex: i32, strqname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setType(&self, nindex: i32, strtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setURI(&self, nindex: i32, struri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setValue(&self, nindex: i32, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXAttributes_Vtbl {
    pub const fn new<Identity: IMXAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn addAttribute<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: *mut core::ffi::c_void, strlocalname: *mut core::ffi::c_void, strqname: *mut core::ffi::c_void, strtype: *mut core::ffi::c_void, strvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::addAttribute(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname), core::mem::transmute(&strqname), core::mem::transmute(&strtype), core::mem::transmute(&strvalue)).into()
            }
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varatts: super::oaidl::VARIANT, nindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::addAttributeFromIndex(this, core::mem::transmute(&varatts), core::mem::transmute_copy(&nindex)).into()
            }
        }
        unsafe extern "system" fn clear<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::clear(this).into()
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::removeAttribute(this, core::mem::transmute_copy(&nindex)).into()
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: *mut core::ffi::c_void, strlocalname: *mut core::ffi::c_void, strqname: *mut core::ffi::c_void, strtype: *mut core::ffi::c_void, strvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setAttribute(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&struri), core::mem::transmute(&strlocalname), core::mem::transmute(&strqname), core::mem::transmute(&strtype), core::mem::transmute(&strvalue)).into()
            }
        }
        unsafe extern "system" fn setAttributes<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varatts: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setAttributes(this, core::mem::transmute(&varatts)).into()
            }
        }
        unsafe extern "system" fn setLocalName<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strlocalname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setLocalName(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strlocalname)).into()
            }
        }
        unsafe extern "system" fn setQName<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strqname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setQName(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strqname)).into()
            }
        }
        unsafe extern "system" fn setType<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strtype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setType(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strtype)).into()
            }
        }
        unsafe extern "system" fn setURI<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setURI(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&struri)).into()
            }
        }
        unsafe extern "system" fn setValue<Identity: IMXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXAttributes_Impl::setValue(this, core::mem::transmute_copy(&nindex), core::mem::transmute(&strvalue)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            addAttribute: addAttribute::<Identity, OFFSET>,
            addAttributeFromIndex: addAttributeFromIndex::<Identity, OFFSET>,
            clear: clear::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            setAttributes: setAttributes::<Identity, OFFSET>,
            setLocalName: setLocalName::<Identity, OFFSET>,
            setQName: setQName::<Identity, OFFSET>,
            setType: setType::<Identity, OFFSET>,
            setURI: setURI::<Identity, OFFSET>,
            setValue: setValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXAttributes as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXAttributes {}
windows_core::imp::define_interface!(IMXNamespaceManager, IMXNamespaceManager_Vtbl, 0xc90352f6_643c_4fbc_bb23_e996eb2d51fd);
windows_core::imp::interface_hierarchy!(IMXNamespaceManager, windows_core::IUnknown);
impl IMXNamespaceManager {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn putAllowOverride(&self, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putAllowOverride)(windows_core::Interface::as_raw(self), foverride) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn getAllowOverride(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAllowOverride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn pushContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).pushContext)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes"))]
    pub unsafe fn pushNodeContext<P0>(&self, contextnode: P0, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).pushNodeContext)(windows_core::Interface::as_raw(self), contextnode.param().abi(), fdeep) }
    }
    pub unsafe fn popContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).popContext)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn declarePrefix<P0, P1>(&self, prefix: P0, namespaceuri: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).declarePrefix)(windows_core::Interface::as_raw(self), prefix.param().abi(), namespaceuri.param().abi()) }
    }
    pub unsafe fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getDeclaredPrefix)(windows_core::Interface::as_raw(self), nindex, pwchprefix as _, pcchprefix as _) }
    }
    pub unsafe fn getPrefix<P0>(&self, pwsznamespaceuri: P0, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).getPrefix)(windows_core::Interface::as_raw(self), pwsznamespaceuri.param().abi(), nindex, pwchprefix as _, pcchprefix as _) }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub unsafe fn getURI<P0, P1>(&self, pwchprefix: P0, pcontextnode: P1, pwchuri: *mut u16, pcchuri: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).getURI)(windows_core::Interface::as_raw(self), pwchprefix.param().abi(), pcontextnode.param().abi(), pwchuri as _, pcchuri as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMXNamespaceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub putAllowOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    putAllowOverride: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub getAllowOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    getAllowOverride: usize,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub pushContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes"))]
    pub pushNodeContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes")))]
    pushNodeContext: usize,
    pub popContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub declarePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub getDeclaredPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut u16, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub getURI: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u16, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_oaidl")))]
    getURI: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes"))]
pub trait IMXNamespaceManager_Impl: windows_core::IUnknownImpl {
    fn putAllowOverride(&self, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAllowOverride(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn pushContext(&self) -> windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: windows_core::Ref<super::msxml::IXMLDOMNode>, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn popContext(&self) -> windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &windows_core::PCWSTR, namespaceuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn getDeclaredPrefix(&self, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::Result<()>;
    fn getPrefix(&self, pwsznamespaceuri: &windows_core::PCWSTR, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::Result<()>;
    fn getURI(&self, pwchprefix: &windows_core::PCWSTR, pcontextnode: windows_core::Ref<super::msxml::IXMLDOMNode>, pwchuri: *mut u16, pcchuri: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes"))]
impl IMXNamespaceManager_Vtbl {
    pub const fn new<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn putAllowOverride<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::putAllowOverride(this, core::mem::transmute_copy(&foverride)).into()
            }
        }
        unsafe extern "system" fn getAllowOverride<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXNamespaceManager_Impl::getAllowOverride(this) {
                    Ok(ok__) => {
                        foverride.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn pushContext<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::pushContext(this).into()
            }
        }
        unsafe extern "system" fn pushNodeContext<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextnode: *mut core::ffi::c_void, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::pushNodeContext(this, core::mem::transmute_copy(&contextnode), core::mem::transmute_copy(&fdeep)).into()
            }
        }
        unsafe extern "system" fn popContext<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::popContext(this).into()
            }
        }
        unsafe extern "system" fn declarePrefix<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: windows_core::PCWSTR, namespaceuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::declarePrefix(this, core::mem::transmute(&prefix), core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::getDeclaredPrefix(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&pcchprefix)).into()
            }
        }
        unsafe extern "system" fn getPrefix<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsznamespaceuri: windows_core::PCWSTR, nindex: i32, pwchprefix: *mut u16, pcchprefix: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::getPrefix(this, core::mem::transmute(&pwsznamespaceuri), core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&pcchprefix)).into()
            }
        }
        unsafe extern "system" fn getURI<Identity: IMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: windows_core::PCWSTR, pcontextnode: *mut core::ffi::c_void, pwchuri: *mut u16, pcchuri: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXNamespaceManager_Impl::getURI(this, core::mem::transmute(&pwchprefix), core::mem::transmute_copy(&pcontextnode), core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&pcchuri)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            putAllowOverride: putAllowOverride::<Identity, OFFSET>,
            getAllowOverride: getAllowOverride::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            pushContext: pushContext::<Identity, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, OFFSET>,
            popContext: popContext::<Identity, OFFSET>,
            declarePrefix: declarePrefix::<Identity, OFFSET>,
            getDeclaredPrefix: getDeclaredPrefix::<Identity, OFFSET>,
            getPrefix: getPrefix::<Identity, OFFSET>,
            getURI: getURI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXNamespaceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_wtypes"))]
impl windows_core::RuntimeName for IMXNamespaceManager {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXNamespacePrefixes, IMXNamespacePrefixes_Vtbl, 0xc90352f4_643c_4fbc_bb23_e996eb2d51fd);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXNamespacePrefixes {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXNamespacePrefixes, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXNamespacePrefixes {
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._newEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXNamespacePrefixes_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXNamespacePrefixes_Impl: super::oaidl::IDispatch_Impl {
    fn item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXNamespacePrefixes_Vtbl {
    pub const fn new<Identity: IMXNamespacePrefixes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn item<Identity: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, prefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXNamespacePrefixes_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        prefix.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXNamespacePrefixes_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IMXNamespacePrefixes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXNamespacePrefixes_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            item: item::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXNamespacePrefixes as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXNamespacePrefixes {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXReaderControl, IMXReaderControl_Vtbl, 0x808f4e35_8d5a_4fbe_8466_33a41279ed30);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXReaderControl {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXReaderControl, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXReaderControl {
    pub unsafe fn abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).abort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).suspend)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXReaderControl_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXReaderControl_Impl: super::oaidl::IDispatch_Impl {
    fn abort(&self) -> windows_core::Result<()>;
    fn resume(&self) -> windows_core::Result<()>;
    fn suspend(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXReaderControl_Vtbl {
    pub const fn new<Identity: IMXReaderControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn abort<Identity: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXReaderControl_Impl::abort(this).into()
            }
        }
        unsafe extern "system" fn resume<Identity: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXReaderControl_Impl::resume(this).into()
            }
        }
        unsafe extern "system" fn suspend<Identity: IMXReaderControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXReaderControl_Impl::suspend(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            abort: abort::<Identity, OFFSET>,
            resume: resume::<Identity, OFFSET>,
            suspend: suspend::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXReaderControl as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXReaderControl {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXSchemaDeclHandler, IMXSchemaDeclHandler_Vtbl, 0xfa4bb38c_faf9_4cca_9302_d1dd0fe520db);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXSchemaDeclHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXSchemaDeclHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXSchemaDeclHandler {
    pub unsafe fn schemaElementDecl<P0>(&self, oschemaelement: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISchemaElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).schemaElementDecl)(windows_core::Interface::as_raw(self), oschemaelement.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXSchemaDeclHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub schemaElementDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXSchemaDeclHandler_Impl: super::oaidl::IDispatch_Impl {
    fn schemaElementDecl(&self, oschemaelement: windows_core::Ref<ISchemaElement>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXSchemaDeclHandler_Vtbl {
    pub const fn new<Identity: IMXSchemaDeclHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn schemaElementDecl<Identity: IMXSchemaDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oschemaelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXSchemaDeclHandler_Impl::schemaElementDecl(this, core::mem::transmute_copy(&oschemaelement)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), schemaElementDecl: schemaElementDecl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXSchemaDeclHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXSchemaDeclHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXWriter, IMXWriter_Vtbl, 0x4d7ff4ba_1565_4ea8_94e1_6e724a46f98d);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXWriter {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXWriter, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXWriter {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setoutput(&self, vardestination: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setoutput)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardestination)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn output(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).output)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setencoding(&self, strencoding: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setencoding)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strencoding)) }
    }
    pub unsafe fn encoding(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).encoding)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetbyteOrderMark(&self, fwritebyteordermark: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetbyteOrderMark)(windows_core::Interface::as_raw(self), fwritebyteordermark) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn byteOrderMark(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).byteOrderMark)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Setindent(&self, findentmode: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setindent)(windows_core::Interface::as_raw(self), findentmode) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn indent(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).indent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Setstandalone(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setstandalone)(windows_core::Interface::as_raw(self), fvalue) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn standalone(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).standalone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetomitXMLDeclaration(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetomitXMLDeclaration)(windows_core::Interface::as_raw(self), fvalue) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn omitXMLDeclaration(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).omitXMLDeclaration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Setversion(&self, strversion: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setversion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strversion)) }
    }
    pub unsafe fn version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetdisableOutputEscaping(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetdisableOutputEscaping)(windows_core::Interface::as_raw(self), fvalue) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn disableOutputEscaping(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).disableOutputEscaping)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).flush)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXWriter_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setoutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setoutput: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub output: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    output: usize,
    pub Setencoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub encoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub SetbyteOrderMark: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetbyteOrderMark: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub byteOrderMark: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    byteOrderMark: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Setindent: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Setindent: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub indent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    indent: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Setstandalone: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Setstandalone: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub standalone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    standalone: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetomitXMLDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetomitXMLDeclaration: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub omitXMLDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    omitXMLDeclaration: usize,
    pub Setversion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub SetdisableOutputEscaping: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetdisableOutputEscaping: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub disableOutputEscaping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    disableOutputEscaping: usize,
    pub flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXWriter_Impl: super::oaidl::IDispatch_Impl {
    fn Setoutput(&self, vardestination: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn output(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Setencoding(&self, strencoding: &windows_core::BSTR) -> windows_core::Result<()>;
    fn encoding(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetbyteOrderMark(&self, fwritebyteordermark: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn byteOrderMark(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Setindent(&self, findentmode: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn indent(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Setstandalone(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn standalone(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetomitXMLDeclaration(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn omitXMLDeclaration(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Setversion(&self, strversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdisableOutputEscaping(&self, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn disableOutputEscaping(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn flush(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXWriter_Vtbl {
    pub const fn new<Identity: IMXWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Setoutput<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestination: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::Setoutput(this, core::mem::transmute(&vardestination)).into()
            }
        }
        unsafe extern "system" fn output<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestination: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::output(this) {
                    Ok(ok__) => {
                        vardestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setencoding<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencoding: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::Setencoding(this, core::mem::transmute(&strencoding)).into()
            }
        }
        unsafe extern "system" fn encoding<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencoding: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::encoding(this) {
                    Ok(ok__) => {
                        strencoding.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwritebyteordermark: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::SetbyteOrderMark(this, core::mem::transmute_copy(&fwritebyteordermark)).into()
            }
        }
        unsafe extern "system" fn byteOrderMark<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwritebyteordermark: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::byteOrderMark(this) {
                    Ok(ok__) => {
                        fwritebyteordermark.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setindent<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findentmode: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::Setindent(this, core::mem::transmute_copy(&findentmode)).into()
            }
        }
        unsafe extern "system" fn indent<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, findentmode: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::indent(this) {
                    Ok(ok__) => {
                        findentmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setstandalone<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::Setstandalone(this, core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn standalone<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::standalone(this) {
                    Ok(ok__) => {
                        fvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::SetomitXMLDeclaration(this, core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::omitXMLDeclaration(this) {
                    Ok(ok__) => {
                        fvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setversion<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strversion: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::Setversion(this, core::mem::transmute(&strversion)).into()
            }
        }
        unsafe extern "system" fn version<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::version(this) {
                    Ok(ok__) => {
                        strversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::SetdisableOutputEscaping(this, core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXWriter_Impl::disableOutputEscaping(this) {
                    Ok(ok__) => {
                        fvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn flush<Identity: IMXWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXWriter_Impl::flush(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Setoutput: Setoutput::<Identity, OFFSET>,
            output: output::<Identity, OFFSET>,
            Setencoding: Setencoding::<Identity, OFFSET>,
            encoding: encoding::<Identity, OFFSET>,
            SetbyteOrderMark: SetbyteOrderMark::<Identity, OFFSET>,
            byteOrderMark: byteOrderMark::<Identity, OFFSET>,
            Setindent: Setindent::<Identity, OFFSET>,
            indent: indent::<Identity, OFFSET>,
            Setstandalone: Setstandalone::<Identity, OFFSET>,
            standalone: standalone::<Identity, OFFSET>,
            SetomitXMLDeclaration: SetomitXMLDeclaration::<Identity, OFFSET>,
            omitXMLDeclaration: omitXMLDeclaration::<Identity, OFFSET>,
            Setversion: Setversion::<Identity, OFFSET>,
            version: version::<Identity, OFFSET>,
            SetdisableOutputEscaping: SetdisableOutputEscaping::<Identity, OFFSET>,
            disableOutputEscaping: disableOutputEscaping::<Identity, OFFSET>,
            flush: flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXWriter as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXWriter {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IMXXMLFilter, IMXXMLFilter_Vtbl, 0xc90352f7_643c_4fbc_bb23_e996eb2d51fd);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IMXXMLFilter {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IMXXMLFilter, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IMXXMLFilter {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), fvalue) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute_copy(varvalue)) }
    }
    pub unsafe fn entityResolver(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).entityResolver)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_entityResolver<P0>(&self, oresolver: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_entityResolver)(windows_core::Interface::as_raw(self), oresolver.param().abi()) }
    }
    pub unsafe fn contentHandler(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_contentHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_contentHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
    pub unsafe fn dtdHandler(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dtdHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_dtdHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_dtdHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
    pub unsafe fn errorHandler(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_errorHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_errorHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMXXMLFilter_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub getFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    getFeature: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub putFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub putProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    putProperty: usize,
    pub entityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_entityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub contentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_contentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dtdHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_dtdHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub errorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_errorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMXXMLFilter_Impl: super::oaidl::IDispatch_Impl {
    fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn entityResolver(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_entityResolver(&self, oresolver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn contentHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_contentHandler(&self, ohandler: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn dtdHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_dtdHandler(&self, ohandler: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn errorHandler(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_errorHandler(&self, ohandler: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMXXMLFilter_Vtbl {
    pub const fn new<Identity: IMXXMLFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getFeature<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, fvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::getFeature(this, core::mem::transmute(&strname)) {
                    Ok(ok__) => {
                        fvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putFeature<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putFeature(this, core::mem::transmute(&strname), core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn getProperty<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::getProperty(this, core::mem::transmute(&strname)) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putProperty<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putProperty(this, core::mem::transmute(&strname), core::mem::transmute(&varvalue)).into()
            }
        }
        unsafe extern "system" fn entityResolver<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::entityResolver(this) {
                    Ok(ok__) => {
                        oresolver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putref_entityResolver(this, core::mem::transmute_copy(&oresolver)).into()
            }
        }
        unsafe extern "system" fn contentHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::contentHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putref_contentHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        unsafe extern "system" fn dtdHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::dtdHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putref_dtdHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        unsafe extern "system" fn errorHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMXXMLFilter_Impl::errorHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: IMXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMXXMLFilter_Impl::putref_errorHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            getFeature: getFeature::<Identity, OFFSET>,
            putFeature: putFeature::<Identity, OFFSET>,
            getProperty: getProperty::<Identity, OFFSET>,
            putProperty: putProperty::<Identity, OFFSET>,
            entityResolver: entityResolver::<Identity, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, OFFSET>,
            contentHandler: contentHandler::<Identity, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, OFFSET>,
            dtdHandler: dtdHandler::<Identity, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, OFFSET>,
            errorHandler: errorHandler::<Identity, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMXXMLFilter as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMXXMLFilter {}
windows_core::imp::define_interface!(ISAXAttributes, ISAXAttributes_Vtbl, 0xf078abe1_45d2_4832_91ea_4466ce2f25c9);
windows_core::imp::interface_hierarchy!(ISAXAttributes, windows_core::IUnknown);
impl ISAXAttributes {
    pub unsafe fn getLength(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getURI)(windows_core::Interface::as_raw(self), nindex, ppwchuri as _, pcchuri as _) }
    }
    pub unsafe fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getLocalName)(windows_core::Interface::as_raw(self), nindex, ppwchlocalname as _, pcchlocalname as _) }
    }
    pub unsafe fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getQName)(windows_core::Interface::as_raw(self), nindex, ppwchqname as _, pcchqname as _) }
    }
    pub unsafe fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getName)(windows_core::Interface::as_raw(self), nindex, ppwchuri as _, pcchuri as _, ppwchlocalname as _, pcchlocalname as _, ppwchqname as _, pcchqname as _) }
    }
    pub unsafe fn getIndexFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getIndexFromName)(windows_core::Interface::as_raw(self), pwchuri, cchuri, pwchlocalname, cchlocalname, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getIndexFromQName(&self, pwchqname: *const u16, cchqname: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getIndexFromQName)(windows_core::Interface::as_raw(self), pwchqname, cchqname, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getType)(windows_core::Interface::as_raw(self), nindex, ppwchtype as _, pcchtype as _) }
    }
    pub unsafe fn getTypeFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getTypeFromName)(windows_core::Interface::as_raw(self), pwchuri, cchuri, pwchlocalname, cchlocalname, ppwchtype as _, pcchtype as _) }
    }
    pub unsafe fn getTypeFromQName(&self, pwchqname: *const u16, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getTypeFromQName)(windows_core::Interface::as_raw(self), pwchqname, cchqname, ppwchtype as _, pcchtype as _) }
    }
    pub unsafe fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getValue)(windows_core::Interface::as_raw(self), nindex, ppwchvalue as _, pcchvalue as _) }
    }
    pub unsafe fn getValueFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getValueFromName)(windows_core::Interface::as_raw(self), pwchuri, cchuri, pwchlocalname, cchlocalname, ppwchvalue as _, pcchvalue as _) }
    }
    pub unsafe fn getValueFromQName(&self, pwchqname: *const u16, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).getValueFromQName)(windows_core::Interface::as_raw(self), pwchqname, cchqname, ppwchvalue as _, pcchvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getURI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getQName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32, *mut *mut u16, *mut i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getIndexFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *mut i32) -> windows_core::HRESULT,
    pub getIndexFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *mut i32) -> windows_core::HRESULT,
    pub getType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getTypeFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getTypeFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getValueFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
    pub getValueFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *mut *mut u16, *mut i32) -> windows_core::HRESULT,
}
pub trait ISAXAttributes_Impl: windows_core::IUnknownImpl {
    fn getLength(&self) -> windows_core::Result<i32>;
    fn getURI(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> windows_core::Result<()>;
    fn getLocalName(&self, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> windows_core::Result<()>;
    fn getQName(&self, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::Result<()>;
    fn getName(&self, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::Result<()>;
    fn getIndexFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32) -> windows_core::Result<i32>;
    fn getIndexFromQName(&self, pwchqname: *const u16, cchqname: i32) -> windows_core::Result<i32>;
    fn getType(&self, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getTypeFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getTypeFromQName(&self, pwchqname: *const u16, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::Result<()>;
    fn getValue(&self, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
    fn getValueFromName(&self, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
    fn getValueFromQName(&self, pwchqname: *const u16, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::Result<()>;
}
impl ISAXAttributes_Vtbl {
    pub const fn new<Identity: ISAXAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getLength<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXAttributes_Impl::getLength(this) {
                    Ok(ok__) => {
                        pnlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getURI<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getURI(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchuri), core::mem::transmute_copy(&pcchuri)).into()
            }
        }
        unsafe extern "system" fn getLocalName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getLocalName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchlocalname), core::mem::transmute_copy(&pcchlocalname)).into()
            }
        }
        unsafe extern "system" fn getQName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getQName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchqname), core::mem::transmute_copy(&pcchqname)).into()
            }
        }
        unsafe extern "system" fn getName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchuri), core::mem::transmute_copy(&pcchuri), core::mem::transmute_copy(&ppwchlocalname), core::mem::transmute_copy(&pcchlocalname), core::mem::transmute_copy(&ppwchqname), core::mem::transmute_copy(&pcchqname)).into()
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pnindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXAttributes_Impl::getIndexFromName(this, core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute_copy(&pwchlocalname), core::mem::transmute_copy(&cchlocalname)) {
                    Ok(ok__) => {
                        pnindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: *const u16, cchqname: i32, pnindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXAttributes_Impl::getIndexFromQName(this, core::mem::transmute_copy(&pwchqname), core::mem::transmute_copy(&cchqname)) {
                    Ok(ok__) => {
                        pnindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getType<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getType(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getTypeFromName(this, core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute_copy(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: *const u16, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getTypeFromQName(this, core::mem::transmute_copy(&pwchqname), core::mem::transmute_copy(&cchqname), core::mem::transmute_copy(&ppwchtype), core::mem::transmute_copy(&pcchtype)).into()
            }
        }
        unsafe extern "system" fn getValue<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getValue(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchuri: *const u16, cchuri: i32, pwchlocalname: *const u16, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getValueFromName(this, core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&cchuri), core::mem::transmute_copy(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: ISAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchqname: *const u16, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXAttributes_Impl::getValueFromQName(this, core::mem::transmute_copy(&pwchqname), core::mem::transmute_copy(&cchqname), core::mem::transmute_copy(&ppwchvalue), core::mem::transmute_copy(&pcchvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getLength: getLength::<Identity, OFFSET>,
            getURI: getURI::<Identity, OFFSET>,
            getLocalName: getLocalName::<Identity, OFFSET>,
            getQName: getQName::<Identity, OFFSET>,
            getName: getName::<Identity, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, OFFSET>,
            getType: getType::<Identity, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, OFFSET>,
            getValue: getValue::<Identity, OFFSET>,
            getValueFromName: getValueFromName::<Identity, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXAttributes {}
windows_core::imp::define_interface!(ISAXContentHandler, ISAXContentHandler_Vtbl, 0x1545cdfa_9e4e_4497_a8a4_2bf7d0112c44);
windows_core::imp::interface_hierarchy!(ISAXContentHandler, windows_core::IUnknown);
impl ISAXContentHandler {
    pub unsafe fn putDocumentLocator<P0>(&self, plocator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).putDocumentLocator)(windows_core::Interface::as_raw(self), plocator.param().abi()) }
    }
    pub unsafe fn startDocument(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startDocument)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn endDocument(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endDocument)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn startPrefixMapping(&self, pwchprefix: *const u16, cchprefix: i32, pwchuri: *const u16, cchuri: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startPrefixMapping)(windows_core::Interface::as_raw(self), pwchprefix, cchprefix, pwchuri, cchuri) }
    }
    pub unsafe fn endPrefixMapping(&self, pwchprefix: *const u16, cchprefix: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endPrefixMapping)(windows_core::Interface::as_raw(self), pwchprefix, cchprefix) }
    }
    pub unsafe fn startElement<P6>(&self, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32, pattributes: P6) -> windows_core::HRESULT
    where
        P6: windows_core::Param<ISAXAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).startElement)(windows_core::Interface::as_raw(self), pwchnamespaceuri, cchnamespaceuri, pwchlocalname, cchlocalname, pwchqname, cchqname, pattributes.param().abi()) }
    }
    pub unsafe fn endElement(&self, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endElement)(windows_core::Interface::as_raw(self), pwchnamespaceuri, cchnamespaceuri, pwchlocalname, cchlocalname, pwchqname, cchqname) }
    }
    pub unsafe fn characters(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).characters)(windows_core::Interface::as_raw(self), pwchchars, cchchars) }
    }
    pub unsafe fn ignorableWhitespace(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ignorableWhitespace)(windows_core::Interface::as_raw(self), pwchchars, cchchars) }
    }
    pub unsafe fn processingInstruction(&self, pwchtarget: *const u16, cchtarget: i32, pwchdata: *const u16, cchdata: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).processingInstruction)(windows_core::Interface::as_raw(self), pwchtarget, cchtarget, pwchdata, cchdata) }
    }
    pub unsafe fn skippedEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).skippedEntity)(windows_core::Interface::as_raw(self), pwchname, cchname) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXContentHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub putDocumentLocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startPrefixMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub endPrefixMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
    pub startElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub characters: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
    pub ignorableWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
    pub processingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub skippedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
}
pub trait ISAXContentHandler_Impl: windows_core::IUnknownImpl {
    fn putDocumentLocator(&self, plocator: windows_core::Ref<ISAXLocator>) -> windows_core::Result<()>;
    fn startDocument(&self) -> windows_core::Result<()>;
    fn endDocument(&self) -> windows_core::Result<()>;
    fn startPrefixMapping(&self, pwchprefix: *const u16, cchprefix: i32, pwchuri: *const u16, cchuri: i32) -> windows_core::Result<()>;
    fn endPrefixMapping(&self, pwchprefix: *const u16, cchprefix: i32) -> windows_core::Result<()>;
    fn startElement(&self, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32, pattributes: windows_core::Ref<ISAXAttributes>) -> windows_core::Result<()>;
    fn endElement(&self, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32) -> windows_core::Result<()>;
    fn characters(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::Result<()>;
    fn ignorableWhitespace(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::Result<()>;
    fn processingInstruction(&self, pwchtarget: *const u16, cchtarget: i32, pwchdata: *const u16, cchdata: i32) -> windows_core::Result<()>;
    fn skippedEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::Result<()>;
}
impl ISAXContentHandler_Vtbl {
    pub const fn new<Identity: ISAXContentHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn putDocumentLocator<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::putDocumentLocator(this, core::mem::transmute_copy(&plocator)).into()
            }
        }
        unsafe extern "system" fn startDocument<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::startDocument(this).into()
            }
        }
        unsafe extern "system" fn endDocument<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::endDocument(this).into()
            }
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: *const u16, cchprefix: i32, pwchuri: *const u16, cchuri: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::startPrefixMapping(this, core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&cchprefix), core::mem::transmute_copy(&pwchuri), core::mem::transmute_copy(&cchuri)).into()
            }
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchprefix: *const u16, cchprefix: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::endPrefixMapping(this, core::mem::transmute_copy(&pwchprefix), core::mem::transmute_copy(&cchprefix)).into()
            }
        }
        unsafe extern "system" fn startElement<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::startElement(this, core::mem::transmute_copy(&pwchnamespaceuri), core::mem::transmute_copy(&cchnamespaceuri), core::mem::transmute_copy(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&pwchqname), core::mem::transmute_copy(&cchqname), core::mem::transmute_copy(&pattributes)).into()
            }
        }
        unsafe extern "system" fn endElement<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchnamespaceuri: *const u16, cchnamespaceuri: i32, pwchlocalname: *const u16, cchlocalname: i32, pwchqname: *const u16, cchqname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::endElement(this, core::mem::transmute_copy(&pwchnamespaceuri), core::mem::transmute_copy(&cchnamespaceuri), core::mem::transmute_copy(&pwchlocalname), core::mem::transmute_copy(&cchlocalname), core::mem::transmute_copy(&pwchqname), core::mem::transmute_copy(&cchqname)).into()
            }
        }
        unsafe extern "system" fn characters<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::characters(this, core::mem::transmute_copy(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
            }
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::ignorableWhitespace(this, core::mem::transmute_copy(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
            }
        }
        unsafe extern "system" fn processingInstruction<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchtarget: *const u16, cchtarget: i32, pwchdata: *const u16, cchdata: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::processingInstruction(this, core::mem::transmute_copy(&pwchtarget), core::mem::transmute_copy(&cchtarget), core::mem::transmute_copy(&pwchdata), core::mem::transmute_copy(&cchdata)).into()
            }
        }
        unsafe extern "system" fn skippedEntity<Identity: ISAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXContentHandler_Impl::skippedEntity(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            putDocumentLocator: putDocumentLocator::<Identity, OFFSET>,
            startDocument: startDocument::<Identity, OFFSET>,
            endDocument: endDocument::<Identity, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, OFFSET>,
            startElement: startElement::<Identity, OFFSET>,
            endElement: endElement::<Identity, OFFSET>,
            characters: characters::<Identity, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, OFFSET>,
            processingInstruction: processingInstruction::<Identity, OFFSET>,
            skippedEntity: skippedEntity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXContentHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXContentHandler {}
windows_core::imp::define_interface!(ISAXDTDHandler, ISAXDTDHandler_Vtbl, 0xe15c1baf_afb3_4d60_8c36_19a8c45defed);
windows_core::imp::interface_hierarchy!(ISAXDTDHandler, windows_core::IUnknown);
impl ISAXDTDHandler {
    pub unsafe fn notationDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).notationDecl)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchpublicid, cchpublicid, pwchsystemid, cchsystemid) }
    }
    pub unsafe fn unparsedEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32, pwchnotationname: *const u16, cchnotationname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).unparsedEntityDecl)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchpublicid, cchpublicid, pwchsystemid, cchsystemid, pwchnotationname, cchnotationname) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXDTDHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub notationDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub unparsedEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
}
pub trait ISAXDTDHandler_Impl: windows_core::IUnknownImpl {
    fn notationDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::Result<()>;
    fn unparsedEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32, pwchnotationname: *const u16, cchnotationname: i32) -> windows_core::Result<()>;
}
impl ISAXDTDHandler_Vtbl {
    pub const fn new<Identity: ISAXDTDHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn notationDecl<Identity: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDTDHandler_Impl::notationDecl(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute_copy(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
            }
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ISAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32, pwchnotationname: *const u16, cchnotationname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDTDHandler_Impl::unparsedEntityDecl(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute_copy(&pwchsystemid), core::mem::transmute_copy(&cchsystemid), core::mem::transmute_copy(&pwchnotationname), core::mem::transmute_copy(&cchnotationname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXDTDHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXDTDHandler {}
windows_core::imp::define_interface!(ISAXDeclHandler, ISAXDeclHandler_Vtbl, 0x862629ac_771a_47b2_8337_4e6843c1be90);
windows_core::imp::interface_hierarchy!(ISAXDeclHandler, windows_core::IUnknown);
impl ISAXDeclHandler {
    pub unsafe fn elementDecl(&self, pwchname: *const u16, cchname: i32, pwchmodel: *const u16, cchmodel: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).elementDecl)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchmodel, cchmodel) }
    }
    pub unsafe fn attributeDecl(&self, pwchelementname: *const u16, cchelementname: i32, pwchattributename: *const u16, cchattributename: i32, pwchtype: *const u16, cchtype: i32, pwchvaluedefault: *const u16, cchvaluedefault: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).attributeDecl)(windows_core::Interface::as_raw(self), pwchelementname, cchelementname, pwchattributename, cchattributename, pwchtype, cchtype, pwchvaluedefault, cchvaluedefault, pwchvalue, cchvalue) }
    }
    pub unsafe fn internalEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).internalEntityDecl)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchvalue, cchvalue) }
    }
    pub unsafe fn externalEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).externalEntityDecl)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchpublicid, cchpublicid, pwchsystemid, cchsystemid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXDeclHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub elementDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub attributeDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub internalEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub externalEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
}
pub trait ISAXDeclHandler_Impl: windows_core::IUnknownImpl {
    fn elementDecl(&self, pwchname: *const u16, cchname: i32, pwchmodel: *const u16, cchmodel: i32) -> windows_core::Result<()>;
    fn attributeDecl(&self, pwchelementname: *const u16, cchelementname: i32, pwchattributename: *const u16, cchattributename: i32, pwchtype: *const u16, cchtype: i32, pwchvaluedefault: *const u16, cchvaluedefault: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::Result<()>;
    fn internalEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::Result<()>;
    fn externalEntityDecl(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::Result<()>;
}
impl ISAXDeclHandler_Vtbl {
    pub const fn new<Identity: ISAXDeclHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn elementDecl<Identity: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchmodel: *const u16, cchmodel: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDeclHandler_Impl::elementDecl(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchmodel), core::mem::transmute_copy(&cchmodel)).into()
            }
        }
        unsafe extern "system" fn attributeDecl<Identity: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchelementname: *const u16, cchelementname: i32, pwchattributename: *const u16, cchattributename: i32, pwchtype: *const u16, cchtype: i32, pwchvaluedefault: *const u16, cchvaluedefault: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDeclHandler_Impl::attributeDecl(this, core::mem::transmute_copy(&pwchelementname), core::mem::transmute_copy(&cchelementname), core::mem::transmute_copy(&pwchattributename), core::mem::transmute_copy(&cchattributename), core::mem::transmute_copy(&pwchtype), core::mem::transmute_copy(&cchtype), core::mem::transmute_copy(&pwchvaluedefault), core::mem::transmute_copy(&cchvaluedefault), core::mem::transmute_copy(&pwchvalue), core::mem::transmute_copy(&cchvalue)).into()
            }
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchvalue: *const u16, cchvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDeclHandler_Impl::internalEntityDecl(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchvalue), core::mem::transmute_copy(&cchvalue)).into()
            }
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ISAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXDeclHandler_Impl::externalEntityDecl(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute_copy(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, OFFSET>,
            attributeDecl: attributeDecl::<Identity, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXDeclHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXDeclHandler {}
windows_core::imp::define_interface!(ISAXEntityResolver, ISAXEntityResolver_Vtbl, 0x99bca7bd_e8c4_4d5f_a0cf_6d907901ff07);
windows_core::imp::interface_hierarchy!(ISAXEntityResolver, windows_core::IUnknown);
impl ISAXEntityResolver {
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn resolveEntity(&self, pwchpublicid: *const u16, pwchsystemid: *const u16) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).resolveEntity)(windows_core::Interface::as_raw(self), pwchpublicid, pwchsystemid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXEntityResolver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub resolveEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    resolveEntity: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISAXEntityResolver_Impl: windows_core::IUnknownImpl {
    fn resolveEntity(&self, pwchpublicid: *const u16, pwchsystemid: *const u16) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISAXEntityResolver_Vtbl {
    pub const fn new<Identity: ISAXEntityResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn resolveEntity<Identity: ISAXEntityResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchpublicid: *const u16, pwchsystemid: *const u16, pvarinput: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXEntityResolver_Impl::resolveEntity(this, core::mem::transmute_copy(&pwchpublicid), core::mem::transmute_copy(&pwchsystemid)) {
                    Ok(ok__) => {
                        pvarinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXEntityResolver as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISAXEntityResolver {}
windows_core::imp::define_interface!(ISAXErrorHandler, ISAXErrorHandler_Vtbl, 0xa60511c4_ccf5_479e_98a3_dc8dc545b7d0);
windows_core::imp::interface_hierarchy!(ISAXErrorHandler, windows_core::IUnknown);
impl ISAXErrorHandler {
    pub unsafe fn error<P0>(&self, plocator: P0, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).error)(windows_core::Interface::as_raw(self), plocator.param().abi(), pwcherrormessage, hrerrorcode) }
    }
    pub unsafe fn fatalError<P0>(&self, plocator: P0, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).fatalError)(windows_core::Interface::as_raw(self), plocator.param().abi(), pwcherrormessage, hrerrorcode) }
    }
    pub unsafe fn ignorableWarning<P0>(&self, plocator: P0, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).ignorableWarning)(windows_core::Interface::as_raw(self), plocator.param().abi(), pwcherrormessage, hrerrorcode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u16, windows_core::HRESULT) -> windows_core::HRESULT,
    pub fatalError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u16, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ignorableWarning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u16, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ISAXErrorHandler_Impl: windows_core::IUnknownImpl {
    fn error(&self, plocator: windows_core::Ref<ISAXLocator>, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn fatalError(&self, plocator: windows_core::Ref<ISAXLocator>, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ignorableWarning(&self, plocator: windows_core::Ref<ISAXLocator>, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ISAXErrorHandler_Vtbl {
    pub const fn new<Identity: ISAXErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn error<Identity: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXErrorHandler_Impl::error(this, core::mem::transmute_copy(&plocator), core::mem::transmute_copy(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
            }
        }
        unsafe extern "system" fn fatalError<Identity: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXErrorHandler_Impl::fatalError(this, core::mem::transmute_copy(&plocator), core::mem::transmute_copy(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
            }
        }
        unsafe extern "system" fn ignorableWarning<Identity: ISAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocator: *mut core::ffi::c_void, pwcherrormessage: *const u16, hrerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXErrorHandler_Impl::ignorableWarning(this, core::mem::transmute_copy(&plocator), core::mem::transmute_copy(&pwcherrormessage), core::mem::transmute_copy(&hrerrorcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, OFFSET>,
            fatalError: fatalError::<Identity, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXErrorHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXErrorHandler {}
windows_core::imp::define_interface!(ISAXLexicalHandler, ISAXLexicalHandler_Vtbl, 0x7f85d5f5_47a8_4497_bda5_84ba04819ea6);
windows_core::imp::interface_hierarchy!(ISAXLexicalHandler, windows_core::IUnknown);
impl ISAXLexicalHandler {
    pub unsafe fn startDTD(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startDTD)(windows_core::Interface::as_raw(self), pwchname, cchname, pwchpublicid, cchpublicid, pwchsystemid, cchsystemid) }
    }
    pub unsafe fn endDTD(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endDTD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn startEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startEntity)(windows_core::Interface::as_raw(self), pwchname, cchname) }
    }
    pub unsafe fn endEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endEntity)(windows_core::Interface::as_raw(self), pwchname, cchname) }
    }
    pub unsafe fn startCDATA(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startCDATA)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn endCDATA(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endCDATA)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn comment(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).comment)(windows_core::Interface::as_raw(self), pwchchars, cchchars) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXLexicalHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub startDTD: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32, *const u16, i32, *const u16, i32) -> windows_core::HRESULT,
    pub endDTD: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
    pub endEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
    pub startCDATA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endCDATA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub comment: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, i32) -> windows_core::HRESULT,
}
pub trait ISAXLexicalHandler_Impl: windows_core::IUnknownImpl {
    fn startDTD(&self, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::Result<()>;
    fn endDTD(&self) -> windows_core::Result<()>;
    fn startEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::Result<()>;
    fn endEntity(&self, pwchname: *const u16, cchname: i32) -> windows_core::Result<()>;
    fn startCDATA(&self) -> windows_core::Result<()>;
    fn endCDATA(&self) -> windows_core::Result<()>;
    fn comment(&self, pwchchars: *const u16, cchchars: i32) -> windows_core::Result<()>;
}
impl ISAXLexicalHandler_Vtbl {
    pub const fn new<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn startDTD<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32, pwchpublicid: *const u16, cchpublicid: i32, pwchsystemid: *const u16, cchsystemid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::startDTD(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pwchpublicid), core::mem::transmute_copy(&cchpublicid), core::mem::transmute_copy(&pwchsystemid), core::mem::transmute_copy(&cchsystemid)).into()
            }
        }
        unsafe extern "system" fn endDTD<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::endDTD(this).into()
            }
        }
        unsafe extern "system" fn startEntity<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::startEntity(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        unsafe extern "system" fn endEntity<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, cchname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::endEntity(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&cchname)).into()
            }
        }
        unsafe extern "system" fn startCDATA<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::startCDATA(this).into()
            }
        }
        unsafe extern "system" fn endCDATA<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::endCDATA(this).into()
            }
        }
        unsafe extern "system" fn comment<Identity: ISAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchchars: *const u16, cchchars: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXLexicalHandler_Impl::comment(this, core::mem::transmute_copy(&pwchchars), core::mem::transmute_copy(&cchchars)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            startDTD: startDTD::<Identity, OFFSET>,
            endDTD: endDTD::<Identity, OFFSET>,
            startEntity: startEntity::<Identity, OFFSET>,
            endEntity: endEntity::<Identity, OFFSET>,
            startCDATA: startCDATA::<Identity, OFFSET>,
            endCDATA: endCDATA::<Identity, OFFSET>,
            comment: comment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXLexicalHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXLexicalHandler {}
windows_core::imp::define_interface!(ISAXLocator, ISAXLocator_Vtbl, 0x9b7e472a_0de4_4640_bff3_84d38a051c31);
windows_core::imp::interface_hierarchy!(ISAXLocator, windows_core::IUnknown);
impl ISAXLocator {
    pub unsafe fn getColumnNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getColumnNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getLineNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getLineNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getPublicId(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getPublicId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getSystemId(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSystemId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXLocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getColumnNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getLineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getPublicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub getSystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
}
pub trait ISAXLocator_Impl: windows_core::IUnknownImpl {
    fn getColumnNumber(&self) -> windows_core::Result<i32>;
    fn getLineNumber(&self) -> windows_core::Result<i32>;
    fn getPublicId(&self) -> windows_core::Result<*mut u16>;
    fn getSystemId(&self) -> windows_core::Result<*mut u16>;
}
impl ISAXLocator_Vtbl {
    pub const fn new<Identity: ISAXLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getColumnNumber<Identity: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncolumn: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXLocator_Impl::getColumnNumber(this) {
                    Ok(ok__) => {
                        pncolumn.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getLineNumber<Identity: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnline: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXLocator_Impl::getLineNumber(this) {
                    Ok(ok__) => {
                        pnline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getPublicId<Identity: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXLocator_Impl::getPublicId(this) {
                    Ok(ok__) => {
                        ppwchpublicid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSystemId<Identity: ISAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXLocator_Impl::getSystemId(this) {
                    Ok(ok__) => {
                        ppwchsystemid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getColumnNumber: getColumnNumber::<Identity, OFFSET>,
            getLineNumber: getLineNumber::<Identity, OFFSET>,
            getPublicId: getPublicId::<Identity, OFFSET>,
            getSystemId: getSystemId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXLocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISAXLocator {}
windows_core::imp::define_interface!(ISAXXMLFilter, ISAXXMLFilter_Vtbl, 0x70409222_ca09_4475_acb8_40312fe8d145);
impl core::ops::Deref for ISAXXMLFilter {
    type Target = ISAXXMLReader;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISAXXMLFilter, windows_core::IUnknown, ISAXXMLReader);
impl ISAXXMLFilter {
    pub unsafe fn getParent(&self) -> windows_core::Result<ISAXXMLReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putParent<P0>(&self, preader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXXMLReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).putParent)(windows_core::Interface::as_raw(self), preader.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXXMLFilter_Vtbl {
    pub base__: ISAXXMLReader_Vtbl,
    pub getParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISAXXMLFilter_Impl: ISAXXMLReader_Impl {
    fn getParent(&self) -> windows_core::Result<ISAXXMLReader>;
    fn putParent(&self, preader: windows_core::Ref<ISAXXMLReader>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISAXXMLFilter_Vtbl {
    pub const fn new<Identity: ISAXXMLFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getParent<Identity: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLFilter_Impl::getParent(this) {
                    Ok(ok__) => {
                        ppreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putParent<Identity: ISAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLFilter_Impl::putParent(this, core::mem::transmute_copy(&preader)).into()
            }
        }
        Self { base__: ISAXXMLReader_Vtbl::new::<Identity, OFFSET>(), getParent: getParent::<Identity, OFFSET>, putParent: putParent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXXMLFilter as windows_core::Interface>::IID || iid == &<ISAXXMLReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISAXXMLFilter {}
windows_core::imp::define_interface!(ISAXXMLReader, ISAXXMLReader_Vtbl, 0xa4f96ed0_f829_476e_81c0_cdc7bd2a0802);
windows_core::imp::interface_hierarchy!(ISAXXMLReader, windows_core::IUnknown);
impl ISAXXMLReader {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn getFeature(&self, pwchname: *const u16) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getFeature)(windows_core::Interface::as_raw(self), pwchname, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn putFeature(&self, pwchname: *const u16, vfvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putFeature)(windows_core::Interface::as_raw(self), pwchname, vfvalue) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getProperty(&self, pwchname: *const u16) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProperty)(windows_core::Interface::as_raw(self), pwchname, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn putProperty(&self, pwchname: *const u16, varvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putProperty)(windows_core::Interface::as_raw(self), pwchname, core::mem::transmute_copy(varvalue)) }
    }
    pub unsafe fn getEntityResolver(&self) -> windows_core::Result<ISAXEntityResolver> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getEntityResolver)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putEntityResolver<P0>(&self, presolver: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXEntityResolver>,
    {
        unsafe { (windows_core::Interface::vtable(self).putEntityResolver)(windows_core::Interface::as_raw(self), presolver.param().abi()) }
    }
    pub unsafe fn getContentHandler(&self) -> windows_core::Result<ISAXContentHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getContentHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putContentHandler<P0>(&self, phandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXContentHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putContentHandler)(windows_core::Interface::as_raw(self), phandler.param().abi()) }
    }
    pub unsafe fn getDTDHandler(&self) -> windows_core::Result<ISAXDTDHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getDTDHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putDTDHandler<P0>(&self, phandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXDTDHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putDTDHandler)(windows_core::Interface::as_raw(self), phandler.param().abi()) }
    }
    pub unsafe fn getErrorHandler(&self) -> windows_core::Result<ISAXErrorHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getErrorHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putErrorHandler<P0>(&self, phandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISAXErrorHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putErrorHandler)(windows_core::Interface::as_raw(self), phandler.param().abi()) }
    }
    pub unsafe fn getBaseURL(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getBaseURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn putBaseURL(&self, pwchbaseurl: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putBaseURL)(windows_core::Interface::as_raw(self), pwchbaseurl) }
    }
    pub unsafe fn getSecureBaseURL(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSecureBaseURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn putSecureBaseURL(&self, pwchsecurebaseurl: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putSecureBaseURL)(windows_core::Interface::as_raw(self), pwchsecurebaseurl) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn parse(&self, varinput: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).parse)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varinput)) }
    }
    pub unsafe fn parseURL(&self, pwchurl: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).parseURL)(windows_core::Interface::as_raw(self), pwchurl) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISAXXMLReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub getFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    getFeature: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub putFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub putProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    putProperty: usize,
    pub getEntityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putEntityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getContentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putContentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getDTDHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putDTDHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getErrorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putErrorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub putBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub getSecureBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    pub putSecureBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub parse: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    parse: usize,
    pub parseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISAXXMLReader_Impl: windows_core::IUnknownImpl {
    fn getFeature(&self, pwchname: *const u16) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn putFeature(&self, pwchname: *const u16, vfvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, pwchname: *const u16) -> windows_core::Result<super::oaidl::VARIANT>;
    fn putProperty(&self, pwchname: *const u16, varvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn getEntityResolver(&self) -> windows_core::Result<ISAXEntityResolver>;
    fn putEntityResolver(&self, presolver: windows_core::Ref<ISAXEntityResolver>) -> windows_core::Result<()>;
    fn getContentHandler(&self) -> windows_core::Result<ISAXContentHandler>;
    fn putContentHandler(&self, phandler: windows_core::Ref<ISAXContentHandler>) -> windows_core::Result<()>;
    fn getDTDHandler(&self) -> windows_core::Result<ISAXDTDHandler>;
    fn putDTDHandler(&self, phandler: windows_core::Ref<ISAXDTDHandler>) -> windows_core::Result<()>;
    fn getErrorHandler(&self) -> windows_core::Result<ISAXErrorHandler>;
    fn putErrorHandler(&self, phandler: windows_core::Ref<ISAXErrorHandler>) -> windows_core::Result<()>;
    fn getBaseURL(&self) -> windows_core::Result<*mut u16>;
    fn putBaseURL(&self, pwchbaseurl: *const u16) -> windows_core::Result<()>;
    fn getSecureBaseURL(&self) -> windows_core::Result<*mut u16>;
    fn putSecureBaseURL(&self, pwchsecurebaseurl: *const u16) -> windows_core::Result<()>;
    fn parse(&self, varinput: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn parseURL(&self, pwchurl: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISAXXMLReader_Vtbl {
    pub const fn new<Identity: ISAXXMLReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getFeature<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, pvfvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getFeature(this, core::mem::transmute_copy(&pwchname)) {
                    Ok(ok__) => {
                        pvfvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putFeature<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, vfvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putFeature(this, core::mem::transmute_copy(&pwchname), core::mem::transmute_copy(&vfvalue)).into()
            }
        }
        unsafe extern "system" fn getProperty<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getProperty(this, core::mem::transmute_copy(&pwchname)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putProperty<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchname: *const u16, varvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putProperty(this, core::mem::transmute_copy(&pwchname), core::mem::transmute(&varvalue)).into()
            }
        }
        unsafe extern "system" fn getEntityResolver<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getEntityResolver(this) {
                    Ok(ok__) => {
                        ppresolver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putEntityResolver<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putEntityResolver(this, core::mem::transmute_copy(&presolver)).into()
            }
        }
        unsafe extern "system" fn getContentHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getContentHandler(this) {
                    Ok(ok__) => {
                        pphandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putContentHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putContentHandler(this, core::mem::transmute_copy(&phandler)).into()
            }
        }
        unsafe extern "system" fn getDTDHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getDTDHandler(this) {
                    Ok(ok__) => {
                        pphandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putDTDHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putDTDHandler(this, core::mem::transmute_copy(&phandler)).into()
            }
        }
        unsafe extern "system" fn getErrorHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getErrorHandler(this) {
                    Ok(ok__) => {
                        pphandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putErrorHandler<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putErrorHandler(this, core::mem::transmute_copy(&phandler)).into()
            }
        }
        unsafe extern "system" fn getBaseURL<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getBaseURL(this) {
                    Ok(ok__) => {
                        ppwchbaseurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putBaseURL<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchbaseurl: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putBaseURL(this, core::mem::transmute_copy(&pwchbaseurl)).into()
            }
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISAXXMLReader_Impl::getSecureBaseURL(this) {
                    Ok(ok__) => {
                        ppwchsecurebaseurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchsecurebaseurl: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::putSecureBaseURL(this, core::mem::transmute_copy(&pwchsecurebaseurl)).into()
            }
        }
        unsafe extern "system" fn parse<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinput: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::parse(this, core::mem::transmute(&varinput)).into()
            }
        }
        unsafe extern "system" fn parseURL<Identity: ISAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchurl: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISAXXMLReader_Impl::parseURL(this, core::mem::transmute_copy(&pwchurl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getFeature: getFeature::<Identity, OFFSET>,
            putFeature: putFeature::<Identity, OFFSET>,
            getProperty: getProperty::<Identity, OFFSET>,
            putProperty: putProperty::<Identity, OFFSET>,
            getEntityResolver: getEntityResolver::<Identity, OFFSET>,
            putEntityResolver: putEntityResolver::<Identity, OFFSET>,
            getContentHandler: getContentHandler::<Identity, OFFSET>,
            putContentHandler: putContentHandler::<Identity, OFFSET>,
            getDTDHandler: getDTDHandler::<Identity, OFFSET>,
            putDTDHandler: putDTDHandler::<Identity, OFFSET>,
            getErrorHandler: getErrorHandler::<Identity, OFFSET>,
            putErrorHandler: putErrorHandler::<Identity, OFFSET>,
            getBaseURL: getBaseURL::<Identity, OFFSET>,
            putBaseURL: putBaseURL::<Identity, OFFSET>,
            getSecureBaseURL: getSecureBaseURL::<Identity, OFFSET>,
            putSecureBaseURL: putSecureBaseURL::<Identity, OFFSET>,
            parse: parse::<Identity, OFFSET>,
            parseURL: parseURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISAXXMLReader as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISAXXMLReader {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchema, ISchema_Vtbl, 0x50ea08b4_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchema {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchema, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchema {
    pub unsafe fn targetNamespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).targetNamespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn types(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).types)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn elements(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).elements)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn attributes(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn attributeGroups(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributeGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn modelGroups(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).modelGroups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn notations(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn schemaLocations(&self) -> windows_core::Result<ISchemaStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).schemaLocations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchema_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub targetNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub types: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub elements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributeGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub modelGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub notations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub schemaLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchema_Impl: ISchemaItem_Impl {
    fn targetNamespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn types(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn elements(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn attributeGroups(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn modelGroups(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn notations(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn schemaLocations(&self) -> windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchema_Vtbl {
    pub const fn new<Identity: ISchema_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn targetNamespace<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::targetNamespace(this) {
                    Ok(ok__) => {
                        targetnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn version<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn types<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, types: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::types(this) {
                    Ok(ok__) => {
                        types.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn elements<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elements: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::elements(this) {
                    Ok(ok__) => {
                        elements.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn attributes<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn attributeGroups<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::attributeGroups(this) {
                    Ok(ok__) => {
                        attributegroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn modelGroups<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modelgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::modelGroups(this) {
                    Ok(ok__) => {
                        modelgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn notations<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::notations(this) {
                    Ok(ok__) => {
                        notations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn schemaLocations<Identity: ISchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schemalocations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchema_Impl::schemaLocations(this) {
                    Ok(ok__) => {
                        schemalocations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(),
            targetNamespace: targetNamespace::<Identity, OFFSET>,
            version: version::<Identity, OFFSET>,
            types: types::<Identity, OFFSET>,
            elements: elements::<Identity, OFFSET>,
            attributes: attributes::<Identity, OFFSET>,
            attributeGroups: attributeGroups::<Identity, OFFSET>,
            modelGroups: modelGroups::<Identity, OFFSET>,
            notations: notations::<Identity, OFFSET>,
            schemaLocations: schemaLocations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchema as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchema {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaAny, ISchemaAny_Vtbl, 0x50ea08bc_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaAny {
    type Target = ISchemaParticle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaAny, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaAny {
    pub unsafe fn namespaces(&self) -> windows_core::Result<ISchemaStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).namespaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn processContents(&self) -> windows_core::Result<SCHEMAPROCESSCONTENTS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).processContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAny_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    pub namespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub processContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMAPROCESSCONTENTS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaAny_Impl: ISchemaParticle_Impl {
    fn namespaces(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn processContents(&self) -> windows_core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaAny_Vtbl {
    pub const fn new<Identity: ISchemaAny_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn namespaces<Identity: ISchemaAny_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAny_Impl::namespaces(this) {
                    Ok(ok__) => {
                        namespaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn processContents<Identity: ISchemaAny_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAny_Impl::processContents(this) {
                    Ok(ok__) => {
                        processcontents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, OFFSET>(),
            namespaces: namespaces::<Identity, OFFSET>,
            processContents: processContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAny as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaAny {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaAttribute, ISchemaAttribute_Vtbl, 0x50ea08b6_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaAttribute {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaAttribute, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaAttribute {
    pub unsafe fn r#type(&self) -> windows_core::Result<ISchemaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn scope(&self) -> windows_core::Result<ISchemaComplexType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).scope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).defaultValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fixedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn r#use(&self) -> windows_core::Result<SCHEMAUSE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#use)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isReference(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAttribute_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub r#type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub defaultValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fixedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub r#use: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMAUSE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub isReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isReference: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaAttribute_Impl: ISchemaItem_Impl {
    fn r#type(&self) -> windows_core::Result<ISchemaType>;
    fn scope(&self) -> windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn r#use(&self) -> windows_core::Result<SCHEMAUSE>;
    fn isReference(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaAttribute_Vtbl {
    pub const fn new<Identity: ISchemaAttribute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn r#type<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::r#type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn scope<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::defaultValue(this) {
                    Ok(ok__) => {
                        defaultvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::fixedValue(this) {
                    Ok(ok__) => {
                        fixedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#use<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#use: *mut SCHEMAUSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::r#use(this) {
                    Ok(ok__) => {
                        r#use.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isReference<Identity: ISchemaAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttribute_Impl::isReference(this) {
                    Ok(ok__) => {
                        reference.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(),
            r#type: r#type::<Identity, OFFSET>,
            scope: scope::<Identity, OFFSET>,
            defaultValue: defaultValue::<Identity, OFFSET>,
            fixedValue: fixedValue::<Identity, OFFSET>,
            r#use: r#use::<Identity, OFFSET>,
            isReference: isReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAttribute as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaAttribute {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaAttributeGroup, ISchemaAttributeGroup_Vtbl, 0x50ea08ba_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaAttributeGroup {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaAttributeGroup, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaAttributeGroup {
    pub unsafe fn anyAttribute(&self) -> windows_core::Result<ISchemaAny> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).anyAttribute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn attributes(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaAttributeGroup_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub anyAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaAttributeGroup_Impl: ISchemaItem_Impl {
    fn anyAttribute(&self) -> windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaAttributeGroup_Vtbl {
    pub const fn new<Identity: ISchemaAttributeGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn anyAttribute<Identity: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, anyattribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttributeGroup_Impl::anyAttribute(this) {
                    Ok(ok__) => {
                        anyattribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn attributes<Identity: ISchemaAttributeGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaAttributeGroup_Impl::attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(), anyAttribute: anyAttribute::<Identity, OFFSET>, attributes: attributes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaAttributeGroup as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaAttributeGroup {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaComplexType, ISchemaComplexType_Vtbl, 0x50ea08b9_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaComplexType {
    type Target = ISchemaType;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaComplexType, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem, ISchemaType);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaComplexType {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isAbstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAbstract)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn anyAttribute(&self) -> windows_core::Result<ISchemaAny> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).anyAttribute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn attributes(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn contentType(&self) -> windows_core::Result<SCHEMACONTENTTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn contentModel(&self) -> windows_core::Result<ISchemaModelGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn prohibitedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).prohibitedSubstitutions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaComplexType_Vtbl {
    pub base__: ISchemaType_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub isAbstract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isAbstract: usize,
    pub anyAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub contentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMACONTENTTYPE) -> windows_core::HRESULT,
    pub contentModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub prohibitedSubstitutions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaComplexType_Impl: ISchemaType_Impl {
    fn isAbstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn anyAttribute(&self) -> windows_core::Result<ISchemaAny>;
    fn attributes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn contentType(&self) -> windows_core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(&self) -> windows_core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaComplexType_Vtbl {
    pub const fn new<Identity: ISchemaComplexType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn isAbstract<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#abstract: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::isAbstract(this) {
                    Ok(ok__) => {
                        r#abstract.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn anyAttribute<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, anyattribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::anyAttribute(this) {
                    Ok(ok__) => {
                        anyattribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn attributes<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn contentType<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::contentType(this) {
                    Ok(ok__) => {
                        contenttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn contentModel<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::contentModel(this) {
                    Ok(ok__) => {
                        contentmodel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: ISchemaComplexType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaComplexType_Impl::prohibitedSubstitutions(this) {
                    Ok(ok__) => {
                        prohibited.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaType_Vtbl::new::<Identity, OFFSET>(),
            isAbstract: isAbstract::<Identity, OFFSET>,
            anyAttribute: anyAttribute::<Identity, OFFSET>,
            attributes: attributes::<Identity, OFFSET>,
            contentType: contentType::<Identity, OFFSET>,
            contentModel: contentModel::<Identity, OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaComplexType as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaComplexType {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaElement, ISchemaElement_Vtbl, 0x50ea08b7_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaElement {
    type Target = ISchemaParticle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaElement, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaElement {
    pub unsafe fn r#type(&self) -> windows_core::Result<ISchemaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn scope(&self) -> windows_core::Result<ISchemaComplexType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).scope)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).defaultValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fixedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isNillable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isNillable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn identityConstraints(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).identityConstraints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn substitutionGroup(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).substitutionGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn substitutionGroupExclusions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).substitutionGroupExclusions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn disallowedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).disallowedSubstitutions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isAbstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAbstract)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isReference(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaElement_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    pub r#type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub defaultValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fixedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub isNillable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isNillable: usize,
    pub identityConstraints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub substitutionGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub substitutionGroupExclusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT,
    pub disallowedSubstitutions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub isAbstract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isAbstract: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub isReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isReference: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaElement_Impl: ISchemaParticle_Impl {
    fn r#type(&self) -> windows_core::Result<ISchemaType>;
    fn scope(&self) -> windows_core::Result<ISchemaComplexType>;
    fn defaultValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fixedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn isNillable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn identityConstraints(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn substitutionGroup(&self) -> windows_core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn isReference(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaElement_Vtbl {
    pub const fn new<Identity: ISchemaElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn r#type<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::r#type(this) {
                    Ok(ok__) => {
                        r#type.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn scope<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::scope(this) {
                    Ok(ok__) => {
                        scope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn defaultValue<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaultvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::defaultValue(this) {
                    Ok(ok__) => {
                        defaultvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fixedValue<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fixedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::fixedValue(this) {
                    Ok(ok__) => {
                        fixedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isNillable<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nillable: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::isNillable(this) {
                    Ok(ok__) => {
                        nillable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn identityConstraints<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, constraints: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::identityConstraints(this) {
                    Ok(ok__) => {
                        constraints.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn substitutionGroup<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::substitutionGroup(this) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::substitutionGroupExclusions(this) {
                    Ok(ok__) => {
                        exclusions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::disallowedSubstitutions(this) {
                    Ok(ok__) => {
                        disallowed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isAbstract<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#abstract: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::isAbstract(this) {
                    Ok(ok__) => {
                        r#abstract.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isReference<Identity: ISchemaElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaElement_Impl::isReference(this) {
                    Ok(ok__) => {
                        reference.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaParticle_Vtbl::new::<Identity, OFFSET>(),
            r#type: r#type::<Identity, OFFSET>,
            scope: scope::<Identity, OFFSET>,
            defaultValue: defaultValue::<Identity, OFFSET>,
            fixedValue: fixedValue::<Identity, OFFSET>,
            isNillable: isNillable::<Identity, OFFSET>,
            identityConstraints: identityConstraints::<Identity, OFFSET>,
            substitutionGroup: substitutionGroup::<Identity, OFFSET>,
            substitutionGroupExclusions: substitutionGroupExclusions::<Identity, OFFSET>,
            disallowedSubstitutions: disallowedSubstitutions::<Identity, OFFSET>,
            isAbstract: isAbstract::<Identity, OFFSET>,
            isReference: isReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaElement as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaElement {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaIdentityConstraint, ISchemaIdentityConstraint_Vtbl, 0x50ea08bd_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaIdentityConstraint {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaIdentityConstraint, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaIdentityConstraint {
    pub unsafe fn selector(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).selector)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fields(&self) -> windows_core::Result<ISchemaStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn referencedKey(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).referencedKey)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaIdentityConstraint_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub selector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub referencedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaIdentityConstraint_Impl: ISchemaItem_Impl {
    fn selector(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fields(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn referencedKey(&self) -> windows_core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaIdentityConstraint_Vtbl {
    pub const fn new<Identity: ISchemaIdentityConstraint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn selector<Identity: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaIdentityConstraint_Impl::selector(this) {
                    Ok(ok__) => {
                        selector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fields<Identity: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaIdentityConstraint_Impl::fields(this) {
                    Ok(ok__) => {
                        fields.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn referencedKey<Identity: ISchemaIdentityConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaIdentityConstraint_Impl::referencedKey(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(),
            selector: selector::<Identity, OFFSET>,
            fields: fields::<Identity, OFFSET>,
            referencedKey: referencedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaIdentityConstraint as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaIdentityConstraint {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaItem, ISchemaItem_Vtbl, 0x50ea08b3_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaItem {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaItem, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaItem {
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).namespaceURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn schema(&self) -> windows_core::Result<ISchema> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).schema)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn itemType(&self) -> windows_core::Result<SOMITEMTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).itemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn unhandledAttributes(&self) -> windows_core::Result<IVBSAXAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).unhandledAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn writeAnnotation<P0>(&self, annotationsink: P0) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).writeAnnotation)(windows_core::Interface::as_raw(self), annotationsink.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaItem_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub namespaceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub schema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub itemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SOMITEMTYPE) -> windows_core::HRESULT,
    pub unhandledAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub writeAnnotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    writeAnnotation: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaItem_Impl: super::oaidl::IDispatch_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn schema(&self) -> windows_core::Result<ISchema>;
    fn id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn itemType(&self) -> windows_core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(&self) -> windows_core::Result<IVBSAXAttributes>;
    fn writeAnnotation(&self, annotationsink: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaItem_Vtbl {
    pub const fn new<Identity: ISchemaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn name<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::namespaceURI(this) {
                    Ok(ok__) => {
                        namespaceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn schema<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::schema(this) {
                    Ok(ok__) => {
                        schema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn id<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::id(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn itemType<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::itemType(this) {
                    Ok(ok__) => {
                        itemtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn unhandledAttributes<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::unhandledAttributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn writeAnnotation<Identity: ISchemaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, annotationsink: *mut core::ffi::c_void, iswritten: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItem_Impl::writeAnnotation(this, core::mem::transmute_copy(&annotationsink)) {
                    Ok(ok__) => {
                        iswritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, OFFSET>,
            namespaceURI: namespaceURI::<Identity, OFFSET>,
            schema: schema::<Identity, OFFSET>,
            id: id::<Identity, OFFSET>,
            itemType: itemType::<Identity, OFFSET>,
            unhandledAttributes: unhandledAttributes::<Identity, OFFSET>,
            writeAnnotation: writeAnnotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaItem {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaItemCollection, ISchemaItemCollection_Vtbl, 0x50ea08b2_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaItemCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaItemCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaItemCollection {
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<ISchemaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn itemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<ISchemaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).itemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn itemByQName(&self, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchemaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).itemByQName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._newEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaItemCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub itemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub itemByQName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaItemCollection_Impl: super::oaidl::IDispatch_Impl {
    fn item(&self, index: i32) -> windows_core::Result<ISchemaItem>;
    fn itemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<ISchemaItem>;
    fn itemByQName(&self, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchemaItem>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaItemCollection_Vtbl {
    pub const fn new<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn item<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItemCollection_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn itemByName<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItemCollection_Impl::itemByName(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn itemByQName<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItemCollection_Impl::itemByQName(this, core::mem::transmute(&name), core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItemCollection_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ISchemaItemCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaItemCollection_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            item: item::<Identity, OFFSET>,
            itemByName: itemByName::<Identity, OFFSET>,
            itemByQName: itemByQName::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaItemCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaItemCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaModelGroup, ISchemaModelGroup_Vtbl, 0x50ea08bb_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaModelGroup {
    type Target = ISchemaParticle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaModelGroup, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem, ISchemaParticle);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaModelGroup {
    pub unsafe fn particles(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).particles)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaModelGroup_Vtbl {
    pub base__: ISchemaParticle_Vtbl,
    pub particles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaModelGroup_Impl: ISchemaParticle_Impl {
    fn particles(&self) -> windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaModelGroup_Vtbl {
    pub const fn new<Identity: ISchemaModelGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn particles<Identity: ISchemaModelGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, particles: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaModelGroup_Impl::particles(this) {
                    Ok(ok__) => {
                        particles.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISchemaParticle_Vtbl::new::<Identity, OFFSET>(), particles: particles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaModelGroup as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID || iid == &<ISchemaParticle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaModelGroup {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaNotation, ISchemaNotation_Vtbl, 0x50ea08be_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaNotation {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaNotation, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaNotation {
    pub unsafe fn systemIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn publicIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).publicIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaNotation_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub systemIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub publicIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaNotation_Impl: ISchemaItem_Impl {
    fn systemIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn publicIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaNotation_Vtbl {
    pub const fn new<Identity: ISchemaNotation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn systemIdentifier<Identity: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaNotation_Impl::systemIdentifier(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn publicIdentifier<Identity: ISchemaNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaNotation_Impl::publicIdentifier(this) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(),
            systemIdentifier: systemIdentifier::<Identity, OFFSET>,
            publicIdentifier: publicIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaNotation as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaNotation {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaParticle, ISchemaParticle_Vtbl, 0x50ea08b5_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaParticle {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaParticle, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaParticle {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn minOccurs(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).minOccurs)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn maxOccurs(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxOccurs)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaParticle_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub minOccurs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    minOccurs: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub maxOccurs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    maxOccurs: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaParticle_Impl: ISchemaItem_Impl {
    fn minOccurs(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn maxOccurs(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaParticle_Vtbl {
    pub const fn new<Identity: ISchemaParticle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn minOccurs<Identity: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minoccurs: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaParticle_Impl::minOccurs(this) {
                    Ok(ok__) => {
                        minoccurs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxOccurs<Identity: ISchemaParticle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxoccurs: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaParticle_Impl::maxOccurs(this) {
                    Ok(ok__) => {
                        maxoccurs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(), minOccurs: minOccurs::<Identity, OFFSET>, maxOccurs: maxOccurs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaParticle as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaParticle {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaStringCollection, ISchemaStringCollection_Vtbl, 0x50ea08b1_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaStringCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaStringCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaStringCollection {
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._newEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaStringCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaStringCollection_Impl: super::oaidl::IDispatch_Impl {
    fn item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaStringCollection_Vtbl {
    pub const fn new<Identity: ISchemaStringCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn item<Identity: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, bstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaStringCollection_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        bstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaStringCollection_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _newEnum<Identity: ISchemaStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaStringCollection_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            item: item::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaStringCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaStringCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ISchemaType, ISchemaType_Vtbl, 0x50ea08b8_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ISchemaType {
    type Target = ISchemaItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ISchemaType, windows_core::IUnknown, super::oaidl::IDispatch, ISchemaItem);
#[cfg(feature = "Win32_oaidl")]
impl ISchemaType {
    pub unsafe fn baseTypes(&self) -> windows_core::Result<ISchemaItemCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseTypes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn r#final(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#final)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn variety(&self) -> windows_core::Result<SCHEMATYPEVARIETY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).variety)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn derivedBy(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).derivedBy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn isValid(&self, data: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isValid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn minExclusive(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).minExclusive)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn minInclusive(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).minInclusive)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn maxExclusive(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxExclusive)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn maxInclusive(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxInclusive)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn totalDigits(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).totalDigits)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn fractionDigits(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fractionDigits)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn length(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn minLength(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).minLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn maxLength(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).maxLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn enumeration(&self) -> windows_core::Result<ISchemaStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).enumeration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn whitespace(&self) -> windows_core::Result<SCHEMAWHITESPACE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).whitespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn patterns(&self) -> windows_core::Result<ISchemaStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).patterns)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchemaType_Vtbl {
    pub base__: ISchemaItem_Vtbl,
    pub baseTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub r#final: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT,
    pub variety: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMATYPEVARIETY) -> windows_core::HRESULT,
    pub derivedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub isValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    isValid: usize,
    pub minExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub minInclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub maxExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub maxInclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub totalDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    totalDigits: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub fractionDigits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    fractionDigits: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    length: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub minLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    minLength: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub maxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    maxLength: usize,
    pub enumeration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub whitespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCHEMAWHITESPACE) -> windows_core::HRESULT,
    pub patterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISchemaType_Impl: ISchemaItem_Impl {
    fn baseTypes(&self) -> windows_core::Result<ISchemaItemCollection>;
    fn r#final(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(&self) -> windows_core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(&self) -> windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(&self, data: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn minExclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn minInclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn maxExclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn maxInclusive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn totalDigits(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn fractionDigits(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn length(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn minLength(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn maxLength(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn enumeration(&self) -> windows_core::Result<ISchemaStringCollection>;
    fn whitespace(&self) -> windows_core::Result<SCHEMAWHITESPACE>;
    fn patterns(&self) -> windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISchemaType_Vtbl {
    pub const fn new<Identity: ISchemaType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn baseTypes<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::baseTypes(this) {
                    Ok(ok__) => {
                        basetypes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#final<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::r#final(this) {
                    Ok(ok__) => {
                        r#final.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn variety<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::variety(this) {
                    Ok(ok__) => {
                        variety.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn derivedBy<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::derivedBy(this) {
                    Ok(ok__) => {
                        derivedby.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isValid<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, valid: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::isValid(this, core::mem::transmute(&data)) {
                    Ok(ok__) => {
                        valid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn minExclusive<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minexclusive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::minExclusive(this) {
                    Ok(ok__) => {
                        minexclusive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn minInclusive<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mininclusive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::minInclusive(this) {
                    Ok(ok__) => {
                        mininclusive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxExclusive<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxexclusive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::maxExclusive(this) {
                    Ok(ok__) => {
                        maxexclusive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxInclusive<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxinclusive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::maxInclusive(this) {
                    Ok(ok__) => {
                        maxinclusive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn totalDigits<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, totaldigits: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::totalDigits(this) {
                    Ok(ok__) => {
                        totaldigits.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fractionDigits<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fractiondigits: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::fractionDigits(this) {
                    Ok(ok__) => {
                        fractiondigits.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn minLength<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minlength: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::minLength(this) {
                    Ok(ok__) => {
                        minlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn maxLength<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlength: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::maxLength(this) {
                    Ok(ok__) => {
                        maxlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn enumeration<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumeration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::enumeration(this) {
                    Ok(ok__) => {
                        enumeration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn whitespace<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::whitespace(this) {
                    Ok(ok__) => {
                        whitespace.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn patterns<Identity: ISchemaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patterns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchemaType_Impl::patterns(this) {
                    Ok(ok__) => {
                        patterns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISchemaItem_Vtbl::new::<Identity, OFFSET>(),
            baseTypes: baseTypes::<Identity, OFFSET>,
            r#final: r#final::<Identity, OFFSET>,
            variety: variety::<Identity, OFFSET>,
            derivedBy: derivedBy::<Identity, OFFSET>,
            isValid: isValid::<Identity, OFFSET>,
            minExclusive: minExclusive::<Identity, OFFSET>,
            minInclusive: minInclusive::<Identity, OFFSET>,
            maxExclusive: maxExclusive::<Identity, OFFSET>,
            maxInclusive: maxInclusive::<Identity, OFFSET>,
            totalDigits: totalDigits::<Identity, OFFSET>,
            fractionDigits: fractionDigits::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            minLength: minLength::<Identity, OFFSET>,
            maxLength: maxLength::<Identity, OFFSET>,
            enumeration: enumeration::<Identity, OFFSET>,
            whitespace: whitespace::<Identity, OFFSET>,
            patterns: patterns::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaType as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ISchemaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISchemaType {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IServerXMLHTTPRequest, IServerXMLHTTPRequest_Vtbl, 0x2e9196bf_13ba_4dd4_91ca_6c571f281495);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IServerXMLHTTPRequest {
    type Target = IXMLHTTPRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IServerXMLHTTPRequest, windows_core::IUnknown, super::oaidl::IDispatch, IXMLHTTPRequest);
#[cfg(feature = "Win32_oaidl")]
impl IServerXMLHTTPRequest {
    pub unsafe fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setTimeouts)(windows_core::Interface::as_raw(self), resolvetimeout, connecttimeout, sendtimeout, receivetimeout) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn waitForResponse(&self, timeoutinseconds: &super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).waitForResponse)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timeoutinseconds), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getOption)(windows_core::Interface::as_raw(self), option, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setOption)(windows_core::Interface::as_raw(self), option, core::mem::transmute_copy(value)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IServerXMLHTTPRequest_Vtbl {
    pub base__: IXMLHTTPRequest_Vtbl,
    pub setTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub waitForResponse: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    waitForResponse: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getOption: unsafe extern "system" fn(*mut core::ffi::c_void, SERVERXMLHTTP_OPTION, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getOption: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setOption: unsafe extern "system" fn(*mut core::ffi::c_void, SERVERXMLHTTP_OPTION, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setOption: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IServerXMLHTTPRequest_Impl: IXMLHTTPRequest_Impl {
    fn setTimeouts(&self, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::Result<()>;
    fn waitForResponse(&self, timeoutinseconds: &super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn getOption(&self, option: SERVERXMLHTTP_OPTION) -> windows_core::Result<super::oaidl::VARIANT>;
    fn setOption(&self, option: SERVERXMLHTTP_OPTION, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IServerXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: IServerXMLHTTPRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn setTimeouts<Identity: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerXMLHTTPRequest_Impl::setTimeouts(this, core::mem::transmute_copy(&resolvetimeout), core::mem::transmute_copy(&connecttimeout), core::mem::transmute_copy(&sendtimeout), core::mem::transmute_copy(&receivetimeout)).into()
            }
        }
        unsafe extern "system" fn waitForResponse<Identity: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinseconds: super::oaidl::VARIANT, issuccessful: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IServerXMLHTTPRequest_Impl::waitForResponse(this, core::mem::transmute(&timeoutinseconds)) {
                    Ok(ok__) => {
                        issuccessful.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getOption<Identity: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IServerXMLHTTPRequest_Impl::getOption(this, core::mem::transmute_copy(&option)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setOption<Identity: IServerXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerXMLHTTPRequest_Impl::setOption(this, core::mem::transmute_copy(&option), core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: IXMLHTTPRequest_Vtbl::new::<Identity, OFFSET>(),
            setTimeouts: setTimeouts::<Identity, OFFSET>,
            waitForResponse: waitForResponse::<Identity, OFFSET>,
            getOption: getOption::<Identity, OFFSET>,
            setOption: setOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IServerXMLHTTPRequest {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IServerXMLHTTPRequest2, IServerXMLHTTPRequest2_Vtbl, 0x2e01311b_c322_4b0a_bd77_b90cfdc8dce7);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IServerXMLHTTPRequest2 {
    type Target = IServerXMLHTTPRequest;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IServerXMLHTTPRequest2, windows_core::IUnknown, super::oaidl::IDispatch, IXMLHTTPRequest, IServerXMLHTTPRequest);
#[cfg(feature = "Win32_oaidl")]
impl IServerXMLHTTPRequest2 {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::oaidl::VARIANT, varbypasslist: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxy)(windows_core::Interface::as_raw(self), proxysetting, core::mem::transmute_copy(varproxyserver), core::mem::transmute_copy(varbypasslist)) }
    }
    pub unsafe fn setProxyCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxyCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IServerXMLHTTPRequest2_Vtbl {
    pub base__: IServerXMLHTTPRequest_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setProxy: unsafe extern "system" fn(*mut core::ffi::c_void, SXH_PROXY_SETTING, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setProxy: usize,
    pub setProxyCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IServerXMLHTTPRequest2_Impl: IServerXMLHTTPRequest_Impl {
    fn setProxy(&self, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::oaidl::VARIANT, varbypasslist: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn setProxyCredentials(&self, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IServerXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn setProxy<Identity: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::oaidl::VARIANT, varbypasslist: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerXMLHTTPRequest2_Impl::setProxy(this, core::mem::transmute_copy(&proxysetting), core::mem::transmute(&varproxyserver), core::mem::transmute(&varbypasslist)).into()
            }
        }
        unsafe extern "system" fn setProxyCredentials<Identity: IServerXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerXMLHTTPRequest2_Impl::setProxyCredentials(this, core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword)).into()
            }
        }
        Self {
            base__: IServerXMLHTTPRequest_Vtbl::new::<Identity, OFFSET>(),
            setProxy: setProxy::<Identity, OFFSET>,
            setProxyCredentials: setProxyCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerXMLHTTPRequest2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest as windows_core::Interface>::IID || iid == &<IServerXMLHTTPRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IServerXMLHTTPRequest2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBMXNamespaceManager, IVBMXNamespaceManager_Vtbl, 0xc90352f5_643c_4fbc_bb23_e996eb2d51fd);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBMXNamespaceManager {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBMXNamespaceManager, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBMXNamespaceManager {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetallowOverride(&self, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetallowOverride)(windows_core::Interface::as_raw(self), foverride) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn allowOverride(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).allowOverride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn pushContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).pushContext)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_wtypes"))]
    pub unsafe fn pushNodeContext<P0>(&self, contextnode: P0, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).pushNodeContext)(windows_core::Interface::as_raw(self), contextnode.param().abi(), fdeep) }
    }
    pub unsafe fn popContext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).popContext)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn declarePrefix(&self, prefix: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).declarePrefix)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(prefix), core::mem::transmute_copy(namespaceuri)) }
    }
    pub unsafe fn getDeclaredPrefixes(&self) -> windows_core::Result<IMXNamespacePrefixes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getDeclaredPrefixes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn getPrefixes(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IMXNamespacePrefixes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getPrefixes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getURI(&self, prefix: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getURI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(prefix), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getURIFromNode<P1>(&self, strprefix: &windows_core::BSTR, contextnode: P1) -> windows_core::Result<super::oaidl::VARIANT>
    where
        P1: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getURIFromNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strprefix), contextnode.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBMXNamespaceManager_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub SetallowOverride: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetallowOverride: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub allowOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    allowOverride: usize,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub pushContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_wtypes"))]
    pub pushNodeContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_wtypes")))]
    pushNodeContext: usize,
    pub popContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub declarePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getDeclaredPrefixes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getPrefixes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getURI: usize,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getURIFromNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getURIFromNode: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBMXNamespaceManager_Impl: super::oaidl::IDispatch_Impl {
    fn SetallowOverride(&self, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn allowOverride(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn pushContext(&self) -> windows_core::Result<()>;
    fn pushNodeContext(&self, contextnode: windows_core::Ref<super::msxml::IXMLDOMNode>, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn popContext(&self) -> windows_core::Result<()>;
    fn declarePrefix(&self, prefix: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getDeclaredPrefixes(&self) -> windows_core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IMXNamespacePrefixes>;
    fn getURI(&self, prefix: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn getURIFromNode(&self, strprefix: &windows_core::BSTR, contextnode: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBMXNamespaceManager_Vtbl {
    pub const fn new<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetallowOverride<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::SetallowOverride(this, core::mem::transmute_copy(&foverride)).into()
            }
        }
        unsafe extern "system" fn allowOverride<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, foverride: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBMXNamespaceManager_Impl::allowOverride(this) {
                    Ok(ok__) => {
                        foverride.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn pushContext<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::pushContext(this).into()
            }
        }
        unsafe extern "system" fn pushNodeContext<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextnode: *mut core::ffi::c_void, fdeep: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::pushNodeContext(this, core::mem::transmute_copy(&contextnode), core::mem::transmute_copy(&fdeep)).into()
            }
        }
        unsafe extern "system" fn popContext<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::popContext(this).into()
            }
        }
        unsafe extern "system" fn declarePrefix<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBMXNamespaceManager_Impl::declarePrefix(this, core::mem::transmute(&prefix), core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBMXNamespaceManager_Impl::getDeclaredPrefixes(this) {
                    Ok(ok__) => {
                        prefixes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getPrefixes<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, prefixes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBMXNamespaceManager_Impl::getPrefixes(this, core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        prefixes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getURI<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefix: *mut core::ffi::c_void, uri: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBMXNamespaceManager_Impl::getURI(this, core::mem::transmute(&prefix)) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getURIFromNode<Identity: IVBMXNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: *mut core::ffi::c_void, contextnode: *mut core::ffi::c_void, uri: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBMXNamespaceManager_Impl::getURIFromNode(this, core::mem::transmute(&strprefix), core::mem::transmute_copy(&contextnode)) {
                    Ok(ok__) => {
                        uri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetallowOverride: SetallowOverride::<Identity, OFFSET>,
            allowOverride: allowOverride::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            pushContext: pushContext::<Identity, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, OFFSET>,
            popContext: popContext::<Identity, OFFSET>,
            declarePrefix: declarePrefix::<Identity, OFFSET>,
            getDeclaredPrefixes: getDeclaredPrefixes::<Identity, OFFSET>,
            getPrefixes: getPrefixes::<Identity, OFFSET>,
            getURI: getURI::<Identity, OFFSET>,
            getURIFromNode: getURIFromNode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBMXNamespaceManager as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBMXNamespaceManager {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXAttributes, IVBSAXAttributes_Vtbl, 0x10dc0586_132b_4cac_8bb3_db00ac8b7ee0);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXAttributes {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXAttributes, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXAttributes {
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getURI(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getURI)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getLocalName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getLocalName)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getQName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getQName)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getIndexFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getIndexFromName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(struri), core::mem::transmute_copy(strlocalname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getIndexFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getIndexFromQName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strqname), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getType(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getType)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getTypeFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getTypeFromName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(struri), core::mem::transmute_copy(strlocalname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getTypeFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getTypeFromQName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strqname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getValue(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getValue)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getValueFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getValueFromName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(struri), core::mem::transmute_copy(strlocalname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getValueFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getValueFromQName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strqname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXAttributes_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getURI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getQName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getIndexFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getIndexFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getTypeFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getTypeFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getValueFromName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getValueFromQName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXAttributes_Impl: super::oaidl::IDispatch_Impl {
    fn length(&self) -> windows_core::Result<i32>;
    fn getURI(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getLocalName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getQName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getIndexFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getIndexFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getType(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getTypeFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getTypeFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getValue(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getValueFromName(&self, struri: &windows_core::BSTR, strlocalname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getValueFromQName(&self, strqname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXAttributes_Vtbl {
    pub const fn new<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn length<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::length(this) {
                    Ok(ok__) => {
                        nlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getURI<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, struri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getURI(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        struri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getLocalName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strlocalname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getLocalName(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        strlocalname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getQName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strqname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getQName(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        strqname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getIndexFromName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: *mut core::ffi::c_void, strlocalname: *mut core::ffi::c_void, nindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getIndexFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                    Ok(ok__) => {
                        nindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getIndexFromQName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: *mut core::ffi::c_void, nindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getIndexFromQName(this, core::mem::transmute(&strqname)) {
                    Ok(ok__) => {
                        nindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getType<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getType(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getTypeFromName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: *mut core::ffi::c_void, strlocalname: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getTypeFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getTypeFromQName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getTypeFromQName(this, core::mem::transmute(&strqname)) {
                    Ok(ok__) => {
                        strtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getValue<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getValue(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        strvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getValueFromName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, struri: *mut core::ffi::c_void, strlocalname: *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getValueFromName(this, core::mem::transmute(&struri), core::mem::transmute(&strlocalname)) {
                    Ok(ok__) => {
                        strvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getValueFromQName<Identity: IVBSAXAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqname: *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXAttributes_Impl::getValueFromQName(this, core::mem::transmute(&strqname)) {
                    Ok(ok__) => {
                        strvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            length: length::<Identity, OFFSET>,
            getURI: getURI::<Identity, OFFSET>,
            getLocalName: getLocalName::<Identity, OFFSET>,
            getQName: getQName::<Identity, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, OFFSET>,
            getType: getType::<Identity, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, OFFSET>,
            getValue: getValue::<Identity, OFFSET>,
            getValueFromName: getValueFromName::<Identity, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXAttributes as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXAttributes {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXContentHandler, IVBSAXContentHandler_Vtbl, 0x2ed7290a_4dd5_4b46_bb26_4e4155e77faa);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXContentHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXContentHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXContentHandler {
    pub unsafe fn putref_documentLocator<P0>(&self, olocator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_documentLocator)(windows_core::Interface::as_raw(self), olocator.param().abi()) }
    }
    pub unsafe fn startDocument(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startDocument)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn endDocument(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endDocument)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn startPrefixMapping(&self, strprefix: *mut windows_core::BSTR, struri: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startPrefixMapping)(windows_core::Interface::as_raw(self), core::mem::transmute(strprefix), core::mem::transmute(struri)) }
    }
    pub unsafe fn endPrefixMapping(&self, strprefix: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endPrefixMapping)(windows_core::Interface::as_raw(self), core::mem::transmute(strprefix)) }
    }
    pub unsafe fn startElement<P3>(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR, oattributes: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IVBSAXAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).startElement)(windows_core::Interface::as_raw(self), core::mem::transmute(strnamespaceuri), core::mem::transmute(strlocalname), core::mem::transmute(strqname), oattributes.param().abi()) }
    }
    pub unsafe fn endElement(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endElement)(windows_core::Interface::as_raw(self), core::mem::transmute(strnamespaceuri), core::mem::transmute(strlocalname), core::mem::transmute(strqname)) }
    }
    pub unsafe fn characters(&self, strchars: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).characters)(windows_core::Interface::as_raw(self), core::mem::transmute(strchars)) }
    }
    pub unsafe fn ignorableWhitespace(&self, strchars: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ignorableWhitespace)(windows_core::Interface::as_raw(self), core::mem::transmute(strchars)) }
    }
    pub unsafe fn processingInstruction(&self, strtarget: *mut windows_core::BSTR, strdata: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).processingInstruction)(windows_core::Interface::as_raw(self), core::mem::transmute(strtarget), core::mem::transmute(strdata)) }
    }
    pub unsafe fn skippedEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).skippedEntity)(windows_core::Interface::as_raw(self), core::mem::transmute(strname)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXContentHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub putref_documentLocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endDocument: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startPrefixMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endPrefixMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub characters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ignorableWhitespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub processingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub skippedEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXContentHandler_Impl: super::oaidl::IDispatch_Impl {
    fn putref_documentLocator(&self, olocator: windows_core::Ref<IVBSAXLocator>) -> windows_core::Result<()>;
    fn startDocument(&self) -> windows_core::Result<()>;
    fn endDocument(&self) -> windows_core::Result<()>;
    fn startPrefixMapping(&self, strprefix: *mut windows_core::BSTR, struri: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endPrefixMapping(&self, strprefix: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn startElement(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR, oattributes: windows_core::Ref<IVBSAXAttributes>) -> windows_core::Result<()>;
    fn endElement(&self, strnamespaceuri: *mut windows_core::BSTR, strlocalname: *mut windows_core::BSTR, strqname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn characters(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ignorableWhitespace(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn processingInstruction(&self, strtarget: *mut windows_core::BSTR, strdata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn skippedEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXContentHandler_Vtbl {
    pub const fn new<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn putref_documentLocator<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::putref_documentLocator(this, core::mem::transmute_copy(&olocator)).into()
            }
        }
        unsafe extern "system" fn startDocument<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::startDocument(this).into()
            }
        }
        unsafe extern "system" fn endDocument<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::endDocument(this).into()
            }
        }
        unsafe extern "system" fn startPrefixMapping<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: *mut *mut core::ffi::c_void, struri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::startPrefixMapping(this, core::mem::transmute_copy(&strprefix), core::mem::transmute_copy(&struri)).into()
            }
        }
        unsafe extern "system" fn endPrefixMapping<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::endPrefixMapping(this, core::mem::transmute_copy(&strprefix)).into()
            }
        }
        unsafe extern "system" fn startElement<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespaceuri: *mut *mut core::ffi::c_void, strlocalname: *mut *mut core::ffi::c_void, strqname: *mut *mut core::ffi::c_void, oattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::startElement(this, core::mem::transmute_copy(&strnamespaceuri), core::mem::transmute_copy(&strlocalname), core::mem::transmute_copy(&strqname), core::mem::transmute_copy(&oattributes)).into()
            }
        }
        unsafe extern "system" fn endElement<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespaceuri: *mut *mut core::ffi::c_void, strlocalname: *mut *mut core::ffi::c_void, strqname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::endElement(this, core::mem::transmute_copy(&strnamespaceuri), core::mem::transmute_copy(&strlocalname), core::mem::transmute_copy(&strqname)).into()
            }
        }
        unsafe extern "system" fn characters<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::characters(this, core::mem::transmute_copy(&strchars)).into()
            }
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::ignorableWhitespace(this, core::mem::transmute_copy(&strchars)).into()
            }
        }
        unsafe extern "system" fn processingInstruction<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtarget: *mut *mut core::ffi::c_void, strdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::processingInstruction(this, core::mem::transmute_copy(&strtarget), core::mem::transmute_copy(&strdata)).into()
            }
        }
        unsafe extern "system" fn skippedEntity<Identity: IVBSAXContentHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXContentHandler_Impl::skippedEntity(this, core::mem::transmute_copy(&strname)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            putref_documentLocator: putref_documentLocator::<Identity, OFFSET>,
            startDocument: startDocument::<Identity, OFFSET>,
            endDocument: endDocument::<Identity, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, OFFSET>,
            startElement: startElement::<Identity, OFFSET>,
            endElement: endElement::<Identity, OFFSET>,
            characters: characters::<Identity, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, OFFSET>,
            processingInstruction: processingInstruction::<Identity, OFFSET>,
            skippedEntity: skippedEntity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXContentHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXContentHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXDTDHandler, IVBSAXDTDHandler_Vtbl, 0x24fb3297_302d_4620_ba39_3a732d850558);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXDTDHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXDTDHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXDTDHandler {
    pub unsafe fn notationDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).notationDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strpublicid), core::mem::transmute(strsystemid)) }
    }
    pub unsafe fn unparsedEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR, strnotationname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).unparsedEntityDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strpublicid), core::mem::transmute(strsystemid), core::mem::transmute(strnotationname)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXDTDHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub notationDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub unparsedEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXDTDHandler_Impl: super::oaidl::IDispatch_Impl {
    fn notationDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn unparsedEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR, strnotationname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXDTDHandler_Vtbl {
    pub const fn new<Identity: IVBSAXDTDHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn notationDecl<Identity: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDTDHandler_Impl::notationDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
            }
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: IVBSAXDTDHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void, strnotationname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDTDHandler_Impl::unparsedEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid), core::mem::transmute_copy(&strnotationname)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            notationDecl: notationDecl::<Identity, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXDTDHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXDTDHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXDeclHandler, IVBSAXDeclHandler_Vtbl, 0xe8917260_7579_4be1_b5dd_7afbfa6f077b);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXDeclHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXDeclHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXDeclHandler {
    pub unsafe fn elementDecl(&self, strname: *mut windows_core::BSTR, strmodel: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).elementDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strmodel)) }
    }
    pub unsafe fn attributeDecl(&self, strelementname: *mut windows_core::BSTR, strattributename: *mut windows_core::BSTR, strtype: *mut windows_core::BSTR, strvaluedefault: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).attributeDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strelementname), core::mem::transmute(strattributename), core::mem::transmute(strtype), core::mem::transmute(strvaluedefault), core::mem::transmute(strvalue)) }
    }
    pub unsafe fn internalEntityDecl(&self, strname: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).internalEntityDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strvalue)) }
    }
    pub unsafe fn externalEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).externalEntityDecl)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strpublicid), core::mem::transmute(strsystemid)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXDeclHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub elementDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributeDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub internalEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub externalEntityDecl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXDeclHandler_Impl: super::oaidl::IDispatch_Impl {
    fn elementDecl(&self, strname: *mut windows_core::BSTR, strmodel: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn attributeDecl(&self, strelementname: *mut windows_core::BSTR, strattributename: *mut windows_core::BSTR, strtype: *mut windows_core::BSTR, strvaluedefault: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn internalEntityDecl(&self, strname: *mut windows_core::BSTR, strvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn externalEntityDecl(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXDeclHandler_Vtbl {
    pub const fn new<Identity: IVBSAXDeclHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn elementDecl<Identity: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDeclHandler_Impl::elementDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strmodel)).into()
            }
        }
        unsafe extern "system" fn attributeDecl<Identity: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strelementname: *mut *mut core::ffi::c_void, strattributename: *mut *mut core::ffi::c_void, strtype: *mut *mut core::ffi::c_void, strvaluedefault: *mut *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDeclHandler_Impl::attributeDecl(this, core::mem::transmute_copy(&strelementname), core::mem::transmute_copy(&strattributename), core::mem::transmute_copy(&strtype), core::mem::transmute_copy(&strvaluedefault), core::mem::transmute_copy(&strvalue)).into()
            }
        }
        unsafe extern "system" fn internalEntityDecl<Identity: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDeclHandler_Impl::internalEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strvalue)).into()
            }
        }
        unsafe extern "system" fn externalEntityDecl<Identity: IVBSAXDeclHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXDeclHandler_Impl::externalEntityDecl(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            elementDecl: elementDecl::<Identity, OFFSET>,
            attributeDecl: attributeDecl::<Identity, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXDeclHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXDeclHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXEntityResolver, IVBSAXEntityResolver_Vtbl, 0x0c05d096_f45b_4aca_ad1a_aa0bc25518dc);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXEntityResolver {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXEntityResolver, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXEntityResolver {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn resolveEntity(&self, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).resolveEntity)(windows_core::Interface::as_raw(self), core::mem::transmute(strpublicid), core::mem::transmute(strsystemid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXEntityResolver_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub resolveEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    resolveEntity: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXEntityResolver_Impl: super::oaidl::IDispatch_Impl {
    fn resolveEntity(&self, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXEntityResolver_Vtbl {
    pub const fn new<Identity: IVBSAXEntityResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn resolveEntity<Identity: IVBSAXEntityResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void, varinput: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXEntityResolver_Impl::resolveEntity(this, core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)) {
                    Ok(ok__) => {
                        varinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), resolveEntity: resolveEntity::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXEntityResolver as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXEntityResolver {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXErrorHandler, IVBSAXErrorHandler_Vtbl, 0xd963d3fe_173c_4862_9095_b92f66995f52);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXErrorHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXErrorHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXErrorHandler {
    pub unsafe fn error<P0>(&self, olocator: P0, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).error)(windows_core::Interface::as_raw(self), olocator.param().abi(), core::mem::transmute(strerrormessage), nerrorcode) }
    }
    pub unsafe fn fatalError<P0>(&self, olocator: P0, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).fatalError)(windows_core::Interface::as_raw(self), olocator.param().abi(), core::mem::transmute(strerrormessage), nerrorcode) }
    }
    pub unsafe fn ignorableWarning<P0>(&self, olocator: P0, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXLocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).ignorableWarning)(windows_core::Interface::as_raw(self), olocator.param().abi(), core::mem::transmute(strerrormessage), nerrorcode) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXErrorHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub fatalError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ignorableWarning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXErrorHandler_Impl: super::oaidl::IDispatch_Impl {
    fn error(&self, olocator: windows_core::Ref<IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
    fn fatalError(&self, olocator: windows_core::Ref<IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
    fn ignorableWarning(&self, olocator: windows_core::Ref<IVBSAXLocator>, strerrormessage: *mut windows_core::BSTR, nerrorcode: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXErrorHandler_Vtbl {
    pub const fn new<Identity: IVBSAXErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn error<Identity: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut *mut core::ffi::c_void, nerrorcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXErrorHandler_Impl::error(this, core::mem::transmute_copy(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
            }
        }
        unsafe extern "system" fn fatalError<Identity: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut *mut core::ffi::c_void, nerrorcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXErrorHandler_Impl::fatalError(this, core::mem::transmute_copy(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
            }
        }
        unsafe extern "system" fn ignorableWarning<Identity: IVBSAXErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, olocator: *mut core::ffi::c_void, strerrormessage: *mut *mut core::ffi::c_void, nerrorcode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXErrorHandler_Impl::ignorableWarning(this, core::mem::transmute_copy(&olocator), core::mem::transmute_copy(&strerrormessage), core::mem::transmute_copy(&nerrorcode)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            error: error::<Identity, OFFSET>,
            fatalError: fatalError::<Identity, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXErrorHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXErrorHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXLexicalHandler, IVBSAXLexicalHandler_Vtbl, 0x032aac35_8c0e_4d9d_979f_e3b702935576);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXLexicalHandler {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXLexicalHandler, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXLexicalHandler {
    pub unsafe fn startDTD(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startDTD)(windows_core::Interface::as_raw(self), core::mem::transmute(strname), core::mem::transmute(strpublicid), core::mem::transmute(strsystemid)) }
    }
    pub unsafe fn endDTD(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endDTD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn startEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startEntity)(windows_core::Interface::as_raw(self), core::mem::transmute(strname)) }
    }
    pub unsafe fn endEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endEntity)(windows_core::Interface::as_raw(self), core::mem::transmute(strname)) }
    }
    pub unsafe fn startCDATA(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startCDATA)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn endCDATA(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).endCDATA)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn comment(&self, strchars: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).comment)(windows_core::Interface::as_raw(self), core::mem::transmute(strchars)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXLexicalHandler_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub startDTD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endDTD: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startCDATA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub endCDATA: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXLexicalHandler_Impl: super::oaidl::IDispatch_Impl {
    fn startDTD(&self, strname: *mut windows_core::BSTR, strpublicid: *mut windows_core::BSTR, strsystemid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endDTD(&self) -> windows_core::Result<()>;
    fn startEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn endEntity(&self, strname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn startCDATA(&self) -> windows_core::Result<()>;
    fn endCDATA(&self) -> windows_core::Result<()>;
    fn comment(&self, strchars: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXLexicalHandler_Vtbl {
    pub const fn new<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn startDTD<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::startDTD(this, core::mem::transmute_copy(&strname), core::mem::transmute_copy(&strpublicid), core::mem::transmute_copy(&strsystemid)).into()
            }
        }
        unsafe extern "system" fn endDTD<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::endDTD(this).into()
            }
        }
        unsafe extern "system" fn startEntity<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::startEntity(this, core::mem::transmute_copy(&strname)).into()
            }
        }
        unsafe extern "system" fn endEntity<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::endEntity(this, core::mem::transmute_copy(&strname)).into()
            }
        }
        unsafe extern "system" fn startCDATA<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::startCDATA(this).into()
            }
        }
        unsafe extern "system" fn endCDATA<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::endCDATA(this).into()
            }
        }
        unsafe extern "system" fn comment<Identity: IVBSAXLexicalHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strchars: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXLexicalHandler_Impl::comment(this, core::mem::transmute_copy(&strchars)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            startDTD: startDTD::<Identity, OFFSET>,
            endDTD: endDTD::<Identity, OFFSET>,
            startEntity: startEntity::<Identity, OFFSET>,
            endEntity: endEntity::<Identity, OFFSET>,
            startCDATA: startCDATA::<Identity, OFFSET>,
            endCDATA: endCDATA::<Identity, OFFSET>,
            comment: comment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXLexicalHandler as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXLexicalHandler {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXLocator, IVBSAXLocator_Vtbl, 0x796e7ac5_5aa2_4eff_acad_3faaf01a3288);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXLocator {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXLocator, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXLocator {
    pub unsafe fn columnNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).columnNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn lineNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lineNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn publicId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).publicId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn systemId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXLocator_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub columnNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub lineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub publicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub systemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXLocator_Impl: super::oaidl::IDispatch_Impl {
    fn columnNumber(&self) -> windows_core::Result<i32>;
    fn lineNumber(&self) -> windows_core::Result<i32>;
    fn publicId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn systemId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXLocator_Vtbl {
    pub const fn new<Identity: IVBSAXLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn columnNumber<Identity: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolumn: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXLocator_Impl::columnNumber(this) {
                    Ok(ok__) => {
                        ncolumn.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lineNumber<Identity: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nline: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXLocator_Impl::lineNumber(this) {
                    Ok(ok__) => {
                        nline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn publicId<Identity: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpublicid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXLocator_Impl::publicId(this) {
                    Ok(ok__) => {
                        strpublicid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn systemId<Identity: IVBSAXLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsystemid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXLocator_Impl::systemId(this) {
                    Ok(ok__) => {
                        strsystemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            columnNumber: columnNumber::<Identity, OFFSET>,
            lineNumber: lineNumber::<Identity, OFFSET>,
            publicId: publicId::<Identity, OFFSET>,
            systemId: systemId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXLocator as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXLocator {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXXMLFilter, IVBSAXXMLFilter_Vtbl, 0x1299eb1b_5b88_433e_82de_82ca75ad4e04);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXXMLFilter {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXXMLFilter, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXXMLFilter {
    pub unsafe fn parent(&self) -> windows_core::Result<IVBSAXXMLReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_parent<P0>(&self, oreader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXXMLReader>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_parent)(windows_core::Interface::as_raw(self), oreader.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXXMLFilter_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXXMLFilter_Impl: super::oaidl::IDispatch_Impl {
    fn parent(&self) -> windows_core::Result<IVBSAXXMLReader>;
    fn putref_parent(&self, oreader: windows_core::Ref<IVBSAXXMLReader>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXXMLFilter_Vtbl {
    pub const fn new<Identity: IVBSAXXMLFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn parent<Identity: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oreader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLFilter_Impl::parent(this) {
                    Ok(ok__) => {
                        oreader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_parent<Identity: IVBSAXXMLFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oreader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLFilter_Impl::putref_parent(this, core::mem::transmute_copy(&oreader)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            parent: parent::<Identity, OFFSET>,
            putref_parent: putref_parent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLFilter as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXXMLFilter {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IVBSAXXMLReader, IVBSAXXMLReader_Vtbl, 0x8c033caa_6cd6_4f73_b728_4531af74945f);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IVBSAXXMLReader {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IVBSAXXMLReader, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IVBSAXXMLReader {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), fvalue) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), core::mem::transmute_copy(varvalue)) }
    }
    pub unsafe fn entityResolver(&self) -> windows_core::Result<IVBSAXEntityResolver> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).entityResolver)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_entityResolver<P0>(&self, oresolver: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXEntityResolver>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_entityResolver)(windows_core::Interface::as_raw(self), oresolver.param().abi()) }
    }
    pub unsafe fn contentHandler(&self) -> windows_core::Result<IVBSAXContentHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_contentHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXContentHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_contentHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
    pub unsafe fn dtdHandler(&self) -> windows_core::Result<IVBSAXDTDHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dtdHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_dtdHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXDTDHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_dtdHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
    pub unsafe fn errorHandler(&self) -> windows_core::Result<IVBSAXErrorHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_errorHandler<P0>(&self, ohandler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVBSAXErrorHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_errorHandler)(windows_core::Interface::as_raw(self), ohandler.param().abi()) }
    }
    pub unsafe fn baseURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetbaseURL(&self, strbaseurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetbaseURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strbaseurl)) }
    }
    pub unsafe fn secureBaseURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).secureBaseURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetsecureBaseURL(&self, strsecurebaseurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetsecureBaseURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsecurebaseurl)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn parse(&self, varinput: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).parse)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varinput)) }
    }
    pub unsafe fn parseURL(&self, strurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).parseURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strurl)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IVBSAXXMLReader_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub getFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    getFeature: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub putFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    putFeature: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub putProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    putProperty: usize,
    pub entityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_entityResolver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub contentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_contentHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dtdHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_dtdHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub errorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_errorHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub baseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetbaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub secureBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetsecureBaseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub parse: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    parse: usize,
    pub parseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IVBSAXXMLReader_Impl: super::oaidl::IDispatch_Impl {
    fn getFeature(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn putFeature(&self, strname: &windows_core::BSTR, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getProperty(&self, strname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn putProperty(&self, strname: &windows_core::BSTR, varvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn entityResolver(&self) -> windows_core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(&self, oresolver: windows_core::Ref<IVBSAXEntityResolver>) -> windows_core::Result<()>;
    fn contentHandler(&self) -> windows_core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(&self, ohandler: windows_core::Ref<IVBSAXContentHandler>) -> windows_core::Result<()>;
    fn dtdHandler(&self) -> windows_core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(&self, ohandler: windows_core::Ref<IVBSAXDTDHandler>) -> windows_core::Result<()>;
    fn errorHandler(&self) -> windows_core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(&self, ohandler: windows_core::Ref<IVBSAXErrorHandler>) -> windows_core::Result<()>;
    fn baseURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetbaseURL(&self, strbaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn secureBaseURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetsecureBaseURL(&self, strsecurebaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parse(&self, varinput: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn parseURL(&self, strurl: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IVBSAXXMLReader_Vtbl {
    pub const fn new<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getFeature<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, fvalue: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::getFeature(this, core::mem::transmute(&strname)) {
                    Ok(ok__) => {
                        fvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putFeature<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, fvalue: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putFeature(this, core::mem::transmute(&strname), core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn getProperty<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::getProperty(this, core::mem::transmute(&strname)) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putProperty<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putProperty(this, core::mem::transmute(&strname), core::mem::transmute(&varvalue)).into()
            }
        }
        unsafe extern "system" fn entityResolver<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::entityResolver(this) {
                    Ok(ok__) => {
                        oresolver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_entityResolver<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oresolver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putref_entityResolver(this, core::mem::transmute_copy(&oresolver)).into()
            }
        }
        unsafe extern "system" fn contentHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::contentHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_contentHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putref_contentHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        unsafe extern "system" fn dtdHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::dtdHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putref_dtdHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        unsafe extern "system" fn errorHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::errorHandler(this) {
                    Ok(ok__) => {
                        ohandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_errorHandler<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ohandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::putref_errorHandler(this, core::mem::transmute_copy(&ohandler)).into()
            }
        }
        unsafe extern "system" fn baseURL<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbaseurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::baseURL(this) {
                    Ok(ok__) => {
                        strbaseurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetbaseURL<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbaseurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::SetbaseURL(this, core::mem::transmute(&strbaseurl)).into()
            }
        }
        unsafe extern "system" fn secureBaseURL<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsecurebaseurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVBSAXXMLReader_Impl::secureBaseURL(this) {
                    Ok(ok__) => {
                        strsecurebaseurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsecurebaseurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::SetsecureBaseURL(this, core::mem::transmute(&strsecurebaseurl)).into()
            }
        }
        unsafe extern "system" fn parse<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varinput: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::parse(this, core::mem::transmute(&varinput)).into()
            }
        }
        unsafe extern "system" fn parseURL<Identity: IVBSAXXMLReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVBSAXXMLReader_Impl::parseURL(this, core::mem::transmute(&strurl)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            getFeature: getFeature::<Identity, OFFSET>,
            putFeature: putFeature::<Identity, OFFSET>,
            getProperty: getProperty::<Identity, OFFSET>,
            putProperty: putProperty::<Identity, OFFSET>,
            entityResolver: entityResolver::<Identity, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, OFFSET>,
            contentHandler: contentHandler::<Identity, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, OFFSET>,
            dtdHandler: dtdHandler::<Identity, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, OFFSET>,
            errorHandler: errorHandler::<Identity, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, OFFSET>,
            baseURL: baseURL::<Identity, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, OFFSET>,
            secureBaseURL: secureBaseURL::<Identity, OFFSET>,
            SetsecureBaseURL: SetsecureBaseURL::<Identity, OFFSET>,
            parse: parse::<Identity, OFFSET>,
            parseURL: parseURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVBSAXXMLReader as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IVBSAXXMLReader {}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::define_interface!(IXMLDOMDocument2, IXMLDOMDocument2_Vtbl, 0x2933bf95_7b36_11d2_b20e_00c04f983e60);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl core::ops::Deref for IXMLDOMDocument2 {
    type Target = super::msxml::IXMLDOMDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::interface_hierarchy!(IXMLDOMDocument2, windows_core::IUnknown, super::oaidl::IDispatch, super::msxml::IXMLDOMNode, super::msxml::IXMLDOMDocument);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl IXMLDOMDocument2 {
    pub unsafe fn namespaces(&self) -> windows_core::Result<IXMLDOMSchemaCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).namespaces)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn schemas(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).schemas)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn putref_schemas(&self, othercollection: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).putref_schemas)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(othercollection)) }
    }
    pub unsafe fn validate(&self) -> windows_core::Result<super::msxml::IXMLDOMParseError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).validate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setProperty(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument2_Vtbl {
    pub base__: super::msxml::IXMLDOMDocument_Vtbl,
    pub namespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub schemas: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    schemas: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub putref_schemas: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    putref_schemas: usize,
    pub validate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getProperty: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMDocument2_Impl: super::msxml::IXMLDOMDocument_Impl {
    fn namespaces(&self) -> windows_core::Result<IXMLDOMSchemaCollection>;
    fn schemas(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn putref_schemas(&self, othercollection: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn validate(&self) -> windows_core::Result<super::msxml::IXMLDOMParseError>;
    fn setProperty(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMDocument2_Vtbl {
    pub const fn new<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn namespaces<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespacecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument2_Impl::namespaces(this) {
                    Ok(ok__) => {
                        namespacecollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn schemas<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument2_Impl::schemas(this) {
                    Ok(ok__) => {
                        othercollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_schemas<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument2_Impl::putref_schemas(this, core::mem::transmute(&othercollection)).into()
            }
        }
        unsafe extern "system" fn validate<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument2_Impl::validate(this) {
                    Ok(ok__) => {
                        errorobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProperty<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument2_Impl::setProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn getProperty<Identity: IXMLDOMDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument2_Impl::getProperty(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::msxml::IXMLDOMDocument_Vtbl::new::<Identity, OFFSET>(),
            namespaces: namespaces::<Identity, OFFSET>,
            schemas: schemas::<Identity, OFFSET>,
            putref_schemas: putref_schemas::<Identity, OFFSET>,
            validate: validate::<Identity, OFFSET>,
            setProperty: setProperty::<Identity, OFFSET>,
            getProperty: getProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMNode as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMDocument as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMDocument2 {}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::define_interface!(IXMLDOMDocument3, IXMLDOMDocument3_Vtbl, 0x2933bf96_7b36_11d2_b20e_00c04f983e60);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl core::ops::Deref for IXMLDOMDocument3 {
    type Target = IXMLDOMDocument2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::interface_hierarchy!(IXMLDOMDocument3, windows_core::IUnknown, super::oaidl::IDispatch, super::msxml::IXMLDOMNode, super::msxml::IXMLDOMDocument, IXMLDOMDocument2);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl IXMLDOMDocument3 {
    pub unsafe fn validateNode<P0>(&self, node: P0) -> windows_core::Result<super::msxml::IXMLDOMParseError>
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).validateNode)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn importNode<P0>(&self, node: P0, deep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::msxml::IXMLDOMNode>
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).importNode)(windows_core::Interface::as_raw(self), node.param().abi(), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument3_Vtbl {
    pub base__: IXMLDOMDocument2_Vtbl,
    pub validateNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub importNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    importNode: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMDocument3_Impl: IXMLDOMDocument2_Impl {
    fn validateNode(&self, node: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<super::msxml::IXMLDOMParseError>;
    fn importNode(&self, node: windows_core::Ref<super::msxml::IXMLDOMNode>, deep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::msxml::IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMDocument3_Vtbl {
    pub const fn new<Identity: IXMLDOMDocument3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn validateNode<Identity: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument3_Impl::validateNode(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        errorobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn importNode<Identity: IXMLDOMDocument3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, deep: super::wtypes::VARIANT_BOOL, clone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument3_Impl::importNode(this, core::mem::transmute_copy(&node), core::mem::transmute_copy(&deep)) {
                    Ok(ok__) => {
                        clone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXMLDOMDocument2_Vtbl::new::<Identity, OFFSET>(),
            validateNode: validateNode::<Identity, OFFSET>,
            importNode: importNode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMNode as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMDocument as windows_core::Interface>::IID || iid == &<IXMLDOMDocument2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMDocument3 {}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::define_interface!(IXMLDOMParseError2, IXMLDOMParseError2_Vtbl, 0x3efaa428_272f_11d2_836f_0000f87a7782);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl core::ops::Deref for IXMLDOMParseError2 {
    type Target = super::msxml::IXMLDOMParseError;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::interface_hierarchy!(IXMLDOMParseError2, windows_core::IUnknown, super::oaidl::IDispatch, super::msxml::IXMLDOMParseError);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl IXMLDOMParseError2 {
    pub unsafe fn errorXPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorXPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn allErrors(&self) -> windows_core::Result<IXMLDOMParseErrorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).allErrors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn errorParameters(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorParameters)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn errorParametersCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorParametersCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseError2_Vtbl {
    pub base__: super::msxml::IXMLDOMParseError_Vtbl,
    pub errorXPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub allErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub errorParameters: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub errorParametersCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMParseError2_Impl: super::msxml::IXMLDOMParseError_Impl {
    fn errorXPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn allErrors(&self) -> windows_core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn errorParametersCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMParseError2_Vtbl {
    pub const fn new<Identity: IXMLDOMParseError2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn errorXPath<Identity: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpathexpr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError2_Impl::errorXPath(this) {
                    Ok(ok__) => {
                        xpathexpr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn allErrors<Identity: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError2_Impl::allErrors(this) {
                    Ok(ok__) => {
                        allerrors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn errorParameters<Identity: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, param: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError2_Impl::errorParameters(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        param.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn errorParametersCount<Identity: IXMLDOMParseError2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError2_Impl::errorParametersCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::msxml::IXMLDOMParseError_Vtbl::new::<Identity, OFFSET>(),
            errorXPath: errorXPath::<Identity, OFFSET>,
            allErrors: allErrors::<Identity, OFFSET>,
            errorParameters: errorParameters::<Identity, OFFSET>,
            errorParametersCount: errorParametersCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMParseError as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMParseError2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMParseErrorCollection, IXMLDOMParseErrorCollection_Vtbl, 0x3efaa429_272f_11d2_836f_0000f87a7782);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMParseErrorCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMParseErrorCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMParseErrorCollection {
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<IXMLDOMParseError2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn next(&self) -> windows_core::Result<IXMLDOMParseError2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._newEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseErrorCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_msxml")]
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    item: usize,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_msxml")]
    pub next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    next: usize,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMParseErrorCollection_Impl: super::oaidl::IDispatch_Impl {
    fn item(&self, index: i32) -> windows_core::Result<IXMLDOMParseError2>;
    fn length(&self) -> windows_core::Result<i32>;
    fn next(&self) -> windows_core::Result<IXMLDOMParseError2>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMParseErrorCollection_Vtbl {
    pub const fn new<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn item<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, error: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseErrorCollection_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        error.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseErrorCollection_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn next<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseErrorCollection_Impl::next(this) {
                    Ok(ok__) => {
                        error.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMParseErrorCollection_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IXMLDOMParseErrorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseErrorCollection_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            item: item::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            next: next::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseErrorCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMParseErrorCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMSchemaCollection, IXMLDOMSchemaCollection_Vtbl, 0x373984c8_b845_449b_91e7_45ac83036ade);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMSchemaCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMSchemaCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMSchemaCollection {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn add(&self, namespaceuri: &windows_core::BSTR, var: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(namespaceuri), core::mem::transmute_copy(var)) }
    }
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn get(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn remove(&self, namespaceuri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(namespaceuri)) }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn namespaceURI(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).namespaceURI)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn addCollection<P0>(&self, othercollection: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).addCollection)(windows_core::Interface::as_raw(self), othercollection.param().abi()) }
    }
    pub unsafe fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._newEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSchemaCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    add: usize,
    #[cfg(feature = "Win32_msxml")]
    pub get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    get: usize,
    pub remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub namespaceURI: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub addCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMSchemaCollection_Impl: super::oaidl::IDispatch_Impl {
    fn add(&self, namespaceuri: &windows_core::BSTR, var: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn get(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn remove(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn namespaceURI(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn addCollection(&self, othercollection: windows_core::Ref<IXMLDOMSchemaCollection>) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMSchemaCollection_Vtbl {
    pub const fn new<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn add<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, var: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSchemaCollection_Impl::add(this, core::mem::transmute(&namespaceuri), core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn get<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, schemanode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection_Impl::get(this, core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        schemanode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSchemaCollection_Impl::remove(this, core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn length<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection_Impl::length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, length: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection_Impl::namespaceURI(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        length.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addCollection<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, othercollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSchemaCollection_Impl::addCollection(this, core::mem::transmute_copy(&othercollection)).into()
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IXMLDOMSchemaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            add: add::<Identity, OFFSET>,
            get: get::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            namespaceURI: namespaceURI::<Identity, OFFSET>,
            addCollection: addCollection::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMSchemaCollection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMSchemaCollection2, IXMLDOMSchemaCollection2_Vtbl, 0x50ea08b0_dd1b_4664_9a50_c2f40f4bd79a);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMSchemaCollection2 {
    type Target = IXMLDOMSchemaCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMSchemaCollection2, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMSchemaCollection);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMSchemaCollection2 {
    pub unsafe fn validate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).validate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetvalidateOnLoad(&self, validateonload: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetvalidateOnLoad)(windows_core::Interface::as_raw(self), validateonload) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn validateOnLoad(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).validateOnLoad)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getSchema(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchema> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSchema)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn getDeclaration<P0>(&self, node: P0) -> windows_core::Result<ISchemaItem>
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getDeclaration)(windows_core::Interface::as_raw(self), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSchemaCollection2_Vtbl {
    pub base__: IXMLDOMSchemaCollection_Vtbl,
    pub validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub SetvalidateOnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetvalidateOnLoad: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub validateOnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    validateOnLoad: usize,
    pub getSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_msxml")]
    pub getDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    getDeclaration: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMSchemaCollection2_Impl: IXMLDOMSchemaCollection_Impl {
    fn validate(&self) -> windows_core::Result<()>;
    fn SetvalidateOnLoad(&self, validateonload: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn validateOnLoad(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn getSchema(&self, namespaceuri: &windows_core::BSTR) -> windows_core::Result<ISchema>;
    fn getDeclaration(&self, node: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMSchemaCollection2_Vtbl {
    pub const fn new<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn validate<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSchemaCollection2_Impl::validate(this).into()
            }
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, validateonload: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSchemaCollection2_Impl::SetvalidateOnLoad(this, core::mem::transmute_copy(&validateonload)).into()
            }
        }
        unsafe extern "system" fn validateOnLoad<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, validateonload: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection2_Impl::validateOnLoad(this) {
                    Ok(ok__) => {
                        validateonload.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSchema<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, schema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection2_Impl::getSchema(this, core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        schema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getDeclaration<Identity: IXMLDOMSchemaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, node: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSchemaCollection2_Impl::getDeclaration(this, core::mem::transmute_copy(&node)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXMLDOMSchemaCollection_Vtbl::new::<Identity, OFFSET>(),
            validate: validate::<Identity, OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Identity, OFFSET>,
            validateOnLoad: validateOnLoad::<Identity, OFFSET>,
            getSchema: getSchema::<Identity, OFFSET>,
            getDeclaration: getDeclaration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSchemaCollection2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMSchemaCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMSchemaCollection2 {}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::define_interface!(IXMLDOMSelection, IXMLDOMSelection_Vtbl, 0xaa634fc7_5888_44a7_a257_3a47150d3a0e);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl core::ops::Deref for IXMLDOMSelection {
    type Target = super::msxml::IXMLDOMNodeList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
windows_core::imp::interface_hierarchy!(IXMLDOMSelection, windows_core::IUnknown, super::oaidl::IDispatch, super::msxml::IXMLDOMNodeList);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
impl IXMLDOMSelection {
    pub unsafe fn expr(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).expr)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setexpr(&self, expression: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setexpr)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(expression)) }
    }
    pub unsafe fn context(&self) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).context)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_context<P0>(&self, pnode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_context)(windows_core::Interface::as_raw(self), pnode.param().abi()) }
    }
    pub unsafe fn peekNode(&self) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).peekNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn matches<P0>(&self, pnode: P0) -> windows_core::Result<super::msxml::IXMLDOMNode>
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).matches)(windows_core::Interface::as_raw(self), pnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeNext(&self) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).removeNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).removeAll)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setProperty(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)) }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMSelection_Vtbl {
    pub base__: super::msxml::IXMLDOMNodeList_Vtbl,
    pub expr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setexpr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub peekNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub matches: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setProperty: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMSelection_Impl: super::msxml::IXMLDOMNodeList_Impl {
    fn expr(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setexpr(&self, expression: &windows_core::BSTR) -> windows_core::Result<()>;
    fn context(&self) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn putref_context(&self, pnode: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<()>;
    fn peekNode(&self) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn matches(&self, pnode: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn removeNext(&self) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn removeAll(&self) -> windows_core::Result<()>;
    fn clone(&self) -> windows_core::Result<IXMLDOMSelection>;
    fn getProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn setProperty(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMSelection_Vtbl {
    pub const fn new<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn expr<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expression: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::expr(this) {
                    Ok(ok__) => {
                        expression.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setexpr<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expression: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSelection_Impl::Setexpr(this, core::mem::transmute(&expression)).into()
            }
        }
        unsafe extern "system" fn context<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::context(this) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_context<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSelection_Impl::putref_context(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn peekNode<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::peekNode(this) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn matches<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::matches(this, core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeNext<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::removeNext(this) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeAll<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSelection_Impl::removeAll(this).into()
            }
        }
        unsafe extern "system" fn clone<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::clone(this) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getProperty<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMSelection_Impl::getProperty(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProperty<Identity: IXMLDOMSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMSelection_Impl::setProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::msxml::IXMLDOMNodeList_Vtbl::new::<Identity, OFFSET>(),
            expr: expr::<Identity, OFFSET>,
            Setexpr: Setexpr::<Identity, OFFSET>,
            context: context::<Identity, OFFSET>,
            putref_context: putref_context::<Identity, OFFSET>,
            peekNode: peekNode::<Identity, OFFSET>,
            matches: matches::<Identity, OFFSET>,
            removeNext: removeNext::<Identity, OFFSET>,
            removeAll: removeAll::<Identity, OFFSET>,
            clone: clone::<Identity, OFFSET>,
            getProperty: getProperty::<Identity, OFFSET>,
            setProperty: setProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMSelection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<super::msxml::IXMLDOMNodeList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMSelection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLHTTPRequest, IXMLHTTPRequest_Vtbl, 0xed8c108d_4349_11d2_91a4_00c04f7969e8);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLHTTPRequest {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLHTTPRequest, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLHTTPRequest {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn open(&self, bstrmethod: &windows_core::BSTR, bstrurl: &windows_core::BSTR, varasync: &super::oaidl::VARIANT, bstruser: &super::oaidl::VARIANT, bstrpassword: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmethod), core::mem::transmute_copy(bstrurl), core::mem::transmute_copy(varasync), core::mem::transmute_copy(bstruser), core::mem::transmute_copy(bstrpassword)) }
    }
    pub unsafe fn setRequestHeader(&self, bstrheader: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setRequestHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrheader), core::mem::transmute_copy(bstrvalue)) }
    }
    pub unsafe fn getResponseHeader(&self, bstrheader: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getResponseHeader)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrheader), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getAllResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAllResponseHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn send(&self, varbody: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).send)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varbody)) }
    }
    pub unsafe fn abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).abort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn status(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn statusText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).statusText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn responseXML(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).responseXML)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn responseText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).responseText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn responseBody(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).responseBody)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn responseStream(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).responseStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Setonreadystatechange<P0>(&self, preadystatesink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Setonreadystatechange)(windows_core::Interface::as_raw(self), preadystatesink.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    open: usize,
    pub setRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getResponseHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAllResponseHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub send: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    send: usize,
    pub abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub statusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub responseXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub responseText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub responseBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    responseBody: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub responseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    responseStream: usize,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Setonreadystatechange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLHTTPRequest_Impl: super::oaidl::IDispatch_Impl {
    fn open(&self, bstrmethod: &windows_core::BSTR, bstrurl: &windows_core::BSTR, varasync: &super::oaidl::VARIANT, bstruser: &super::oaidl::VARIANT, bstrpassword: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn setRequestHeader(&self, bstrheader: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getResponseHeader(&self, bstrheader: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getAllResponseHeaders(&self) -> windows_core::Result<windows_core::BSTR>;
    fn send(&self, varbody: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn abort(&self) -> windows_core::Result<()>;
    fn status(&self) -> windows_core::Result<i32>;
    fn statusText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn responseXML(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn responseText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn responseBody(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn responseStream(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn Setonreadystatechange(&self, preadystatesink: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLHTTPRequest_Vtbl {
    pub const fn new<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn open<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethod: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, varasync: super::oaidl::VARIANT, bstruser: super::oaidl::VARIANT, bstrpassword: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest_Impl::open(this, core::mem::transmute(&bstrmethod), core::mem::transmute(&bstrurl), core::mem::transmute(&varasync), core::mem::transmute(&bstruser), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn setRequestHeader<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest_Impl::setRequestHeader(this, core::mem::transmute(&bstrheader), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn getResponseHeader<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::getResponseHeader(this, core::mem::transmute(&bstrheader)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrheaders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::getAllResponseHeaders(this) {
                    Ok(ok__) => {
                        pbstrheaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn send<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest_Impl::send(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn abort<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest_Impl::abort(this).into()
            }
        }
        unsafe extern "system" fn status<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::status(this) {
                    Ok(ok__) => {
                        plstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn statusText<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::statusText(this) {
                    Ok(ok__) => {
                        pbstrstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseXML<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::responseXML(this) {
                    Ok(ok__) => {
                        ppbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseText<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::responseText(this) {
                    Ok(ok__) => {
                        pbstrbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseBody<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::responseBody(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseStream<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::responseStream(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest_Impl::readyState(this) {
                    Ok(ok__) => {
                        plstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: IXMLHTTPRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadystatesink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest_Impl::Setonreadystatechange(this, core::mem::transmute_copy(&preadystatesink)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            open: open::<Identity, OFFSET>,
            setRequestHeader: setRequestHeader::<Identity, OFFSET>,
            getResponseHeader: getResponseHeader::<Identity, OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Identity, OFFSET>,
            send: send::<Identity, OFFSET>,
            abort: abort::<Identity, OFFSET>,
            status: status::<Identity, OFFSET>,
            statusText: statusText::<Identity, OFFSET>,
            responseXML: responseXML::<Identity, OFFSET>,
            responseText: responseText::<Identity, OFFSET>,
            responseBody: responseBody::<Identity, OFFSET>,
            responseStream: responseStream::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLHTTPRequest {}
windows_core::imp::define_interface!(IXMLHTTPRequest2, IXMLHTTPRequest2_Vtbl, 0xe5d37dc0_552a_4d52_9cc0_a14d546fbd04);
windows_core::imp::interface_hierarchy!(IXMLHTTPRequest2, windows_core::IUnknown);
impl IXMLHTTPRequest2 {
    pub unsafe fn Open<P2>(&self, pwszmethod: *const u16, pwszurl: *const u16, pstatuscallback: P2, pwszusername: *const u16, pwszpassword: *const u16, pwszproxyusername: *const u16, pwszproxypassword: *const u16) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IXMLHTTPRequest2Callback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pwszmethod, pwszurl, pstatuscallback.param().abi(), pwszusername, pwszpassword, pwszproxyusername, pwszproxypassword) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Send<P0>(&self, pbody: P0, cbbody: u64) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pbody.param().abi(), cbbody) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetCookie)(windows_core::Interface::as_raw(self), pcookie, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn SetCustomResponseStream<P0>(&self, psequentialstream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomResponseStream)(windows_core::Interface::as_raw(self), psequentialstream.param().abi()) }
    }
    pub unsafe fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), eproperty, ullvalue) }
    }
    pub unsafe fn SetRequestHeader(&self, pwszheader: *const u16, pwszvalue: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequestHeader)(windows_core::Interface::as_raw(self), pwszheader, pwszvalue) }
    }
    pub unsafe fn GetAllResponseHeaders(&self) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllResponseHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetCookie(&self, pwszurl: *const u16, pwszname: *const u16, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCookie)(windows_core::Interface::as_raw(self), pwszurl, pwszname, dwflags, pccookies as _, ppcookies as _) }
    }
    pub unsafe fn GetResponseHeader(&self, pwszheader: *const u16) -> windows_core::Result<*mut u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResponseHeader)(windows_core::Interface::as_raw(self), pwszheader, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, *mut core::ffi::c_void, *const u16, *const u16, *const u16, *const u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Send: usize,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub SetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *const XHR_COOKIE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    SetCookie: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub SetCustomResponseStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    SetCustomResponseStream: usize,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, XHR_PROPERTY, u64) -> windows_core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16) -> windows_core::HRESULT,
    pub GetAllResponseHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *const u16, u32, *mut u32, *mut *mut XHR_COOKIE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetCookie: usize,
    pub GetResponseHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut *mut u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
pub trait IXMLHTTPRequest2_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pwszmethod: *const u16, pwszurl: *const u16, pstatuscallback: windows_core::Ref<IXMLHTTPRequest2Callback>, pwszusername: *const u16, pwszpassword: *const u16, pwszproxyusername: *const u16, pwszproxypassword: *const u16) -> windows_core::Result<()>;
    fn Send(&self, pbody: windows_core::Ref<super::objidlbase::ISequentialStream>, cbbody: u64) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn SetCookie(&self, pcookie: *const XHR_COOKIE) -> windows_core::Result<u32>;
    fn SetCustomResponseStream(&self, psequentialstream: windows_core::Ref<super::objidlbase::ISequentialStream>) -> windows_core::Result<()>;
    fn SetProperty(&self, eproperty: XHR_PROPERTY, ullvalue: u64) -> windows_core::Result<()>;
    fn SetRequestHeader(&self, pwszheader: *const u16, pwszvalue: *const u16) -> windows_core::Result<()>;
    fn GetAllResponseHeaders(&self) -> windows_core::Result<*mut u16>;
    fn GetCookie(&self, pwszurl: *const u16, pwszname: *const u16, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> windows_core::Result<()>;
    fn GetResponseHeader(&self, pwszheader: *const u16) -> windows_core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
impl IXMLHTTPRequest2_Vtbl {
    pub const fn new<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmethod: *const u16, pwszurl: *const u16, pstatuscallback: *mut core::ffi::c_void, pwszusername: *const u16, pwszpassword: *const u16, pwszproxyusername: *const u16, pwszproxypassword: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::Open(this, core::mem::transmute_copy(&pwszmethod), core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pstatuscallback), core::mem::transmute_copy(&pwszusername), core::mem::transmute_copy(&pwszpassword), core::mem::transmute_copy(&pwszproxyusername), core::mem::transmute_copy(&pwszproxypassword)).into()
            }
        }
        unsafe extern "system" fn Send<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *mut core::ffi::c_void, cbbody: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::Send(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&cbbody)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn SetCookie<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest2_Impl::SetCookie(this, core::mem::transmute_copy(&pcookie)) {
                    Ok(ok__) => {
                        pdwcookiestate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psequentialstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::SetCustomResponseStream(this, core::mem::transmute_copy(&psequentialstream)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::SetProperty(this, core::mem::transmute_copy(&eproperty), core::mem::transmute_copy(&ullvalue)).into()
            }
        }
        unsafe extern "system" fn SetRequestHeader<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszheader: *const u16, pwszvalue: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::SetRequestHeader(this, core::mem::transmute_copy(&pwszheader), core::mem::transmute_copy(&pwszvalue)).into()
            }
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszheaders: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest2_Impl::GetAllResponseHeaders(this) {
                    Ok(ok__) => {
                        ppwszheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCookie<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: *const u16, pwszname: *const u16, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2_Impl::GetCookie(this, core::mem::transmute_copy(&pwszurl), core::mem::transmute_copy(&pwszname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pccookies), core::mem::transmute_copy(&ppcookies)).into()
            }
        }
        unsafe extern "system" fn GetResponseHeader<Identity: IXMLHTTPRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszheader: *const u16, ppwszvalue: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHTTPRequest2_Impl::GetResponseHeader(this, core::mem::transmute_copy(&pwszheader)) {
                    Ok(ok__) => {
                        ppwszvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            SetCookie: SetCookie::<Identity, OFFSET>,
            SetCustomResponseStream: SetCustomResponseStream::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, OFFSET>,
            GetCookie: GetCookie::<Identity, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
impl windows_core::RuntimeName for IXMLHTTPRequest2 {}
windows_core::imp::define_interface!(IXMLHTTPRequest2Callback, IXMLHTTPRequest2Callback_Vtbl, 0xa44a9299_e321_40de_8866_341b41669162);
windows_core::imp::interface_hierarchy!(IXMLHTTPRequest2Callback, windows_core::IUnknown);
impl IXMLHTTPRequest2Callback {
    pub unsafe fn OnRedirect<P0>(&self, pxhr: P0, pwszredirecturl: *const u16) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnRedirect)(windows_core::Interface::as_raw(self), pxhr.param().abi(), pwszredirecturl) }
    }
    pub unsafe fn OnHeadersAvailable<P0>(&self, pxhr: P0, dwstatus: u32, pwszstatus: *const u16) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnHeadersAvailable)(windows_core::Interface::as_raw(self), pxhr.param().abi(), dwstatus, pwszstatus) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn OnDataAvailable<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest2>,
        P1: windows_core::Param<super::objidlbase::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDataAvailable)(windows_core::Interface::as_raw(self), pxhr.param().abi(), presponsestream.param().abi()) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn OnResponseReceived<P0, P1>(&self, pxhr: P0, presponsestream: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest2>,
        P1: windows_core::Param<super::objidlbase::ISequentialStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnResponseReceived)(windows_core::Interface::as_raw(self), pxhr.param().abi(), presponsestream.param().abi()) }
    }
    pub unsafe fn OnError<P0>(&self, pxhr: P0, hrerror: windows_core::HRESULT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), pxhr.param().abi(), hrerror) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest2Callback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u16) -> windows_core::HRESULT,
    pub OnHeadersAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub OnDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    OnDataAvailable: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub OnResponseReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    OnResponseReceived: usize,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IXMLHTTPRequest2Callback_Impl: windows_core::IUnknownImpl {
    fn OnRedirect(&self, pxhr: windows_core::Ref<IXMLHTTPRequest2>, pwszredirecturl: *const u16) -> windows_core::Result<()>;
    fn OnHeadersAvailable(&self, pxhr: windows_core::Ref<IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: *const u16) -> windows_core::Result<()>;
    fn OnDataAvailable(&self, pxhr: windows_core::Ref<IXMLHTTPRequest2>, presponsestream: windows_core::Ref<super::objidlbase::ISequentialStream>) -> windows_core::Result<()>;
    fn OnResponseReceived(&self, pxhr: windows_core::Ref<IXMLHTTPRequest2>, presponsestream: windows_core::Ref<super::objidlbase::ISequentialStream>) -> windows_core::Result<()>;
    fn OnError(&self, pxhr: windows_core::Ref<IXMLHTTPRequest2>, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IXMLHTTPRequest2Callback_Vtbl {
    pub const fn new<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRedirect<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, pwszredirecturl: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2Callback_Impl::OnRedirect(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&pwszredirecturl)).into()
            }
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, dwstatus: u32, pwszstatus: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2Callback_Impl::OnHeadersAvailable(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&dwstatus), core::mem::transmute_copy(&pwszstatus)).into()
            }
        }
        unsafe extern "system" fn OnDataAvailable<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, presponsestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2Callback_Impl::OnDataAvailable(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&presponsestream)).into()
            }
        }
        unsafe extern "system" fn OnResponseReceived<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, presponsestream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2Callback_Impl::OnResponseReceived(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&presponsestream)).into()
            }
        }
        unsafe extern "system" fn OnError<Identity: IXMLHTTPRequest2Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest2Callback_Impl::OnError(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&hrerror)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest2Callback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IXMLHTTPRequest2Callback {}
windows_core::imp::define_interface!(IXMLHTTPRequest3, IXMLHTTPRequest3_Vtbl, 0xa1c9feee_0617_4f23_9d58_8961ea43567c);
impl core::ops::Deref for IXMLHTTPRequest3 {
    type Target = IXMLHTTPRequest2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXMLHTTPRequest3, windows_core::IUnknown, IXMLHTTPRequest2);
impl IXMLHTTPRequest3 {
    pub unsafe fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: *const u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificate)(windows_core::Interface::as_raw(self), cbclientcertificatehash, pbclientcertificatehash, pwszpin) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest3_Vtbl {
    pub base__: IXMLHTTPRequest2_Vtbl,
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *const u16) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
pub trait IXMLHTTPRequest3_Impl: IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(&self, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: *const u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
impl IXMLHTTPRequest3_Vtbl {
    pub const fn new<Identity: IXMLHTTPRequest3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetClientCertificate<Identity: IXMLHTTPRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest3_Impl::SetClientCertificate(this, core::mem::transmute_copy(&cbclientcertificatehash), core::mem::transmute_copy(&pbclientcertificatehash), core::mem::transmute_copy(&pwszpin)).into()
            }
        }
        Self { base__: IXMLHTTPRequest2_Vtbl::new::<Identity, OFFSET>(), SetClientCertificate: SetClientCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3 as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
impl windows_core::RuntimeName for IXMLHTTPRequest3 {}
windows_core::imp::define_interface!(IXMLHTTPRequest3Callback, IXMLHTTPRequest3Callback_Vtbl, 0xb9e57830_8c6c_4a6f_9c13_47772bb047bb);
impl core::ops::Deref for IXMLHTTPRequest3Callback {
    type Target = IXMLHTTPRequest2Callback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXMLHTTPRequest3Callback, windows_core::IUnknown, IXMLHTTPRequest2Callback);
impl IXMLHTTPRequest3Callback {
    pub unsafe fn OnServerCertificateReceived<P0>(&self, pxhr: P0, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest3>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnServerCertificateReceived)(windows_core::Interface::as_raw(self), pxhr.param().abi(), dwcertificateerrors, cservercertificatechain, rgservercertificatechain) }
    }
    pub unsafe fn OnClientCertificateRequested<P0>(&self, pxhr: P0, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLHTTPRequest3>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnClientCertificateRequested)(windows_core::Interface::as_raw(self), pxhr.param().abi(), cissuerlist, rgpwszissuerlist) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLHTTPRequest3Callback_Vtbl {
    pub base__: IXMLHTTPRequest2Callback_Vtbl,
    pub OnServerCertificateReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const XHR_CERT) -> windows_core::HRESULT,
    pub OnClientCertificateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const *const u16) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IXMLHTTPRequest3Callback_Impl: IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(&self, pxhr: windows_core::Ref<IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> windows_core::Result<()>;
    fn OnClientCertificateRequested(&self, pxhr: windows_core::Ref<IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IXMLHTTPRequest3Callback_Vtbl {
    pub const fn new<Identity: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest3Callback_Impl::OnServerCertificateReceived(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&dwcertificateerrors), core::mem::transmute_copy(&cservercertificatechain), core::mem::transmute_copy(&rgservercertificatechain)).into()
            }
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: IXMLHTTPRequest3Callback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxhr: *mut core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHTTPRequest3Callback_Impl::OnClientCertificateRequested(this, core::mem::transmute_copy(&pxhr), core::mem::transmute_copy(&cissuerlist), core::mem::transmute_copy(&rgpwszissuerlist)).into()
            }
        }
        Self {
            base__: IXMLHTTPRequest2Callback_Vtbl::new::<Identity, OFFSET>(),
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLHTTPRequest3Callback as windows_core::Interface>::IID || iid == &<IXMLHTTPRequest2Callback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IXMLHTTPRequest3Callback {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXSLProcessor, IXSLProcessor_Vtbl, 0x2933bf92_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXSLProcessor {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXSLProcessor, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXSLProcessor {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setinput(&self, var: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setinput)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn input(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).input)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ownerTemplate(&self) -> windows_core::Result<IXSLTemplate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ownerTemplate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn setStartMode(&self, mode: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setStartMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mode), core::mem::transmute_copy(namespaceuri)) }
    }
    pub unsafe fn startMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).startMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn startModeURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).startModeURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setoutput(&self, output: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setoutput)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(output)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn output(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).output)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn transform(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).transform)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn addParameter(&self, basename: &windows_core::BSTR, parameter: &super::oaidl::VARIANT, namespaceuri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).addParameter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(basename), core::mem::transmute_copy(parameter), core::mem::transmute_copy(namespaceuri)) }
    }
    pub unsafe fn addObject<P0>(&self, obj: P0, namespaceuri: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).addObject)(windows_core::Interface::as_raw(self), obj.param().abi(), core::mem::transmute_copy(namespaceuri)) }
    }
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn stylesheet(&self) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).stylesheet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXSLProcessor_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setinput: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setinput: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub input: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    input: usize,
    pub ownerTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setStartMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub startModeURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setoutput: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setoutput: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub output: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    output: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub transform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    transform: usize,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub addParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    addParameter: usize,
    pub addObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_msxml")]
    pub stylesheet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    stylesheet: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXSLProcessor_Impl: super::oaidl::IDispatch_Impl {
    fn Setinput(&self, var: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn input(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn ownerTemplate(&self) -> windows_core::Result<IXSLTemplate>;
    fn setStartMode(&self, mode: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn startMode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn startModeURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setoutput(&self, output: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn output(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn transform(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn reset(&self) -> windows_core::Result<()>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn addParameter(&self, basename: &windows_core::BSTR, parameter: &super::oaidl::VARIANT, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addObject(&self, obj: windows_core::Ref<super::oaidl::IDispatch>, namespaceuri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn stylesheet(&self) -> windows_core::Result<super::msxml::IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXSLProcessor_Vtbl {
    pub const fn new<Identity: IXSLProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Setinput<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::Setinput(this, core::mem::transmute(&var)).into()
            }
        }
        unsafe extern "system" fn input<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::input(this) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ownerTemplate<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::ownerTemplate(this) {
                    Ok(ok__) => {
                        pptemplate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setStartMode<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::setStartMode(this, core::mem::transmute(&mode), core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn startMode<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::startMode(this) {
                    Ok(ok__) => {
                        mode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn startModeURI<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::startModeURI(this) {
                    Ok(ok__) => {
                        namespaceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setoutput<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::Setoutput(this, core::mem::transmute(&output)).into()
            }
        }
        unsafe extern "system" fn output<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::output(this) {
                    Ok(ok__) => {
                        poutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn transform<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdone: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::transform(this) {
                    Ok(ok__) => {
                        pdone.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn readyState<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadystate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::readyState(this) {
                    Ok(ok__) => {
                        preadystate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn addParameter<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: *mut core::ffi::c_void, parameter: super::oaidl::VARIANT, namespaceuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::addParameter(this, core::mem::transmute(&basename), core::mem::transmute(&parameter), core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn addObject<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLProcessor_Impl::addObject(this, core::mem::transmute_copy(&obj), core::mem::transmute(&namespaceuri)).into()
            }
        }
        unsafe extern "system" fn stylesheet<Identity: IXSLProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLProcessor_Impl::stylesheet(this) {
                    Ok(ok__) => {
                        stylesheet.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Setinput: Setinput::<Identity, OFFSET>,
            input: input::<Identity, OFFSET>,
            ownerTemplate: ownerTemplate::<Identity, OFFSET>,
            setStartMode: setStartMode::<Identity, OFFSET>,
            startMode: startMode::<Identity, OFFSET>,
            startModeURI: startModeURI::<Identity, OFFSET>,
            Setoutput: Setoutput::<Identity, OFFSET>,
            output: output::<Identity, OFFSET>,
            transform: transform::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
            addParameter: addParameter::<Identity, OFFSET>,
            addObject: addObject::<Identity, OFFSET>,
            stylesheet: stylesheet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXSLProcessor as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXSLProcessor {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXSLTemplate, IXSLTemplate_Vtbl, 0x2933bf93_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXSLTemplate {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXSLTemplate, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXSLTemplate {
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn putref_stylesheet<P0>(&self, stylesheet: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::msxml::IXMLDOMNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_stylesheet)(windows_core::Interface::as_raw(self), stylesheet.param().abi()) }
    }
    #[cfg(feature = "Win32_msxml")]
    pub unsafe fn stylesheet(&self) -> windows_core::Result<super::msxml::IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).stylesheet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createProcessor(&self) -> windows_core::Result<IXSLProcessor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createProcessor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXSLTemplate_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_msxml")]
    pub putref_stylesheet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    putref_stylesheet: usize,
    #[cfg(feature = "Win32_msxml")]
    pub stylesheet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_msxml"))]
    stylesheet: usize,
    pub createProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXSLTemplate_Impl: super::oaidl::IDispatch_Impl {
    fn putref_stylesheet(&self, stylesheet: windows_core::Ref<super::msxml::IXMLDOMNode>) -> windows_core::Result<()>;
    fn stylesheet(&self) -> windows_core::Result<super::msxml::IXMLDOMNode>;
    fn createProcessor(&self) -> windows_core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXSLTemplate_Vtbl {
    pub const fn new<Identity: IXSLTemplate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn putref_stylesheet<Identity: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXSLTemplate_Impl::putref_stylesheet(this, core::mem::transmute_copy(&stylesheet)).into()
            }
        }
        unsafe extern "system" fn stylesheet<Identity: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLTemplate_Impl::stylesheet(this) {
                    Ok(ok__) => {
                        stylesheet.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createProcessor<Identity: IXSLTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXSLTemplate_Impl::createProcessor(this) {
                    Ok(ok__) => {
                        ppprocessor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            putref_stylesheet: putref_stylesheet::<Identity, OFFSET>,
            stylesheet: stylesheet::<Identity, OFFSET>,
            createProcessor: createProcessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXSLTemplate as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXSLTemplate {}
pub const MXHTMLWriter60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a10_f192_11d4_a65f_0040963251e5);
pub const MXNamespaceManager60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a11_f192_11d4_a65f_0040963251e5);
pub const MXXMLWriter60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a0f_f192_11d4_a65f_0040963251e5);
pub const SAXAttributes60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a0e_f192_11d4_a65f_0040963251e5);
pub const SAXXMLReader60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a0c_f192_11d4_a65f_0040963251e5);
pub type SCHEMACONTENTTYPE = i32;
pub const SCHEMACONTENTTYPE_ELEMENTONLY: SCHEMACONTENTTYPE = 2;
pub const SCHEMACONTENTTYPE_EMPTY: SCHEMACONTENTTYPE = 0;
pub const SCHEMACONTENTTYPE_MIXED: SCHEMACONTENTTYPE = 3;
pub const SCHEMACONTENTTYPE_TEXTONLY: SCHEMACONTENTTYPE = 1;
pub type SCHEMADERIVATIONMETHOD = i32;
pub const SCHEMADERIVATIONMETHOD_ALL: SCHEMADERIVATIONMETHOD = 255;
pub const SCHEMADERIVATIONMETHOD_EMPTY: SCHEMADERIVATIONMETHOD = 0;
pub const SCHEMADERIVATIONMETHOD_EXTENSION: SCHEMADERIVATIONMETHOD = 2;
pub const SCHEMADERIVATIONMETHOD_LIST: SCHEMADERIVATIONMETHOD = 8;
pub const SCHEMADERIVATIONMETHOD_NONE: SCHEMADERIVATIONMETHOD = 256;
pub const SCHEMADERIVATIONMETHOD_RESTRICTION: SCHEMADERIVATIONMETHOD = 4;
pub const SCHEMADERIVATIONMETHOD_SUBSTITUTION: SCHEMADERIVATIONMETHOD = 1;
pub const SCHEMADERIVATIONMETHOD_UNION: SCHEMADERIVATIONMETHOD = 16;
pub type SCHEMAPROCESSCONTENTS = i32;
pub const SCHEMAPROCESSCONTENTS_LAX: SCHEMAPROCESSCONTENTS = 2;
pub const SCHEMAPROCESSCONTENTS_NONE: SCHEMAPROCESSCONTENTS = 0;
pub const SCHEMAPROCESSCONTENTS_SKIP: SCHEMAPROCESSCONTENTS = 1;
pub const SCHEMAPROCESSCONTENTS_STRICT: SCHEMAPROCESSCONTENTS = 3;
pub type SCHEMATYPEVARIETY = i32;
pub const SCHEMATYPEVARIETY_ATOMIC: SCHEMATYPEVARIETY = 0;
pub const SCHEMATYPEVARIETY_LIST: SCHEMATYPEVARIETY = 1;
pub const SCHEMATYPEVARIETY_NONE: SCHEMATYPEVARIETY = -1;
pub const SCHEMATYPEVARIETY_UNION: SCHEMATYPEVARIETY = 2;
pub type SCHEMAUSE = i32;
pub const SCHEMAUSE_OPTIONAL: SCHEMAUSE = 0;
pub const SCHEMAUSE_PROHIBITED: SCHEMAUSE = 1;
pub const SCHEMAUSE_REQUIRED: SCHEMAUSE = 2;
pub type SCHEMAWHITESPACE = i32;
pub const SCHEMAWHITESPACE_COLLAPSE: SCHEMAWHITESPACE = 2;
pub const SCHEMAWHITESPACE_NONE: SCHEMAWHITESPACE = -1;
pub const SCHEMAWHITESPACE_PRESERVE: SCHEMAWHITESPACE = 0;
pub const SCHEMAWHITESPACE_REPLACE: SCHEMAWHITESPACE = 1;
pub type SERVERXMLHTTP_OPTION = i32;
pub type SOMITEMTYPE = i32;
pub const SOMITEM_ALL: SOMITEMTYPE = 16641;
pub const SOMITEM_ANNOTATION: SOMITEMTYPE = 4100;
pub const SOMITEM_ANY: SOMITEMTYPE = 16385;
pub const SOMITEM_ANYATTRIBUTE: SOMITEMTYPE = 16386;
pub const SOMITEM_ANYTYPE: SOMITEMTYPE = 8192;
pub const SOMITEM_ATTRIBUTE: SOMITEMTYPE = 4097;
pub const SOMITEM_ATTRIBUTEGROUP: SOMITEMTYPE = 4098;
pub const SOMITEM_CHOICE: SOMITEMTYPE = 16642;
pub const SOMITEM_COMPLEXTYPE: SOMITEMTYPE = 9216;
pub const SOMITEM_DATATYPE: SOMITEMTYPE = 8448;
pub const SOMITEM_DATATYPE_ANYSIMPLETYPE: SOMITEMTYPE = 8703;
pub const SOMITEM_DATATYPE_ANYTYPE: SOMITEMTYPE = 8449;
pub const SOMITEM_DATATYPE_ANYURI: SOMITEMTYPE = 8450;
pub const SOMITEM_DATATYPE_BASE64BINARY: SOMITEMTYPE = 8451;
pub const SOMITEM_DATATYPE_BOOLEAN: SOMITEMTYPE = 8452;
pub const SOMITEM_DATATYPE_BYTE: SOMITEMTYPE = 8453;
pub const SOMITEM_DATATYPE_DATE: SOMITEMTYPE = 8454;
pub const SOMITEM_DATATYPE_DATETIME: SOMITEMTYPE = 8455;
pub const SOMITEM_DATATYPE_DAY: SOMITEMTYPE = 8456;
pub const SOMITEM_DATATYPE_DECIMAL: SOMITEMTYPE = 8457;
pub const SOMITEM_DATATYPE_DOUBLE: SOMITEMTYPE = 8458;
pub const SOMITEM_DATATYPE_DURATION: SOMITEMTYPE = 8459;
pub const SOMITEM_DATATYPE_ENTITIES: SOMITEMTYPE = 8460;
pub const SOMITEM_DATATYPE_ENTITY: SOMITEMTYPE = 8461;
pub const SOMITEM_DATATYPE_FLOAT: SOMITEMTYPE = 8462;
pub const SOMITEM_DATATYPE_HEXBINARY: SOMITEMTYPE = 8463;
pub const SOMITEM_DATATYPE_ID: SOMITEMTYPE = 8464;
pub const SOMITEM_DATATYPE_IDREF: SOMITEMTYPE = 8465;
pub const SOMITEM_DATATYPE_IDREFS: SOMITEMTYPE = 8466;
pub const SOMITEM_DATATYPE_INT: SOMITEMTYPE = 8467;
pub const SOMITEM_DATATYPE_INTEGER: SOMITEMTYPE = 8468;
pub const SOMITEM_DATATYPE_LANGUAGE: SOMITEMTYPE = 8469;
pub const SOMITEM_DATATYPE_LONG: SOMITEMTYPE = 8470;
pub const SOMITEM_DATATYPE_MONTH: SOMITEMTYPE = 8471;
pub const SOMITEM_DATATYPE_MONTHDAY: SOMITEMTYPE = 8472;
pub const SOMITEM_DATATYPE_NAME: SOMITEMTYPE = 8473;
pub const SOMITEM_DATATYPE_NCNAME: SOMITEMTYPE = 8474;
pub const SOMITEM_DATATYPE_NEGATIVEINTEGER: SOMITEMTYPE = 8475;
pub const SOMITEM_DATATYPE_NMTOKEN: SOMITEMTYPE = 8476;
pub const SOMITEM_DATATYPE_NMTOKENS: SOMITEMTYPE = 8477;
pub const SOMITEM_DATATYPE_NONNEGATIVEINTEGER: SOMITEMTYPE = 8478;
pub const SOMITEM_DATATYPE_NONPOSITIVEINTEGER: SOMITEMTYPE = 8479;
pub const SOMITEM_DATATYPE_NORMALIZEDSTRING: SOMITEMTYPE = 8480;
pub const SOMITEM_DATATYPE_NOTATION: SOMITEMTYPE = 8481;
pub const SOMITEM_DATATYPE_POSITIVEINTEGER: SOMITEMTYPE = 8482;
pub const SOMITEM_DATATYPE_QNAME: SOMITEMTYPE = 8483;
pub const SOMITEM_DATATYPE_SHORT: SOMITEMTYPE = 8484;
pub const SOMITEM_DATATYPE_STRING: SOMITEMTYPE = 8485;
pub const SOMITEM_DATATYPE_TIME: SOMITEMTYPE = 8486;
pub const SOMITEM_DATATYPE_TOKEN: SOMITEMTYPE = 8487;
pub const SOMITEM_DATATYPE_UNSIGNEDBYTE: SOMITEMTYPE = 8488;
pub const SOMITEM_DATATYPE_UNSIGNEDINT: SOMITEMTYPE = 8489;
pub const SOMITEM_DATATYPE_UNSIGNEDLONG: SOMITEMTYPE = 8490;
pub const SOMITEM_DATATYPE_UNSIGNEDSHORT: SOMITEMTYPE = 8491;
pub const SOMITEM_DATATYPE_YEAR: SOMITEMTYPE = 8492;
pub const SOMITEM_DATATYPE_YEARMONTH: SOMITEMTYPE = 8493;
pub const SOMITEM_ELEMENT: SOMITEMTYPE = 16387;
pub const SOMITEM_EMPTYPARTICLE: SOMITEMTYPE = 16644;
pub const SOMITEM_GROUP: SOMITEMTYPE = 16640;
pub const SOMITEM_IDENTITYCONSTRAINT: SOMITEMTYPE = 4352;
pub const SOMITEM_KEY: SOMITEMTYPE = 4353;
pub const SOMITEM_KEYREF: SOMITEMTYPE = 4354;
pub const SOMITEM_NOTATION: SOMITEMTYPE = 4099;
pub const SOMITEM_NULL: SOMITEMTYPE = 2048;
pub const SOMITEM_NULL_ANY: SOMITEMTYPE = 18433;
pub const SOMITEM_NULL_ANYATTRIBUTE: SOMITEMTYPE = 18434;
pub const SOMITEM_NULL_ELEMENT: SOMITEMTYPE = 18435;
pub const SOMITEM_NULL_TYPE: SOMITEMTYPE = 10240;
pub const SOMITEM_PARTICLE: SOMITEMTYPE = 16384;
pub const SOMITEM_SCHEMA: SOMITEMTYPE = 4096;
pub const SOMITEM_SEQUENCE: SOMITEMTYPE = 16643;
pub const SOMITEM_SIMPLETYPE: SOMITEMTYPE = 8704;
pub const SOMITEM_UNIQUE: SOMITEMTYPE = 4355;
pub const SXH_OPTION_ESCAPE_PERCENT_IN_URL: SERVERXMLHTTP_OPTION = 1;
pub const SXH_OPTION_IGNORE_SERVER_SSL_CERT_ERROR_FLAGS: SERVERXMLHTTP_OPTION = 2;
pub const SXH_OPTION_SELECT_CLIENT_SSL_CERT: SERVERXMLHTTP_OPTION = 3;
pub const SXH_OPTION_URL: SERVERXMLHTTP_OPTION = -1;
pub const SXH_OPTION_URL_CODEPAGE: SERVERXMLHTTP_OPTION = 0;
pub type SXH_PROXY_SETTING = i32;
pub const SXH_PROXY_SET_DEFAULT: SXH_PROXY_SETTING = 0;
pub const SXH_PROXY_SET_DIRECT: SXH_PROXY_SETTING = 1;
pub const SXH_PROXY_SET_PRECONFIG: SXH_PROXY_SETTING = 0;
pub const SXH_PROXY_SET_PROXY: SXH_PROXY_SETTING = 2;
pub const SXH_SERVER_CERT_IGNORE_ALL_SERVER_ERRORS: SXH_SERVER_CERT_OPTION = 13056;
pub const SXH_SERVER_CERT_IGNORE_CERT_CN_INVALID: SXH_SERVER_CERT_OPTION = 4096;
pub const SXH_SERVER_CERT_IGNORE_CERT_DATE_INVALID: SXH_SERVER_CERT_OPTION = 8192;
pub const SXH_SERVER_CERT_IGNORE_UNKNOWN_CA: SXH_SERVER_CERT_OPTION = 256;
pub const SXH_SERVER_CERT_IGNORE_WRONG_USAGE: SXH_SERVER_CERT_OPTION = 512;
pub type SXH_SERVER_CERT_OPTION = i32;
pub const ServerXMLHTTP60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a0b_f192_11d4_a65f_0040963251e5);
pub type XHR_AUTH = i32;
pub const XHR_AUTH_ALL: XHR_AUTH = 0;
pub const XHR_AUTH_NONE: XHR_AUTH = 1;
pub const XHR_AUTH_PROXY: XHR_AUTH = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XHR_CERT {
    pub cbCert: u32,
    pub pbCert: *mut u8,
}
impl Default for XHR_CERT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XHR_CERT_ERROR_ALL_SERVER_ERRORS: XHR_CERT_ERROR_FLAG = 125829120;
pub const XHR_CERT_ERROR_CERT_CN_INVALID: XHR_CERT_ERROR_FLAG = 33554432;
pub const XHR_CERT_ERROR_CERT_DATE_INVALID: XHR_CERT_ERROR_FLAG = 67108864;
pub type XHR_CERT_ERROR_FLAG = i32;
pub const XHR_CERT_ERROR_REVOCATION_FAILED: XHR_CERT_ERROR_FLAG = 8388608;
pub const XHR_CERT_ERROR_UNKNOWN_CA: XHR_CERT_ERROR_FLAG = 16777216;
pub const XHR_CERT_IGNORE_ALL_SERVER_ERRORS: XHR_CERT_IGNORE_FLAG = 12672;
pub const XHR_CERT_IGNORE_CERT_CN_INVALID: XHR_CERT_IGNORE_FLAG = 4096;
pub const XHR_CERT_IGNORE_CERT_DATE_INVALID: XHR_CERT_IGNORE_FLAG = 8192;
pub type XHR_CERT_IGNORE_FLAG = i32;
pub const XHR_CERT_IGNORE_REVOCATION_FAILED: XHR_CERT_IGNORE_FLAG = 128;
pub const XHR_CERT_IGNORE_UNKNOWN_CA: XHR_CERT_IGNORE_FLAG = 256;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XHR_COOKIE {
    pub pwszUrl: *mut u16,
    pub pwszName: *mut u16,
    pub pwszValue: *mut u16,
    pub pwszP3PPolicy: *mut u16,
    pub ftExpires: super::minwindef::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for XHR_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XHR_COOKIE_APPLY_P3P: XHR_COOKIE_FLAG = 128;
pub const XHR_COOKIE_EVALUATE_P3P: XHR_COOKIE_FLAG = 64;
pub type XHR_COOKIE_FLAG = i32;
pub const XHR_COOKIE_HTTPONLY: XHR_COOKIE_FLAG = 8192;
pub const XHR_COOKIE_IE6: XHR_COOKIE_FLAG = 1024;
pub const XHR_COOKIE_IS_LEGACY: XHR_COOKIE_FLAG = 2048;
pub const XHR_COOKIE_IS_RESTRICTED: XHR_COOKIE_FLAG = 512;
pub const XHR_COOKIE_IS_SECURE: XHR_COOKIE_FLAG = 1;
pub const XHR_COOKIE_IS_SESSION: XHR_COOKIE_FLAG = 2;
pub const XHR_COOKIE_NON_SCRIPT: XHR_COOKIE_FLAG = 4096;
pub const XHR_COOKIE_P3P_ENABLED: XHR_COOKIE_FLAG = 256;
pub const XHR_COOKIE_PROMPT_REQUIRED: XHR_COOKIE_FLAG = 32;
pub type XHR_COOKIE_STATE = i32;
pub const XHR_COOKIE_STATE_ACCEPT: XHR_COOKIE_STATE = 1;
pub const XHR_COOKIE_STATE_DOWNGRADE: XHR_COOKIE_STATE = 4;
pub const XHR_COOKIE_STATE_LEASH: XHR_COOKIE_STATE = 3;
pub const XHR_COOKIE_STATE_PROMPT: XHR_COOKIE_STATE = 2;
pub const XHR_COOKIE_STATE_REJECT: XHR_COOKIE_STATE = 5;
pub const XHR_COOKIE_STATE_UNKNOWN: XHR_COOKIE_STATE = 0;
pub const XHR_COOKIE_THIRD_PARTY: XHR_COOKIE_FLAG = 16;
pub type XHR_CRED_PROMPT = i32;
pub const XHR_CRED_PROMPT_ALL: XHR_CRED_PROMPT = 0;
pub const XHR_CRED_PROMPT_NONE: XHR_CRED_PROMPT = 1;
pub const XHR_CRED_PROMPT_PROXY: XHR_CRED_PROMPT = 2;
pub type XHR_PROPERTY = i32;
pub const XHR_PROP_EXTENDED_ERROR: XHR_PROPERTY = 6;
pub const XHR_PROP_IGNORE_CERT_ERRORS: XHR_PROPERTY = 8;
pub const XHR_PROP_MAX_CONNECTIONS: XHR_PROPERTY = 11;
pub const XHR_PROP_NO_AUTH: XHR_PROPERTY = 1;
pub const XHR_PROP_NO_CACHE: XHR_PROPERTY = 5;
pub const XHR_PROP_NO_CRED_PROMPT: XHR_PROPERTY = 0;
pub const XHR_PROP_NO_DEFAULT_HEADERS: XHR_PROPERTY = 3;
pub const XHR_PROP_ONDATA_ALWAYS: u32 = 0;
pub const XHR_PROP_ONDATA_NEVER: u64 = 18446744073709551615;
pub const XHR_PROP_ONDATA_THRESHOLD: XHR_PROPERTY = 9;
pub const XHR_PROP_QUERY_STRING_UTF8: XHR_PROPERTY = 7;
pub const XHR_PROP_REPORT_REDIRECT_STATUS: XHR_PROPERTY = 4;
pub const XHR_PROP_SET_ENTERPRISEID: XHR_PROPERTY = 10;
pub const XHR_PROP_TIMEOUT: XHR_PROPERTY = 2;
pub const XMLHTTP60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a0a_f192_11d4_a65f_0040963251e5);
pub const XMLSchemaCache60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a07_f192_11d4_a65f_0040963251e5);
pub const XSLTemplate60: windows_core::GUID = windows_core::GUID::from_u128(0x88d96a08_f192_11d4_a65f_0040963251e5);
#[repr(C)]
#[cfg(feature = "Win32_msxml")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct __msxml6_ReferenceRemainingTypes__ {
    pub __tagDomNodeType__: super::msxml::DOMNodeType,
    pub __domNodeType__: super::msxml::DOMNodeType,
    pub __serverXmlHttpOptionEnum__: SERVERXMLHTTP_OPTION,
    pub __serverXmlHttpOption__: SERVERXMLHTTP_OPTION,
    pub __serverCertOptionEnum__: SXH_SERVER_CERT_OPTION,
    pub __serverCertOption__: SXH_SERVER_CERT_OPTION,
    pub __proxySettingEnum__: SXH_PROXY_SETTING,
    pub __proxySetting__: SXH_PROXY_SETTING,
    pub __somItemTypeEnum__: SOMITEMTYPE,
    pub __somItemType__: SOMITEMTYPE,
    pub __schemaUseEnum__: SCHEMAUSE,
    pub __schemaUse__: SCHEMAUSE,
    pub __schemaDerivationMethodEnum__: SCHEMADERIVATIONMETHOD,
    pub __schemaDerivationMethod__: SCHEMADERIVATIONMETHOD,
    pub __schemaContentTypeEnum__: SCHEMACONTENTTYPE,
    pub __schemaContentType__: SCHEMACONTENTTYPE,
    pub __schemaProcessContentsEnum__: SCHEMAPROCESSCONTENTS,
    pub __schemaProcessContents__: SCHEMAPROCESSCONTENTS,
    pub __schemaWhitespaceEnum__: SCHEMAWHITESPACE,
    pub __schemaWhitespace__: SCHEMAWHITESPACE,
    pub __schemaTypeVarietyEnum__: SCHEMATYPEVARIETY,
    pub __schemaTypeVariety__: SCHEMATYPEVARIETY,
}
