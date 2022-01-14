pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl: Sized {
    fn CreateFromWin32Handle(&mut self, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWindowsDevicesAllJoynBusAttachmentInterop_Impl: Sized {
    fn Win32Handle(&mut self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusAttachmentInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl {
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
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
pub trait IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl: Sized {
    fn CreateFromWin32Handle(&mut self, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IWindowsDevicesAllJoynBusObjectInterop_Impl: Sized {
    fn AddPropertyGetHandler(&mut self, context: *const ::core::ffi::c_void, interfacename: ::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()>;
    fn AddPropertySetHandler(&mut self, context: *const ::core::ffi::c_void, interfacename: ::windows::core::HSTRING, callback: isize) -> ::windows::core::Result<()>;
    fn Win32Handle(&mut self) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectInterop {
    const NAME: &'static str = "";
}
impl IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
        unsafe extern "system" fn AddPropertyGetHandler<Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyGetHandler(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&interfacename), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn AddPropertySetHandler<Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertySetHandler(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&interfacename), ::core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
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
