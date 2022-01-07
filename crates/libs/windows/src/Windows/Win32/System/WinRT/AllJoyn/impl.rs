pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle();
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.AllJoyn.IWindowsDevicesAllJoynBusAttachmentFactoryInterop";
}
impl IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl {
    pub const fn new<Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDevicesAllJoynBusAttachmentFactoryInteropVtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromWin32Handle(win32handle, enableaboutdata, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDevicesAllJoynBusAttachmentFactoryInterop>, base.5, CreateFromWin32Handle::<Impl, OFFSET>)
    }
}
pub trait IWindowsDevicesAllJoynBusAttachmentInteropImpl: Sized {
    fn Win32Handle();
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusAttachmentInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.AllJoyn.IWindowsDevicesAllJoynBusAttachmentInterop";
}
impl IWindowsDevicesAllJoynBusAttachmentInteropVtbl {
    pub const fn new<Impl: IWindowsDevicesAllJoynBusAttachmentInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDevicesAllJoynBusAttachmentInteropVtbl {
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusAttachmentInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Win32Handle(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDevicesAllJoynBusAttachmentInterop>, base.5, Win32Handle::<Impl, OFFSET>)
    }
}
pub trait IWindowsDevicesAllJoynBusObjectFactoryInteropImpl: Sized {
    fn CreateFromWin32Handle();
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.AllJoyn.IWindowsDevicesAllJoynBusObjectFactoryInterop";
}
impl IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl {
    pub const fn new<Impl: IWindowsDevicesAllJoynBusObjectFactoryInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDevicesAllJoynBusObjectFactoryInteropVtbl {
        unsafe extern "system" fn CreateFromWin32Handle<Impl: IWindowsDevicesAllJoynBusObjectFactoryInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromWin32Handle(win32handle, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDevicesAllJoynBusObjectFactoryInterop>, base.5, CreateFromWin32Handle::<Impl, OFFSET>)
    }
}
pub trait IWindowsDevicesAllJoynBusObjectInteropImpl: Sized {
    fn AddPropertyGetHandler();
    fn AddPropertySetHandler();
    fn Win32Handle();
}
impl ::windows::core::RuntimeName for IWindowsDevicesAllJoynBusObjectInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.AllJoyn.IWindowsDevicesAllJoynBusObjectInterop";
}
impl IWindowsDevicesAllJoynBusObjectInteropVtbl {
    pub const fn new<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDevicesAllJoynBusObjectInteropVtbl {
        unsafe extern "system" fn AddPropertyGetHandler<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPropertyGetHandler(&*(&context as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&interfacename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), callback) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertySetHandler<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callback: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddPropertySetHandler(&*(&context as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&interfacename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), callback) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Win32Handle<Impl: IWindowsDevicesAllJoynBusObjectInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Win32Handle(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDevicesAllJoynBusObjectInterop>, base.5, AddPropertyGetHandler::<Impl, OFFSET>, AddPropertySetHandler::<Impl, OFFSET>, Win32Handle::<Impl, OFFSET>)
    }
}
