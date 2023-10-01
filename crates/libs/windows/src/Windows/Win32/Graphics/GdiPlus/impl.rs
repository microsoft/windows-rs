pub trait GdiplusAbort_Impl: Sized {
    fn Abort(&self) -> ::windows_core::Result<()>;
}
impl GdiplusAbort_Vtbl {
    pub const fn new<Impl: GdiplusAbort_Impl>() -> GdiplusAbort_Vtbl {
        unsafe extern "system" fn Abort<Impl: GdiplusAbort_Impl>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.Abort().into()
        }
        Self { Abort: Abort::<Impl> }
    }
}
#[doc(hidden)]
struct GdiplusAbort_ImplVtbl<T: GdiplusAbort_Impl>(::std::marker::PhantomData<T>);
impl<T: GdiplusAbort_Impl> GdiplusAbort_ImplVtbl<T> {
    const VTABLE: GdiplusAbort_Vtbl = GdiplusAbort_Vtbl::new::<T>();
}
impl GdiplusAbort {
    pub fn new<'a, T: GdiplusAbort_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &GdiplusAbort_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
    }
}
pub trait IImageBytes_Impl: Sized {
    fn CountBytes(&self, pcb: *mut u32) -> ::windows_core::Result<()>;
    fn LockBytes(&self, cb: u32, uloffset: u32, ppvbytes: *const *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnlockBytes(&self, pvbytes: *const ::core::ffi::c_void, cb: u32, uloffset: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IImageBytes {}
impl IImageBytes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: isize>() -> IImageBytes_Vtbl {
        unsafe extern "system" fn CountBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CountBytes(::core::mem::transmute_copy(&pcb)).into()
        }
        unsafe extern "system" fn LockBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: u32, uloffset: u32, ppvbytes: *const *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockBytes(::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ppvbytes)).into()
        }
        unsafe extern "system" fn UnlockBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytes: *const ::core::ffi::c_void, cb: u32, uloffset: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockBytes(::core::mem::transmute_copy(&pvbytes), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&uloffset)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CountBytes: CountBytes::<Identity, Impl, OFFSET>,
            LockBytes: LockBytes::<Identity, Impl, OFFSET>,
            UnlockBytes: UnlockBytes::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IImageBytes as ::windows_core::ComInterface>::IID
    }
}
