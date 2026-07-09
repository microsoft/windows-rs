pub const ENUMEXT_OBJECTID: u32 = 1;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertServerExit, ICertServerExit_Vtbl, 0x4ba9eb90_732c_11d0_8816_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertServerExit {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertServerExit, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertServerExit {
    pub unsafe fn SetContext(&self, context: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), context) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), propertytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strattributename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), propertytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strextensionname), r#type, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateExtensionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateExtensionsSetup)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerateExtensionsClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateExtensionsClose)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateAttributesSetup)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerateAttributesClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateAttributesClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertServerExit_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetRequestProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetRequestProperty: usize,
    pub GetRequestAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCertificateExtension: usize,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertServerExit_Impl: super::oaidl::IDispatch_Impl {
    fn SetContext(&self, context: i32) -> windows_core::Result<()>;
    fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32>;
    fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateExtensionsClose(&self) -> windows_core::Result<()>;
    fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateAttributesClose(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertServerExit_Vtbl {
    pub const fn new<Identity: ICertServerExit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContext<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerExit_Impl::SetContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn GetRequestProperty<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertytype: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::GetRequestProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strattributename: *mut core::ffi::c_void, pstrattributevalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::GetRequestAttribute(this, core::mem::transmute(&strattributename)) {
                    Ok(ok__) => {
                        pstrattributevalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertytype: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::GetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateExtension<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: *mut core::ffi::c_void, r#type: i32, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::GetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::GetCertificateExtensionFlags(this) {
                    Ok(ok__) => {
                        pextflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerExit_Impl::EnumerateExtensionsSetup(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumerateExtensions<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrextensionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::EnumerateExtensions(this) {
                    Ok(ok__) => {
                        pstrextensionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerExit_Impl::EnumerateExtensionsClose(this).into()
            }
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerExit_Impl::EnumerateAttributesSetup(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumerateAttributes<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrattributename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerExit_Impl::EnumerateAttributes(this) {
                    Ok(ok__) => {
                        pstrattributename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerExit_Impl::EnumerateAttributesClose(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetRequestProperty: GetRequestProperty::<Identity, OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Identity, OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Identity, OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Identity, OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Identity, OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Identity, OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Identity, OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Identity, OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Identity, OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Identity, OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertServerExit as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertServerExit {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertServerPolicy, ICertServerPolicy_Vtbl, 0xaa000922_ffbe_11cf_8800_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertServerPolicy {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertServerPolicy, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertServerPolicy {
    pub unsafe fn SetContext(&self, context: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), context) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), propertytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequestAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strattributename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), propertytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), propertytype, core::mem::transmute(pvarpropertyvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strextensionname), r#type, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificateExtensionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32, extflags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCertificateExtension)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strextensionname), r#type, extflags, core::mem::transmute(pvarvalue)) }
    }
    pub unsafe fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateExtensionsSetup)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerateExtensionsClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateExtensionsClose)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateAttributesSetup)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EnumerateAttributesClose(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumerateAttributesClose)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertServerPolicy_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetRequestProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetRequestProperty: usize,
    pub GetRequestAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetCertificateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetCertificateProperty: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetCertificateExtension: usize,
    pub GetCertificateExtensionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetCertificateExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetCertificateExtension: usize,
    pub EnumerateExtensionsSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateExtensionsClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesSetup: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumerateAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateAttributesClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertServerPolicy_Impl: super::oaidl::IDispatch_Impl {
    fn SetContext(&self, context: i32) -> windows_core::Result<()>;
    fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32>;
    fn SetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32, extflags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateExtensionsClose(&self) -> windows_core::Result<()>;
    fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateAttributesClose(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertServerPolicy_Vtbl {
    pub const fn new<Identity: ICertServerPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContext<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::SetContext(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn GetRequestProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertytype: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::GetRequestProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strattributename: *mut core::ffi::c_void, pstrattributevalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::GetRequestAttribute(this, core::mem::transmute(&strattributename)) {
                    Ok(ok__) => {
                        pstrattributevalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertytype: i32, pvarpropertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::GetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        pvarpropertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCertificateProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertytype: i32, pvarpropertyvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::SetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype), core::mem::transmute_copy(&pvarpropertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetCertificateExtension<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: *mut core::ffi::c_void, r#type: i32, pvarvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::GetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::GetCertificateExtensionFlags(this) {
                    Ok(ok__) => {
                        pextflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCertificateExtension<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: *mut core::ffi::c_void, r#type: i32, extflags: i32, pvarvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::SetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&extflags), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::EnumerateExtensionsSetup(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumerateExtensions<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrextensionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::EnumerateExtensions(this) {
                    Ok(ok__) => {
                        pstrextensionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::EnumerateExtensionsClose(this).into()
            }
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::EnumerateAttributesSetup(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumerateAttributes<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrattributename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertServerPolicy_Impl::EnumerateAttributes(this) {
                    Ok(ok__) => {
                        pstrattributename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertServerPolicy_Impl::EnumerateAttributesClose(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetRequestProperty: GetRequestProperty::<Identity, OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Identity, OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Identity, OFFSET>,
            SetCertificateProperty: SetCertificateProperty::<Identity, OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Identity, OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Identity, OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Identity, OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Identity, OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Identity, OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Identity, OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Identity, OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Identity, OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertServerPolicy as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertServerPolicy {}
