#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"implement\"`*"]
pub trait ID3DBlob_Impl: Sized {
    fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(&self) -> usize;
}
impl ::windows::core::RuntimeName for ID3DBlob {}
impl ID3DBlob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>() -> ID3DBlob_Vtbl {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferPointer()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferSize()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DBlob as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"implement\"`*"]
pub trait ID3DDestructionNotifier_Impl: Sized {
    fn RegisterDestructionCallback(&self, callbackfn: PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn UnregisterDestructionCallback(&self, callbackid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ID3DDestructionNotifier {}
impl ID3DDestructionNotifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>() -> ID3DDestructionNotifier_Vtbl {
        unsafe extern "system" fn RegisterDestructionCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackfn: PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDestructionCallback(::core::mem::transmute_copy(&callbackfn), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallbackid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDestructionCallback(::core::mem::transmute_copy(&callbackid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDestructionCallback: RegisterDestructionCallback::<Identity, Impl, OFFSET>,
            UnregisterDestructionCallback: UnregisterDestructionCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ID3DDestructionNotifier as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"implement\"`*"]
pub trait ID3DInclude_Impl: Sized {
    fn Open(&self, includetype: D3D_INCLUDE_TYPE, pfilename: &::windows::core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&self, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ID3DInclude_Vtbl {
    pub const fn new<Impl: ID3DInclude_Impl>() -> ID3DInclude_Vtbl {
        unsafe extern "system" fn Open<Impl: ID3DInclude_Impl>(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: ::windows::core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.Open(::core::mem::transmute_copy(&includetype), ::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&pparentdata), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pbytes)).into()
        }
        unsafe extern "system" fn Close<Impl: ID3DInclude_Impl>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows::core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.Close(::core::mem::transmute_copy(&pdata)).into()
        }
        Self { Open: Open::<Impl>, Close: Close::<Impl> }
    }
}
#[doc(hidden)]
struct ID3DInclude_ImplVtbl<T: ID3DInclude_Impl>(::std::marker::PhantomData<T>);
impl<T: ID3DInclude_Impl> ID3DInclude_ImplVtbl<T> {
    const VTABLE: ID3DInclude_Vtbl = ID3DInclude_Vtbl::new::<T>();
}
impl ID3DInclude {
    pub fn new<'a, T: ID3DInclude_Impl>(this: &'a T) -> ::windows::core::ScopedInterface<'a, Self> {
        let this = ::windows::core::ScopedHeap { vtable: &ID3DInclude_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::windows::core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
    }
}
