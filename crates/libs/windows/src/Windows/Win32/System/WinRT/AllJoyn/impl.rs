pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle(&mut self, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateFromWin32Handle(::core::mem::transmute_copy(&win32handle), ::core::mem::transmute_copy(&enableaboutdata), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsDevicesAllJoynBusAttachmentFactoryInterop, BASE_OFFSET>(),
            CreateFromWin32Handle: CreateFromWin32Handle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusAttachmentFactoryInterop as ::windows::core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusAttachmentInteropImpl: Sized {
    fn Win32Handle(&mut self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusAttachmentInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusAttachmentInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentInteropVtbl {
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Win32Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsDevicesAllJoynBusAttachmentInterop, BASE_OFFSET>(),
            Win32Handle: Win32Handle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusAttachmentInterop as ::windows::core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusObjectFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle(&mut self, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusObjectFactoryInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusObjectFactoryInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateFromWin32Handle(::core::mem::transmute_copy(&win32handle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsDevicesAllJoynBusObjectFactoryInterop, BASE_OFFSET>(),
            CreateFromWin32Handle: CreateFromWin32Handle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusObjectFactoryInterop as ::windows::core::Interface>::IID
    }
}
pub trait IWindowsDevicesAllJoynBusObjectInteropImpl: Sized {
    fn AddPropertyGetHandler(&mut self, context: *const ::core::ffi::c_void, interfacename: ::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()>;
    fn AddPropertySetHandler(&mut self, context: *const ::core::ffi::c_void, interfacename: ::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()>;
    fn Win32Handle(&mut self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusObjectInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectInteropVtbl {
        unsafe extern "system" fn AddPropertyGetHandler<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyGetHandler(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&interfacename), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn AddPropertySetHandler<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertySetHandler(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&interfacename), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Win32Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsDevicesAllJoynBusObjectInterop, BASE_OFFSET>(),
            AddPropertyGetHandler: AddPropertyGetHandler::<Impl, IMPL_OFFSET>,
            AddPropertySetHandler: AddPropertySetHandler::<Impl, IMPL_OFFSET>,
            Win32Handle: Win32Handle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDevicesAllJoynBusObjectInterop as ::windows::core::Interface>::IID
    }
}
