#[cfg(all(feature = "objidlbase", feature = "shared"))]
#[inline]
pub unsafe fn MFDeserializeAttributesFromStream<P0, P2>(pattr: P0, dwoptions: super::DWORD, pstm: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<IMFAttributes>,
    P2: windows_core::Param<super::IStream>,
{
    windows_core::link!("mfplat.dll" "system" fn MFDeserializeAttributesFromStream(pattr : *mut core::ffi::c_void, dwoptions : super::DWORD, pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFDeserializeAttributesFromStream(pattr.param().abi(), dwoptions, pstm.param().abi()) }
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
#[inline]
pub unsafe fn MFSerializeAttributesToStream<P0, P2>(pattr: P0, dwoptions: super::DWORD, pstm: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<IMFAttributes>,
    P2: windows_core::Param<super::IStream>,
{
    windows_core::link!("mfplat.dll" "system" fn MFSerializeAttributesToStream(pattr : *mut core::ffi::c_void, dwoptions : super::DWORD, pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFSerializeAttributesToStream(pattr.param().abi(), dwoptions, pstm.param().abi()) }
}
pub const CLSID_MFByteStreamProxyClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0x770e8e77_4916_441c_a9a7_b342d0eebc71);
windows_core::imp::define_interface!(IMF2DBuffer, IMF2DBuffer_Vtbl, 0x7dc9d5f9_9ed9_44ec_9bbf_0600bb589fbb);
windows_core::imp::interface_hierarchy!(IMF2DBuffer, windows_core::IUnknown);
impl IMF2DBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn Lock2D(&self, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock2D)(windows_core::Interface::as_raw(self), ppbscanline0 as _, plpitch as _) }
    }
    pub unsafe fn Unlock2D(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock2D)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetScanline0AndPitch(&self, pbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScanline0AndPitch)(windows_core::Interface::as_raw(self), pbscanline0 as _, plpitch as _) }
    }
    pub unsafe fn IsContiguousFormat(&self, pfiscontiguous: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsContiguousFormat)(windows_core::Interface::as_raw(self), pfiscontiguous as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetContiguousLength(&self, pcblength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContiguousLength)(windows_core::Interface::as_raw(self), pcblength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ContiguousCopyTo(&self, pbdestbuffer: *mut super::BYTE, cbdestbuffer: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContiguousCopyTo)(windows_core::Interface::as_raw(self), pbdestbuffer as _, cbdestbuffer) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ContiguousCopyFrom(&self, pbsrcbuffer: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ContiguousCopyFrom)(windows_core::Interface::as_raw(self), core::mem::transmute(pbsrcbuffer.as_ptr()), pbsrcbuffer.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMF2DBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Lock2D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Lock2D: usize,
    pub Unlock2D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetScanline0AndPitch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetScanline0AndPitch: usize,
    pub IsContiguousFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetContiguousLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetContiguousLength: usize,
    #[cfg(feature = "shared")]
    pub ContiguousCopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ContiguousCopyTo: usize,
    #[cfg(feature = "shared")]
    pub ContiguousCopyFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ContiguousCopyFrom: usize,
}
#[cfg(feature = "shared")]
pub trait IMF2DBuffer_Impl: windows_core::IUnknownImpl {
    fn Lock2D(&self, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::Result<()>;
    fn Unlock2D(&self) -> windows_core::Result<()>;
    fn GetScanline0AndPitch(&self, pbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::Result<()>;
    fn IsContiguousFormat(&self, pfiscontiguous: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetContiguousLength(&self, pcblength: *mut super::DWORD) -> windows_core::Result<()>;
    fn ContiguousCopyTo(&self, pbdestbuffer: *mut super::BYTE, cbdestbuffer: super::DWORD) -> windows_core::Result<()>;
    fn ContiguousCopyFrom(&self, pbsrcbuffer: *const super::BYTE, cbsrcbuffer: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMF2DBuffer_Vtbl {
    pub const fn new<Identity: IMF2DBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock2D<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::Lock2D(this, core::mem::transmute_copy(&ppbscanline0), core::mem::transmute_copy(&plpitch)).into()
            }
        }
        unsafe extern "system" fn Unlock2D<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::Unlock2D(this).into()
            }
        }
        unsafe extern "system" fn GetScanline0AndPitch<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::GetScanline0AndPitch(this, core::mem::transmute_copy(&pbscanline0), core::mem::transmute_copy(&plpitch)).into()
            }
        }
        unsafe extern "system" fn IsContiguousFormat<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscontiguous: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::IsContiguousFormat(this, core::mem::transmute_copy(&pfiscontiguous)).into()
            }
        }
        unsafe extern "system" fn GetContiguousLength<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcblength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::GetContiguousLength(this, core::mem::transmute_copy(&pcblength)).into()
            }
        }
        unsafe extern "system" fn ContiguousCopyTo<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdestbuffer: *mut super::BYTE, cbdestbuffer: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::ContiguousCopyTo(this, core::mem::transmute_copy(&pbdestbuffer), core::mem::transmute_copy(&cbdestbuffer)).into()
            }
        }
        unsafe extern "system" fn ContiguousCopyFrom<Identity: IMF2DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsrcbuffer: *const super::BYTE, cbsrcbuffer: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer_Impl::ContiguousCopyFrom(this, core::mem::transmute_copy(&pbsrcbuffer), core::mem::transmute_copy(&cbsrcbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Lock2D: Lock2D::<Identity, OFFSET>,
            Unlock2D: Unlock2D::<Identity, OFFSET>,
            GetScanline0AndPitch: GetScanline0AndPitch::<Identity, OFFSET>,
            IsContiguousFormat: IsContiguousFormat::<Identity, OFFSET>,
            GetContiguousLength: GetContiguousLength::<Identity, OFFSET>,
            ContiguousCopyTo: ContiguousCopyTo::<Identity, OFFSET>,
            ContiguousCopyFrom: ContiguousCopyFrom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMF2DBuffer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMF2DBuffer {}
windows_core::imp::define_interface!(IMF2DBuffer2, IMF2DBuffer2_Vtbl, 0x33ae5ea6_4316_436f_8ddd_d73d22f829ec);
impl core::ops::Deref for IMF2DBuffer2 {
    type Target = IMF2DBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMF2DBuffer2, windows_core::IUnknown, IMF2DBuffer);
impl IMF2DBuffer2 {
    #[cfg(feature = "shared")]
    pub unsafe fn Lock2DSize(&self, lockflags: MF2DBuffer_LockFlags, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG, ppbbufferstart: *mut *mut super::BYTE, pcbbufferlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock2DSize)(windows_core::Interface::as_raw(self), lockflags, ppbscanline0 as _, plpitch as _, ppbbufferstart as _, pcbbufferlength as _) }
    }
    pub unsafe fn Copy2DTo<P0>(&self, pdestbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy2DTo)(windows_core::Interface::as_raw(self), pdestbuffer.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMF2DBuffer2_Vtbl {
    pub base__: IMF2DBuffer_Vtbl,
    #[cfg(feature = "shared")]
    pub Lock2DSize: unsafe extern "system" fn(*mut core::ffi::c_void, MF2DBuffer_LockFlags, *mut *mut super::BYTE, *mut super::LONG, *mut *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Lock2DSize: usize,
    pub Copy2DTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IMF2DBuffer2_Impl: IMF2DBuffer_Impl {
    fn Lock2DSize(&self, lockflags: MF2DBuffer_LockFlags, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG, ppbbufferstart: *mut *mut super::BYTE, pcbbufferlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn Copy2DTo(&self, pdestbuffer: windows_core::Ref<IMF2DBuffer2>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMF2DBuffer2_Vtbl {
    pub const fn new<Identity: IMF2DBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock2DSize<Identity: IMF2DBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lockflags: MF2DBuffer_LockFlags, ppbscanline0: *mut *mut super::BYTE, plpitch: *mut super::LONG, ppbbufferstart: *mut *mut super::BYTE, pcbbufferlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer2_Impl::Lock2DSize(this, core::mem::transmute_copy(&lockflags), core::mem::transmute_copy(&ppbscanline0), core::mem::transmute_copy(&plpitch), core::mem::transmute_copy(&ppbbufferstart), core::mem::transmute_copy(&pcbbufferlength)).into()
            }
        }
        unsafe extern "system" fn Copy2DTo<Identity: IMF2DBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMF2DBuffer2_Impl::Copy2DTo(this, core::mem::transmute_copy(&pdestbuffer)).into()
            }
        }
        Self { base__: IMF2DBuffer_Vtbl::new::<Identity, OFFSET>(), Lock2DSize: Lock2DSize::<Identity, OFFSET>, Copy2DTo: Copy2DTo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMF2DBuffer2 as windows_core::Interface>::IID || iid == &<IMF2DBuffer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMF2DBuffer2 {}
windows_core::imp::define_interface!(IMFActivate, IMFActivate_Vtbl, 0x7fee9e9a_4a89_47a6_899c_b6a53a70fb67);
impl core::ops::Deref for IMFActivate {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFActivate, windows_core::IUnknown, IMFAttributes);
impl IMFActivate {
    #[cfg(feature = "guiddef")]
    pub unsafe fn ActivateObject<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ActivateObject)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    pub unsafe fn ShutdownObject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DetachObject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetachObject)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFActivate_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    #[cfg(feature = "guiddef")]
    pub ActivateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    ActivateObject: usize,
    pub ShutdownObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFActivate_Impl: IMFAttributes_Impl {
    fn ActivateObject(&self, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShutdownObject(&self) -> windows_core::Result<()>;
    fn DetachObject(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFActivate_Vtbl {
    pub const fn new<Identity: IMFActivate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateObject<Identity: IMFActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFActivate_Impl::ActivateObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn ShutdownObject<Identity: IMFActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFActivate_Impl::ShutdownObject(this).into()
            }
        }
        unsafe extern "system" fn DetachObject<Identity: IMFActivate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFActivate_Impl::DetachObject(this).into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            ActivateObject: ActivateObject::<Identity, OFFSET>,
            ShutdownObject: ShutdownObject::<Identity, OFFSET>,
            DetachObject: DetachObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFActivate as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFActivate {}
windows_core::imp::define_interface!(IMFAsyncCallback, IMFAsyncCallback_Vtbl, 0xa27003cf_2354_4f2a_8d6a_ab7cff15437e);
windows_core::imp::interface_hierarchy!(IMFAsyncCallback, windows_core::IUnknown);
impl IMFAsyncCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn GetParameters(&self, pdwflags: *mut super::DWORD, pdwqueue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pdwflags as _, pdwqueue as _) }
    }
    pub unsafe fn Invoke<P0>(&self, pasyncresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pasyncresult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetParameters: usize,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IMFAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn GetParameters(&self, pdwflags: *mut super::DWORD, pdwqueue: *mut super::DWORD) -> windows_core::Result<()>;
    fn Invoke(&self, pasyncresult: windows_core::Ref<IMFAsyncResult>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFAsyncCallback_Vtbl {
    pub const fn new<Identity: IMFAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameters<Identity: IMFAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut super::DWORD, pdwqueue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncCallback_Impl::GetParameters(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwqueue)).into()
            }
        }
        unsafe extern "system" fn Invoke<Identity: IMFAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncCallback_Impl::Invoke(this, core::mem::transmute_copy(&pasyncresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAsyncCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFAsyncCallback {}
windows_core::imp::define_interface!(IMFAsyncCallbackLogging, IMFAsyncCallbackLogging_Vtbl, 0xc7a4dca1_f5f0_47b6_b92b_bf0106d25791);
impl core::ops::Deref for IMFAsyncCallbackLogging {
    type Target = IMFAsyncCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFAsyncCallbackLogging, windows_core::IUnknown, IMFAsyncCallback);
impl IMFAsyncCallbackLogging {
    pub unsafe fn GetObjectPointer(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).GetObjectPointer)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetObjectTag(&self) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).GetObjectTag)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAsyncCallbackLogging_Vtbl {
    pub base__: IMFAsyncCallback_Vtbl,
    pub GetObjectPointer: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    #[cfg(feature = "shared")]
    pub GetObjectTag: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    GetObjectTag: usize,
}
#[cfg(feature = "shared")]
pub trait IMFAsyncCallbackLogging_Impl: IMFAsyncCallback_Impl {
    fn GetObjectPointer(&self) -> *mut core::ffi::c_void;
    fn GetObjectTag(&self) -> super::DWORD;
}
#[cfg(feature = "shared")]
impl IMFAsyncCallbackLogging_Vtbl {
    pub const fn new<Identity: IMFAsyncCallbackLogging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectPointer<Identity: IMFAsyncCallbackLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncCallbackLogging_Impl::GetObjectPointer(this)
            }
        }
        unsafe extern "system" fn GetObjectTag<Identity: IMFAsyncCallbackLogging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncCallbackLogging_Impl::GetObjectTag(this)
            }
        }
        Self {
            base__: IMFAsyncCallback_Vtbl::new::<Identity, OFFSET>(),
            GetObjectPointer: GetObjectPointer::<Identity, OFFSET>,
            GetObjectTag: GetObjectTag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAsyncCallbackLogging as windows_core::Interface>::IID || iid == &<IMFAsyncCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFAsyncCallbackLogging {}
windows_core::imp::define_interface!(IMFAsyncResult, IMFAsyncResult_Vtbl, 0xac6b7889_0740_4d51_8619_905994a55cc6);
windows_core::imp::interface_hierarchy!(IMFAsyncResult, windows_core::IUnknown);
impl IMFAsyncResult {
    pub unsafe fn GetState(&self, ppunkstate: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), core::mem::transmute(ppunkstate)) }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), hrstatus) }
    }
    pub unsafe fn GetObject(&self, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), core::mem::transmute(ppobject)) }
    }
    pub unsafe fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown> {
        unsafe { (windows_core::Interface::vtable(self).GetStateNoAddRef)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStateNoAddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<windows_core::IUnknown>,
}
pub trait IMFAsyncResult_Impl: windows_core::IUnknownImpl {
    fn GetState(&self, ppunkstate: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<()>;
    fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetObject(&self, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown>;
}
impl IMFAsyncResult_Vtbl {
    pub const fn new<Identity: IMFAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IMFAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncResult_Impl::GetState(this, core::mem::transmute_copy(&ppunkstate)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IMFAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncResult_Impl::GetStatus(this).into()
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IMFAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncResult_Impl::SetStatus(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IMFAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncResult_Impl::GetObject(this, core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn GetStateNoAddRef<Identity: IMFAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<windows_core::IUnknown> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAsyncResult_Impl::GetStateNoAddRef(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAsyncResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFAsyncResult {}
windows_core::imp::define_interface!(IMFAttributes, IMFAttributes_Vtbl, 0x2cd2d921_c447_44a7_a13c_4adabfc247e3);
windows_core::imp::interface_hierarchy!(IMFAttributes, windows_core::IUnknown);
impl IMFAttributes {
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetItem(&self, guidkey: *const windows_core::GUID, pvalue: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), guidkey, pvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetItemType(&self, guidkey: *const windows_core::GUID, ptype: *mut MF_ATTRIBUTE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), guidkey, ptype as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CompareItem(&self, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT, pbresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompareItem)(windows_core::Interface::as_raw(self), guidkey, value, pbresult as _) }
    }
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE, pbresult: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), ptheirs.param().abi(), matchtype, pbresult as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUINT32(&self, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUINT32)(windows_core::Interface::as_raw(self), guidkey, punvalue as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetUINT64(&self, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUINT64)(windows_core::Interface::as_raw(self), guidkey, punvalue as _) }
    }
    pub unsafe fn GetDouble(&self, guidkey: *const windows_core::GUID, pfvalue: *mut f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDouble)(windows_core::Interface::as_raw(self), guidkey, pfvalue as _) }
    }
    pub unsafe fn GetGUID(&self, guidkey: *const windows_core::GUID, pguidvalue: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), guidkey, pguidvalue as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStringLength(&self, guidkey: *const windows_core::GUID, pcchlength: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), guidkey, pcchlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetString(&self, guidkey: *const windows_core::GUID, pwszvalue: windows_core::PWSTR, cchbufsize: super::UINT32, pcchlength: Option<*mut super::UINT32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), guidkey, pwszvalue, cchbufsize, pcchlength.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocatedString(&self, guidkey: *const windows_core::GUID, ppwszvalue: *mut windows_core::PWSTR, pcchlength: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllocatedString)(windows_core::Interface::as_raw(self), guidkey, ppwszvalue as _, pcchlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBlobSize(&self, guidkey: *const windows_core::GUID, pcbblobsize: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBlobSize)(windows_core::Interface::as_raw(self), guidkey, pcbblobsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBlob(&self, guidkey: *const windows_core::GUID, pbuf: *mut super::UINT8, cbbufsize: super::UINT32, pcbblobsize: Option<*mut super::UINT32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBlob)(windows_core::Interface::as_raw(self), guidkey, pbuf as _, cbbufsize, pcbblobsize.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const windows_core::GUID, ppbuf: *mut *mut super::UINT8, pcbsize: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllocatedBlob)(windows_core::Interface::as_raw(self), guidkey, ppbuf as _, pcbsize as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetUnknown)(windows_core::Interface::as_raw(self), guidkey, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetItem(&self, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetItem)(windows_core::Interface::as_raw(self), guidkey, value) }
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), guidkey) }
    }
    pub unsafe fn DeleteAllItems(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAllItems)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetUINT32(&self, guidkey: *const windows_core::GUID, unvalue: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUINT32)(windows_core::Interface::as_raw(self), guidkey, unvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetUINT64(&self, guidkey: *const windows_core::GUID, unvalue: super::UINT64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUINT64)(windows_core::Interface::as_raw(self), guidkey, unvalue) }
    }
    pub unsafe fn SetDouble(&self, guidkey: *const windows_core::GUID, fvalue: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDouble)(windows_core::Interface::as_raw(self), guidkey, fvalue) }
    }
    pub unsafe fn SetGUID(&self, guidkey: *const windows_core::GUID, guidvalue: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), guidkey, guidvalue) }
    }
    pub unsafe fn SetString<P1>(&self, guidkey: *const windows_core::GUID, wszvalue: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetString)(windows_core::Interface::as_raw(self), guidkey, wszvalue.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetBlob(&self, guidkey: *const windows_core::GUID, pbuf: &[super::UINT8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBlob)(windows_core::Interface::as_raw(self), guidkey, pbuf.as_ptr(), pbuf.len().try_into().unwrap()) }
    }
    pub unsafe fn SetUnknown<P1>(&self, guidkey: *const windows_core::GUID, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUnknown)(windows_core::Interface::as_raw(self), guidkey, punknown.param().abi()) }
    }
    pub unsafe fn LockStore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockStore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UnlockStore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockStore)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcitems: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcitems as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetItemByIndex(&self, unindex: super::UINT32, pguidkey: *mut windows_core::GUID, pvalue: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetItemByIndex)(windows_core::Interface::as_raw(self), unindex, pguidkey as _, pvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyAllItems)(windows_core::Interface::as_raw(self), pdest.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetItem: usize,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut MF_ATTRIBUTE_TYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CompareItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CompareItem: usize,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, MF_ATTRIBUTES_MATCH_TYPE, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetUINT32: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUINT32: usize,
    #[cfg(feature = "shared")]
    pub GetUINT64: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::UINT64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetUINT64: usize,
    pub GetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut f64) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStringLength: usize,
    #[cfg(feature = "shared")]
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PWSTR, super::UINT32, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetString: usize,
    #[cfg(feature = "shared")]
    pub GetAllocatedString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::PWSTR, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocatedString: usize,
    #[cfg(feature = "shared")]
    pub GetBlobSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBlobSize: usize,
    #[cfg(feature = "shared")]
    pub GetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::UINT8, super::UINT32, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBlob: usize,
    #[cfg(feature = "shared")]
    pub GetAllocatedBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut super::UINT8, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllocatedBlob: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetUnknown: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub SetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    SetItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetUINT32: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetUINT32: usize,
    #[cfg(feature = "shared")]
    pub SetUINT64: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::UINT64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetUINT64: usize,
    pub SetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, f64) -> windows_core::HRESULT,
    pub SetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::UINT8, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetBlob: usize,
    pub SetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetItemByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetItemByIndex: usize,
    pub CopyAllItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFAttributes_Impl: windows_core::IUnknownImpl {
    fn GetItem(&self, guidkey: *const windows_core::GUID, pvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetItemType(&self, guidkey: *const windows_core::GUID, ptype: *mut MF_ATTRIBUTE_TYPE) -> windows_core::Result<()>;
    fn CompareItem(&self, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT, pbresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Compare(&self, ptheirs: windows_core::Ref<IMFAttributes>, matchtype: MF_ATTRIBUTES_MATCH_TYPE, pbresult: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetUINT32(&self, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetUINT64(&self, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT64) -> windows_core::Result<()>;
    fn GetDouble(&self, guidkey: *const windows_core::GUID, pfvalue: *mut f64) -> windows_core::Result<()>;
    fn GetGUID(&self, guidkey: *const windows_core::GUID, pguidvalue: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetStringLength(&self, guidkey: *const windows_core::GUID, pcchlength: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetString(&self, guidkey: *const windows_core::GUID, pwszvalue: windows_core::PWSTR, cchbufsize: super::UINT32, pcchlength: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetAllocatedString(&self, guidkey: *const windows_core::GUID, ppwszvalue: *mut windows_core::PWSTR, pcchlength: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetBlobSize(&self, guidkey: *const windows_core::GUID, pcbblobsize: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetBlob(&self, guidkey: *const windows_core::GUID, pbuf: *mut super::UINT8, cbbufsize: super::UINT32, pcbblobsize: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetAllocatedBlob(&self, guidkey: *const windows_core::GUID, ppbuf: *mut *mut super::UINT8, pcbsize: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetUnknown(&self, guidkey: *const windows_core::GUID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::Result<()>;
    fn SetItem(&self, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn DeleteItem(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteAllItems(&self) -> windows_core::Result<()>;
    fn SetUINT32(&self, guidkey: *const windows_core::GUID, unvalue: super::UINT32) -> windows_core::Result<()>;
    fn SetUINT64(&self, guidkey: *const windows_core::GUID, unvalue: super::UINT64) -> windows_core::Result<()>;
    fn SetDouble(&self, guidkey: *const windows_core::GUID, fvalue: f64) -> windows_core::Result<()>;
    fn SetGUID(&self, guidkey: *const windows_core::GUID, guidvalue: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetString(&self, guidkey: *const windows_core::GUID, wszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBlob(&self, guidkey: *const windows_core::GUID, pbuf: *const super::UINT8, cbbufsize: super::UINT32) -> windows_core::Result<()>;
    fn SetUnknown(&self, guidkey: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LockStore(&self) -> windows_core::Result<()>;
    fn UnlockStore(&self) -> windows_core::Result<()>;
    fn GetCount(&self, pcitems: *mut super::UINT32) -> windows_core::Result<()>;
    fn GetItemByIndex(&self, unindex: super::UINT32, pguidkey: *mut windows_core::GUID, pvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn CopyAllItems(&self, pdest: windows_core::Ref<IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFAttributes_Vtbl {
    pub const fn new<Identity: IMFAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItem(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, ptype: *mut MF_ATTRIBUTE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItemType(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn CompareItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT, pbresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::CompareItem(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&value), core::mem::transmute_copy(&pbresult)).into()
            }
        }
        unsafe extern "system" fn Compare<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptheirs: *mut core::ffi::c_void, matchtype: MF_ATTRIBUTES_MATCH_TYPE, pbresult: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::Compare(this, core::mem::transmute_copy(&ptheirs), core::mem::transmute_copy(&matchtype), core::mem::transmute_copy(&pbresult)).into()
            }
        }
        unsafe extern "system" fn GetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetUINT32(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&punvalue)).into()
            }
        }
        unsafe extern "system" fn GetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, punvalue: *mut super::UINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetUINT64(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&punvalue)).into()
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pfvalue: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetDouble(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pfvalue)).into()
            }
        }
        unsafe extern "system" fn GetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pguidvalue: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetGUID(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pguidvalue)).into()
            }
        }
        unsafe extern "system" fn GetStringLength<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pcchlength: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetStringLength(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pcchlength)).into()
            }
        }
        unsafe extern "system" fn GetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pwszvalue: windows_core::PWSTR, cchbufsize: super::UINT32, pcchlength: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetString(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pwszvalue), core::mem::transmute_copy(&cchbufsize), core::mem::transmute_copy(&pcchlength)).into()
            }
        }
        unsafe extern "system" fn GetAllocatedString<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, ppwszvalue: *mut windows_core::PWSTR, pcchlength: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedString(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&ppwszvalue), core::mem::transmute_copy(&pcchlength)).into()
            }
        }
        unsafe extern "system" fn GetBlobSize<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pcbblobsize: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetBlobSize(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pcbblobsize)).into()
            }
        }
        unsafe extern "system" fn GetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pbuf: *mut super::UINT8, cbbufsize: super::UINT32, pcbblobsize: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetBlob(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&cbbufsize), core::mem::transmute_copy(&pcbblobsize)).into()
            }
        }
        unsafe extern "system" fn GetAllocatedBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, ppbuf: *mut *mut super::UINT8, pcbsize: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedBlob(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&ppbuf), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetUnknown(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn SetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, value: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetItem(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteItem(this, core::mem::transmute_copy(&guidkey)).into()
            }
        }
        unsafe extern "system" fn DeleteAllItems<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteAllItems(this).into()
            }
        }
        unsafe extern "system" fn SetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, unvalue: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT32(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&unvalue)).into()
            }
        }
        unsafe extern "system" fn SetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, unvalue: super::UINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT64(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&unvalue)).into()
            }
        }
        unsafe extern "system" fn SetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, fvalue: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetDouble(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&fvalue)).into()
            }
        }
        unsafe extern "system" fn SetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, guidvalue: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetGUID(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&guidvalue)).into()
            }
        }
        unsafe extern "system" fn SetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, wszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetString(this, core::mem::transmute_copy(&guidkey), core::mem::transmute(&wszvalue)).into()
            }
        }
        unsafe extern "system" fn SetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, pbuf: *const super::UINT8, cbbufsize: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetBlob(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&cbbufsize)).into()
            }
        }
        unsafe extern "system" fn SetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidkey: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUnknown(this, core::mem::transmute_copy(&guidkey), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn LockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::LockStore(this).into()
            }
        }
        unsafe extern "system" fn UnlockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::UnlockStore(this).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcitems: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetCount(this, core::mem::transmute_copy(&pcitems)).into()
            }
        }
        unsafe extern "system" fn GetItemByIndex<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unindex: super::UINT32, pguidkey: *mut windows_core::GUID, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItemByIndex(this, core::mem::transmute_copy(&unindex), core::mem::transmute_copy(&pguidkey), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn CopyAllItems<Identity: IMFAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::CopyAllItems(this, core::mem::transmute_copy(&pdest)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            CompareItem: CompareItem::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            GetUINT32: GetUINT32::<Identity, OFFSET>,
            GetUINT64: GetUINT64::<Identity, OFFSET>,
            GetDouble: GetDouble::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetAllocatedString: GetAllocatedString::<Identity, OFFSET>,
            GetBlobSize: GetBlobSize::<Identity, OFFSET>,
            GetBlob: GetBlob::<Identity, OFFSET>,
            GetAllocatedBlob: GetAllocatedBlob::<Identity, OFFSET>,
            GetUnknown: GetUnknown::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            DeleteAllItems: DeleteAllItems::<Identity, OFFSET>,
            SetUINT32: SetUINT32::<Identity, OFFSET>,
            SetUINT64: SetUINT64::<Identity, OFFSET>,
            SetDouble: SetDouble::<Identity, OFFSET>,
            SetGUID: SetGUID::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            SetBlob: SetBlob::<Identity, OFFSET>,
            SetUnknown: SetUnknown::<Identity, OFFSET>,
            LockStore: LockStore::<Identity, OFFSET>,
            UnlockStore: UnlockStore::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetItemByIndex: GetItemByIndex::<Identity, OFFSET>,
            CopyAllItems: CopyAllItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFAttributes {}
windows_core::imp::define_interface!(IMFAudioMediaType, IMFAudioMediaType_Vtbl, 0x26a0adc3_ce26_4672_9304_69552edd3faf);
impl core::ops::Deref for IMFAudioMediaType {
    type Target = IMFMediaType;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFAudioMediaType, windows_core::IUnknown, IMFAttributes, IMFMediaType);
impl IMFAudioMediaType {
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn GetAudioFormat(&self) -> *const super::WAVEFORMATEX {
        unsafe { (windows_core::Interface::vtable(self).GetAudioFormat)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAudioMediaType_Vtbl {
    pub base__: IMFMediaType_Vtbl,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub GetAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void) -> *const super::WAVEFORMATEX,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    GetAudioFormat: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFAudioMediaType_Impl: IMFMediaType_Impl {
    fn GetAudioFormat(&self) -> *const super::WAVEFORMATEX;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFAudioMediaType_Vtbl {
    pub const fn new<Identity: IMFAudioMediaType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAudioFormat<Identity: IMFAudioMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *const super::WAVEFORMATEX {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioMediaType_Impl::GetAudioFormat(this)
            }
        }
        Self { base__: IMFMediaType_Vtbl::new::<Identity, OFFSET>(), GetAudioFormat: GetAudioFormat::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAudioMediaType as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID || iid == &<IMFMediaType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFAudioMediaType {}
windows_core::imp::define_interface!(IMFByteStream, IMFByteStream_Vtbl, 0xad4c1b00_4bf7_422f_9175_756693d9130d);
windows_core::imp::interface_hierarchy!(IMFByteStream, windows_core::IUnknown);
impl IMFByteStream {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCapabilities(&self, pdwcapabilities: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), pdwcapabilities as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLength(&self, pqwlength: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self), pqwlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetLength(&self, qwlength: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), qwlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentPosition(&self, pqwposition: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), pqwposition as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentPosition(&self, qwposition: super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), qwposition) }
    }
    pub unsafe fn IsEndOfStream(&self, pfendofstream: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsEndOfStream)(windows_core::Interface::as_raw(self), pfendofstream as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, pb: *mut super::BYTE, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pb as _, cb, pcbread as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn BeginRead<P2, P3>(&self, pb: *mut super::BYTE, cb: super::ULONG, pcallback: P2, punkstate: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginRead)(windows_core::Interface::as_raw(self), pb as _, cb, pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EndRead<P0>(&self, presult: P0, pcbread: *mut super::ULONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndRead)(windows_core::Interface::as_raw(self), presult.param().abi(), pcbread as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Write(&self, pb: &[super::BYTE], pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pb.as_ptr(), pb.len().try_into().unwrap(), pcbwritten as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn BeginWrite<P2, P3>(&self, pb: &[u8], pcallback: P2, punkstate: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginWrite)(windows_core::Interface::as_raw(self), core::mem::transmute(pb.as_ptr()), pb.len().try_into().unwrap(), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EndWrite<P0>(&self, presult: P0, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndWrite)(windows_core::Interface::as_raw(self), presult.param().abi(), pcbwritten as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Seek(&self, seekorigin: MFBYTESTREAM_SEEK_ORIGIN, llseekoffset: super::LONGLONG, dwseekflags: super::DWORD, pqwcurrentposition: *mut super::QWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), seekorigin, llseekoffset, dwseekflags, pqwcurrentposition as _) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCapabilities: usize,
    #[cfg(feature = "shared")]
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLength: usize,
    #[cfg(feature = "shared")]
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetLength: usize,
    #[cfg(feature = "shared")]
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentPosition: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentPosition: usize,
    pub IsEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
    #[cfg(feature = "shared")]
    pub BeginRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::ULONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BeginRead: usize,
    #[cfg(feature = "shared")]
    pub EndRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EndRead: usize,
    #[cfg(feature = "shared")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Write: usize,
    #[cfg(feature = "shared")]
    pub BeginWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    BeginWrite: usize,
    #[cfg(feature = "shared")]
    pub EndWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EndWrite: usize,
    #[cfg(feature = "shared")]
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, MFBYTESTREAM_SEEK_ORIGIN, super::LONGLONG, super::DWORD, *mut super::QWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Seek: usize,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IMFByteStream_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self, pdwcapabilities: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetLength(&self, pqwlength: *mut super::QWORD) -> windows_core::Result<()>;
    fn SetLength(&self, qwlength: super::QWORD) -> windows_core::Result<()>;
    fn GetCurrentPosition(&self, pqwposition: *mut super::QWORD) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, qwposition: super::QWORD) -> windows_core::Result<()>;
    fn IsEndOfStream(&self, pfendofstream: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Read(&self, pb: *mut super::BYTE, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::Result<()>;
    fn BeginRead(&self, pb: *mut super::BYTE, cb: super::ULONG, pcallback: windows_core::Ref<IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndRead(&self, presult: windows_core::Ref<IMFAsyncResult>, pcbread: *mut super::ULONG) -> windows_core::Result<()>;
    fn Write(&self, pb: *const super::BYTE, cb: super::ULONG, pcbwritten: *mut super::ULONG) -> windows_core::Result<()>;
    fn BeginWrite(&self, pb: *const super::BYTE, cb: super::ULONG, pcallback: windows_core::Ref<IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndWrite(&self, presult: windows_core::Ref<IMFAsyncResult>, pcbwritten: *mut super::ULONG) -> windows_core::Result<()>;
    fn Seek(&self, seekorigin: MFBYTESTREAM_SEEK_ORIGIN, llseekoffset: super::LONGLONG, dwseekflags: super::DWORD, pqwcurrentposition: *mut super::QWORD) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFByteStream_Vtbl {
    pub const fn new<Identity: IMFByteStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdwcapabilities)).into()
            }
        }
        unsafe extern "system" fn GetLength<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwlength: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::GetLength(this, core::mem::transmute_copy(&pqwlength)).into()
            }
        }
        unsafe extern "system" fn SetLength<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwlength: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::SetLength(this, core::mem::transmute_copy(&qwlength)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwposition: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pqwposition)).into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwposition: super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&qwposition)).into()
            }
        }
        unsafe extern "system" fn IsEndOfStream<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfendofstream: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::IsEndOfStream(this, core::mem::transmute_copy(&pfendofstream)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::BYTE, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::Read(this, core::mem::transmute_copy(&pb), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn BeginRead<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::BYTE, cb: super::ULONG, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::BeginRead(this, core::mem::transmute_copy(&pb), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndRead<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::EndRead(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *const super::BYTE, cb: super::ULONG, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::Write(this, core::mem::transmute_copy(&pb), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn BeginWrite<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *const super::BYTE, cb: super::ULONG, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::BeginWrite(this, core::mem::transmute_copy(&pb), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndWrite<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::EndWrite(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Seek<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seekorigin: MFBYTESTREAM_SEEK_ORIGIN, llseekoffset: super::LONGLONG, dwseekflags: super::DWORD, pqwcurrentposition: *mut super::QWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::Seek(this, core::mem::transmute_copy(&seekorigin), core::mem::transmute_copy(&llseekoffset), core::mem::transmute_copy(&dwseekflags), core::mem::transmute_copy(&pqwcurrentposition)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMFByteStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStream_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetLength: GetLength::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            IsEndOfStream: IsEndOfStream::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            BeginRead: BeginRead::<Identity, OFFSET>,
            EndRead: EndRead::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            BeginWrite: BeginWrite::<Identity, OFFSET>,
            EndWrite: EndWrite::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFByteStream {}
windows_core::imp::define_interface!(IMFByteStreamProxyClassFactory, IMFByteStreamProxyClassFactory_Vtbl, 0xa6b43f84_5c0a_42e8_a44d_b1857a76992f);
windows_core::imp::interface_hierarchy!(IMFByteStreamProxyClassFactory, windows_core::IUnknown);
impl IMFByteStreamProxyClassFactory {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn CreateByteStreamProxy<P0, P1, T>(&self, pbytestream: P0, pattributes: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IMFByteStream>,
        P1: windows_core::Param<IMFAttributes>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateByteStreamProxy)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), pattributes.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamProxyClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub CreateByteStreamProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    CreateByteStreamProxy: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub trait IMFByteStreamProxyClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateByteStreamProxy(&self, pbytestream: windows_core::Ref<IMFByteStream>, pattributes: windows_core::Ref<IMFAttributes>, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl IMFByteStreamProxyClassFactory_Vtbl {
    pub const fn new<Identity: IMFByteStreamProxyClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateByteStreamProxy<Identity: IMFByteStreamProxyClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamProxyClassFactory_Impl::CreateByteStreamProxy(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateByteStreamProxy: CreateByteStreamProxy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamProxyClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
impl windows_core::RuntimeName for IMFByteStreamProxyClassFactory {}
windows_core::imp::define_interface!(IMFCollection, IMFCollection_Vtbl, 0x5bc8a76b_869a_46a3_9b03_fa218a66aebe);
windows_core::imp::interface_hierarchy!(IMFCollection, windows_core::IUnknown);
impl IMFCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn GetElementCount(&self, pcelements: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetElementCount)(windows_core::Interface::as_raw(self), pcelements as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetElement(&self, dwelementindex: super::DWORD, ppunkelement: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), dwelementindex, core::mem::transmute(ppunkelement)) }
    }
    pub unsafe fn AddElement<P0>(&self, punkelement: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddElement)(windows_core::Interface::as_raw(self), punkelement.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveElement(&self, dwelementindex: super::DWORD, ppunkelement: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveElement)(windows_core::Interface::as_raw(self), dwelementindex, core::mem::transmute(ppunkelement)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn InsertElementAt<P1>(&self, dwindex: super::DWORD, punknown: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertElementAt)(windows_core::Interface::as_raw(self), dwindex, punknown.param().abi()) }
    }
    pub unsafe fn RemoveAllElements(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllElements)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetElementCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetElementCount: usize,
    #[cfg(feature = "shared")]
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetElement: usize,
    pub AddElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveElement: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveElement: usize,
    #[cfg(feature = "shared")]
    pub InsertElementAt: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InsertElementAt: usize,
    pub RemoveAllElements: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IMFCollection_Impl: windows_core::IUnknownImpl {
    fn GetElementCount(&self, pcelements: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetElement(&self, dwelementindex: super::DWORD, ppunkelement: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn AddElement(&self, punkelement: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveElement(&self, dwelementindex: super::DWORD, ppunkelement: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn InsertElementAt(&self, dwindex: super::DWORD, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllElements(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFCollection_Vtbl {
    pub const fn new<Identity: IMFCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetElementCount<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelements: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::GetElementCount(this, core::mem::transmute_copy(&pcelements)).into()
            }
        }
        unsafe extern "system" fn GetElement<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwelementindex: super::DWORD, ppunkelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::GetElement(this, core::mem::transmute_copy(&dwelementindex), core::mem::transmute_copy(&ppunkelement)).into()
            }
        }
        unsafe extern "system" fn AddElement<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::AddElement(this, core::mem::transmute_copy(&punkelement)).into()
            }
        }
        unsafe extern "system" fn RemoveElement<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwelementindex: super::DWORD, ppunkelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::RemoveElement(this, core::mem::transmute_copy(&dwelementindex), core::mem::transmute_copy(&ppunkelement)).into()
            }
        }
        unsafe extern "system" fn InsertElementAt<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::InsertElementAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn RemoveAllElements<Identity: IMFCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCollection_Impl::RemoveAllElements(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetElementCount: GetElementCount::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
            AddElement: AddElement::<Identity, OFFSET>,
            RemoveElement: RemoveElement::<Identity, OFFSET>,
            InsertElementAt: InsertElementAt::<Identity, OFFSET>,
            RemoveAllElements: RemoveAllElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFCollection {}
windows_core::imp::define_interface!(IMFDXGIBuffer, IMFDXGIBuffer_Vtbl, 0xe7174cfa_1c9e_48b1_8866_626226bfc258);
windows_core::imp::interface_hierarchy!(IMFDXGIBuffer, windows_core::IUnknown);
impl IMFDXGIBuffer {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetResource(&self, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), riid, ppvobject as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSubresourceIndex(&self, pusubresource: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubresourceIndex)(windows_core::Interface::as_raw(self), pusubresource as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetUnknown(&self, guid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUnknown)(windows_core::Interface::as_raw(self), guid, riid, ppvobject as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetUnknown<P1>(&self, guid: *const super::IID, punkdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUnknown)(windows_core::Interface::as_raw(self), guid, punkdata.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDXGIBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetResource: usize,
    #[cfg(feature = "shared")]
    pub GetSubresourceIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSubresourceIndex: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetUnknown: usize,
    #[cfg(feature = "guiddef")]
    pub SetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetUnknown: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub trait IMFDXGIBuffer_Impl: windows_core::IUnknownImpl {
    fn GetResource(&self, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetSubresourceIndex(&self, pusubresource: *mut super::UINT) -> windows_core::Result<()>;
    fn GetUnknown(&self, guid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn SetUnknown(&self, guid: *const super::IID, punkdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl IMFDXGIBuffer_Vtbl {
    pub const fn new<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetSubresourceIndex<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusubresource: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::GetSubresourceIndex(this, core::mem::transmute_copy(&pusubresource)).into()
            }
        }
        unsafe extern "system" fn GetUnknown<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::GetUnknown(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn SetUnknown<Identity: IMFDXGIBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const super::IID, punkdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIBuffer_Impl::SetUnknown(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&punkdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResource: GetResource::<Identity, OFFSET>,
            GetSubresourceIndex: GetSubresourceIndex::<Identity, OFFSET>,
            GetUnknown: GetUnknown::<Identity, OFFSET>,
            SetUnknown: SetUnknown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGIBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IMFDXGIBuffer {}
windows_core::imp::define_interface!(IMFDXGICrossAdapterBuffer, IMFDXGICrossAdapterBuffer_Vtbl, 0xb25d03fb_d148_45ef_bfed_f778b7566c07);
windows_core::imp::interface_hierarchy!(IMFDXGICrossAdapterBuffer, windows_core::IUnknown);
impl IMFDXGICrossAdapterBuffer {
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetResourceForDevice<P0, T>(&self, punkdevice: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResourceForDevice)(windows_core::Interface::as_raw(self), punkdevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSubresourceIndexForDevice<P0>(&self, punkdevice: P0, pusubresource: *mut super::UINT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSubresourceIndexForDevice)(windows_core::Interface::as_raw(self), punkdevice.param().abi(), pusubresource as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub unsafe fn GetUnknownForDevice<P0, T>(&self, punkdevice: P0, guid: *const super::IID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetUnknownForDevice)(windows_core::Interface::as_raw(self), punkdevice.param().abi(), guid, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetUnknownForDevice<P0, P2>(&self, punkdevice: P0, guid: *const super::IID, punkdata: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUnknownForDevice)(windows_core::Interface::as_raw(self), punkdevice.param().abi(), guid, punkdata.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDXGICrossAdapterBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetResourceForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetResourceForDevice: usize,
    #[cfg(feature = "shared")]
    pub GetSubresourceIndexForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSubresourceIndexForDevice: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef"))]
    pub GetUnknownForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *const super::IID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef")))]
    GetUnknownForDevice: usize,
    #[cfg(feature = "guiddef")]
    pub SetUnknownForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetUnknownForDevice: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub trait IMFDXGICrossAdapterBuffer_Impl: windows_core::IUnknownImpl {
    fn GetResourceForDevice(&self, punkdevice: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetSubresourceIndexForDevice(&self, punkdevice: windows_core::Ref<windows_core::IUnknown>, pusubresource: *mut super::UINT) -> windows_core::Result<()>;
    fn GetUnknownForDevice(&self, punkdevice: windows_core::Ref<windows_core::IUnknown>, guid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn SetUnknownForDevice(&self, punkdevice: windows_core::Ref<windows_core::IUnknown>, guid: *const super::IID, punkdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl IMFDXGICrossAdapterBuffer_Vtbl {
    pub const fn new<Identity: IMFDXGICrossAdapterBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResourceForDevice<Identity: IMFDXGICrossAdapterBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevice: *mut core::ffi::c_void, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGICrossAdapterBuffer_Impl::GetResourceForDevice(this, core::mem::transmute_copy(&punkdevice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn GetSubresourceIndexForDevice<Identity: IMFDXGICrossAdapterBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevice: *mut core::ffi::c_void, pusubresource: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGICrossAdapterBuffer_Impl::GetSubresourceIndexForDevice(this, core::mem::transmute_copy(&punkdevice), core::mem::transmute_copy(&pusubresource)).into()
            }
        }
        unsafe extern "system" fn GetUnknownForDevice<Identity: IMFDXGICrossAdapterBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevice: *mut core::ffi::c_void, guid: *const super::IID, riid: *const super::IID, ppvobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGICrossAdapterBuffer_Impl::GetUnknownForDevice(this, core::mem::transmute_copy(&punkdevice), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn SetUnknownForDevice<Identity: IMFDXGICrossAdapterBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevice: *mut core::ffi::c_void, guid: *const super::IID, punkdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGICrossAdapterBuffer_Impl::SetUnknownForDevice(this, core::mem::transmute_copy(&punkdevice), core::mem::transmute_copy(&guid), core::mem::transmute_copy(&punkdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceForDevice: GetResourceForDevice::<Identity, OFFSET>,
            GetSubresourceIndexForDevice: GetSubresourceIndexForDevice::<Identity, OFFSET>,
            GetUnknownForDevice: GetUnknownForDevice::<Identity, OFFSET>,
            SetUnknownForDevice: SetUnknownForDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGICrossAdapterBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IMFDXGICrossAdapterBuffer {}
windows_core::imp::define_interface!(IMFDXGIDeviceManager, IMFDXGIDeviceManager_Vtbl, 0xeb533d5d_2db6_40f8_97a9_494692014f07);
windows_core::imp::interface_hierarchy!(IMFDXGIDeviceManager, windows_core::IUnknown);
impl IMFDXGIDeviceManager {
    #[cfg(feature = "winnt")]
    pub unsafe fn CloseDeviceHandle(&self, hdevice: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseDeviceHandle)(windows_core::Interface::as_raw(self), hdevice) }
    }
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub unsafe fn GetVideoService(&self, hdevice: super::HANDLE, riid: *const super::IID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoService)(windows_core::Interface::as_raw(self), hdevice, riid, ppservice as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub unsafe fn LockDevice(&self, hdevice: super::HANDLE, riid: *const super::IID, ppunkdevice: *mut *mut core::ffi::c_void, fblock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self), hdevice, riid, ppunkdevice as _, fblock.into()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn OpenDeviceHandle(&self, phdevice: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OpenDeviceHandle)(windows_core::Interface::as_raw(self), phdevice as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ResetDevice<P0>(&self, punkdevice: P0, resettoken: super::UINT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResetDevice)(windows_core::Interface::as_raw(self), punkdevice.param().abi(), resettoken) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn TestDevice(&self, hdevice: super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestDevice)(windows_core::Interface::as_raw(self), hdevice) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn UnlockDevice(&self, hdevice: super::HANDLE, fsavestate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockDevice)(windows_core::Interface::as_raw(self), hdevice, fsavestate.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDXGIDeviceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub CloseDeviceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    CloseDeviceHandle: usize,
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub GetVideoService: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "winnt")))]
    GetVideoService: usize,
    #[cfg(all(feature = "guiddef", feature = "winnt"))]
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE, *const super::IID, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "winnt")))]
    LockDevice: usize,
    #[cfg(feature = "winnt")]
    pub OpenDeviceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    OpenDeviceHandle: usize,
    #[cfg(feature = "shared")]
    pub ResetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ResetDevice: usize,
    #[cfg(feature = "winnt")]
    pub TestDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    TestDevice: usize,
    #[cfg(feature = "winnt")]
    pub UnlockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    UnlockDevice: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
pub trait IMFDXGIDeviceManager_Impl: windows_core::IUnknownImpl {
    fn CloseDeviceHandle(&self, hdevice: super::HANDLE) -> windows_core::Result<()>;
    fn GetVideoService(&self, hdevice: super::HANDLE, riid: *const super::IID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LockDevice(&self, hdevice: super::HANDLE, riid: *const super::IID, ppunkdevice: *mut *mut core::ffi::c_void, fblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn OpenDeviceHandle(&self, phdevice: *mut super::HANDLE) -> windows_core::Result<()>;
    fn ResetDevice(&self, punkdevice: windows_core::Ref<windows_core::IUnknown>, resettoken: super::UINT) -> windows_core::Result<()>;
    fn TestDevice(&self, hdevice: super::HANDLE) -> windows_core::Result<()>;
    fn UnlockDevice(&self, hdevice: super::HANDLE, fsavestate: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
impl IMFDXGIDeviceManager_Vtbl {
    pub const fn new<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CloseDeviceHandle<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::CloseDeviceHandle(this, core::mem::transmute_copy(&hdevice)).into()
            }
        }
        unsafe extern "system" fn GetVideoService<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::HANDLE, riid: *const super::IID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::GetVideoService(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppservice)).into()
            }
        }
        unsafe extern "system" fn LockDevice<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::HANDLE, riid: *const super::IID, ppunkdevice: *mut *mut core::ffi::c_void, fblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::LockDevice(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunkdevice), core::mem::transmute_copy(&fblock)).into()
            }
        }
        unsafe extern "system" fn OpenDeviceHandle<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdevice: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::OpenDeviceHandle(this, core::mem::transmute_copy(&phdevice)).into()
            }
        }
        unsafe extern "system" fn ResetDevice<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevice: *mut core::ffi::c_void, resettoken: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::ResetDevice(this, core::mem::transmute_copy(&punkdevice), core::mem::transmute_copy(&resettoken)).into()
            }
        }
        unsafe extern "system" fn TestDevice<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::TestDevice(this, core::mem::transmute_copy(&hdevice)).into()
            }
        }
        unsafe extern "system" fn UnlockDevice<Identity: IMFDXGIDeviceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::HANDLE, fsavestate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDXGIDeviceManager_Impl::UnlockDevice(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&fsavestate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseDeviceHandle: CloseDeviceHandle::<Identity, OFFSET>,
            GetVideoService: GetVideoService::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            OpenDeviceHandle: OpenDeviceHandle::<Identity, OFFSET>,
            ResetDevice: ResetDevice::<Identity, OFFSET>,
            TestDevice: TestDevice::<Identity, OFFSET>,
            UnlockDevice: UnlockDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGIDeviceManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IMFDXGIDeviceManager {}
windows_core::imp::define_interface!(IMFMediaBuffer, IMFMediaBuffer_Vtbl, 0x045fa593_8799_42b8_bc8d_8968c6453507);
windows_core::imp::interface_hierarchy!(IMFMediaBuffer, windows_core::IUnknown);
impl IMFMediaBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn Lock(&self, ppbbuffer: *mut *mut super::BYTE, pcbmaxlength: Option<*mut super::DWORD>, pcbcurrentlength: Option<*mut super::DWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), ppbbuffer as _, pcbmaxlength.unwrap_or(core::mem::zeroed()) as _, pcbcurrentlength.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentLength(&self, pcbcurrentlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentLength)(windows_core::Interface::as_raw(self), pcbcurrentlength as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentLength(&self, cbcurrentlength: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentLength)(windows_core::Interface::as_raw(self), cbcurrentlength) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaxLength(&self, pcbmaxlength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaxLength)(windows_core::Interface::as_raw(self), pcbmaxlength as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Lock: usize,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCurrentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentLength: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentLength: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentLength: usize,
    #[cfg(feature = "shared")]
    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaxLength: usize,
}
#[cfg(feature = "shared")]
pub trait IMFMediaBuffer_Impl: windows_core::IUnknownImpl {
    fn Lock(&self, ppbbuffer: *mut *mut super::BYTE, pcbmaxlength: *mut super::DWORD, pcbcurrentlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetCurrentLength(&self, pcbcurrentlength: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetCurrentLength(&self, cbcurrentlength: super::DWORD) -> windows_core::Result<()>;
    fn GetMaxLength(&self, pcbmaxlength: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFMediaBuffer_Vtbl {
    pub const fn new<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbbuffer: *mut *mut super::BYTE, pcbmaxlength: *mut super::DWORD, pcbcurrentlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::Lock(this, core::mem::transmute_copy(&ppbbuffer), core::mem::transmute_copy(&pcbmaxlength), core::mem::transmute_copy(&pcbcurrentlength)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetCurrentLength<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbcurrentlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::GetCurrentLength(this, core::mem::transmute_copy(&pcbcurrentlength)).into()
            }
        }
        unsafe extern "system" fn SetCurrentLength<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbcurrentlength: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::SetCurrentLength(this, core::mem::transmute_copy(&cbcurrentlength)).into()
            }
        }
        unsafe extern "system" fn GetMaxLength<Identity: IMFMediaBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbmaxlength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaBuffer_Impl::GetMaxLength(this, core::mem::transmute_copy(&pcbmaxlength)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetCurrentLength: GetCurrentLength::<Identity, OFFSET>,
            SetCurrentLength: SetCurrentLength::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaBuffer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFMediaBuffer {}
windows_core::imp::define_interface!(IMFMediaEvent, IMFMediaEvent_Vtbl, 0xdf598932_f10c_4e39_bba2_c308f101daa3);
impl core::ops::Deref for IMFMediaEvent {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEvent, windows_core::IUnknown, IMFAttributes);
impl IMFMediaEvent {
    pub unsafe fn GetType(&self, pmet: *mut MediaEventType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), pmet as _) }
    }
    pub unsafe fn GetExtendedType(&self, pguidextendedtype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedType)(windows_core::Interface::as_raw(self), pguidextendedtype as _) }
    }
    pub unsafe fn GetStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), phrstatus as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, pvvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvvalue) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEvent_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaEventType) -> windows_core::HRESULT,
    pub GetExtendedType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaEvent_Impl: IMFAttributes_Impl {
    fn GetType(&self, pmet: *mut MediaEventType) -> windows_core::Result<()>;
    fn GetExtendedType(&self, pguidextendedtype: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetValue(&self, pvvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaEvent_Vtbl {
    pub const fn new<Identity: IMFMediaEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IMFMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmet: *mut MediaEventType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEvent_Impl::GetType(this, core::mem::transmute_copy(&pmet)).into()
            }
        }
        unsafe extern "system" fn GetExtendedType<Identity: IMFMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidextendedtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEvent_Impl::GetExtendedType(this, core::mem::transmute_copy(&pguidextendedtype)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IMFMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEvent_Impl::GetStatus(this, core::mem::transmute_copy(&phrstatus)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IMFMediaEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEvent_Impl::GetValue(this, core::mem::transmute_copy(&pvvalue)).into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetExtendedType: GetExtendedType::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEvent as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaEvent {}
windows_core::imp::define_interface!(IMFMediaEventGenerator, IMFMediaEventGenerator_Vtbl, 0x2cd0bd52_bcd5_4b89_b62c_eadc0c031e7d);
windows_core::imp::interface_hierarchy!(IMFMediaEventGenerator, windows_core::IUnknown);
impl IMFMediaEventGenerator {
    #[cfg(feature = "shared")]
    pub unsafe fn GetEvent(&self, dwflags: super::DWORD, ppevent: *mut Option<IMFMediaEvent>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(ppevent)) }
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginGetEvent)(windows_core::Interface::as_raw(self), pcallback.param().abi(), punkstate.param().abi()) }
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0, ppevent: *mut Option<IMFMediaEvent>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetEvent)(windows_core::Interface::as_raw(self), presult.param().abi(), core::mem::transmute(ppevent)) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn QueueEvent(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: Option<*const super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueueEvent)(windows_core::Interface::as_raw(self), met, guidextendedtype, hrstatus, pvvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEventGenerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEvent: usize,
    pub BeginGetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndGetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub QueueEvent: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEventType, *const windows_core::GUID, windows_core::HRESULT, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    QueueEvent: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaEventGenerator_Impl: windows_core::IUnknownImpl {
    fn GetEvent(&self, dwflags: super::DWORD, ppevent: windows_core::OutRef<IMFMediaEvent>) -> windows_core::Result<()>;
    fn BeginGetEvent(&self, pcallback: windows_core::Ref<IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndGetEvent(&self, presult: windows_core::Ref<IMFAsyncResult>, ppevent: windows_core::OutRef<IMFMediaEvent>) -> windows_core::Result<()>;
    fn QueueEvent(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaEventGenerator_Vtbl {
    pub const fn new<Identity: IMFMediaEventGenerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEvent<Identity: IMFMediaEventGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventGenerator_Impl::GetEvent(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppevent)).into()
            }
        }
        unsafe extern "system" fn BeginGetEvent<Identity: IMFMediaEventGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventGenerator_Impl::BeginGetEvent(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndGetEvent<Identity: IMFMediaEventGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventGenerator_Impl::EndGetEvent(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppevent)).into()
            }
        }
        unsafe extern "system" fn QueueEvent<Identity: IMFMediaEventGenerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventGenerator_Impl::QueueEvent(this, core::mem::transmute_copy(&met), core::mem::transmute_copy(&guidextendedtype), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&pvvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, OFFSET>,
            BeginGetEvent: BeginGetEvent::<Identity, OFFSET>,
            EndGetEvent: EndGetEvent::<Identity, OFFSET>,
            QueueEvent: QueueEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEventGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaEventGenerator {}
windows_core::imp::define_interface!(IMFMediaEventQueue, IMFMediaEventQueue_Vtbl, 0x36f846fc_2256_48b6_b58e_e2b638316581);
windows_core::imp::interface_hierarchy!(IMFMediaEventQueue, windows_core::IUnknown);
impl IMFMediaEventQueue {
    #[cfg(feature = "shared")]
    pub unsafe fn GetEvent(&self, dwflags: super::DWORD, ppevent: *mut Option<IMFMediaEvent>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(ppevent)) }
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginGetEvent)(windows_core::Interface::as_raw(self), pcallback.param().abi(), punkstate.param().abi()) }
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0, ppevent: *mut Option<IMFMediaEvent>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetEvent)(windows_core::Interface::as_raw(self), presult.param().abi(), core::mem::transmute(ppevent)) }
    }
    pub unsafe fn QueueEvent<P0>(&self, pevent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueueEvent)(windows_core::Interface::as_raw(self), pevent.param().abi()) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn QueueEventParamVar(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueueEventParamVar)(windows_core::Interface::as_raw(self), met, guidextendedtype, hrstatus, pvvalue) }
    }
    pub unsafe fn QueueEventParamUnk<P3>(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, punk: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueueEventParamUnk)(windows_core::Interface::as_raw(self), met, guidextendedtype, hrstatus, punk.param().abi()) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEventQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEvent: usize,
    pub BeginGetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndGetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueueEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub QueueEventParamVar: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEventType, *const windows_core::GUID, windows_core::HRESULT, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    QueueEventParamVar: usize,
    pub QueueEventParamUnk: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEventType, *const windows_core::GUID, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaEventQueue_Impl: windows_core::IUnknownImpl {
    fn GetEvent(&self, dwflags: super::DWORD, ppevent: windows_core::OutRef<IMFMediaEvent>) -> windows_core::Result<()>;
    fn BeginGetEvent(&self, pcallback: windows_core::Ref<IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndGetEvent(&self, presult: windows_core::Ref<IMFAsyncResult>, ppevent: windows_core::OutRef<IMFMediaEvent>) -> windows_core::Result<()>;
    fn QueueEvent(&self, pevent: windows_core::Ref<IMFMediaEvent>) -> windows_core::Result<()>;
    fn QueueEventParamVar(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn QueueEventParamUnk(&self, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaEventQueue_Vtbl {
    pub const fn new<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEvent<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::GetEvent(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppevent)).into()
            }
        }
        unsafe extern "system" fn BeginGetEvent<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::BeginGetEvent(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndGetEvent<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::EndGetEvent(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppevent)).into()
            }
        }
        unsafe extern "system" fn QueueEvent<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::QueueEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn QueueEventParamVar<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, pvvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::QueueEventParamVar(this, core::mem::transmute_copy(&met), core::mem::transmute_copy(&guidextendedtype), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&pvvalue)).into()
            }
        }
        unsafe extern "system" fn QueueEventParamUnk<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, met: MediaEventType, guidextendedtype: *const windows_core::GUID, hrstatus: windows_core::HRESULT, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::QueueEventParamUnk(this, core::mem::transmute_copy(&met), core::mem::transmute_copy(&guidextendedtype), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaEventQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEventQueue_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, OFFSET>,
            BeginGetEvent: BeginGetEvent::<Identity, OFFSET>,
            EndGetEvent: EndGetEvent::<Identity, OFFSET>,
            QueueEvent: QueueEvent::<Identity, OFFSET>,
            QueueEventParamVar: QueueEventParamVar::<Identity, OFFSET>,
            QueueEventParamUnk: QueueEventParamUnk::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEventQueue as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaEventQueue {}
windows_core::imp::define_interface!(IMFMediaType, IMFMediaType_Vtbl, 0x44ae0fa8_ea31_4109_8d2e_4cae4997c555);
impl core::ops::Deref for IMFMediaType {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaType, windows_core::IUnknown, IMFAttributes);
impl IMFMediaType {
    pub unsafe fn GetMajorType(&self, pguidmajortype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMajorType)(windows_core::Interface::as_raw(self), pguidmajortype as _) }
    }
    pub unsafe fn IsCompressedFormat(&self, pfcompressed: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsCompressedFormat)(windows_core::Interface::as_raw(self), pfcompressed as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn IsEqual<P0>(&self, pimediatype: P0, pdwflags: *mut super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pimediatype.param().abi(), pdwflags as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetRepresentation(&self, guidrepresentation: windows_core::GUID, ppvrepresentation: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRepresentation)(windows_core::Interface::as_raw(self), guidrepresentation, ppvrepresentation as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn FreeRepresentation(&self, guidrepresentation: windows_core::GUID, pvrepresentation: super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeRepresentation)(windows_core::Interface::as_raw(self), guidrepresentation, pvrepresentation) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaType_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    pub GetMajorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsCompressedFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    IsEqual: usize,
    #[cfg(feature = "minwindef")]
    pub GetRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetRepresentation: usize,
    #[cfg(feature = "minwindef")]
    pub FreeRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    FreeRepresentation: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaType_Impl: IMFAttributes_Impl {
    fn GetMajorType(&self, pguidmajortype: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn IsCompressedFormat(&self, pfcompressed: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsEqual(&self, pimediatype: windows_core::Ref<IMFMediaType>, pdwflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetRepresentation(&self, guidrepresentation: &windows_core::GUID, ppvrepresentation: *mut super::LPVOID) -> windows_core::Result<()>;
    fn FreeRepresentation(&self, guidrepresentation: &windows_core::GUID, pvrepresentation: super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaType_Vtbl {
    pub const fn new<Identity: IMFMediaType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMajorType<Identity: IMFMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmajortype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::GetMajorType(this, core::mem::transmute_copy(&pguidmajortype)).into()
            }
        }
        unsafe extern "system" fn IsCompressedFormat<Identity: IMFMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcompressed: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::IsCompressedFormat(this, core::mem::transmute_copy(&pfcompressed)).into()
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IMFMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimediatype: *mut core::ffi::c_void, pdwflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::IsEqual(this, core::mem::transmute_copy(&pimediatype), core::mem::transmute_copy(&pdwflags)).into()
            }
        }
        unsafe extern "system" fn GetRepresentation<Identity: IMFMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidrepresentation: windows_core::GUID, ppvrepresentation: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::GetRepresentation(this, core::mem::transmute(&guidrepresentation), core::mem::transmute_copy(&ppvrepresentation)).into()
            }
        }
        unsafe extern "system" fn FreeRepresentation<Identity: IMFMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidrepresentation: windows_core::GUID, pvrepresentation: super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaType_Impl::FreeRepresentation(this, core::mem::transmute(&guidrepresentation), core::mem::transmute_copy(&pvrepresentation)).into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetMajorType: GetMajorType::<Identity, OFFSET>,
            IsCompressedFormat: IsCompressedFormat::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            GetRepresentation: GetRepresentation::<Identity, OFFSET>,
            FreeRepresentation: FreeRepresentation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaType as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaType {}
windows_core::imp::define_interface!(IMFMuxStreamAttributesManager, IMFMuxStreamAttributesManager_Vtbl, 0xce8bd576_e440_43b3_be34_1e53f565f7e8);
windows_core::imp::interface_hierarchy!(IMFMuxStreamAttributesManager, windows_core::IUnknown);
impl IMFMuxStreamAttributesManager {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pdwmuxstreamcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAttributes(&self, dwmuxstreamindex: super::DWORD, ppstreamattributes: *mut Option<IMFAttributes>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAttributes)(windows_core::Interface::as_raw(self), dwmuxstreamindex, core::mem::transmute(ppstreamattributes)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMuxStreamAttributesManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAttributes: usize,
}
#[cfg(feature = "shared")]
pub trait IMFMuxStreamAttributesManager_Impl: windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetAttributes(&self, dwmuxstreamindex: super::DWORD, ppstreamattributes: windows_core::OutRef<IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFMuxStreamAttributesManager_Vtbl {
    pub const fn new<Identity: IMFMuxStreamAttributesManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamCount<Identity: IMFMuxStreamAttributesManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamAttributesManager_Impl::GetStreamCount(this, core::mem::transmute_copy(&pdwmuxstreamcount)).into()
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: IMFMuxStreamAttributesManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmuxstreamindex: super::DWORD, ppstreamattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamAttributesManager_Impl::GetAttributes(this, core::mem::transmute_copy(&dwmuxstreamindex), core::mem::transmute_copy(&ppstreamattributes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMuxStreamAttributesManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFMuxStreamAttributesManager {}
windows_core::imp::define_interface!(IMFMuxStreamMediaTypeManager, IMFMuxStreamMediaTypeManager_Vtbl, 0x505a2c72_42f7_4690_aeab_8f513d0ffdb8);
windows_core::imp::interface_hierarchy!(IMFMuxStreamMediaTypeManager, windows_core::IUnknown);
impl IMFMuxStreamMediaTypeManager {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pdwmuxstreamcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMediaType(&self, dwmuxstreamindex: super::DWORD, ppmediatype: *mut Option<IMFMediaType>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMediaType)(windows_core::Interface::as_raw(self), dwmuxstreamindex, core::mem::transmute(ppmediatype)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamConfigurationCount(&self, pdwcount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamConfigurationCount)(windows_core::Interface::as_raw(self), pdwcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddStreamConfiguration(&self, ullstreammask: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddStreamConfiguration)(windows_core::Interface::as_raw(self), ullstreammask) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveStreamConfiguration(&self, ullstreammask: super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStreamConfiguration)(windows_core::Interface::as_raw(self), ullstreammask) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamConfiguration(&self, ulindex: super::DWORD, pullstreammask: *mut super::ULONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamConfiguration)(windows_core::Interface::as_raw(self), ulindex, pullstreammask as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMuxStreamMediaTypeManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMediaType: usize,
    #[cfg(feature = "shared")]
    pub GetStreamConfigurationCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamConfigurationCount: usize,
    #[cfg(feature = "shared")]
    pub AddStreamConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddStreamConfiguration: usize,
    #[cfg(feature = "shared")]
    pub RemoveStreamConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveStreamConfiguration: usize,
    #[cfg(feature = "shared")]
    pub GetStreamConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut super::ULONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamConfiguration: usize,
}
#[cfg(feature = "shared")]
pub trait IMFMuxStreamMediaTypeManager_Impl: windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetMediaType(&self, dwmuxstreamindex: super::DWORD, ppmediatype: windows_core::OutRef<IMFMediaType>) -> windows_core::Result<()>;
    fn GetStreamConfigurationCount(&self, pdwcount: *mut super::DWORD) -> windows_core::Result<()>;
    fn AddStreamConfiguration(&self, ullstreammask: super::ULONGLONG) -> windows_core::Result<()>;
    fn RemoveStreamConfiguration(&self, ullstreammask: super::ULONGLONG) -> windows_core::Result<()>;
    fn GetStreamConfiguration(&self, ulindex: super::DWORD, pullstreammask: *mut super::ULONGLONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IMFMuxStreamMediaTypeManager_Vtbl {
    pub const fn new<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamCount<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::GetStreamCount(this, core::mem::transmute_copy(&pdwmuxstreamcount)).into()
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmuxstreamindex: super::DWORD, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::GetMediaType(this, core::mem::transmute_copy(&dwmuxstreamindex), core::mem::transmute_copy(&ppmediatype)).into()
            }
        }
        unsafe extern "system" fn GetStreamConfigurationCount<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::GetStreamConfigurationCount(this, core::mem::transmute_copy(&pdwcount)).into()
            }
        }
        unsafe extern "system" fn AddStreamConfiguration<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullstreammask: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::AddStreamConfiguration(this, core::mem::transmute_copy(&ullstreammask)).into()
            }
        }
        unsafe extern "system" fn RemoveStreamConfiguration<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullstreammask: super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::RemoveStreamConfiguration(this, core::mem::transmute_copy(&ullstreammask)).into()
            }
        }
        unsafe extern "system" fn GetStreamConfiguration<Identity: IMFMuxStreamMediaTypeManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: super::DWORD, pullstreammask: *mut super::ULONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamMediaTypeManager_Impl::GetStreamConfiguration(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&pullstreammask)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetMediaType: GetMediaType::<Identity, OFFSET>,
            GetStreamConfigurationCount: GetStreamConfigurationCount::<Identity, OFFSET>,
            AddStreamConfiguration: AddStreamConfiguration::<Identity, OFFSET>,
            RemoveStreamConfiguration: RemoveStreamConfiguration::<Identity, OFFSET>,
            GetStreamConfiguration: GetStreamConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMuxStreamMediaTypeManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFMuxStreamMediaTypeManager {}
windows_core::imp::define_interface!(IMFMuxStreamSampleManager, IMFMuxStreamSampleManager_Vtbl, 0x74abbc19_b1cc_4e41_bb8b_9d9b86a8f6ca);
windows_core::imp::interface_hierarchy!(IMFMuxStreamSampleManager, windows_core::IUnknown);
impl IMFMuxStreamSampleManager {
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamCount)(windows_core::Interface::as_raw(self), pdwmuxstreamcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSample(&self, dwmuxstreamindex: super::DWORD, ppsample: *mut Option<IMFSample>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSample)(windows_core::Interface::as_raw(self), dwmuxstreamindex, core::mem::transmute(ppsample)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStreamConfiguration(&self) -> super::ULONGLONG {
        unsafe { (windows_core::Interface::vtable(self).GetStreamConfiguration)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMuxStreamSampleManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStreamCount: usize,
    #[cfg(feature = "shared")]
    pub GetSample: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSample: usize,
    #[cfg(feature = "shared")]
    pub GetStreamConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::ULONGLONG,
    #[cfg(not(feature = "shared"))]
    GetStreamConfiguration: usize,
}
#[cfg(feature = "shared")]
pub trait IMFMuxStreamSampleManager_Impl: windows_core::IUnknownImpl {
    fn GetStreamCount(&self, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSample(&self, dwmuxstreamindex: super::DWORD, ppsample: windows_core::OutRef<IMFSample>) -> windows_core::Result<()>;
    fn GetStreamConfiguration(&self) -> super::ULONGLONG;
}
#[cfg(feature = "shared")]
impl IMFMuxStreamSampleManager_Vtbl {
    pub const fn new<Identity: IMFMuxStreamSampleManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamCount<Identity: IMFMuxStreamSampleManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmuxstreamcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamSampleManager_Impl::GetStreamCount(this, core::mem::transmute_copy(&pdwmuxstreamcount)).into()
            }
        }
        unsafe extern "system" fn GetSample<Identity: IMFMuxStreamSampleManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmuxstreamindex: super::DWORD, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamSampleManager_Impl::GetSample(this, core::mem::transmute_copy(&dwmuxstreamindex), core::mem::transmute_copy(&ppsample)).into()
            }
        }
        unsafe extern "system" fn GetStreamConfiguration<Identity: IMFMuxStreamSampleManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::ULONGLONG {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMuxStreamSampleManager_Impl::GetStreamConfiguration(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, OFFSET>,
            GetSample: GetSample::<Identity, OFFSET>,
            GetStreamConfiguration: GetStreamConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMuxStreamSampleManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFMuxStreamSampleManager {}
windows_core::imp::define_interface!(IMFPluginControl, IMFPluginControl_Vtbl, 0x5c6c44bf_1db6_435b_9249_e8cd10fdec96);
windows_core::imp::interface_hierarchy!(IMFPluginControl, windows_core::IUnknown);
impl IMFPluginControl {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetPreferredClsid<P1>(&self, plugintype: super::DWORD, selector: P1, clsid: *mut super::CLSID) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredClsid)(windows_core::Interface::as_raw(self), plugintype, selector.param().abi(), clsid as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetPreferredClsidByIndex(&self, plugintype: super::DWORD, index: super::DWORD, selector: *mut windows_core::PWSTR, clsid: *mut super::CLSID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPreferredClsidByIndex)(windows_core::Interface::as_raw(self), plugintype, index, selector as _, clsid as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetPreferredClsid<P1>(&self, plugintype: super::DWORD, selector: P1, clsid: Option<*const super::CLSID>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredClsid)(windows_core::Interface::as_raw(self), plugintype, selector.param().abi(), clsid.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn IsDisabled(&self, plugintype: super::DWORD, clsid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDisabled)(windows_core::Interface::as_raw(self), plugintype, clsid) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetDisabledByIndex(&self, plugintype: super::DWORD, index: super::DWORD, clsid: *mut super::CLSID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisabledByIndex)(windows_core::Interface::as_raw(self), plugintype, index, clsid as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetDisabled(&self, plugintype: super::DWORD, clsid: *const super::IID, disabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisabled)(windows_core::Interface::as_raw(self), plugintype, clsid, disabled.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPluginControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetPreferredClsid: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetPreferredClsid: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetPreferredClsidByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut windows_core::PWSTR, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetPreferredClsidByIndex: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetPreferredClsid: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, *const super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetPreferredClsid: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub IsDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    IsDisabled: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetDisabledByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetDisabledByIndex: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetDisabled: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IMFPluginControl_Impl: windows_core::IUnknownImpl {
    fn GetPreferredClsid(&self, plugintype: super::DWORD, selector: &windows_core::PCWSTR, clsid: *mut super::CLSID) -> windows_core::Result<()>;
    fn GetPreferredClsidByIndex(&self, plugintype: super::DWORD, index: super::DWORD, selector: *mut windows_core::PWSTR, clsid: *mut super::CLSID) -> windows_core::Result<()>;
    fn SetPreferredClsid(&self, plugintype: super::DWORD, selector: &windows_core::PCWSTR, clsid: *const super::CLSID) -> windows_core::Result<()>;
    fn IsDisabled(&self, plugintype: super::DWORD, clsid: *const super::IID) -> windows_core::Result<()>;
    fn GetDisabledByIndex(&self, plugintype: super::DWORD, index: super::DWORD, clsid: *mut super::CLSID) -> windows_core::Result<()>;
    fn SetDisabled(&self, plugintype: super::DWORD, clsid: *const super::IID, disabled: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IMFPluginControl_Vtbl {
    pub const fn new<Identity: IMFPluginControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreferredClsid<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, selector: windows_core::PCWSTR, clsid: *mut super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::GetPreferredClsid(this, core::mem::transmute_copy(&plugintype), core::mem::transmute(&selector), core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn GetPreferredClsidByIndex<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, index: super::DWORD, selector: *mut windows_core::PWSTR, clsid: *mut super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::GetPreferredClsidByIndex(this, core::mem::transmute_copy(&plugintype), core::mem::transmute_copy(&index), core::mem::transmute_copy(&selector), core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn SetPreferredClsid<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, selector: windows_core::PCWSTR, clsid: *const super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::SetPreferredClsid(this, core::mem::transmute_copy(&plugintype), core::mem::transmute(&selector), core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn IsDisabled<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, clsid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::IsDisabled(this, core::mem::transmute_copy(&plugintype), core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn GetDisabledByIndex<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, index: super::DWORD, clsid: *mut super::CLSID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::GetDisabledByIndex(this, core::mem::transmute_copy(&plugintype), core::mem::transmute_copy(&index), core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn SetDisabled<Identity: IMFPluginControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugintype: super::DWORD, clsid: *const super::IID, disabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl_Impl::SetDisabled(this, core::mem::transmute_copy(&plugintype), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&disabled)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPreferredClsid: GetPreferredClsid::<Identity, OFFSET>,
            GetPreferredClsidByIndex: GetPreferredClsidByIndex::<Identity, OFFSET>,
            SetPreferredClsid: SetPreferredClsid::<Identity, OFFSET>,
            IsDisabled: IsDisabled::<Identity, OFFSET>,
            GetDisabledByIndex: GetDisabledByIndex::<Identity, OFFSET>,
            SetDisabled: SetDisabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPluginControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IMFPluginControl {}
windows_core::imp::define_interface!(IMFPluginControl2, IMFPluginControl2_Vtbl, 0xc6982083_3ddc_45cb_af5e_0f7a8ce4de77);
impl core::ops::Deref for IMFPluginControl2 {
    type Target = IMFPluginControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFPluginControl2, windows_core::IUnknown, IMFPluginControl);
impl IMFPluginControl2 {
    pub unsafe fn SetPolicy(&self, policy: MF_PLUGIN_CONTROL_POLICY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPolicy)(windows_core::Interface::as_raw(self), policy) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPluginControl2_Vtbl {
    pub base__: IMFPluginControl_Vtbl,
    pub SetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, MF_PLUGIN_CONTROL_POLICY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IMFPluginControl2_Impl: IMFPluginControl_Impl {
    fn SetPolicy(&self, policy: MF_PLUGIN_CONTROL_POLICY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IMFPluginControl2_Vtbl {
    pub const fn new<Identity: IMFPluginControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPolicy<Identity: IMFPluginControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: MF_PLUGIN_CONTROL_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPluginControl2_Impl::SetPolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        Self { base__: IMFPluginControl_Vtbl::new::<Identity, OFFSET>(), SetPolicy: SetPolicy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPluginControl2 as windows_core::Interface>::IID || iid == &<IMFPluginControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IMFPluginControl2 {}
windows_core::imp::define_interface!(IMFRemoteAsyncCallback, IMFRemoteAsyncCallback_Vtbl, 0xa27003d0_2354_4f2a_8d6a_ab7cff15437e);
windows_core::imp::interface_hierarchy!(IMFRemoteAsyncCallback, windows_core::IUnknown);
impl IMFRemoteAsyncCallback {
    pub unsafe fn Invoke<P1>(&self, hr: windows_core::HRESULT, premoteresult: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), hr, premoteresult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRemoteAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFRemoteAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, hr: windows_core::HRESULT, premoteresult: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IMFRemoteAsyncCallback_Vtbl {
    pub const fn new<Identity: IMFRemoteAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IMFRemoteAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, premoteresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRemoteAsyncCallback_Impl::Invoke(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&premoteresult)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRemoteAsyncCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRemoteAsyncCallback {}
windows_core::imp::define_interface!(IMFSample, IMFSample_Vtbl, 0xc40a00f2_b93a_4d80_ae8c_5a1c634f58e4);
impl core::ops::Deref for IMFSample {
    type Target = IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSample, windows_core::IUnknown, IMFAttributes);
impl IMFSample {
    #[cfg(feature = "shared")]
    pub unsafe fn GetSampleFlags(&self, pdwsampleflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSampleFlags)(windows_core::Interface::as_raw(self), pdwsampleflags as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleFlags(&self, dwsampleflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleFlags)(windows_core::Interface::as_raw(self), dwsampleflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSampleTime(&self, phnssampletime: *mut super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSampleTime)(windows_core::Interface::as_raw(self), phnssampletime as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleTime(&self, hnssampletime: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleTime)(windows_core::Interface::as_raw(self), hnssampletime) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSampleDuration(&self, phnssampleduration: *mut super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSampleDuration)(windows_core::Interface::as_raw(self), phnssampleduration as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSampleDuration(&self, hnssampleduration: super::LONGLONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleDuration)(windows_core::Interface::as_raw(self), hnssampleduration) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferCount(&self, pdwbuffercount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferCount)(windows_core::Interface::as_raw(self), pdwbuffercount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBufferByIndex(&self, dwindex: super::DWORD, ppbuffer: *mut Option<IMFMediaBuffer>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferByIndex)(windows_core::Interface::as_raw(self), dwindex, core::mem::transmute(ppbuffer)) }
    }
    pub unsafe fn ConvertToContiguousBuffer(&self, ppbuffer: *mut Option<IMFMediaBuffer>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ConvertToContiguousBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbuffer)) }
    }
    pub unsafe fn AddBuffer<P0>(&self, pbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveBufferByIndex(&self, dwindex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveBufferByIndex)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn RemoveAllBuffers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllBuffers)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTotalLength(&self, pcbtotallength: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), pcbtotallength as _) }
    }
    pub unsafe fn CopyToBuffer<P0>(&self, pbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyToBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSample_Vtbl {
    pub base__: IMFAttributes_Vtbl,
    #[cfg(feature = "shared")]
    pub GetSampleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSampleFlags: usize,
    #[cfg(feature = "shared")]
    pub SetSampleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleFlags: usize,
    #[cfg(feature = "shared")]
    pub GetSampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSampleTime: usize,
    #[cfg(feature = "shared")]
    pub SetSampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleTime: usize,
    #[cfg(feature = "shared")]
    pub GetSampleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSampleDuration: usize,
    #[cfg(feature = "shared")]
    pub SetSampleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSampleDuration: usize,
    #[cfg(feature = "shared")]
    pub GetBufferCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferCount: usize,
    #[cfg(feature = "shared")]
    pub GetBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBufferByIndex: usize,
    pub ConvertToContiguousBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RemoveBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveBufferByIndex: usize,
    pub RemoveAllBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetTotalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTotalLength: usize,
    pub CopyToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSample_Impl: IMFAttributes_Impl {
    fn GetSampleFlags(&self, pdwsampleflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetSampleFlags(&self, dwsampleflags: super::DWORD) -> windows_core::Result<()>;
    fn GetSampleTime(&self, phnssampletime: *mut super::LONGLONG) -> windows_core::Result<()>;
    fn SetSampleTime(&self, hnssampletime: super::LONGLONG) -> windows_core::Result<()>;
    fn GetSampleDuration(&self, phnssampleduration: *mut super::LONGLONG) -> windows_core::Result<()>;
    fn SetSampleDuration(&self, hnssampleduration: super::LONGLONG) -> windows_core::Result<()>;
    fn GetBufferCount(&self, pdwbuffercount: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetBufferByIndex(&self, dwindex: super::DWORD, ppbuffer: windows_core::OutRef<IMFMediaBuffer>) -> windows_core::Result<()>;
    fn ConvertToContiguousBuffer(&self, ppbuffer: windows_core::OutRef<IMFMediaBuffer>) -> windows_core::Result<()>;
    fn AddBuffer(&self, pbuffer: windows_core::Ref<IMFMediaBuffer>) -> windows_core::Result<()>;
    fn RemoveBufferByIndex(&self, dwindex: super::DWORD) -> windows_core::Result<()>;
    fn RemoveAllBuffers(&self) -> windows_core::Result<()>;
    fn GetTotalLength(&self, pcbtotallength: *mut super::DWORD) -> windows_core::Result<()>;
    fn CopyToBuffer(&self, pbuffer: windows_core::Ref<IMFMediaBuffer>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSample_Vtbl {
    pub const fn new<Identity: IMFSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSampleFlags<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsampleflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetSampleFlags(this, core::mem::transmute_copy(&pdwsampleflags)).into()
            }
        }
        unsafe extern "system" fn SetSampleFlags<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsampleflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleFlags(this, core::mem::transmute_copy(&dwsampleflags)).into()
            }
        }
        unsafe extern "system" fn GetSampleTime<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnssampletime: *mut super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetSampleTime(this, core::mem::transmute_copy(&phnssampletime)).into()
            }
        }
        unsafe extern "system" fn SetSampleTime<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssampletime: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleTime(this, core::mem::transmute_copy(&hnssampletime)).into()
            }
        }
        unsafe extern "system" fn GetSampleDuration<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnssampleduration: *mut super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetSampleDuration(this, core::mem::transmute_copy(&phnssampleduration)).into()
            }
        }
        unsafe extern "system" fn SetSampleDuration<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssampleduration: super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::SetSampleDuration(this, core::mem::transmute_copy(&hnssampleduration)).into()
            }
        }
        unsafe extern "system" fn GetBufferCount<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbuffercount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetBufferCount(this, core::mem::transmute_copy(&pdwbuffercount)).into()
            }
        }
        unsafe extern "system" fn GetBufferByIndex<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetBufferByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn ConvertToContiguousBuffer<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::ConvertToContiguousBuffer(this, core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn AddBuffer<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::AddBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn RemoveBufferByIndex<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::RemoveBufferByIndex(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn RemoveAllBuffers<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::RemoveAllBuffers(this).into()
            }
        }
        unsafe extern "system" fn GetTotalLength<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbtotallength: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::GetTotalLength(this, core::mem::transmute_copy(&pcbtotallength)).into()
            }
        }
        unsafe extern "system" fn CopyToBuffer<Identity: IMFSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSample_Impl::CopyToBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetSampleFlags: GetSampleFlags::<Identity, OFFSET>,
            SetSampleFlags: SetSampleFlags::<Identity, OFFSET>,
            GetSampleTime: GetSampleTime::<Identity, OFFSET>,
            SetSampleTime: SetSampleTime::<Identity, OFFSET>,
            GetSampleDuration: GetSampleDuration::<Identity, OFFSET>,
            SetSampleDuration: SetSampleDuration::<Identity, OFFSET>,
            GetBufferCount: GetBufferCount::<Identity, OFFSET>,
            GetBufferByIndex: GetBufferByIndex::<Identity, OFFSET>,
            ConvertToContiguousBuffer: ConvertToContiguousBuffer::<Identity, OFFSET>,
            AddBuffer: AddBuffer::<Identity, OFFSET>,
            RemoveBufferByIndex: RemoveBufferByIndex::<Identity, OFFSET>,
            RemoveAllBuffers: RemoveAllBuffers::<Identity, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, OFFSET>,
            CopyToBuffer: CopyToBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSample as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSample {}
windows_core::imp::define_interface!(IMFSampleOutputStream, IMFSampleOutputStream_Vtbl, 0x8feed468_6f7e_440d_869a_49bdd283ad0d);
windows_core::imp::interface_hierarchy!(IMFSampleOutputStream, windows_core::IUnknown);
impl IMFSampleOutputStream {
    pub unsafe fn BeginWriteSample<P0, P1, P2>(&self, psample: P0, pcallback: P1, punkstate: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSample>,
        P1: windows_core::Param<IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginWriteSample)(windows_core::Interface::as_raw(self), psample.param().abi(), pcallback.param().abi(), punkstate.param().abi()) }
    }
    pub unsafe fn EndWriteSample<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndWriteSample)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSampleOutputStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginWriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndWriteSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSampleOutputStream_Impl: windows_core::IUnknownImpl {
    fn BeginWriteSample(&self, psample: windows_core::Ref<IMFSample>, pcallback: windows_core::Ref<IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndWriteSample(&self, presult: windows_core::Ref<IMFAsyncResult>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IMFSampleOutputStream_Vtbl {
    pub const fn new<Identity: IMFSampleOutputStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginWriteSample<Identity: IMFSampleOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psample: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleOutputStream_Impl::BeginWriteSample(this, core::mem::transmute_copy(&psample), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndWriteSample<Identity: IMFSampleOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleOutputStream_Impl::EndWriteSample(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMFSampleOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleOutputStream_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginWriteSample: BeginWriteSample::<Identity, OFFSET>,
            EndWriteSample: EndWriteSample::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSampleOutputStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSampleOutputStream {}
windows_core::imp::define_interface!(IMFSecureBuffer, IMFSecureBuffer_Vtbl, 0xc1209904_e584_4752_a2d6_7f21693f8b21);
windows_core::imp::interface_hierarchy!(IMFSecureBuffer, windows_core::IUnknown);
impl IMFSecureBuffer {
    pub unsafe fn GetIdentifier(&self, pguididentifier: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), pguididentifier as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSecureBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IMFSecureBuffer_Impl: windows_core::IUnknownImpl {
    fn GetIdentifier(&self, pguididentifier: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IMFSecureBuffer_Vtbl {
    pub const fn new<Identity: IMFSecureBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentifier<Identity: IMFSecureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguididentifier: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSecureBuffer_Impl::GetIdentifier(this, core::mem::transmute_copy(&pguididentifier)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSecureBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSecureBuffer {}
windows_core::imp::define_interface!(IMFVideoMediaType, IMFVideoMediaType_Vtbl, 0xb99f381f_a8f9_47a2_a5af_ca3a225a3890);
impl core::ops::Deref for IMFVideoMediaType {
    type Target = IMFMediaType;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoMediaType, windows_core::IUnknown, IMFAttributes, IMFMediaType);
impl IMFVideoMediaType {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetVideoFormat(&self) -> *const MFVIDEOFORMAT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoFormat)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetVideoRepresentation(&self, guidrepresentation: windows_core::GUID, ppvrepresentation: *mut super::LPVOID, lstride: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoRepresentation)(windows_core::Interface::as_raw(self), guidrepresentation, ppvrepresentation as _, lstride) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoMediaType_Vtbl {
    pub base__: IMFMediaType_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetVideoFormat: unsafe extern "system" fn(*mut core::ffi::c_void) -> *const MFVIDEOFORMAT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetVideoFormat: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetVideoRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut super::LPVOID, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetVideoRepresentation: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFVideoMediaType_Impl: IMFMediaType_Impl {
    fn GetVideoFormat(&self) -> *const MFVIDEOFORMAT;
    fn GetVideoRepresentation(&self, guidrepresentation: &windows_core::GUID, ppvrepresentation: *mut super::LPVOID, lstride: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFVideoMediaType_Vtbl {
    pub const fn new<Identity: IMFVideoMediaType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVideoFormat<Identity: IMFVideoMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *const MFVIDEOFORMAT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoMediaType_Impl::GetVideoFormat(this)
            }
        }
        unsafe extern "system" fn GetVideoRepresentation<Identity: IMFVideoMediaType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidrepresentation: windows_core::GUID, ppvrepresentation: *mut super::LPVOID, lstride: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoMediaType_Impl::GetVideoRepresentation(this, core::mem::transmute(&guidrepresentation), core::mem::transmute_copy(&ppvrepresentation), core::mem::transmute_copy(&lstride)).into()
            }
        }
        Self {
            base__: IMFMediaType_Vtbl::new::<Identity, OFFSET>(),
            GetVideoFormat: GetVideoFormat::<Identity, OFFSET>,
            GetVideoRepresentation: GetVideoRepresentation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoMediaType as windows_core::Interface>::IID || iid == &<IMFAttributes as windows_core::Interface>::IID || iid == &<IMFMediaType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFVideoMediaType {}
pub const MEAudioSessionDeviceRemoved: i32 = 315;
pub const MEAudioSessionDisconnected: i32 = 320;
pub const MEAudioSessionExclusiveModeOverride: i32 = 321;
pub const MEAudioSessionFormatChanged: i32 = 319;
pub const MEAudioSessionGroupingParamChanged: i32 = 317;
pub const MEAudioSessionIconChanged: i32 = 318;
pub const MEAudioSessionNameChanged: i32 = 313;
pub const MEAudioSessionServerShutdown: i32 = 316;
pub const MEAudioSessionVolumeChanged: i32 = 314;
pub const MEBufferingStarted: i32 = 122;
pub const MEBufferingStopped: i32 = 123;
pub const MEByteStreamCharacteristicsChanged: i32 = 700;
pub const MECaptureAudioSessionDeviceRemoved: i32 = 323;
pub const MECaptureAudioSessionDisconnected: i32 = 325;
pub const MECaptureAudioSessionExclusiveModeOverride: i32 = 326;
pub const MECaptureAudioSessionFormatChanged: i32 = 324;
pub const MECaptureAudioSessionServerShutdown: i32 = 327;
pub const MECaptureAudioSessionVolumeChanged: i32 = 322;
pub const MEConnectEnd: i32 = 125;
pub const MEConnectStart: i32 = 124;
pub const MEContentProtectionMessage: i32 = 402;
pub const MEContentProtectionMetadata: i32 = 900;
pub const MEDeviceThermalStateChanged: i32 = 950;
pub const MEEnablerCompleted: i32 = 119;
pub const MEEnablerProgress: i32 = 118;
pub const MEEncodingParameters: i32 = 803;
pub const MEEndOfPresentation: i32 = 211;
pub const MEEndOfPresentationSegment: i32 = 218;
pub const MEEndOfStream: i32 = 212;
pub const MEError: i32 = 1;
pub const MEExtendedType: i32 = 2;
pub const MEGenericV1Anchor: i32 = 3;
pub const MEIndividualizationCompleted: i32 = 117;
pub const MEIndividualizationStart: i32 = 116;
pub const MELicenseAcquisitionCompleted: i32 = 115;
pub const MELicenseAcquisitionStart: i32 = 114;
pub const MEMediaSample: i32 = 213;
pub const MENewPresentation: i32 = 113;
pub const MENewStream: i32 = 205;
pub const MENonFatalError: i32 = 3;
pub const MEPolicyChanged: i32 = 401;
pub const MEPolicyError: i32 = 120;
pub const MEPolicyReport: i32 = 121;
pub const MEPolicySet: i32 = 403;
pub const MEQualityNotify: i32 = 311;
pub const MEReconnectEnd: i32 = 127;
pub const MEReconnectStart: i32 = 126;
pub const MERendererEvent: i32 = 128;
pub const MEReservedMax: i32 = 10000;
pub const MESequencerSourceTopologyUpdated: i32 = 222;
pub const MESessionCapabilitiesChanged: i32 = 110;
pub const MESessionClosed: i32 = 106;
pub const MESessionEnded: i32 = 107;
pub const MESessionNotifyPresentationTime: i32 = 112;
pub const MESessionPaused: i32 = 104;
pub const MESessionRateChanged: i32 = 108;
pub const MESessionScrubSampleComplete: i32 = 109;
pub const MESessionStarted: i32 = 103;
pub const MESessionStopped: i32 = 105;
pub const MESessionStreamSinkFormatChanged: i32 = 129;
pub const MESessionTopologiesCleared: i32 = 102;
pub const MESessionTopologySet: i32 = 101;
pub const MESessionTopologyStatus: i32 = 111;
pub const MESessionUnknown: i32 = 100;
pub const MESessionV1Anchor: i32 = 129;
pub const MESinkInvalidated: i32 = 312;
pub const MESinkUnknown: i32 = 300;
pub const MESinkV1Anchor: i32 = 321;
pub const MESinkV2Anchor: i32 = 327;
pub const MESourceCharacteristicsChanged: i32 = 219;
pub const MESourceMetadataChanged: i32 = 221;
pub const MESourcePaused: i32 = 209;
pub const MESourceRateChangeRequested: i32 = 220;
pub const MESourceRateChanged: i32 = 217;
pub const MESourceSeeked: i32 = 203;
pub const MESourceStarted: i32 = 201;
pub const MESourceStopped: i32 = 207;
pub const MESourceUnknown: i32 = 200;
pub const MESourceV1Anchor: i32 = 222;
pub const MEStreamFormatChanged: i32 = 216;
pub const MEStreamPaused: i32 = 210;
pub const MEStreamSeeked: i32 = 204;
pub const MEStreamSinkDeviceChanged: i32 = 310;
pub const MEStreamSinkFormatChanged: i32 = 309;
pub const MEStreamSinkFormatInvalidated: i32 = 802;
pub const MEStreamSinkMarker: i32 = 306;
pub const MEStreamSinkPaused: i32 = 303;
pub const MEStreamSinkPrerolled: i32 = 307;
pub const MEStreamSinkRateChanged: i32 = 304;
pub const MEStreamSinkRequestSample: i32 = 305;
pub const MEStreamSinkScrubSampleComplete: i32 = 308;
pub const MEStreamSinkStarted: i32 = 301;
pub const MEStreamSinkStopped: i32 = 302;
pub const MEStreamStarted: i32 = 202;
pub const MEStreamStopped: i32 = 208;
pub const MEStreamThinMode: i32 = 215;
pub const MEStreamTick: i32 = 214;
pub const METransformDrainComplete: i32 = 603;
pub const METransformHaveOutput: i32 = 602;
pub const METransformInputStreamStateChanged: i32 = 605;
pub const METransformMarker: i32 = 604;
pub const METransformNeedInput: i32 = 601;
pub const METransformUnknown: i32 = 600;
pub const METrustUnknown: i32 = 400;
pub const METrustV1Anchor: i32 = 403;
pub const MEUnknown: i32 = 0;
pub const MEUpdatedStream: i32 = 206;
pub const MEVideoCaptureDevicePreempted: i32 = 801;
pub const MEVideoCaptureDeviceRemoved: i32 = 800;
pub const MEWMDRMIndividualizationCompleted: i32 = 508;
pub const MEWMDRMIndividualizationProgress: i32 = 513;
pub const MEWMDRMLicenseAcquisitionCompleted: i32 = 506;
pub const MEWMDRMLicenseBackupCompleted: i32 = 500;
pub const MEWMDRMLicenseBackupProgress: i32 = 501;
pub const MEWMDRMLicenseRestoreCompleted: i32 = 502;
pub const MEWMDRMLicenseRestoreProgress: i32 = 503;
pub const MEWMDRMLicenseStoreCleaned: i32 = 515;
pub const MEWMDRMProximityCompleted: i32 = 514;
pub const MEWMDRMRevocationDownloadCompleted: i32 = 516;
pub const MEWMDRMV1Anchor: i32 = 516;
pub type MF2DBuffer_LockFlags = i32;
pub const MF2DBuffer_LockFlags_ForceDWORD: MF2DBuffer_LockFlags = 2147483647;
pub const MF2DBuffer_LockFlags_LockTypeMask: MF2DBuffer_LockFlags = 3;
pub const MF2DBuffer_LockFlags_Read: MF2DBuffer_LockFlags = 1;
pub const MF2DBuffer_LockFlags_ReadWrite: MF2DBuffer_LockFlags = 3;
pub const MF2DBuffer_LockFlags_Write: MF2DBuffer_LockFlags = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFARGB {
    pub rgbBlue: super::BYTE,
    pub rgbGreen: super::BYTE,
    pub rgbRed: super::BYTE,
    pub rgbAlpha: super::BYTE,
}
pub const MFASYNC_BLOCKING_CALLBACK: i32 = 4;
pub const MFASYNC_CALLBACK_QUEUE_ALL: u32 = 4294967295;
pub const MFASYNC_CALLBACK_QUEUE_IO: i32 = 3;
pub const MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION: i32 = 7;
pub const MFASYNC_CALLBACK_QUEUE_MULTITHREADED: i32 = 5;
pub const MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK: u32 = 4294901760;
pub const MFASYNC_CALLBACK_QUEUE_RT: i32 = 2;
pub const MFASYNC_CALLBACK_QUEUE_STANDARD: i32 = 1;
pub const MFASYNC_CALLBACK_QUEUE_TIMER: i32 = 4;
pub const MFASYNC_CALLBACK_QUEUE_UNDEFINED: i32 = 0;
pub const MFASYNC_FAST_IO_PROCESSING_CALLBACK: i32 = 1;
pub const MFASYNC_LOCALIZE_REMOTE_CALLBACK: i32 = 16;
pub const MFASYNC_REPLY_CALLBACK: i32 = 8;
pub const MFASYNC_SIGNAL_CALLBACK: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFAYUVSample {
    pub bCrValue: super::BYTE,
    pub bCbValue: super::BYTE,
    pub bYValue: super::BYTE,
    pub bSampleAlpha8: super::BYTE,
}
pub const MFBYTESTREAM_DOES_NOT_USE_NETWORK: i32 = 2048;
pub const MFBYTESTREAM_HAS_SLOW_SEEK: i32 = 256;
pub const MFBYTESTREAM_IS_DIRECTORY: i32 = 128;
pub const MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED: i32 = 512;
pub const MFBYTESTREAM_IS_READABLE: i32 = 1;
pub const MFBYTESTREAM_IS_REMOTE: i32 = 8;
pub const MFBYTESTREAM_IS_SEEKABLE: i32 = 4;
pub const MFBYTESTREAM_IS_WRITABLE: i32 = 2;
pub const MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO: i32 = 1;
pub type MFBYTESTREAM_SEEK_ORIGIN = i32;
pub const MFBYTESTREAM_SHARE_WRITE: i32 = 1024;
pub type MFNominalRange = i32;
pub const MFNominalRange_0_255: MFNominalRange = 1;
pub const MFNominalRange_16_235: MFNominalRange = 2;
pub const MFNominalRange_48_208: MFNominalRange = 3;
pub const MFNominalRange_64_127: MFNominalRange = 4;
pub const MFNominalRange_ForceDWORD: MFNominalRange = 2147483647;
pub const MFNominalRange_Last: MFNominalRange = 5;
pub const MFNominalRange_Normal: MFNominalRange = 1;
pub const MFNominalRange_Unknown: MFNominalRange = 0;
pub const MFNominalRange_Wide: MFNominalRange = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFOffset {
    pub fract: super::WORD,
    pub value: i16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MFPaletteEntry {
    pub ARGB: MFARGB,
    pub AYCbCr: MFAYUVSample,
}
#[cfg(feature = "shared")]
impl Default for MFPaletteEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFRatio {
    pub Numerator: super::DWORD,
    pub Denominator: super::DWORD,
}
pub type MFStandardVideoFormat = i32;
pub const MFStdVideoFormat_ATSC_HD1080i: MFStandardVideoFormat = 8;
pub const MFStdVideoFormat_ATSC_HD720p: MFStandardVideoFormat = 9;
pub const MFStdVideoFormat_ATSC_SD480i: MFStandardVideoFormat = 7;
pub const MFStdVideoFormat_DVD_NTSC: MFStandardVideoFormat = 3;
pub const MFStdVideoFormat_DVD_PAL: MFStandardVideoFormat = 4;
pub const MFStdVideoFormat_DV_NTSC: MFStandardVideoFormat = 6;
pub const MFStdVideoFormat_DV_PAL: MFStandardVideoFormat = 5;
pub const MFStdVideoFormat_NTSC: MFStandardVideoFormat = 1;
pub const MFStdVideoFormat_PAL: MFStandardVideoFormat = 2;
pub const MFStdVideoFormat_reserved: MFStandardVideoFormat = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFT_REGISTER_TYPE_INFO {
    pub guidMajorType: windows_core::GUID,
    pub guidSubtype: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct MFVIDEOFORMAT {
    pub dwSize: super::DWORD,
    pub videoInfo: MFVideoInfo,
    pub guidFormat: windows_core::GUID,
    pub compressedInfo: MFVideoCompressedInfo,
    pub surfaceInfo: MFVideoSurfaceInfo,
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for MFVIDEOFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFVideoArea {
    pub OffsetX: MFOffset,
    pub OffsetY: MFOffset,
    pub Area: super::SIZE,
}
pub type MFVideoChromaSubsampling = i32;
pub const MFVideoChromaSubsampling_Cosited: MFVideoChromaSubsampling = 7;
pub const MFVideoChromaSubsampling_DV_PAL: MFVideoChromaSubsampling = 6;
pub const MFVideoChromaSubsampling_ForceDWORD: MFVideoChromaSubsampling = 2147483647;
pub const MFVideoChromaSubsampling_Horizontally_Cosited: MFVideoChromaSubsampling = 4;
pub const MFVideoChromaSubsampling_Last: MFVideoChromaSubsampling = 8;
pub const MFVideoChromaSubsampling_MPEG1: MFVideoChromaSubsampling = 1;
pub const MFVideoChromaSubsampling_MPEG2: MFVideoChromaSubsampling = 5;
pub const MFVideoChromaSubsampling_ProgressiveChroma: MFVideoChromaSubsampling = 8;
pub const MFVideoChromaSubsampling_Unknown: MFVideoChromaSubsampling = 0;
pub const MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes: MFVideoChromaSubsampling = 1;
pub const MFVideoChromaSubsampling_Vertically_Cosited: MFVideoChromaSubsampling = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFVideoCompressedInfo {
    pub AvgBitrate: super::LONGLONG,
    pub AvgBitErrorRate: super::LONGLONG,
    pub MaxKeyFrameSpacing: super::DWORD,
}
pub const MFVideoFlag_AnalogProtected: MFVideoFlags = 32;
pub const MFVideoFlag_BottomUpLinearRep: MFVideoFlags = 524288;
pub const MFVideoFlag_DigitallyProtected: MFVideoFlags = 64;
pub const MFVideoFlag_FieldRepeatCountMask: MFVideoFlags = 1792;
pub const MFVideoFlag_FieldRepeatCountShift: MFVideoFlags = 8;
pub const MFVideoFlag_LowerFieldFirst: MFVideoFlags = 262144;
pub const MFVideoFlag_PAD_TO_16x9: MFVideoFlags = 2;
pub const MFVideoFlag_PAD_TO_4x3: MFVideoFlags = 1;
pub const MFVideoFlag_PAD_TO_Mask: MFVideoFlags = 3;
pub const MFVideoFlag_PAD_TO_None: MFVideoFlags = 0;
pub const MFVideoFlag_PanScanEnabled: MFVideoFlags = 131072;
pub const MFVideoFlag_ProgressiveContent: MFVideoFlags = 128;
pub const MFVideoFlag_ProgressiveSeqReset: MFVideoFlags = 2048;
pub const MFVideoFlag_SrcContentHint16x9: MFVideoFlags = 4;
pub const MFVideoFlag_SrcContentHint235_1: MFVideoFlags = 8;
pub const MFVideoFlag_SrcContentHintMask: MFVideoFlags = 28;
pub const MFVideoFlag_SrcContentHintNone: MFVideoFlags = 0;
pub type MFVideoFlags = i32;
pub const MFVideoFlags_DXVASurface: MFVideoFlags = 1048576;
pub const MFVideoFlags_ForceQWORD: MFVideoFlags = 2147483647;
pub const MFVideoFlags_RenderTargetSurface: MFVideoFlags = 4194304;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFVideoInfo {
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub PixelAspectRatio: MFRatio,
    pub SourceChromaSubsampling: MFVideoChromaSubsampling,
    pub InterlaceMode: MFVideoInterlaceMode,
    pub TransferFunction: MFVideoTransferFunction,
    pub ColorPrimaries: MFVideoPrimaries,
    pub TransferMatrix: MFVideoTransferMatrix,
    pub SourceLighting: MFVideoLighting,
    pub FramesPerSecond: MFRatio,
    pub NominalRange: MFNominalRange,
    pub GeometricAperture: MFVideoArea,
    pub MinimumDisplayAperture: MFVideoArea,
    pub PanScanAperture: MFVideoArea,
    pub VideoFlags: u64,
}
pub type MFVideoInterlaceMode = i32;
pub const MFVideoInterlace_FieldInterleavedLowerFirst: MFVideoInterlaceMode = 4;
pub const MFVideoInterlace_FieldInterleavedUpperFirst: MFVideoInterlaceMode = 3;
pub const MFVideoInterlace_FieldSingleLower: MFVideoInterlaceMode = 6;
pub const MFVideoInterlace_FieldSingleLowerFirst: MFVideoInterlaceMode = 6;
pub const MFVideoInterlace_FieldSingleUpper: MFVideoInterlaceMode = 5;
pub const MFVideoInterlace_FieldSingleUpperFirst: MFVideoInterlaceMode = 5;
pub const MFVideoInterlace_ForceDWORD: MFVideoInterlaceMode = 2147483647;
pub const MFVideoInterlace_Last: MFVideoInterlaceMode = 8;
pub const MFVideoInterlace_MixedInterlaceOrProgressive: MFVideoInterlaceMode = 7;
pub const MFVideoInterlace_Progressive: MFVideoInterlaceMode = 2;
pub const MFVideoInterlace_Unknown: MFVideoInterlaceMode = 0;
pub type MFVideoLighting = i32;
pub const MFVideoLighting_ForceDWORD: MFVideoLighting = 2147483647;
pub const MFVideoLighting_Last: MFVideoLighting = 5;
pub const MFVideoLighting_Unknown: MFVideoLighting = 0;
pub const MFVideoLighting_bright: MFVideoLighting = 1;
pub const MFVideoLighting_dark: MFVideoLighting = 4;
pub const MFVideoLighting_dim: MFVideoLighting = 3;
pub const MFVideoLighting_office: MFVideoLighting = 2;
pub type MFVideoPrimaries = i32;
pub const MFVideoPrimaries_ACES: MFVideoPrimaries = 12;
pub const MFVideoPrimaries_BT2020: MFVideoPrimaries = 9;
pub const MFVideoPrimaries_BT470_2_SysBG: MFVideoPrimaries = 4;
pub const MFVideoPrimaries_BT470_2_SysM: MFVideoPrimaries = 3;
pub const MFVideoPrimaries_BT709: MFVideoPrimaries = 2;
pub const MFVideoPrimaries_DCI_P3: MFVideoPrimaries = 11;
pub const MFVideoPrimaries_Display_P3: MFVideoPrimaries = 13;
pub const MFVideoPrimaries_EBU3213: MFVideoPrimaries = 7;
pub const MFVideoPrimaries_ForceDWORD: MFVideoPrimaries = 2147483647;
pub const MFVideoPrimaries_Last: MFVideoPrimaries = 14;
pub const MFVideoPrimaries_SMPTE170M: MFVideoPrimaries = 5;
pub const MFVideoPrimaries_SMPTE240M: MFVideoPrimaries = 6;
pub const MFVideoPrimaries_SMPTE_C: MFVideoPrimaries = 8;
pub const MFVideoPrimaries_Unknown: MFVideoPrimaries = 0;
pub const MFVideoPrimaries_XYZ: MFVideoPrimaries = 10;
pub const MFVideoPrimaries_reserved: MFVideoPrimaries = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MFVideoSurfaceInfo {
    pub Format: super::DWORD,
    pub PaletteEntries: super::DWORD,
    pub Palette: [MFPaletteEntry; 1],
}
#[cfg(feature = "shared")]
impl Default for MFVideoSurfaceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFVideoTransFunc_10: MFVideoTransferFunction = 1;
pub const MFVideoTransFunc_10_rel: MFVideoTransferFunction = 17;
pub const MFVideoTransFunc_18: MFVideoTransferFunction = 2;
pub const MFVideoTransFunc_20: MFVideoTransferFunction = 3;
pub const MFVideoTransFunc_2020: MFVideoTransferFunction = 13;
pub const MFVideoTransFunc_2020_const: MFVideoTransferFunction = 12;
pub const MFVideoTransFunc_2084: MFVideoTransferFunction = 15;
pub const MFVideoTransFunc_22: MFVideoTransferFunction = 4;
pub const MFVideoTransFunc_240M: MFVideoTransferFunction = 6;
pub const MFVideoTransFunc_26: MFVideoTransferFunction = 14;
pub const MFVideoTransFunc_28: MFVideoTransferFunction = 8;
pub const MFVideoTransFunc_709: MFVideoTransferFunction = 5;
pub const MFVideoTransFunc_709_sym: MFVideoTransferFunction = 11;
pub const MFVideoTransFunc_BT1361_ECG: MFVideoTransferFunction = 18;
pub const MFVideoTransFunc_ForceDWORD: MFVideoTransferFunction = 2147483647;
pub const MFVideoTransFunc_HLG: MFVideoTransferFunction = 16;
pub const MFVideoTransFunc_Last: MFVideoTransferFunction = 20;
pub const MFVideoTransFunc_Log_100: MFVideoTransferFunction = 9;
pub const MFVideoTransFunc_Log_316: MFVideoTransferFunction = 10;
pub const MFVideoTransFunc_SMPTE428: MFVideoTransferFunction = 19;
pub const MFVideoTransFunc_Unknown: MFVideoTransferFunction = 0;
pub const MFVideoTransFunc_sRGB: MFVideoTransferFunction = 7;
pub type MFVideoTransferFunction = i32;
pub type MFVideoTransferMatrix = i32;
pub const MFVideoTransferMatrix_BT2020_10: MFVideoTransferMatrix = 4;
pub const MFVideoTransferMatrix_BT2020_12: MFVideoTransferMatrix = 5;
pub const MFVideoTransferMatrix_BT601: MFVideoTransferMatrix = 2;
pub const MFVideoTransferMatrix_BT709: MFVideoTransferMatrix = 1;
pub const MFVideoTransferMatrix_Chroma: MFVideoTransferMatrix = 10;
pub const MFVideoTransferMatrix_Chroma_const: MFVideoTransferMatrix = 11;
pub const MFVideoTransferMatrix_FCC47: MFVideoTransferMatrix = 7;
pub const MFVideoTransferMatrix_ForceDWORD: MFVideoTransferMatrix = 2147483647;
pub const MFVideoTransferMatrix_ICtCp: MFVideoTransferMatrix = 12;
pub const MFVideoTransferMatrix_Identity: MFVideoTransferMatrix = 6;
pub const MFVideoTransferMatrix_Last: MFVideoTransferMatrix = 13;
pub const MFVideoTransferMatrix_SMPTE2085: MFVideoTransferMatrix = 9;
pub const MFVideoTransferMatrix_SMPTE240M: MFVideoTransferMatrix = 3;
pub const MFVideoTransferMatrix_Unknown: MFVideoTransferMatrix = 0;
pub const MFVideoTransferMatrix_YCgCo: MFVideoTransferMatrix = 8;
pub const MF_ACCESSMODE_READ: MF_FILE_ACCESSMODE = 1;
pub const MF_ACCESSMODE_READWRITE: MF_FILE_ACCESSMODE = 3;
pub const MF_ACCESSMODE_WRITE: MF_FILE_ACCESSMODE = 2;
pub const MF_ATTRIBUTES_MATCH_ALL_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 2;
pub const MF_ATTRIBUTES_MATCH_INTERSECTION: MF_ATTRIBUTES_MATCH_TYPE = 3;
pub const MF_ATTRIBUTES_MATCH_OUR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 0;
pub const MF_ATTRIBUTES_MATCH_SMALLER: MF_ATTRIBUTES_MATCH_TYPE = 4;
pub const MF_ATTRIBUTES_MATCH_THEIR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 1;
pub type MF_ATTRIBUTES_MATCH_TYPE = i32;
pub const MF_ATTRIBUTE_BLOB: MF_ATTRIBUTE_TYPE = 4113;
pub const MF_ATTRIBUTE_DOUBLE: MF_ATTRIBUTE_TYPE = 5;
pub const MF_ATTRIBUTE_GUID: MF_ATTRIBUTE_TYPE = 72;
pub const MF_ATTRIBUTE_IUNKNOWN: MF_ATTRIBUTE_TYPE = 13;
pub type MF_ATTRIBUTE_SERIALIZE_OPTIONS = i32;
pub const MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF: MF_ATTRIBUTE_SERIALIZE_OPTIONS = 1;
pub const MF_ATTRIBUTE_STRING: MF_ATTRIBUTE_TYPE = 31;
pub type MF_ATTRIBUTE_TYPE = i32;
pub const MF_ATTRIBUTE_UINT32: MF_ATTRIBUTE_TYPE = 19;
pub const MF_ATTRIBUTE_UINT64: MF_ATTRIBUTE_TYPE = 21;
pub const MF_BYTESTREAM_CONTENT_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xfc358289_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_DLNA_PROFILE_ID: windows_core::GUID = windows_core::GUID::from_u128(0xfc35828d_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_DURATION: windows_core::GUID = windows_core::GUID::from_u128(0xfc35828a_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_EFFECTIVE_URL: windows_core::GUID = windows_core::GUID::from_u128(0x9afa0209_89d1_42af_8456_1de6b562d691);
pub const MF_BYTESTREAM_IFO_FILE_URI: windows_core::GUID = windows_core::GUID::from_u128(0xfc35828c_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_LAST_MODIFIED_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xfc35828b_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_ORIGIN_NAME: windows_core::GUID = windows_core::GUID::from_u128(0xfc358288_3cb6_460c_a424_b6681260375a);
pub const MF_BYTESTREAM_TRANSCODED: windows_core::GUID = windows_core::GUID::from_u128(0xb6c5c282_4dc9_4db9_ab48_cf3b6d8bc5e0);
pub type MF_DXGI_DEVICE_MANAGER_MODE = i32;
pub const MF_DXGI_DEVICE_MANAGER_MODE_D3D11: MF_DXGI_DEVICE_MANAGER_MODE = 1;
pub const MF_DXGI_DEVICE_MANAGER_MODE_D3D12: MF_DXGI_DEVICE_MANAGER_MODE = 2;
pub const MF_DXGI_DEVICE_MANAGER_MODE_INVALID: MF_DXGI_DEVICE_MANAGER_MODE = 0;
pub const MF_EVENT_FLAG_NO_WAIT: i32 = 1;
pub const MF_FILEFLAGS_ALLOW_WRITE_SHARING: MF_FILE_FLAGS = 2;
pub const MF_FILEFLAGS_NOBUFFERING: MF_FILE_FLAGS = 1;
pub const MF_FILEFLAGS_NONE: MF_FILE_FLAGS = 0;
pub type MF_FILE_ACCESSMODE = i32;
pub type MF_FILE_FLAGS = i32;
pub type MF_FILE_OPENMODE = i32;
pub const MF_MEDIATYPE_EQUAL_FORMAT_DATA: i32 = 4;
pub const MF_MEDIATYPE_EQUAL_FORMAT_TYPES: i32 = 2;
pub const MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA: i32 = 8;
pub const MF_MEDIATYPE_EQUAL_MAJOR_TYPES: i32 = 1;
pub const MF_OPENMODE_APPEND_IF_EXIST: MF_FILE_OPENMODE = 3;
pub const MF_OPENMODE_DELETE_IF_EXIST: MF_FILE_OPENMODE = 4;
pub const MF_OPENMODE_FAIL_IF_EXIST: MF_FILE_OPENMODE = 1;
pub const MF_OPENMODE_FAIL_IF_NOT_EXIST: MF_FILE_OPENMODE = 0;
pub const MF_OPENMODE_RESET_IF_EXIST: MF_FILE_OPENMODE = 2;
pub type MF_PLUGIN_CONTROL_POLICY = i32;
pub const MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 0;
pub const MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 1;
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 2;
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE: MF_PLUGIN_CONTROL_POLICY = 3;
pub type MF_Plugin_Type = i32;
pub const MF_Plugin_Type_MFT: MF_Plugin_Type = 0;
pub const MF_Plugin_Type_MFT_MatchOutputType: MF_Plugin_Type = 2;
pub const MF_Plugin_Type_MediaSource: MF_Plugin_Type = 1;
pub const MF_Plugin_Type_Other: MF_Plugin_Type = -1;
pub type MF_STREAM_STATE = i32;
pub const MF_STREAM_STATE_PAUSED: MF_STREAM_STATE = 1;
pub const MF_STREAM_STATE_RUNNING: MF_STREAM_STATE = 2;
pub const MF_STREAM_STATE_STOPPED: MF_STREAM_STATE = 0;
pub type MediaEventType = u32;
pub const msoBegin: MFBYTESTREAM_SEEK_ORIGIN = 0;
pub const msoCurrent: MFBYTESTREAM_SEEK_ORIGIN = 1;
