#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2A_Impl: Sized + IDirectInputA_Impl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2A_Impl, const OFFSET: isize>() -> IDirectInput2A_Vtbl {
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputA_Vtbl::new::<Identity, Impl, OFFSET>(), FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2A as ::windows::core::Interface>::IID || iid == &<IDirectInputA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2W_Impl: Sized + IDirectInputW_Impl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2W_Impl, const OFFSET: isize>() -> IDirectInput2W_Vtbl {
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputW_Vtbl::new::<Identity, Impl, OFFSET>(), FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2W as ::windows::core::Interface>::IID || iid == &<IDirectInputW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7A_Impl: Sized + IDirectInputA_Impl + IDirectInput2A_Impl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7A_Impl, const OFFSET: isize>() -> IDirectInput7A_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2A_Vtbl::new::<Identity, Impl, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7A as ::windows::core::Interface>::IID || iid == &<IDirectInputA as ::windows::core::Interface>::IID || iid == &<IDirectInput2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7W_Impl: Sized + IDirectInputW_Impl + IDirectInput2W_Impl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7W_Impl, const OFFSET: isize>() -> IDirectInput7W_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2W_Vtbl::new::<Identity, Impl, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7W as ::windows::core::Interface>::IID || iid == &<IDirectInputW as ::windows::core::Interface>::IID || iid == &<IDirectInput2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8A_Impl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8A>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: &LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&mut self, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: &LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&mut self, param0: &LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>() -> IDirectInput8A_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FindDevice: FindDevice::<Identity, Impl, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, Impl, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput8A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8W_Impl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8W>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: &LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: &LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&mut self, param0: &LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>() -> IDirectInput8W_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            FindDevice: FindDevice::<Identity, Impl, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, Impl, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput8W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputA_Impl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: &LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>() -> IDirectInputA_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2A_Impl: Sized + IDirectInputDeviceA_Impl {
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: &LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: &LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>() -> IDirectInputDevice2A_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceA_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2A as ::windows::core::Interface>::IID || iid == &<IDirectInputDeviceA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2W_Impl: Sized + IDirectInputDeviceW_Impl {
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: &LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: &LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>() -> IDirectInputDevice2W_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceW_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2W as ::windows::core::Interface>::IID || iid == &<IDirectInputDeviceW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7A_Impl: Sized + IDirectInputDeviceA_Impl + IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>() -> IDirectInputDevice7A_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2A_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as ::windows::core::Interface>::IID || iid == &<IDirectInputDeviceA as ::windows::core::Interface>::IID || iid == &<IDirectInputDevice2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7W_Impl: Sized + IDirectInputDeviceW_Impl + IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PWSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>() -> IDirectInputDevice7W_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2W_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as ::windows::core::Interface>::IID || iid == &<IDirectInputDeviceW as ::windows::core::Interface>::IID || iid == &<IDirectInputDevice2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8A_Impl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: &LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&mut self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&mut self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&mut self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&mut self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&mut self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: &LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: &LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&mut self, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&mut self, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&mut self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>() -> IDirectInputDevice8A_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, Impl, OFFSET>,
            SetActionMap: SetActionMap::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice8A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8W_Impl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: &LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&mut self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&mut self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&mut self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&mut self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&mut self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: &LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: &LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PWSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&mut self, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&mut self, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&mut self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>() -> IDirectInputDevice8W_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateEffect: CreateEffect::<Identity, Impl, OFFSET>,
            EnumEffects: EnumEffects::<Identity, Impl, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            Poll: Poll::<Identity, Impl, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, Impl, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, Impl, OFFSET>,
            SetActionMap: SetActionMap::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice8W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceA_Impl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: &LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&mut self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&mut self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&mut self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&mut self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&mut self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>() -> IDirectInputDeviceA_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDeviceA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceW_Impl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: &LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&mut self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&mut self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&mut self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&mut self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&mut self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>() -> IDirectInputDeviceW_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, Impl, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, Impl, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDeviceW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputEffect_Impl: Sized {
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetEffectGuid(&mut self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetParameters(&mut self, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::Result<()>;
    fn SetParameters(&mut self, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::Result<()>;
    fn Start(&mut self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn GetEffectStatus(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn Download(&mut self) -> ::windows::core::Result<()>;
    fn Unload(&mut self) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>() -> IDirectInputEffect_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectGuid<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectGuid(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn Unload<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unload().into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetEffectGuid: GetEffectGuid::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDirectInputEffectDriver_Impl: Sized {
    fn DeviceID(&mut self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersions(&mut self, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn SetGain(&mut self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::Result<()>;
    fn DownloadEffect(&mut self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::Result<()>;
    fn DestroyEffect(&mut self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn StartEffect(&mut self, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::Result<()>;
    fn StopEffect(&mut self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn GetEffectStatus(&mut self, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirectInputEffectDriver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>() -> IDirectInputEffectDriver_Vtbl {
        unsafe extern "system" fn DeviceID<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceID(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetVersions<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetVersions(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetGain<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGain(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn DownloadEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DestroyEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestroyEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn StopEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeviceID: DeviceID::<Identity, Impl, OFFSET>,
            GetVersions: GetVersions::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            SetGain: SetGain::<Identity, Impl, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, Impl, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, Impl, OFFSET>,
            DownloadEffect: DownloadEffect::<Identity, Impl, OFFSET>,
            DestroyEffect: DestroyEffect::<Identity, Impl, OFFSET>,
            StartEffect: StartEffect::<Identity, Impl, OFFSET>,
            StopEffect: StopEffect::<Identity, Impl, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputEffectDriver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig_Impl: Sized {
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&mut self) -> ::windows::core::Result<()>;
    fn EnumTypes(&mut self, param0: &LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTypeInfo(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn SetTypeInfo(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteType(&mut self, param0: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetConfig(&mut self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn SetConfig(&mut self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteConfig(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetUserValues(&mut self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn SetUserValues(&mut self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn AddNewHardware(&mut self, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OpenTypeKey(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn OpenConfigKey(&mut self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>() -> IDirectInputJoyConfig_Vtbl {
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenConfigKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenConfigKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SendNotify: SendNotify::<Identity, Impl, OFFSET>,
            EnumTypes: EnumTypes::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, Impl, OFFSET>,
            DeleteType: DeleteType::<Identity, Impl, OFFSET>,
            GetConfig: GetConfig::<Identity, Impl, OFFSET>,
            SetConfig: SetConfig::<Identity, Impl, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, Impl, OFFSET>,
            GetUserValues: GetUserValues::<Identity, Impl, OFFSET>,
            SetUserValues: SetUserValues::<Identity, Impl, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, Impl, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, Impl, OFFSET>,
            OpenConfigKey: OpenConfigKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig8_Impl: Sized {
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&mut self) -> ::windows::core::Result<()>;
    fn EnumTypes(&mut self, param0: &LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTypeInfo(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn SetTypeInfo(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteType(&mut self, param0: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetConfig(&mut self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn SetConfig(&mut self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteConfig(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn GetUserValues(&mut self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn SetUserValues(&mut self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn AddNewHardware(&mut self, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OpenTypeKey(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn OpenAppStatusKey(&mut self, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfig8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>() -> IDirectInputJoyConfig8_Vtbl {
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenAppStatusKey<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenAppStatusKey(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, Impl, OFFSET>,
            Unacquire: Unacquire::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            SendNotify: SendNotify::<Identity, Impl, OFFSET>,
            EnumTypes: EnumTypes::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, Impl, OFFSET>,
            DeleteType: DeleteType::<Identity, Impl, OFFSET>,
            GetConfig: GetConfig::<Identity, Impl, OFFSET>,
            SetConfig: SetConfig::<Identity, Impl, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, Impl, OFFSET>,
            GetUserValues: GetUserValues::<Identity, Impl, OFFSET>,
            SetUserValues: SetUserValues::<Identity, Impl, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, Impl, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, Impl, OFFSET>,
            OpenAppStatusKey: OpenAppStatusKey::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputW_Impl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: &LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>() -> IDirectInputW_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputW as ::windows::core::Interface>::IID
    }
}
