#[cfg(feature = "d3d12")]
#[inline]
pub unsafe fn MFCreateD3D12SynchronizationObject<P0, T>(pdevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::ID3D12Device>,
    T: windows_core::Interface,
{
    windows_core::link!("mfplat.dll" "C" fn MFCreateD3D12SynchronizationObject(pdevice : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppvsyncobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateD3D12SynchronizationObject(pdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
windows_core::imp::define_interface!(IMFD3D12SynchronizationObject, IMFD3D12SynchronizationObject_Vtbl, 0x802302b0_82de_45e1_b421_f19ee5bdaf23);
windows_core::imp::interface_hierarchy!(IMFD3D12SynchronizationObject, windows_core::IUnknown);
impl IMFD3D12SynchronizationObject {
    #[cfg(feature = "winnt")]
    pub unsafe fn SignalEventOnFinalResourceRelease(&self, hevent: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SignalEventOnFinalResourceRelease)(windows_core::Interface::as_raw(self), hevent) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFD3D12SynchronizationObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub SignalEventOnFinalResourceRelease: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SignalEventOnFinalResourceRelease: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IMFD3D12SynchronizationObject_Impl: windows_core::IUnknownImpl {
    fn SignalEventOnFinalResourceRelease(&self, hevent: super::HANDLE) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IMFD3D12SynchronizationObject_Vtbl {
    pub const fn new<Identity: IMFD3D12SynchronizationObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SignalEventOnFinalResourceRelease<Identity: IMFD3D12SynchronizationObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObject_Impl::SignalEventOnFinalResourceRelease(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMFD3D12SynchronizationObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObject_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SignalEventOnFinalResourceRelease: SignalEventOnFinalResourceRelease::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFD3D12SynchronizationObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IMFD3D12SynchronizationObject {}
windows_core::imp::define_interface!(IMFD3D12SynchronizationObjectCommands, IMFD3D12SynchronizationObjectCommands_Vtbl, 0x09d0f835_92ff_4e53_8efa_40faa551f233);
windows_core::imp::interface_hierarchy!(IMFD3D12SynchronizationObjectCommands, windows_core::IUnknown);
impl IMFD3D12SynchronizationObjectCommands {
    #[cfg(feature = "d3d12")]
    pub unsafe fn EnqueueResourceReady<P0>(&self, pproducercommandqueue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnqueueResourceReady)(windows_core::Interface::as_raw(self), pproducercommandqueue.param().abi()) }
    }
    #[cfg(feature = "d3d12")]
    pub unsafe fn EnqueueResourceReadyWait<P0>(&self, pconsumercommandqueue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnqueueResourceReadyWait)(windows_core::Interface::as_raw(self), pconsumercommandqueue.param().abi()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SignalEventOnResourceReady(&self, hevent: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SignalEventOnResourceReady)(windows_core::Interface::as_raw(self), hevent) }
    }
    #[cfg(feature = "d3d12")]
    pub unsafe fn EnqueueResourceRelease<P0>(&self, pconsumercommandqueue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnqueueResourceRelease)(windows_core::Interface::as_raw(self), pconsumercommandqueue.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFD3D12SynchronizationObjectCommands_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "d3d12")]
    pub EnqueueResourceReady: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "d3d12"))]
    EnqueueResourceReady: usize,
    #[cfg(feature = "d3d12")]
    pub EnqueueResourceReadyWait: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "d3d12"))]
    EnqueueResourceReadyWait: usize,
    #[cfg(feature = "winnt")]
    pub SignalEventOnResourceReady: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SignalEventOnResourceReady: usize,
    #[cfg(feature = "d3d12")]
    pub EnqueueResourceRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "d3d12"))]
    EnqueueResourceRelease: usize,
}
#[cfg(all(feature = "d3d12", feature = "winnt"))]
pub trait IMFD3D12SynchronizationObjectCommands_Impl: windows_core::IUnknownImpl {
    fn EnqueueResourceReady(&self, pproducercommandqueue: windows_core::Ref<super::ID3D12CommandQueue>) -> windows_core::Result<()>;
    fn EnqueueResourceReadyWait(&self, pconsumercommandqueue: windows_core::Ref<super::ID3D12CommandQueue>) -> windows_core::Result<()>;
    fn SignalEventOnResourceReady(&self, hevent: super::HANDLE) -> windows_core::Result<()>;
    fn EnqueueResourceRelease(&self, pconsumercommandqueue: windows_core::Ref<super::ID3D12CommandQueue>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "d3d12", feature = "winnt"))]
impl IMFD3D12SynchronizationObjectCommands_Vtbl {
    pub const fn new<Identity: IMFD3D12SynchronizationObjectCommands_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnqueueResourceReady<Identity: IMFD3D12SynchronizationObjectCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproducercommandqueue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObjectCommands_Impl::EnqueueResourceReady(this, core::mem::transmute_copy(&pproducercommandqueue)).into()
            }
        }
        unsafe extern "system" fn EnqueueResourceReadyWait<Identity: IMFD3D12SynchronizationObjectCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconsumercommandqueue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObjectCommands_Impl::EnqueueResourceReadyWait(this, core::mem::transmute_copy(&pconsumercommandqueue)).into()
            }
        }
        unsafe extern "system" fn SignalEventOnResourceReady<Identity: IMFD3D12SynchronizationObjectCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObjectCommands_Impl::SignalEventOnResourceReady(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        unsafe extern "system" fn EnqueueResourceRelease<Identity: IMFD3D12SynchronizationObjectCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconsumercommandqueue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFD3D12SynchronizationObjectCommands_Impl::EnqueueResourceRelease(this, core::mem::transmute_copy(&pconsumercommandqueue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnqueueResourceReady: EnqueueResourceReady::<Identity, OFFSET>,
            EnqueueResourceReadyWait: EnqueueResourceReadyWait::<Identity, OFFSET>,
            SignalEventOnResourceReady: SignalEventOnResourceReady::<Identity, OFFSET>,
            EnqueueResourceRelease: EnqueueResourceRelease::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFD3D12SynchronizationObjectCommands as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "d3d12", feature = "winnt"))]
impl windows_core::RuntimeName for IMFD3D12SynchronizationObjectCommands {}
pub const MF_D3D11_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 0;
pub const MF_D3D12_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 1;
pub type MF_MT_D3D_RESOURCE_VERSION_ENUM = i32;
