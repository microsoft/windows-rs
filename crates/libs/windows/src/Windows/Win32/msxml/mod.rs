pub const DOMDocument: windows_core::GUID = windows_core::GUID::from_u128(0x2933bf90_7b36_11d2_b20e_00c04f983e60);
pub const DOMFreeThreadedDocument: windows_core::GUID = windows_core::GUID::from_u128(0x2933bf91_7b36_11d2_b20e_00c04f983e60);
pub type DOMNodeType = i32;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLAttribute, IXMLAttribute_Vtbl, 0xd4d4a0fc_3b73_11d1_b2b4_00c04fb92596);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLAttribute {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLAttribute, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLAttribute {
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLAttribute_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLAttribute_Impl: super::oaidl::IDispatch_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn value(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLAttribute_Vtbl {
    pub const fn new<Identity: IXMLAttribute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn name<Identity: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, n: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLAttribute_Impl::name(this) {
                    Ok(ok__) => {
                        n.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn value<Identity: IXMLAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLAttribute_Impl::value(this) {
                    Ok(ok__) => {
                        v.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), name: name::<Identity, OFFSET>, value: value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLAttribute as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLAttribute {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMAttribute, IXMLDOMAttribute_Vtbl, 0x2933bf85_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMAttribute {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMAttribute, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMAttribute {
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn value(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setvalue(&self, attributevalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setvalue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(attributevalue)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMAttribute_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    value: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setvalue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setvalue: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMAttribute_Impl: IXMLDOMNode_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn value(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn Setvalue(&self, attributevalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMAttribute_Vtbl {
    pub const fn new<Identity: IXMLDOMAttribute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn name<Identity: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMAttribute_Impl::name(this) {
                    Ok(ok__) => {
                        attributename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn value<Identity: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributevalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMAttribute_Impl::value(this) {
                    Ok(ok__) => {
                        attributevalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setvalue<Identity: IXMLDOMAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributevalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMAttribute_Impl::Setvalue(this, core::mem::transmute(&attributevalue)).into()
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, OFFSET>,
            value: value::<Identity, OFFSET>,
            Setvalue: Setvalue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMAttribute as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMAttribute {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMCDATASection, IXMLDOMCDATASection_Vtbl, 0x2933bf8a_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMCDATASection {
    type Target = IXMLDOMText;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMCDATASection, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode, IXMLDOMCharacterData, IXMLDOMText);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMCDATASection_Vtbl {
    pub base__: IXMLDOMText_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMCDATASection_Impl: IXMLDOMText_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMCDATASection_Vtbl {
    pub const fn new<Identity: IXMLDOMCDATASection_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IXMLDOMText_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMCDATASection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID || iid == &<IXMLDOMText as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMCDATASection {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMCharacterData, IXMLDOMCharacterData_Vtbl, 0x2933bf84_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMCharacterData {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMCharacterData, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMCharacterData {
    pub unsafe fn data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setdata(&self, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setdata)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)) }
    }
    pub unsafe fn length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn substringData(&self, offset: i32, count: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).substringData)(windows_core::Interface::as_raw(self), offset, count, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn appendData(&self, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).appendData)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)) }
    }
    pub unsafe fn insertData(&self, offset: i32, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).insertData)(windows_core::Interface::as_raw(self), offset, core::mem::transmute_copy(data)) }
    }
    pub unsafe fn deleteData(&self, offset: i32, count: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).deleteData)(windows_core::Interface::as_raw(self), offset, count) }
    }
    pub unsafe fn replaceData(&self, offset: i32, count: i32, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).replaceData)(windows_core::Interface::as_raw(self), offset, count, core::mem::transmute_copy(data)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMCharacterData_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setdata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub substringData: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub appendData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub insertData: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub deleteData: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub replaceData: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMCharacterData_Impl: IXMLDOMNode_Impl {
    fn data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setdata(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn substringData(&self, offset: i32, count: i32) -> windows_core::Result<windows_core::BSTR>;
    fn appendData(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn insertData(&self, offset: i32, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn deleteData(&self, offset: i32, count: i32) -> windows_core::Result<()>;
    fn replaceData(&self, offset: i32, count: i32, data: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMCharacterData_Vtbl {
    pub const fn new<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn data<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMCharacterData_Impl::data(this) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setdata<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMCharacterData_Impl::Setdata(this, core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn length<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datalength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMCharacterData_Impl::length(this) {
                    Ok(ok__) => {
                        datalength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn substringData<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMCharacterData_Impl::substringData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn appendData<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMCharacterData_Impl::appendData(this, core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn insertData<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMCharacterData_Impl::insertData(this, core::mem::transmute_copy(&offset), core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn deleteData<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMCharacterData_Impl::deleteData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn replaceData<Identity: IXMLDOMCharacterData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, count: i32, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMCharacterData_Impl::replaceData(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count), core::mem::transmute(&data)).into()
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            data: data::<Identity, OFFSET>,
            Setdata: Setdata::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            substringData: substringData::<Identity, OFFSET>,
            appendData: appendData::<Identity, OFFSET>,
            insertData: insertData::<Identity, OFFSET>,
            deleteData: deleteData::<Identity, OFFSET>,
            replaceData: replaceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMCharacterData {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMComment, IXMLDOMComment_Vtbl, 0x2933bf88_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMComment {
    type Target = IXMLDOMCharacterData;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMComment, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode, IXMLDOMCharacterData);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMComment_Vtbl {
    pub base__: IXMLDOMCharacterData_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMComment_Impl: IXMLDOMCharacterData_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMComment_Vtbl {
    pub const fn new<Identity: IXMLDOMComment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMComment as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMComment {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMDocument, IXMLDOMDocument_Vtbl, 0x2933bf81_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMDocument {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMDocument, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMDocument {
    pub unsafe fn doctype(&self) -> windows_core::Result<IXMLDOMDocumentType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).doctype)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn implementation(&self) -> windows_core::Result<IXMLDOMImplementation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).implementation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn documentElement(&self) -> windows_core::Result<IXMLDOMElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).documentElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn putref_documentElement<P0>(&self, domelement: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLDOMElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).putref_documentElement)(windows_core::Interface::as_raw(self), domelement.param().abi()) }
    }
    pub unsafe fn createElement(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createElement)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createDocumentFragment(&self) -> windows_core::Result<IXMLDOMDocumentFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createDocumentFragment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createTextNode(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMText> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createTextNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createComment(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMComment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createComment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createCDATASection(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMCDATASection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createCDATASection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createProcessingInstruction(&self, target: &windows_core::BSTR, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMProcessingInstruction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createProcessingInstruction)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(target), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn createEntityReference(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMEntityReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createEntityReference)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getElementsByTagName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn createNode(&self, r#type: &super::oaidl::VARIANT, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type), core::mem::transmute_copy(name), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn nodeFromID(&self, idstring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeFromID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(idstring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn load(&self, xmlsource: &super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).load)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xmlsource), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn parseError(&self) -> windows_core::Result<IXMLDOMParseError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parseError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn url(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).url)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn r#async(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#async)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Setasync(&self, isasync: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setasync)(windows_core::Interface::as_raw(self), isasync) }
    }
    pub unsafe fn abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).abort)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn loadXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).loadXML)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn save(&self, destination: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).save)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(destination)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn validateOnParse(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).validateOnParse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetvalidateOnParse(&self, isvalidating: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetvalidateOnParse)(windows_core::Interface::as_raw(self), isvalidating) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn resolveExternals(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).resolveExternals)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetresolveExternals(&self, isresolving: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetresolveExternals)(windows_core::Interface::as_raw(self), isresolving) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn preserveWhiteSpace(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).preserveWhiteSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetpreserveWhiteSpace(&self, ispreserving: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetpreserveWhiteSpace)(windows_core::Interface::as_raw(self), ispreserving) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setonreadystatechange(&self, readystatechangesink: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setonreadystatechange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(readystatechangesink)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setondataavailable(&self, ondataavailablesink: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setondataavailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(ondataavailablesink)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Setontransformnode(&self, ontransformnodesink: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setontransformnode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(ontransformnodesink)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocument_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub doctype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub implementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub documentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub putref_documentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createDocumentFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createTextNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createComment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createCDATASection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createProcessingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub createEntityReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getElementsByTagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub createNode: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    createNode: usize,
    pub nodeFromID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub load: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    load: usize,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub parseError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub r#async: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    r#async: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Setasync: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Setasync: usize,
    pub abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub loadXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    loadXML: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub save: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    save: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub validateOnParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    validateOnParse: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetvalidateOnParse: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetvalidateOnParse: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub resolveExternals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    resolveExternals: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetresolveExternals: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetresolveExternals: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub preserveWhiteSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    preserveWhiteSpace: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetpreserveWhiteSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetpreserveWhiteSpace: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setonreadystatechange: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setonreadystatechange: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setondataavailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setondataavailable: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Setontransformnode: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Setontransformnode: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMDocument_Impl: IXMLDOMNode_Impl {
    fn doctype(&self) -> windows_core::Result<IXMLDOMDocumentType>;
    fn implementation(&self) -> windows_core::Result<IXMLDOMImplementation>;
    fn documentElement(&self) -> windows_core::Result<IXMLDOMElement>;
    fn putref_documentElement(&self, domelement: windows_core::Ref<IXMLDOMElement>) -> windows_core::Result<()>;
    fn createElement(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMElement>;
    fn createDocumentFragment(&self) -> windows_core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMText>;
    fn createComment(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMComment>;
    fn createCDATASection(&self, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(&self, target: &windows_core::BSTR, data: &windows_core::BSTR) -> windows_core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute>;
    fn createEntityReference(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn createNode(&self, r#type: &super::oaidl::VARIANT, name: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn nodeFromID(&self, idstring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn load(&self, xmlsource: &super::oaidl::VARIANT) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn parseError(&self) -> windows_core::Result<IXMLDOMParseError>;
    fn url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn r#async(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Setasync(&self, isasync: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn abort(&self) -> windows_core::Result<()>;
    fn loadXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn save(&self, destination: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn validateOnParse(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetvalidateOnParse(&self, isvalidating: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn resolveExternals(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetresolveExternals(&self, isresolving: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn preserveWhiteSpace(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetpreserveWhiteSpace(&self, ispreserving: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setonreadystatechange(&self, readystatechangesink: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Setondataavailable(&self, ondataavailablesink: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Setontransformnode(&self, ontransformnodesink: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMDocument_Vtbl {
    pub const fn new<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn doctype<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documenttype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::doctype(this) {
                    Ok(ok__) => {
                        documenttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn implementation<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#impl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::implementation(this) {
                    Ok(ok__) => {
                        r#impl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn documentElement<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::documentElement(this) {
                    Ok(ok__) => {
                        domelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn putref_documentElement<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::putref_documentElement(this, core::mem::transmute_copy(&domelement)).into()
            }
        }
        unsafe extern "system" fn createElement<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createElement(this, core::mem::transmute(&tagname)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createDocumentFragment<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, docfrag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createDocumentFragment(this) {
                    Ok(ok__) => {
                        docfrag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createTextNode<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createTextNode(this, core::mem::transmute(&data)) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createComment<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, comment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createComment(this, core::mem::transmute(&data)) {
                    Ok(ok__) => {
                        comment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createCDATASection<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, cdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createCDATASection(this, core::mem::transmute(&data)) {
                    Ok(ok__) => {
                        cdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, data: *mut core::ffi::c_void, pi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createProcessingInstruction(this, core::mem::transmute(&target), core::mem::transmute(&data)) {
                    Ok(ok__) => {
                        pi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createAttribute<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, attribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        attribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createEntityReference<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, entityref: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createEntityReference(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        entityref.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: *mut core::ffi::c_void, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::getElementsByTagName(this, core::mem::transmute(&tagname)) {
                    Ok(ok__) => {
                        resultlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createNode<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::oaidl::VARIANT, name: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::createNode(this, core::mem::transmute(&r#type), core::mem::transmute(&name), core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        node.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nodeFromID<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idstring: *mut core::ffi::c_void, node: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::nodeFromID(this, core::mem::transmute(&idstring)) {
                    Ok(ok__) => {
                        node.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn load<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlsource: super::oaidl::VARIANT, issuccessful: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::load(this, core::mem::transmute(&xmlsource)) {
                    Ok(ok__) => {
                        issuccessful.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::readyState(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn parseError<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::parseError(this) {
                    Ok(ok__) => {
                        errorobj.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn url<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urlstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::url(this) {
                    Ok(ok__) => {
                        urlstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#async<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isasync: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::r#async(this) {
                    Ok(ok__) => {
                        isasync.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setasync<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isasync: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::Setasync(this, core::mem::transmute_copy(&isasync)).into()
            }
        }
        unsafe extern "system" fn abort<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::abort(this).into()
            }
        }
        unsafe extern "system" fn loadXML<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void, issuccessful: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::loadXML(this, core::mem::transmute(&bstrxml)) {
                    Ok(ok__) => {
                        issuccessful.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn save<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::save(this, core::mem::transmute(&destination)).into()
            }
        }
        unsafe extern "system" fn validateOnParse<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalidating: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::validateOnParse(this) {
                    Ok(ok__) => {
                        isvalidating.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isvalidating: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::SetvalidateOnParse(this, core::mem::transmute_copy(&isvalidating)).into()
            }
        }
        unsafe extern "system" fn resolveExternals<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isresolving: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::resolveExternals(this) {
                    Ok(ok__) => {
                        isresolving.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetresolveExternals<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isresolving: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::SetresolveExternals(this, core::mem::transmute_copy(&isresolving)).into()
            }
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispreserving: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocument_Impl::preserveWhiteSpace(this) {
                    Ok(ok__) => {
                        ispreserving.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ispreserving: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::SetpreserveWhiteSpace(this, core::mem::transmute_copy(&ispreserving)).into()
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, readystatechangesink: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::Setonreadystatechange(this, core::mem::transmute(&readystatechangesink)).into()
            }
        }
        unsafe extern "system" fn Setondataavailable<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ondataavailablesink: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::Setondataavailable(this, core::mem::transmute(&ondataavailablesink)).into()
            }
        }
        unsafe extern "system" fn Setontransformnode<Identity: IXMLDOMDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ontransformnodesink: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMDocument_Impl::Setontransformnode(this, core::mem::transmute(&ontransformnodesink)).into()
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            doctype: doctype::<Identity, OFFSET>,
            implementation: implementation::<Identity, OFFSET>,
            documentElement: documentElement::<Identity, OFFSET>,
            putref_documentElement: putref_documentElement::<Identity, OFFSET>,
            createElement: createElement::<Identity, OFFSET>,
            createDocumentFragment: createDocumentFragment::<Identity, OFFSET>,
            createTextNode: createTextNode::<Identity, OFFSET>,
            createComment: createComment::<Identity, OFFSET>,
            createCDATASection: createCDATASection::<Identity, OFFSET>,
            createProcessingInstruction: createProcessingInstruction::<Identity, OFFSET>,
            createAttribute: createAttribute::<Identity, OFFSET>,
            createEntityReference: createEntityReference::<Identity, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, OFFSET>,
            createNode: createNode::<Identity, OFFSET>,
            nodeFromID: nodeFromID::<Identity, OFFSET>,
            load: load::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
            parseError: parseError::<Identity, OFFSET>,
            url: url::<Identity, OFFSET>,
            r#async: r#async::<Identity, OFFSET>,
            Setasync: Setasync::<Identity, OFFSET>,
            abort: abort::<Identity, OFFSET>,
            loadXML: loadXML::<Identity, OFFSET>,
            save: save::<Identity, OFFSET>,
            validateOnParse: validateOnParse::<Identity, OFFSET>,
            SetvalidateOnParse: SetvalidateOnParse::<Identity, OFFSET>,
            resolveExternals: resolveExternals::<Identity, OFFSET>,
            SetresolveExternals: SetresolveExternals::<Identity, OFFSET>,
            preserveWhiteSpace: preserveWhiteSpace::<Identity, OFFSET>,
            SetpreserveWhiteSpace: SetpreserveWhiteSpace::<Identity, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, OFFSET>,
            Setondataavailable: Setondataavailable::<Identity, OFFSET>,
            Setontransformnode: Setontransformnode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocument as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMDocument {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMDocumentFragment, IXMLDOMDocumentFragment_Vtbl, 0x3efaa413_272f_11d2_836f_0000f87a7782);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMDocumentFragment {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMDocumentFragment, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocumentFragment_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMDocumentFragment_Impl: IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMDocumentFragment_Vtbl {
    pub const fn new<Identity: IXMLDOMDocumentFragment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentFragment as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMDocumentFragment {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMDocumentType, IXMLDOMDocumentType_Vtbl, 0x2933bf8b_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMDocumentType {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMDocumentType, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMDocumentType {
    pub unsafe fn name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn entities(&self) -> windows_core::Result<IXMLDOMNamedNodeMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).entities)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn notations(&self) -> windows_core::Result<IXMLDOMNamedNodeMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMDocumentType_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub entities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub notations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMDocumentType_Impl: IXMLDOMNode_Impl {
    fn name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn entities(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
    fn notations(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMDocumentType_Vtbl {
    pub const fn new<Identity: IXMLDOMDocumentType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn name<Identity: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocumentType_Impl::name(this) {
                    Ok(ok__) => {
                        rootname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn entities<Identity: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entitymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocumentType_Impl::entities(this) {
                    Ok(ok__) => {
                        entitymap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn notations<Identity: IXMLDOMDocumentType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notationmap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMDocumentType_Impl::notations(this) {
                    Ok(ok__) => {
                        notationmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, OFFSET>,
            entities: entities::<Identity, OFFSET>,
            notations: notations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMDocumentType as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMDocumentType {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMElement, IXMLDOMElement_Vtbl, 0x2933bf86_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMElement {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMElement, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMElement {
    pub unsafe fn tagName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).tagName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setAttribute(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn getAttributeNode(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttributeNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn setAttributeNode<P0>(&self, domattribute: P0) -> windows_core::Result<IXMLDOMAttribute>
    where
        P0: windows_core::Param<IXMLDOMAttribute>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).setAttributeNode)(windows_core::Interface::as_raw(self), domattribute.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeAttributeNode<P0>(&self, domattribute: P0) -> windows_core::Result<IXMLDOMAttribute>
    where
        P0: windows_core::Param<IXMLDOMAttribute>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).removeAttributeNode)(windows_core::Interface::as_raw(self), domattribute.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getElementsByTagName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn normalize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).normalize)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMElement_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub tagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getElementsByTagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub normalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMElement_Impl: IXMLDOMNode_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getAttributeNode(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(&self, domattribute: windows_core::Ref<IXMLDOMAttribute>) -> windows_core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(&self, domattribute: windows_core::Ref<IXMLDOMAttribute>) -> windows_core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(&self, tagname: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn normalize(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMElement_Vtbl {
    pub const fn new<Identity: IXMLDOMElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn tagName<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::tagName(this) {
                    Ok(ok__) => {
                        tagname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAttribute<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::getAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMElement_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMElement_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn getAttributeNode<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::getAttributeNode(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        attributenode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttributeNode<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domattribute: *mut core::ffi::c_void, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::setAttributeNode(this, core::mem::transmute_copy(&domattribute)) {
                    Ok(ok__) => {
                        attributenode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeAttributeNode<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domattribute: *mut core::ffi::c_void, attributenode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::removeAttributeNode(this, core::mem::transmute_copy(&domattribute)) {
                    Ok(ok__) => {
                        attributenode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getElementsByTagName<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: *mut core::ffi::c_void, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMElement_Impl::getElementsByTagName(this, core::mem::transmute(&tagname)) {
                    Ok(ok__) => {
                        resultlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn normalize<Identity: IXMLDOMElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMElement_Impl::normalize(this).into()
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            tagName: tagName::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            getAttributeNode: getAttributeNode::<Identity, OFFSET>,
            setAttributeNode: setAttributeNode::<Identity, OFFSET>,
            removeAttributeNode: removeAttributeNode::<Identity, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, OFFSET>,
            normalize: normalize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMElement as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMElement {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMEntity, IXMLDOMEntity_Vtbl, 0x2933bf8d_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMEntity {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMEntity, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMEntity {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn publicId(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).publicId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn systemId(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn notationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).notationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMEntity_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub publicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    publicId: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub systemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    systemId: usize,
    pub notationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMEntity_Impl: IXMLDOMNode_Impl {
    fn publicId(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn systemId(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn notationName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMEntity_Vtbl {
    pub const fn new<Identity: IXMLDOMEntity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn publicId<Identity: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publicid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMEntity_Impl::publicId(this) {
                    Ok(ok__) => {
                        publicid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn systemId<Identity: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMEntity_Impl::systemId(this) {
                    Ok(ok__) => {
                        systemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn notationName<Identity: IXMLDOMEntity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMEntity_Impl::notationName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            publicId: publicId::<Identity, OFFSET>,
            systemId: systemId::<Identity, OFFSET>,
            notationName: notationName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntity as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMEntity {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMEntityReference, IXMLDOMEntityReference_Vtbl, 0x2933bf8e_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMEntityReference {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMEntityReference, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMEntityReference_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMEntityReference_Impl: IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMEntityReference_Vtbl {
    pub const fn new<Identity: IXMLDOMEntityReference_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMEntityReference as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMEntityReference {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMImplementation, IXMLDOMImplementation_Vtbl, 0x2933bf8f_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMImplementation {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMImplementation, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMImplementation {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn hasFeature(&self, feature: &windows_core::BSTR, version: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasFeature)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(feature), core::mem::transmute_copy(version), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMImplementation_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub hasFeature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    hasFeature: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMImplementation_Impl: super::oaidl::IDispatch_Impl {
    fn hasFeature(&self, feature: &windows_core::BSTR, version: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMImplementation_Vtbl {
    pub const fn new<Identity: IXMLDOMImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn hasFeature<Identity: IXMLDOMImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: *mut core::ffi::c_void, version: *mut core::ffi::c_void, hasfeature: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMImplementation_Impl::hasFeature(this, core::mem::transmute(&feature), core::mem::transmute(&version)) {
                    Ok(ok__) => {
                        hasfeature.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), hasFeature: hasFeature::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMImplementation as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMImplementation {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMNamedNodeMap, IXMLDOMNamedNodeMap_Vtbl, 0x2933bf83_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMNamedNodeMap {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMNamedNodeMap, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMNamedNodeMap {
    pub unsafe fn getNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getNamedItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn setNamedItem<P0>(&self, newitem: P0) -> windows_core::Result<IXMLDOMNode>
    where
        P0: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).setNamedItem)(windows_core::Interface::as_raw(self), newitem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).removeNamedItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<IXMLDOMNode> {
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
    pub unsafe fn getQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getQualifiedItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(basename), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).removeQualifiedItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(basename), core::mem::transmute_copy(namespaceuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn nextNode(&self) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nextNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct IXMLDOMNamedNodeMap_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub getNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getQualifiedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeQualifiedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub nextNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMNamedNodeMap_Impl: super::oaidl::IDispatch_Impl {
    fn getNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn setNamedItem(&self, newitem: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn removeNamedItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn item(&self, index: i32) -> windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> windows_core::Result<i32>;
    fn getQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(&self, basename: &windows_core::BSTR, namespaceuri: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn nextNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMNamedNodeMap_Vtbl {
    pub const fn new<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getNamedItem<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, nameditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::getNamedItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        nameditem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setNamedItem<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newitem: *mut core::ffi::c_void, nameitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::setNamedItem(this, core::mem::transmute_copy(&newitem)) {
                    Ok(ok__) => {
                        nameitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeNamedItem<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, nameditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::removeNamedItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        nameditem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, listitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        listitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::length(this) {
                    Ok(ok__) => {
                        listlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getQualifiedItem<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, qualifieditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::getQualifiedItem(this, core::mem::transmute(&basename), core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        qualifieditem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basename: *mut core::ffi::c_void, namespaceuri: *mut core::ffi::c_void, qualifieditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::removeQualifiedItem(this, core::mem::transmute(&basename), core::mem::transmute(&namespaceuri)) {
                    Ok(ok__) => {
                        qualifieditem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nextNode<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::nextNode(this) {
                    Ok(ok__) => {
                        nextitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNamedNodeMap_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IXMLDOMNamedNodeMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNamedNodeMap_Impl::_newEnum(this) {
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
            getNamedItem: getNamedItem::<Identity, OFFSET>,
            setNamedItem: setNamedItem::<Identity, OFFSET>,
            removeNamedItem: removeNamedItem::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            getQualifiedItem: getQualifiedItem::<Identity, OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Identity, OFFSET>,
            nextNode: nextNode::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNamedNodeMap as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMNamedNodeMap {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMNode, IXMLDOMNode_Vtbl, 0x2933bf80_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMNode {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMNode, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMNode {
    pub unsafe fn nodeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn nodeValue(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetnodeValue(&self, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetnodeValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn nodeType(&self) -> windows_core::Result<DOMNodeType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn parentNode(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parentNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn childNodes(&self) -> windows_core::Result<IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).childNodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn firstChild(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).firstChild)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn lastChild(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).lastChild)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn previousSibling(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).previousSibling)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn nextSibling(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nextSibling)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn attributes(&self) -> windows_core::Result<IXMLDOMNamedNodeMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn insertBefore<P0>(&self, newchild: P0, refchild: &super::oaidl::VARIANT) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).insertBefore)(windows_core::Interface::as_raw(self), newchild.param().abi(), core::mem::transmute_copy(refchild), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn replaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).replaceChild)(windows_core::Interface::as_raw(self), newchild.param().abi(), oldchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn removeChild<P0>(&self, childnode: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).removeChild)(windows_core::Interface::as_raw(self), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn appendChild<P0>(&self, newchild: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).appendChild)(windows_core::Interface::as_raw(self), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn hasChildNodes(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).hasChildNodes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ownerDocument(&self) -> windows_core::Result<IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ownerDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn cloneNode(&self, deep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).cloneNode)(windows_core::Interface::as_raw(self), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn nodeTypeString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeTypeString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn text(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).text)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Settext(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Settext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn specified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).specified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn definition(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).definition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn nodeTypedValue(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nodeTypedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetnodeTypedValue(&self, typedvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetnodeTypedValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(typedvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn dataType(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetdataType(&self, datatypename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetdataType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(datatypename)) }
    }
    pub unsafe fn xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn transformNode<P0>(&self, stylesheet: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).transformNode)(windows_core::Interface::as_raw(self), stylesheet.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn selectNodes(&self, querystring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).selectNodes)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(querystring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn selectSingleNode(&self, querystring: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).selectSingleNode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(querystring), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn parsed(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).namespaceURI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn prefix(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).prefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn baseName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).baseName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn transformNodeToObject<P0>(&self, stylesheet: P0, outputobject: &super::oaidl::VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).transformNodeToObject)(windows_core::Interface::as_raw(self), stylesheet.param().abi(), core::mem::transmute_copy(outputobject)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNode_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub nodeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub nodeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    nodeValue: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetnodeValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetnodeValue: usize,
    pub nodeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DOMNodeType) -> windows_core::HRESULT,
    pub parentNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub childNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub firstChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub lastChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub previousSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub nextSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub insertBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    insertBefore: usize,
    pub replaceChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub appendChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub hasChildNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    hasChildNodes: usize,
    pub ownerDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub cloneNode: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    cloneNode: usize,
    pub nodeTypeString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub specified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    specified: usize,
    pub definition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub nodeTypedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    nodeTypedValue: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetnodeTypedValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetnodeTypedValue: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub dataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    dataType: usize,
    pub SetdataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub transformNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub selectNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub selectSingleNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub parsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    parsed: usize,
    pub namespaceURI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub prefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub baseName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub transformNodeToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    transformNodeToObject: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMNode_Impl: super::oaidl::IDispatch_Impl {
    fn nodeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn nodeValue(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetnodeValue(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn nodeType(&self) -> windows_core::Result<DOMNodeType>;
    fn parentNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn childNodes(&self) -> windows_core::Result<IXMLDOMNodeList>;
    fn firstChild(&self) -> windows_core::Result<IXMLDOMNode>;
    fn lastChild(&self) -> windows_core::Result<IXMLDOMNode>;
    fn previousSibling(&self) -> windows_core::Result<IXMLDOMNode>;
    fn nextSibling(&self) -> windows_core::Result<IXMLDOMNode>;
    fn attributes(&self) -> windows_core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(&self, newchild: windows_core::Ref<IXMLDOMNode>, refchild: &super::oaidl::VARIANT) -> windows_core::Result<IXMLDOMNode>;
    fn replaceChild(&self, newchild: windows_core::Ref<IXMLDOMNode>, oldchild: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn removeChild(&self, childnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn appendChild(&self, newchild: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<IXMLDOMNode>;
    fn hasChildNodes(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn ownerDocument(&self) -> windows_core::Result<IXMLDOMDocument>;
    fn cloneNode(&self, deep: super::wtypes::VARIANT_BOOL) -> windows_core::Result<IXMLDOMNode>;
    fn nodeTypeString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn specified(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn definition(&self) -> windows_core::Result<IXMLDOMNode>;
    fn nodeTypedValue(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetnodeTypedValue(&self, typedvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn dataType(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetdataType(&self, datatypename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn transformNode(&self, stylesheet: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<windows_core::BSTR>;
    fn selectNodes(&self, querystring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(&self, querystring: &windows_core::BSTR) -> windows_core::Result<IXMLDOMNode>;
    fn parsed(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn namespaceURI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn prefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn baseName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn transformNodeToObject(&self, stylesheet: windows_core::Ref<IXMLDOMNode>, outputobject: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMNode_Vtbl {
    pub const fn new<Identity: IXMLDOMNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn nodeName<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nodeName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nodeValue<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nodeValue(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetnodeValue<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNode_Impl::SetnodeValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn nodeType<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DOMNodeType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nodeType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn parentNode<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::parentNode(this) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn childNodes<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::childNodes(this) {
                    Ok(ok__) => {
                        childlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn firstChild<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::firstChild(this) {
                    Ok(ok__) => {
                        firstchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn lastChild<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::lastChild(this) {
                    Ok(ok__) => {
                        lastchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn previousSibling<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previoussibling: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::previousSibling(this) {
                    Ok(ok__) => {
                        previoussibling.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nextSibling<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextsibling: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nextSibling(this) {
                    Ok(ok__) => {
                        nextsibling.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn attributes<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributemap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::attributes(this) {
                    Ok(ok__) => {
                        attributemap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn insertBefore<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, refchild: super::oaidl::VARIANT, outnewchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::insertBefore(this, core::mem::transmute_copy(&newchild), core::mem::transmute(&refchild)) {
                    Ok(ok__) => {
                        outnewchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn replaceChild<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void, outoldchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::replaceChild(this, core::mem::transmute_copy(&newchild), core::mem::transmute_copy(&oldchild)) {
                    Ok(ok__) => {
                        outoldchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeChild<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childnode: *mut core::ffi::c_void, oldchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::removeChild(this, core::mem::transmute_copy(&childnode)) {
                    Ok(ok__) => {
                        oldchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn appendChild<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, outnewchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::appendChild(this, core::mem::transmute_copy(&newchild)) {
                    Ok(ok__) => {
                        outnewchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn hasChildNodes<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, haschild: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::hasChildNodes(this) {
                    Ok(ok__) => {
                        haschild.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ownerDocument<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmldomdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::ownerDocument(this) {
                    Ok(ok__) => {
                        xmldomdocument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn cloneNode<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deep: super::wtypes::VARIANT_BOOL, cloneroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::cloneNode(this, core::mem::transmute_copy(&deep)) {
                    Ok(ok__) => {
                        cloneroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nodeTypeString<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nodeTypeString(this) {
                    Ok(ok__) => {
                        nodetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn text<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::text(this) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settext<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNode_Impl::Settext(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn specified<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isspecified: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::specified(this) {
                    Ok(ok__) => {
                        isspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn definition<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, definitionnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::definition(this) {
                    Ok(ok__) => {
                        definitionnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nodeTypedValue<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typedvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::nodeTypedValue(this) {
                    Ok(ok__) => {
                        typedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typedvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNode_Impl::SetnodeTypedValue(this, core::mem::transmute(&typedvalue)).into()
            }
        }
        unsafe extern "system" fn dataType<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatypename: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::dataType(this) {
                    Ok(ok__) => {
                        datatypename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetdataType<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatypename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNode_Impl::SetdataType(this, core::mem::transmute(&datatypename)).into()
            }
        }
        unsafe extern "system" fn xml<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xmlstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::xml(this) {
                    Ok(ok__) => {
                        xmlstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn transformNode<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void, xmlstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::transformNode(this, core::mem::transmute_copy(&stylesheet)) {
                    Ok(ok__) => {
                        xmlstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn selectNodes<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querystring: *mut core::ffi::c_void, resultlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::selectNodes(this, core::mem::transmute(&querystring)) {
                    Ok(ok__) => {
                        resultlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn selectSingleNode<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querystring: *mut core::ffi::c_void, resultnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::selectSingleNode(this, core::mem::transmute(&querystring)) {
                    Ok(ok__) => {
                        resultnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn parsed<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isparsed: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::parsed(this) {
                    Ok(ok__) => {
                        isparsed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn namespaceURI<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceuri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::namespaceURI(this) {
                    Ok(ok__) => {
                        namespaceuri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn prefix<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefixstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::prefix(this) {
                    Ok(ok__) => {
                        prefixstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn baseName<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNode_Impl::baseName(this) {
                    Ok(ok__) => {
                        namestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn transformNodeToObject<Identity: IXMLDOMNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stylesheet: *mut core::ffi::c_void, outputobject: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNode_Impl::transformNodeToObject(this, core::mem::transmute_copy(&stylesheet), core::mem::transmute(&outputobject)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            nodeName: nodeName::<Identity, OFFSET>,
            nodeValue: nodeValue::<Identity, OFFSET>,
            SetnodeValue: SetnodeValue::<Identity, OFFSET>,
            nodeType: nodeType::<Identity, OFFSET>,
            parentNode: parentNode::<Identity, OFFSET>,
            childNodes: childNodes::<Identity, OFFSET>,
            firstChild: firstChild::<Identity, OFFSET>,
            lastChild: lastChild::<Identity, OFFSET>,
            previousSibling: previousSibling::<Identity, OFFSET>,
            nextSibling: nextSibling::<Identity, OFFSET>,
            attributes: attributes::<Identity, OFFSET>,
            insertBefore: insertBefore::<Identity, OFFSET>,
            replaceChild: replaceChild::<Identity, OFFSET>,
            removeChild: removeChild::<Identity, OFFSET>,
            appendChild: appendChild::<Identity, OFFSET>,
            hasChildNodes: hasChildNodes::<Identity, OFFSET>,
            ownerDocument: ownerDocument::<Identity, OFFSET>,
            cloneNode: cloneNode::<Identity, OFFSET>,
            nodeTypeString: nodeTypeString::<Identity, OFFSET>,
            text: text::<Identity, OFFSET>,
            Settext: Settext::<Identity, OFFSET>,
            specified: specified::<Identity, OFFSET>,
            definition: definition::<Identity, OFFSET>,
            nodeTypedValue: nodeTypedValue::<Identity, OFFSET>,
            SetnodeTypedValue: SetnodeTypedValue::<Identity, OFFSET>,
            dataType: dataType::<Identity, OFFSET>,
            SetdataType: SetdataType::<Identity, OFFSET>,
            xml: xml::<Identity, OFFSET>,
            transformNode: transformNode::<Identity, OFFSET>,
            selectNodes: selectNodes::<Identity, OFFSET>,
            selectSingleNode: selectSingleNode::<Identity, OFFSET>,
            parsed: parsed::<Identity, OFFSET>,
            namespaceURI: namespaceURI::<Identity, OFFSET>,
            prefix: prefix::<Identity, OFFSET>,
            baseName: baseName::<Identity, OFFSET>,
            transformNodeToObject: transformNodeToObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMNode {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMNodeList, IXMLDOMNodeList_Vtbl, 0x2933bf82_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMNodeList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMNodeList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMNodeList {
    pub unsafe fn item(&self, index: i32) -> windows_core::Result<IXMLDOMNode> {
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
    pub unsafe fn nextNode(&self) -> windows_core::Result<IXMLDOMNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nextNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
pub struct IXMLDOMNodeList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub nextNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMNodeList_Impl: super::oaidl::IDispatch_Impl {
    fn item(&self, index: i32) -> windows_core::Result<IXMLDOMNode>;
    fn length(&self) -> windows_core::Result<i32>;
    fn nextNode(&self) -> windows_core::Result<IXMLDOMNode>;
    fn reset(&self) -> windows_core::Result<()>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMNodeList_Vtbl {
    pub const fn new<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn item<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, listitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNodeList_Impl::item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        listitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn length<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, listlength: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNodeList_Impl::length(this) {
                    Ok(ok__) => {
                        listlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn nextNode<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNodeList_Impl::nextNode(this) {
                    Ok(ok__) => {
                        nextitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reset<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMNodeList_Impl::reset(this).into()
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IXMLDOMNodeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNodeList_Impl::_newEnum(this) {
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
            nextNode: nextNode::<Identity, OFFSET>,
            reset: reset::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNodeList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMNodeList {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMNotation, IXMLDOMNotation_Vtbl, 0x2933bf8c_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMNotation {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMNotation, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMNotation {
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn publicId(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).publicId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn systemId(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).systemId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMNotation_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub publicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    publicId: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub systemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    systemId: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMNotation_Impl: IXMLDOMNode_Impl {
    fn publicId(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn systemId(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMNotation_Vtbl {
    pub const fn new<Identity: IXMLDOMNotation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn publicId<Identity: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, publicid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNotation_Impl::publicId(this) {
                    Ok(ok__) => {
                        publicid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn systemId<Identity: IXMLDOMNotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemid: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMNotation_Impl::systemId(this) {
                    Ok(ok__) => {
                        systemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(), publicId: publicId::<Identity, OFFSET>, systemId: systemId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMNotation as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMNotation {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMParseError, IXMLDOMParseError_Vtbl, 0x3efaa426_272f_11d2_836f_0000f87a7782);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMParseError {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMParseError, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMParseError {
    pub unsafe fn errorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).errorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn url(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).url)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn reason(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn srcText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).srcText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn line(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).line)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn linepos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).linepos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn filepos(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).filepos)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMParseError_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub errorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub url: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub srcText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub line: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub linepos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub filepos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMParseError_Impl: super::oaidl::IDispatch_Impl {
    fn errorCode(&self) -> windows_core::Result<i32>;
    fn url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn reason(&self) -> windows_core::Result<windows_core::BSTR>;
    fn srcText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn line(&self) -> windows_core::Result<i32>;
    fn linepos(&self) -> windows_core::Result<i32>;
    fn filepos(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMParseError_Vtbl {
    pub const fn new<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn errorCode<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::errorCode(this) {
                    Ok(ok__) => {
                        errorcode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn url<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urlstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::url(this) {
                    Ok(ok__) => {
                        urlstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn reason<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reasonstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::reason(this) {
                    Ok(ok__) => {
                        reasonstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn srcText<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::srcText(this) {
                    Ok(ok__) => {
                        sourcestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn line<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linenumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::line(this) {
                    Ok(ok__) => {
                        linenumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn linepos<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::linepos(this) {
                    Ok(ok__) => {
                        lineposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn filepos<Identity: IXMLDOMParseError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMParseError_Impl::filepos(this) {
                    Ok(ok__) => {
                        fileposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            errorCode: errorCode::<Identity, OFFSET>,
            url: url::<Identity, OFFSET>,
            reason: reason::<Identity, OFFSET>,
            srcText: srcText::<Identity, OFFSET>,
            line: line::<Identity, OFFSET>,
            linepos: linepos::<Identity, OFFSET>,
            filepos: filepos::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMParseError as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMParseError {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMProcessingInstruction, IXMLDOMProcessingInstruction_Vtbl, 0x2933bf89_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMProcessingInstruction {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMProcessingInstruction, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMProcessingInstruction {
    pub unsafe fn target(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).target)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setdata(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setdata)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMProcessingInstruction_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setdata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMProcessingInstruction_Impl: IXMLDOMNode_Impl {
    fn target(&self) -> windows_core::Result<windows_core::BSTR>;
    fn data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setdata(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMProcessingInstruction_Vtbl {
    pub const fn new<Identity: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn target<Identity: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMProcessingInstruction_Impl::target(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn data<Identity: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMProcessingInstruction_Impl::data(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setdata<Identity: IXMLDOMProcessingInstruction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDOMProcessingInstruction_Impl::Setdata(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            target: target::<Identity, OFFSET>,
            data: data::<Identity, OFFSET>,
            Setdata: Setdata::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMProcessingInstruction as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMProcessingInstruction {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDOMText, IXMLDOMText_Vtbl, 0x2933bf87_7b36_11d2_b20e_00c04f983e60);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDOMText {
    type Target = IXMLDOMCharacterData;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDOMText, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode, IXMLDOMCharacterData);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDOMText {
    pub unsafe fn splitText(&self, offset: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).splitText)(windows_core::Interface::as_raw(self), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDOMText_Vtbl {
    pub base__: IXMLDOMCharacterData_Vtbl,
    pub splitText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDOMText_Impl: IXMLDOMCharacterData_Impl {
    fn splitText(&self, offset: i32) -> windows_core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDOMText_Vtbl {
    pub const fn new<Identity: IXMLDOMText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn splitText<Identity: IXMLDOMText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDOMText_Impl::splitText(this, core::mem::transmute_copy(&offset)) {
                    Ok(ok__) => {
                        righthandtextnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IXMLDOMCharacterData_Vtbl::new::<Identity, OFFSET>(), splitText: splitText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDOMText as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID || iid == &<IXMLDOMCharacterData as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDOMText {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDSOControl, IXMLDSOControl_Vtbl, 0x310afa62_0575_11d2_9ca9_0060b0ec3d39);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDSOControl {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDSOControl, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDSOControl {
    pub unsafe fn XMLDocument(&self) -> windows_core::Result<IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XMLDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetXMLDocument<P0>(&self, ppdoc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXMLDOMDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXMLDocument)(windows_core::Interface::as_raw(self), ppdoc.param().abi()) }
    }
    pub unsafe fn JavaDSOCompatible(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JavaDSOCompatible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetJavaDSOCompatible(&self, fjavadsocompatible: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetJavaDSOCompatible)(windows_core::Interface::as_raw(self), fjavadsocompatible.into()) }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDSOControl_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub XMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JavaDSOCompatible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetJavaDSOCompatible: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDSOControl_Impl: super::oaidl::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(&self, ppdoc: windows_core::Ref<IXMLDOMDocument>) -> windows_core::Result<()>;
    fn JavaDSOCompatible(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetJavaDSOCompatible(&self, fjavadsocompatible: windows_core::BOOL) -> windows_core::Result<()>;
    fn readyState(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDSOControl_Vtbl {
    pub const fn new<Identity: IXMLDSOControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XMLDocument<Identity: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDSOControl_Impl::XMLDocument(this) {
                    Ok(ok__) => {
                        ppdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXMLDocument<Identity: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdoc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDSOControl_Impl::SetXMLDocument(this, core::mem::transmute_copy(&ppdoc)).into()
            }
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fjavadsocompatible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDSOControl_Impl::JavaDSOCompatible(this) {
                    Ok(ok__) => {
                        fjavadsocompatible.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fjavadsocompatible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDSOControl_Impl::SetJavaDSOCompatible(this, core::mem::transmute_copy(&fjavadsocompatible)).into()
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLDSOControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDSOControl_Impl::readyState(this) {
                    Ok(ok__) => {
                        state.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, OFFSET>,
            SetXMLDocument: SetXMLDocument::<Identity, OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Identity, OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDSOControl as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDSOControl {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDocument, IXMLDocument_Vtbl, 0xf52e2b61_18a1_11d1_b105_00805f49916b);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDocument {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDocument, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDocument {
    pub unsafe fn root(&self) -> windows_core::Result<IXMLElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn fileSize(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileModifiedDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileUpdatedDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn URL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).URL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn mimeType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).mimeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn charset(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).charset)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setcharset)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn doctype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).doctype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dtdURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn createElement(&self, vtype: &super::oaidl::VARIANT, var1: &super::oaidl::VARIANT) -> windows_core::Result<IXMLElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createElement)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtype), core::mem::transmute_copy(var1), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDocument_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileModifiedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileUpdatedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub mimeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub charset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setcharset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub doctype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dtdURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub createElement: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    createElement: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDocument_Impl: super::oaidl::IDispatch_Impl {
    fn root(&self) -> windows_core::Result<IXMLElement>;
    fn fileSize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn mimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn charset(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn doctype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn createElement(&self, vtype: &super::oaidl::VARIANT, var1: &super::oaidl::VARIANT) -> windows_core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDocument_Vtbl {
    pub const fn new<Identity: IXMLDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn root<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::root(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileSize<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::fileSize(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::fileModifiedDate(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::fileUpdatedDate(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn URL<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::URL(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURL<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDocument_Impl::SetURL(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn mimeType<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::mimeType(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::readyState(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn charset<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::charset(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setcharset<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDocument_Impl::Setcharset(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn version<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::version(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn doctype<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::doctype(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn dtdURL<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::dtdURL(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createElement<Identity: IXMLDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtype: super::oaidl::VARIANT, var1: super::oaidl::VARIANT, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument_Impl::createElement(this, core::mem::transmute(&vtype), core::mem::transmute(&var1)) {
                    Ok(ok__) => {
                        ppelem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            root: root::<Identity, OFFSET>,
            fileSize: fileSize::<Identity, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            mimeType: mimeType::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
            charset: charset::<Identity, OFFSET>,
            Setcharset: Setcharset::<Identity, OFFSET>,
            version: version::<Identity, OFFSET>,
            doctype: doctype::<Identity, OFFSET>,
            dtdURL: dtdURL::<Identity, OFFSET>,
            createElement: createElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDocument as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDocument {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLDocument2, IXMLDocument2_Vtbl, 0x2b8de2fe_8d2d_11d1_b2fc_00c04fd915a9);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLDocument2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLDocument2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLDocument2 {
    pub unsafe fn root(&self) -> windows_core::Result<IXMLElement2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn fileSize(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileModifiedDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).fileUpdatedDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn URL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).URL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn mimeType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).mimeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn readyState(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).readyState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn charset(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).charset)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setcharset)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn doctype(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).doctype)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dtdURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn createElement(&self, vtype: &super::oaidl::VARIANT, var1: &super::oaidl::VARIANT) -> windows_core::Result<IXMLElement2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createElement)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtype), core::mem::transmute_copy(var1), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn r#async(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#async)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn Setasync(&self, f: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setasync)(windows_core::Interface::as_raw(self), f) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLDocument2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileModifiedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fileUpdatedDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub mimeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub readyState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub charset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setcharset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub doctype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dtdURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub createElement: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    createElement: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub r#async: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    r#async: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub Setasync: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    Setasync: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLDocument2_Impl: super::oaidl::IDispatch_Impl {
    fn root(&self) -> windows_core::Result<IXMLElement2>;
    fn fileSize(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileModifiedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn fileUpdatedDate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetURL(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn mimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn readyState(&self) -> windows_core::Result<i32>;
    fn charset(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setcharset(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn doctype(&self) -> windows_core::Result<windows_core::BSTR>;
    fn dtdURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn createElement(&self, vtype: &super::oaidl::VARIANT, var1: &super::oaidl::VARIANT) -> windows_core::Result<IXMLElement2>;
    fn r#async(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn Setasync(&self, f: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLDocument2_Vtbl {
    pub const fn new<Identity: IXMLDocument2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn root<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::root(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileSize<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::fileSize(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileModifiedDate<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::fileModifiedDate(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::fileUpdatedDate(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn URL<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::URL(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURL<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDocument2_Impl::SetURL(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn mimeType<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::mimeType(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::readyState(this) {
                    Ok(ok__) => {
                        pl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn charset<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::charset(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setcharset<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDocument2_Impl::Setcharset(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn version<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::version(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn doctype<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::doctype(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn dtdURL<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::dtdURL(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createElement<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtype: super::oaidl::VARIANT, var1: super::oaidl::VARIANT, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::createElement(this, core::mem::transmute(&vtype), core::mem::transmute(&var1)) {
                    Ok(ok__) => {
                        ppelem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#async<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pf: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLDocument2_Impl::r#async(this) {
                    Ok(ok__) => {
                        pf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setasync<Identity: IXMLDocument2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, f: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLDocument2_Impl::Setasync(this, core::mem::transmute_copy(&f)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            root: root::<Identity, OFFSET>,
            fileSize: fileSize::<Identity, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            mimeType: mimeType::<Identity, OFFSET>,
            readyState: readyState::<Identity, OFFSET>,
            charset: charset::<Identity, OFFSET>,
            Setcharset: Setcharset::<Identity, OFFSET>,
            version: version::<Identity, OFFSET>,
            doctype: doctype::<Identity, OFFSET>,
            dtdURL: dtdURL::<Identity, OFFSET>,
            createElement: createElement::<Identity, OFFSET>,
            r#async: r#async::<Identity, OFFSET>,
            Setasync: Setasync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLDocument2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLDocument2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLElement, IXMLElement_Vtbl, 0x3f7f31ac_e15f_11d0_9c25_00c04fc99c8e);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLElement {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLElement, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLElement {
    pub unsafe fn tagName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).tagName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SettagName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn parent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), core::mem::transmute_copy(propertyvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname)) }
    }
    pub unsafe fn children(&self) -> windows_core::Result<IXMLElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn r#type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn text(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).text)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Settext(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Settext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn addChild<P0>(&self, pchildelem: P0, lindex: i32, lreserved: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).addChild)(windows_core::Interface::as_raw(self), pchildelem.param().abi(), lindex, lreserved) }
    }
    pub unsafe fn removeChild<P0>(&self, pchildelem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).removeChild)(windows_core::Interface::as_raw(self), pchildelem.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElement_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub tagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setAttribute: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub r#type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub addChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub removeChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLElement_Impl: super::oaidl::IDispatch_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parent(&self) -> windows_core::Result<IXMLElement>;
    fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn children(&self) -> windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> windows_core::Result<i32>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addChild(&self, pchildelem: windows_core::Ref<IXMLElement>, lindex: i32, lreserved: i32) -> windows_core::Result<()>;
    fn removeChild(&self, pchildelem: windows_core::Ref<IXMLElement>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLElement_Vtbl {
    pub const fn new<Identity: IXMLElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn tagName<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::tagName(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettagName<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::SettagName(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn parent<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::parent(this) {
                    Ok(ok__) => {
                        ppparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertyvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::setAttribute(this, core::mem::transmute(&strpropertyname), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn getAttribute<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::getAttribute(this, core::mem::transmute(&strpropertyname)) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::removeAttribute(this, core::mem::transmute(&strpropertyname)).into()
            }
        }
        unsafe extern "system" fn children<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::children(this) {
                    Ok(ok__) => {
                        pp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#type<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::r#type(this) {
                    Ok(ok__) => {
                        pltype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn text<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement_Impl::text(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settext<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::Settext(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn addChild<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void, lindex: i32, lreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::addChild(this, core::mem::transmute_copy(&pchildelem), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&lreserved)).into()
            }
        }
        unsafe extern "system" fn removeChild<Identity: IXMLElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement_Impl::removeChild(this, core::mem::transmute_copy(&pchildelem)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            tagName: tagName::<Identity, OFFSET>,
            SettagName: SettagName::<Identity, OFFSET>,
            parent: parent::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            children: children::<Identity, OFFSET>,
            r#type: r#type::<Identity, OFFSET>,
            text: text::<Identity, OFFSET>,
            Settext: Settext::<Identity, OFFSET>,
            addChild: addChild::<Identity, OFFSET>,
            removeChild: removeChild::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElement as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLElement {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLElement2, IXMLElement2_Vtbl, 0x2b8de2ff_8d2d_11d1_b2fc_00c04fd915a9);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLElement2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLElement2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLElement2 {
    pub unsafe fn tagName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).tagName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SettagName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn parent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), core::mem::transmute_copy(propertyvalue)) }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpropertyname)) }
    }
    pub unsafe fn children(&self) -> windows_core::Result<IXMLElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn r#type(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).r#type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn text(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).text)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Settext(&self, p: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Settext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(p)) }
    }
    pub unsafe fn addChild<P0>(&self, pchildelem: P0, lindex: i32, lreserved: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).addChild)(windows_core::Interface::as_raw(self), pchildelem.param().abi(), lindex, lreserved) }
    }
    pub unsafe fn removeChild<P0>(&self, pchildelem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).removeChild)(windows_core::Interface::as_raw(self), pchildelem.param().abi()) }
    }
    pub unsafe fn attributes(&self) -> windows_core::Result<IXMLElementCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElement2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub tagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    setAttribute: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    getAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub r#type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub addChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub removeChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLElement2_Impl: super::oaidl::IDispatch_Impl {
    fn tagName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettagName(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn parent(&self) -> windows_core::Result<IXMLElement2>;
    fn setAttribute(&self, strpropertyname: &windows_core::BSTR, propertyvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn getAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn removeAttribute(&self, strpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn children(&self) -> windows_core::Result<IXMLElementCollection>;
    fn r#type(&self) -> windows_core::Result<i32>;
    fn text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settext(&self, p: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addChild(&self, pchildelem: windows_core::Ref<IXMLElement2>, lindex: i32, lreserved: i32) -> windows_core::Result<()>;
    fn removeChild(&self, pchildelem: windows_core::Ref<IXMLElement2>) -> windows_core::Result<()>;
    fn attributes(&self) -> windows_core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLElement2_Vtbl {
    pub const fn new<Identity: IXMLElement2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn tagName<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::tagName(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettagName<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::SettagName(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn parent<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::parent(this) {
                    Ok(ok__) => {
                        ppparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertyvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::setAttribute(this, core::mem::transmute(&strpropertyname), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn getAttribute<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void, propertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::getAttribute(this, core::mem::transmute(&strpropertyname)) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::removeAttribute(this, core::mem::transmute(&strpropertyname)).into()
            }
        }
        unsafe extern "system" fn children<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::children(this) {
                    Ok(ok__) => {
                        pp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn r#type<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::r#type(this) {
                    Ok(ok__) => {
                        pltype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn text<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::text(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settext<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::Settext(this, core::mem::transmute(&p)).into()
            }
        }
        unsafe extern "system" fn addChild<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void, lindex: i32, lreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::addChild(this, core::mem::transmute_copy(&pchildelem), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&lreserved)).into()
            }
        }
        unsafe extern "system" fn removeChild<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildelem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElement2_Impl::removeChild(this, core::mem::transmute_copy(&pchildelem)).into()
            }
        }
        unsafe extern "system" fn attributes<Identity: IXMLElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElement2_Impl::attributes(this) {
                    Ok(ok__) => {
                        pp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            tagName: tagName::<Identity, OFFSET>,
            SettagName: SettagName::<Identity, OFFSET>,
            parent: parent::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            children: children::<Identity, OFFSET>,
            r#type: r#type::<Identity, OFFSET>,
            text: text::<Identity, OFFSET>,
            Settext: Settext::<Identity, OFFSET>,
            addChild: addChild::<Identity, OFFSET>,
            removeChild: removeChild::<Identity, OFFSET>,
            attributes: attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElement2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLElement2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLElementCollection, IXMLElementCollection_Vtbl, 0x65725580_9b5d_11d0_9bfe_00c04fc99c8e);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLElementCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLElementCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLElementCollection {
    pub unsafe fn Setlength(&self, v: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setlength)(windows_core::Interface::as_raw(self), v) }
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
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn item(&self, var1: &super::oaidl::VARIANT, var2: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var1), core::mem::transmute_copy(var2), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXMLElementCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Setlength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _newEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    item: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXMLElementCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Setlength(&self, v: i32) -> windows_core::Result<()>;
    fn length(&self) -> windows_core::Result<i32>;
    fn _newEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn item(&self, var1: &super::oaidl::VARIANT, var2: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::IDispatch>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXMLElementCollection_Vtbl {
    pub const fn new<Identity: IXMLElementCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Setlength<Identity: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLElementCollection_Impl::Setlength(this, core::mem::transmute_copy(&v)).into()
            }
        }
        unsafe extern "system" fn length<Identity: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElementCollection_Impl::length(this) {
                    Ok(ok__) => {
                        p.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _newEnum<Identity: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElementCollection_Impl::_newEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IXMLElementCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var1: super::oaidl::VARIANT, var2: super::oaidl::VARIANT, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLElementCollection_Impl::item(this, core::mem::transmute(&var1), core::mem::transmute(&var2)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Setlength: Setlength::<Identity, OFFSET>,
            length: length::<Identity, OFFSET>,
            _newEnum: _newEnum::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLElementCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLElementCollection {}
windows_core::imp::define_interface!(IXMLError, IXMLError_Vtbl, 0x948c5ad3_c58d_11d0_9c0b_00c04fc99c8e);
windows_core::imp::interface_hierarchy!(IXMLError, windows_core::IUnknown);
impl IXMLError {
    pub unsafe fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(perrorreturn)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXMLError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XML_ERROR) -> windows_core::HRESULT,
}
pub trait IXMLError_Impl: windows_core::IUnknownImpl {
    fn GetErrorInfo(&self, perrorreturn: *mut XML_ERROR) -> windows_core::Result<()>;
}
impl IXMLError_Vtbl {
    pub const fn new<Identity: IXMLError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorInfo<Identity: IXMLError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLError_Impl::GetErrorInfo(this, core::mem::transmute_copy(&perrorreturn)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXMLError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXMLError {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXMLHttpRequest, IXMLHttpRequest_Vtbl, 0xed8c108d_4349_11d2_91a4_00c04f7969e8);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXMLHttpRequest {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXMLHttpRequest, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IXMLHttpRequest {
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
pub struct IXMLHttpRequest_Vtbl {
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
pub trait IXMLHttpRequest_Impl: super::oaidl::IDispatch_Impl {
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
impl IXMLHttpRequest_Vtbl {
    pub const fn new<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn open<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethod: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, varasync: super::oaidl::VARIANT, bstruser: super::oaidl::VARIANT, bstrpassword: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHttpRequest_Impl::open(this, core::mem::transmute(&bstrmethod), core::mem::transmute(&bstrurl), core::mem::transmute(&varasync), core::mem::transmute(&bstruser), core::mem::transmute(&bstrpassword)).into()
            }
        }
        unsafe extern "system" fn setRequestHeader<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHttpRequest_Impl::setRequestHeader(this, core::mem::transmute(&bstrheader), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn getResponseHeader<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheader: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::getResponseHeader(this, core::mem::transmute(&bstrheader)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrheaders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::getAllResponseHeaders(this) {
                    Ok(ok__) => {
                        pbstrheaders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn send<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHttpRequest_Impl::send(this, core::mem::transmute(&varbody)).into()
            }
        }
        unsafe extern "system" fn abort<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHttpRequest_Impl::abort(this).into()
            }
        }
        unsafe extern "system" fn status<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstatus: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::status(this) {
                    Ok(ok__) => {
                        plstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn statusText<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::statusText(this) {
                    Ok(ok__) => {
                        pbstrstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseXML<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::responseXML(this) {
                    Ok(ok__) => {
                        ppbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseText<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::responseText(this) {
                    Ok(ok__) => {
                        pbstrbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseBody<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::responseBody(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn responseStream<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::responseStream(this) {
                    Ok(ok__) => {
                        pvarbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn readyState<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXMLHttpRequest_Impl::readyState(this) {
                    Ok(ok__) => {
                        plstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: IXMLHttpRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preadystatesink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXMLHttpRequest_Impl::Setonreadystatechange(this, core::mem::transmute_copy(&preadystatesink)).into()
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
        iid == &<IXMLHttpRequest as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXMLHttpRequest {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IXTLRuntime, IXTLRuntime_Vtbl, 0x3efaa425_272f_11d2_836f_0000f87a7782);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IXTLRuntime {
    type Target = IXMLDOMNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IXTLRuntime, windows_core::IUnknown, super::oaidl::IDispatch, IXMLDOMNode);
#[cfg(feature = "Win32_oaidl")]
impl IXTLRuntime {
    pub unsafe fn uniqueID<P0>(&self, pnode: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).uniqueID)(windows_core::Interface::as_raw(self), pnode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn depth<P0>(&self, pnode: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).depth)(windows_core::Interface::as_raw(self), pnode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn childNumber<P0>(&self, pnode: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).childNumber)(windows_core::Interface::as_raw(self), pnode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ancestorChildNumber<P1>(&self, bstrnodename: &windows_core::BSTR, pnode: P1) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ancestorChildNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrnodename), pnode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn absoluteChildNumber<P0>(&self, pnode: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IXMLDOMNode>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).absoluteChildNumber)(windows_core::Interface::as_raw(self), pnode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn formatIndex(&self, lindex: i32, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).formatIndex)(windows_core::Interface::as_raw(self), lindex, core::mem::transmute_copy(bstrformat), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn formatNumber(&self, dblnumber: f64, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).formatNumber)(windows_core::Interface::as_raw(self), dblnumber, core::mem::transmute_copy(bstrformat), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn formatDate(&self, vardate: &super::oaidl::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).formatDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vardate), core::mem::transmute_copy(bstrformat), core::mem::transmute_copy(vardestlocale), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn formatTime(&self, vartime: &super::oaidl::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).formatTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vartime), core::mem::transmute_copy(bstrformat), core::mem::transmute_copy(vardestlocale), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IXTLRuntime_Vtbl {
    pub base__: IXMLDOMNode_Vtbl,
    pub uniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub depth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub childNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ancestorChildNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub absoluteChildNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub formatIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub formatNumber: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub formatDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    formatDate: usize,
    #[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub formatTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    formatTime: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IXTLRuntime_Impl: IXMLDOMNode_Impl {
    fn uniqueID(&self, pnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<i32>;
    fn depth(&self, pnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<i32>;
    fn childNumber(&self, pnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<i32>;
    fn ancestorChildNumber(&self, bstrnodename: &windows_core::BSTR, pnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<i32>;
    fn absoluteChildNumber(&self, pnode: windows_core::Ref<IXMLDOMNode>) -> windows_core::Result<i32>;
    fn formatIndex(&self, lindex: i32, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn formatNumber(&self, dblnumber: f64, bstrformat: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn formatDate(&self, vardate: &super::oaidl::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn formatTime(&self, vartime: &super::oaidl::VARIANT, bstrformat: &windows_core::BSTR, vardestlocale: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IXTLRuntime_Vtbl {
    pub const fn new<Identity: IXTLRuntime_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn uniqueID<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::uniqueID(this, core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn depth<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pdepth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::depth(this, core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        pdepth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn childNumber<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::childNumber(this, core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        pnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnodename: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::ancestorChildNumber(this, core::mem::transmute(&bstrnodename), core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        pnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, pnumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::absoluteChildNumber(this, core::mem::transmute_copy(&pnode)) {
                    Ok(ok__) => {
                        pnumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn formatIndex<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, bstrformat: *mut core::ffi::c_void, pbstrformattedstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::formatIndex(this, core::mem::transmute_copy(&lindex), core::mem::transmute(&bstrformat)) {
                    Ok(ok__) => {
                        pbstrformattedstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn formatNumber<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dblnumber: f64, bstrformat: *mut core::ffi::c_void, pbstrformattedstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::formatNumber(this, core::mem::transmute_copy(&dblnumber), core::mem::transmute(&bstrformat)) {
                    Ok(ok__) => {
                        pbstrformattedstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn formatDate<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardate: super::oaidl::VARIANT, bstrformat: *mut core::ffi::c_void, vardestlocale: super::oaidl::VARIANT, pbstrformattedstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::formatDate(this, core::mem::transmute(&vardate), core::mem::transmute(&bstrformat), core::mem::transmute(&vardestlocale)) {
                    Ok(ok__) => {
                        pbstrformattedstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn formatTime<Identity: IXTLRuntime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartime: super::oaidl::VARIANT, bstrformat: *mut core::ffi::c_void, vardestlocale: super::oaidl::VARIANT, pbstrformattedstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXTLRuntime_Impl::formatTime(this, core::mem::transmute(&vartime), core::mem::transmute(&bstrformat), core::mem::transmute(&vardestlocale)) {
                    Ok(ok__) => {
                        pbstrformattedstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IXMLDOMNode_Vtbl::new::<Identity, OFFSET>(),
            uniqueID: uniqueID::<Identity, OFFSET>,
            depth: depth::<Identity, OFFSET>,
            childNumber: childNumber::<Identity, OFFSET>,
            ancestorChildNumber: ancestorChildNumber::<Identity, OFFSET>,
            absoluteChildNumber: absoluteChildNumber::<Identity, OFFSET>,
            formatIndex: formatIndex::<Identity, OFFSET>,
            formatNumber: formatNumber::<Identity, OFFSET>,
            formatDate: formatDate::<Identity, OFFSET>,
            formatTime: formatTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXTLRuntime as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IXMLDOMNode as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IXTLRuntime {}
pub const NODE_ATTRIBUTE: DOMNodeType = 2;
pub const NODE_CDATA_SECTION: DOMNodeType = 4;
pub const NODE_COMMENT: DOMNodeType = 8;
pub const NODE_DOCUMENT: DOMNodeType = 9;
pub const NODE_DOCUMENT_FRAGMENT: DOMNodeType = 11;
pub const NODE_DOCUMENT_TYPE: DOMNodeType = 10;
pub const NODE_ELEMENT: DOMNodeType = 1;
pub const NODE_ENTITY: DOMNodeType = 6;
pub const NODE_ENTITY_REFERENCE: DOMNodeType = 5;
pub const NODE_INVALID: DOMNodeType = 0;
pub const NODE_NOTATION: DOMNodeType = 12;
pub const NODE_PROCESSING_INSTRUCTION: DOMNodeType = 7;
pub const NODE_TEXT: DOMNodeType = 3;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(XMLDOMDocumentEvents, XMLDOMDocumentEvents_Vtbl, 0x3efaa427_272f_11d2_836f_0000f87a7782);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for XMLDOMDocumentEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(XMLDOMDocumentEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct XMLDOMDocumentEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait XMLDOMDocumentEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl XMLDOMDocumentEvents_Vtbl {
    pub const fn new<Identity: XMLDOMDocumentEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<XMLDOMDocumentEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for XMLDOMDocumentEvents {}
pub const XMLDSOControl: windows_core::GUID = windows_core::GUID::from_u128(0x550dda30_0541_11d2_9ca9_0060b0ec3d39);
pub const XMLDocument: windows_core::GUID = windows_core::GUID::from_u128(0xcfc399af_d876_11d0_9c10_00c04fc99c8e);
pub const XMLELEMTYPE_COMMENT: XMLELEM_TYPE = 2;
pub const XMLELEMTYPE_DOCUMENT: XMLELEM_TYPE = 3;
pub const XMLELEMTYPE_DTD: XMLELEM_TYPE = 4;
pub const XMLELEMTYPE_ELEMENT: XMLELEM_TYPE = 0;
pub const XMLELEMTYPE_OTHER: XMLELEM_TYPE = 6;
pub const XMLELEMTYPE_PI: XMLELEM_TYPE = 5;
pub const XMLELEMTYPE_TEXT: XMLELEM_TYPE = 1;
pub type XMLELEM_TYPE = i32;
pub const XMLHTTPRequest: windows_core::GUID = windows_core::GUID::from_u128(0xed8c108e_4349_11d2_91a4_00c04f7969e8);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XML_ERROR {
    pub _nLine: u32,
    pub _pchBuf: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub _cchBuf: u32,
    pub _ich: u32,
    pub _pszFound: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub _pszExpected: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub _reserved1: u32,
    pub _reserved2: u32,
}
