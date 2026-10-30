windows_core::imp::define_interface!(IObjectArray, IObjectArray_Vtbl, 0x92ca9dcd_5622_4bba_a805_5e9f541bd8c9);
windows_core::imp::interface_hierarchy!(IObjectArray, windows_core::IUnknown);
impl IObjectArray {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcobjects: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcobjects as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetAt<T>(&self, uiindex: super::UINT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), uiindex, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetAt: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IObjectArray_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcobjects: *mut super::UINT) -> windows_core::Result<()>;
    fn GetAt(&self, uiindex: super::UINT, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IObjectArray_Vtbl {
    pub const fn new<Identity: IObjectArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IObjectArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcobjects: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectArray_Impl::GetCount(this, core::mem::transmute_copy(&pcobjects)).into()
            }
        }
        unsafe extern "system" fn GetAt<Identity: IObjectArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: super::UINT, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectArray_Impl::GetAt(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IObjectArray {}
windows_core::imp::define_interface!(IObjectCollection, IObjectCollection_Vtbl, 0x5632b1a4_e38a_400a_928a_d4cd63230295);
impl core::ops::Deref for IObjectCollection {
    type Target = IObjectArray;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectCollection, windows_core::IUnknown, IObjectArray);
impl IObjectCollection {
    pub unsafe fn AddObject<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddObject)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn AddFromArray<P0>(&self, poasource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IObjectArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFromArray)(windows_core::Interface::as_raw(self), poasource.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveObjectAt(&self, uiindex: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveObjectAt)(windows_core::Interface::as_raw(self), uiindex) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectCollection_Vtbl {
    pub base__: IObjectArray_Vtbl,
    pub AddObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveObjectAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveObjectAt: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IObjectCollection_Impl: IObjectArray_Impl {
    fn AddObject(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddFromArray(&self, poasource: windows_core::Ref<IObjectArray>) -> windows_core::Result<()>;
    fn RemoveObjectAt(&self, uiindex: super::UINT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IObjectCollection_Vtbl {
    pub const fn new<Identity: IObjectCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddObject<Identity: IObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectCollection_Impl::AddObject(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn AddFromArray<Identity: IObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poasource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectCollection_Impl::AddFromArray(this, core::mem::transmute_copy(&poasource)).into()
            }
        }
        unsafe extern "system" fn RemoveObjectAt<Identity: IObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectCollection_Impl::RemoveObjectAt(this, core::mem::transmute_copy(&uiindex)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: IObjectArray_Vtbl::new::<Identity, OFFSET>(),
            AddObject: AddObject::<Identity, OFFSET>,
            AddFromArray: AddFromArray::<Identity, OFFSET>,
            RemoveObjectAt: RemoveObjectAt::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectCollection as windows_core::Interface>::IID || iid == &<IObjectArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IObjectCollection {}
