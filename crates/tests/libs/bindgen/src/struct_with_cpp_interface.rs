#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_RESOURCE_UAV_BARRIER {
    pub pResource: core::mem::ManuallyDrop<Option<ID3D12Resource>>,
}
windows_core::imp::define_interface!(
    ID3D12DeviceChild,
    ID3D12DeviceChild_Vtbl,
    0x905db94b_a00c_4140_9df5_2b64ca9ea357
);
impl core::ops::Deref for ID3D12DeviceChild {
    type Target = ID3D12Object;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12DeviceChild, windows_core::IUnknown, ID3D12Object);
impl ID3D12DeviceChild {
    pub unsafe fn GetDevice<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetDevice)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                result__ as *mut _ as *mut _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12DeviceChild_Vtbl {
    pub base__: ID3D12Object_Vtbl,
    pub GetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D12DeviceChild {}
unsafe impl Sync for ID3D12DeviceChild {}
pub trait ID3D12DeviceChild_Impl: ID3D12Object_Impl {
    fn GetDevice(
        &self,
        riid: *const windows_core::GUID,
        ppvdevice: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl ID3D12DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D12DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<
            Identity: ID3D12DeviceChild_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvdevice: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12DeviceChild_Impl::GetDevice(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvdevice),
                )
                .into()
            }
        }
        Self {
            base__: ID3D12Object_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D12DeviceChild {}
windows_core::imp::define_interface!(
    ID3D12Object,
    ID3D12Object_Vtbl,
    0xc4fec28f_7966_4e95_9f94_f431cb56c3b8
);
windows_core::imp::interface_hierarchy!(ID3D12Object, windows_core::IUnknown);
impl ID3D12Object {
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: Option<*mut core::ffi::c_void>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                pdatasize as _,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: Option<*const core::ffi::c_void>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                datasize,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(
        &self,
        guid: *const windows_core::GUID,
        pdata: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                guid,
                pdata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetName)(
                windows_core::Interface::as_raw(self),
                name.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Object_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D12Object {}
unsafe impl Sync for ID3D12Object {}
pub trait ID3D12Object_Impl: windows_core::IUnknownImpl {
    fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        guid: *const windows_core::GUID,
        pdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ID3D12Object_Vtbl {
    pub const fn new<Identity: ID3D12Object_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPrivateData<
            Identity: ID3D12Object_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12Object_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateData<
            Identity: ID3D12Object_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12Object_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: ID3D12Object_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12Object_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetName<Identity: ID3D12Object_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            name: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12Object_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Object as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D12Object {}
windows_core::imp::define_interface!(
    ID3D12Pageable,
    ID3D12Pageable_Vtbl,
    0x63ee58fb_1268_4835_86da_f008ce62f0d6
);
impl core::ops::Deref for ID3D12Pageable {
    type Target = ID3D12DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D12Pageable,
    windows_core::IUnknown,
    ID3D12Object,
    ID3D12DeviceChild
);
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Pageable_Vtbl {
    pub base__: ID3D12DeviceChild_Vtbl,
}
unsafe impl Send for ID3D12Pageable {}
unsafe impl Sync for ID3D12Pageable {}
pub trait ID3D12Pageable_Impl: ID3D12DeviceChild_Impl {}
impl ID3D12Pageable_Vtbl {
    pub const fn new<Identity: ID3D12Pageable_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID3D12DeviceChild_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Pageable as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D12Pageable {}
windows_core::imp::define_interface!(
    ID3D12Resource,
    ID3D12Resource_Vtbl,
    0x696442be_a72e_4059_bc79_5b5c98040fad
);
impl core::ops::Deref for ID3D12Resource {
    type Target = ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D12Resource,
    windows_core::IUnknown,
    ID3D12Object,
    ID3D12DeviceChild,
    ID3D12Pageable
);
impl ID3D12Resource {
    pub unsafe fn GetGPUVirtualAddress(&self) -> u64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetGPUVirtualAddress)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12Resource_Vtbl {
    pub base__: ID3D12Pageable_Vtbl,
    Map: usize,
    Unmap: usize,
    GetDesc: usize,
    pub GetGPUVirtualAddress: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    WriteToSubresource: usize,
    ReadFromSubresource: usize,
    GetHeapProperties: usize,
}
unsafe impl Send for ID3D12Resource {}
unsafe impl Sync for ID3D12Resource {}
pub trait ID3D12Resource_Impl: ID3D12Pageable_Impl {
    fn GetGPUVirtualAddress(&self) -> u64;
}
impl ID3D12Resource_Vtbl {
    pub const fn new<Identity: ID3D12Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGPUVirtualAddress<
            Identity: ID3D12Resource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12Resource_Impl::GetGPUVirtualAddress(this)
            }
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            Map: 0,
            Unmap: 0,
            GetDesc: 0,
            GetGPUVirtualAddress: GetGPUVirtualAddress::<Identity, OFFSET>,
            WriteToSubresource: 0,
            ReadFromSubresource: 0,
            GetHeapProperties: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Resource as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
            || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D12Resource {}
