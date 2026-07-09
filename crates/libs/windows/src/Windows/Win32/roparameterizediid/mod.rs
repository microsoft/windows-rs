#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE) {
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoFreeParameterizedTypeExtra(extra : ROPARAMIIDHANDLE));
    unsafe { RoFreeParameterizedTypeExtra(extra) }
}
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<P2>(nameelements: &[windows_core::PCWSTR], metadatalocator: P2, iid: *mut windows_core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<IRoMetaDataLocator>,
{
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoGetParameterizedTypeInstanceIID(nameelementcount : u32, nameelements : *const windows_core::PCWSTR, metadatalocator : *mut core::ffi::c_void, iid : *mut windows_core::GUID, pextra : *mut ROPARAMIIDHANDLE) -> windows_core::HRESULT);
    unsafe { RoGetParameterizedTypeInstanceIID(nameelements.len().try_into().unwrap(), core::mem::transmute(nameelements.as_ptr()), metadatalocator.param().abi(), iid as _, pextra as _) }
}
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> windows_core::PCSTR {
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoParameterizedTypeExtraGetTypeSignature(extra : ROPARAMIIDHANDLE) -> windows_core::PCSTR);
    unsafe { RoParameterizedTypeExtraGetTypeSignature(extra) }
}
windows_core::imp::define_interface!(IRoMetaDataLocator, IRoMetaDataLocator_Vtbl);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IRoSimpleMetaDataBuilder>,
    {
        unsafe { (windows_core::Interface::vtable(self).Locate)(windows_core::Interface::as_raw(self), nameelement.param().abi(), metadatadestination.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRoMetaDataLocator_Impl {
    fn Locate(&self, nameelement: &windows_core::PCWSTR, metadatadestination: windows_core::Ref<IRoSimpleMetaDataBuilder>) -> windows_core::Result<()>;
}
impl IRoMetaDataLocator_Vtbl {
    pub const fn new<Identity: IRoMetaDataLocator_Impl>() -> Self {
        unsafe extern "system" fn Locate<Identity: IRoMetaDataLocator_Impl>(this: *mut core::ffi::c_void, nameelement: windows_core::PCWSTR, metadatadestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoMetaDataLocator_Impl::Locate(this, core::mem::transmute(&nameelement), core::mem::transmute_copy(&metadatadestination)).into()
            }
        }
        Self { Locate: Locate::<Identity> }
    }
}
struct IRoMetaDataLocator_ImplVtbl<T: IRoMetaDataLocator_Impl>(core::marker::PhantomData<T>);
impl<T: IRoMetaDataLocator_Impl> IRoMetaDataLocator_ImplVtbl<T> {
    const VTABLE: IRoMetaDataLocator_Vtbl = IRoMetaDataLocator_Vtbl::new::<T>();
}
impl IRoMetaDataLocator {
    pub fn new<'a, T: IRoMetaDataLocator_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoMetaDataLocator_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IRoSimpleMetaDataBuilder, IRoSimpleMetaDataBuilder_Vtbl);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWinRtInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)) }
    }
    pub unsafe fn SetDelegate(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)) }
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())) }
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())) }
    }
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStruct)(windows_core::Interface::as_raw(self), name.param().abi(), fieldtypenames.len().try_into().unwrap(), core::mem::transmute(fieldtypenames.as_ptr())) }
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnum)(windows_core::Interface::as_raw(self), name.param().abi(), basetype.param().abi()) }
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs) }
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IRoSimpleMetaDataBuilder_Impl {
    fn SetWinRtInterface(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetDelegate(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRuntimeClassSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetRuntimeClassParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetStruct(&self, name: &windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetEnum(&self, name: &windows_core::PCWSTR, basetype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetParameterizedInterface(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
    fn SetParameterizedDelegate(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
}
impl IRoSimpleMetaDataBuilder_Vtbl {
    pub const fn new<Identity: IRoSimpleMetaDataBuilder_Impl>() -> Self {
        unsafe extern "system" fn SetWinRtInterface<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetWinRtInterface(this, core::mem::transmute(&iid)).into()
            }
        }
        unsafe extern "system" fn SetDelegate<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetDelegate(this, core::mem::transmute(&iid)).into()
            }
        }
        unsafe extern "system" fn SetInterfaceGroupSimpleDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
            }
        }
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
            }
        }
        unsafe extern "system" fn SetRuntimeClassSimpleDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
            }
        }
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
            }
        }
        unsafe extern "system" fn SetStruct<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetStruct(this, core::mem::transmute(&name), core::mem::transmute_copy(&numfields), core::mem::transmute_copy(&fieldtypenames)).into()
            }
        }
        unsafe extern "system" fn SetEnum<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, basetype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetEnum(this, core::mem::transmute(&name), core::mem::transmute(&basetype)).into()
            }
        }
        unsafe extern "system" fn SetParameterizedInterface<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetParameterizedInterface(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
            }
        }
        unsafe extern "system" fn SetParameterizedDelegate<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetParameterizedDelegate(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
            }
        }
        Self {
            SetWinRtInterface: SetWinRtInterface::<Identity>,
            SetDelegate: SetDelegate::<Identity>,
            SetInterfaceGroupSimpleDefault: SetInterfaceGroupSimpleDefault::<Identity>,
            SetInterfaceGroupParameterizedDefault: SetInterfaceGroupParameterizedDefault::<Identity>,
            SetRuntimeClassSimpleDefault: SetRuntimeClassSimpleDefault::<Identity>,
            SetRuntimeClassParameterizedDefault: SetRuntimeClassParameterizedDefault::<Identity>,
            SetStruct: SetStruct::<Identity>,
            SetEnum: SetEnum::<Identity>,
            SetParameterizedInterface: SetParameterizedInterface::<Identity>,
            SetParameterizedDelegate: SetParameterizedDelegate::<Identity>,
        }
    }
}
struct IRoSimpleMetaDataBuilder_ImplVtbl<T: IRoSimpleMetaDataBuilder_Impl>(core::marker::PhantomData<T>);
impl<T: IRoSimpleMetaDataBuilder_Impl> IRoSimpleMetaDataBuilder_ImplVtbl<T> {
    const VTABLE: IRoSimpleMetaDataBuilder_Vtbl = IRoSimpleMetaDataBuilder_Vtbl::new::<T>();
}
impl IRoSimpleMetaDataBuilder {
    pub fn new<'a, T: IRoSimpleMetaDataBuilder_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoSimpleMetaDataBuilder_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ROPARAMIIDHANDLE(pub *mut core::ffi::c_void);
impl ROPARAMIIDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
