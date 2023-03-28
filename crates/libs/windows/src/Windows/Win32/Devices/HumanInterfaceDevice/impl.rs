#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2A_Impl: Sized + IDirectInputA_Impl {
    fn FindDevice(&self, param0: *const ::windows::core::GUID, param1: &::windows::core::PCSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput2A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput2A_Impl, const OFFSET: isize>() -> IDirectInput2A_Vtbl {
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: ::windows::core::PCSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base__: IDirectInputA_Vtbl::new::<Identity, Impl, OFFSET>(), FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2A as ::windows::core::ComInterface>::IID || iid == &<IDirectInputA as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2W_Impl: Sized + IDirectInputW_Impl {
    fn FindDevice(&self, param0: *const ::windows::core::GUID, param1: &::windows::core::PCWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput2W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput2W_Impl, const OFFSET: isize>() -> IDirectInput2W_Vtbl {
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: ::windows::core::PCWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base__: IDirectInputW_Vtbl::new::<Identity, Impl, OFFSET>(), FindDevice: FindDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2W as ::windows::core::ComInterface>::IID || iid == &<IDirectInputW as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7A_Impl: Sized + IDirectInput2A_Impl {
    fn CreateDeviceEx(&self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput7A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput7A_Impl, const OFFSET: isize>() -> IDirectInput7A_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        Self { base__: IDirectInput2A_Vtbl::new::<Identity, Impl, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7A as ::windows::core::ComInterface>::IID || iid == &<IDirectInputA as ::windows::core::ComInterface>::IID || iid == &<IDirectInput2A as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7W_Impl: Sized + IDirectInput2W_Impl {
    fn CreateDeviceEx(&self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput7W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput7W_Impl, const OFFSET: isize>() -> IDirectInput7W_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        Self { base__: IDirectInput2W_Vtbl::new::<Identity, Impl, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7W as ::windows::core::ComInterface>::IID || iid == &<IDirectInputW as ::windows::core::ComInterface>::IID || iid == &<IDirectInput2W as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8A_Impl: Sized {
    fn CreateDevice(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8A>, param2: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&self, param0: *const ::windows::core::GUID, param1: &::windows::core::PCSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &::windows::core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput8A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>() -> IDirectInput8A_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows::core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: ::windows::core::PCSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevicesBySemantics(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInput8A as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8W_Impl: Sized {
    fn CreateDevice(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8W>, param2: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&self, param0: *const ::windows::core::GUID, param1: &::windows::core::PCWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &::windows::core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInput8W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>() -> IDirectInput8W_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows::core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: ::windows::core::PCWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevicesBySemantics(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInput8W as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputA_Impl: Sized {
    fn CreateDevice(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputA {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>() -> IDirectInputA_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows::core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputA as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2A_Impl: Sized + IDirectInputDeviceA_Impl {
    fn CreateEffect(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice2A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>() -> IDirectInputDevice2A_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDeviceA_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDirectInputDevice2A as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDeviceA as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2W_Impl: Sized + IDirectInputDeviceW_Impl {
    fn CreateEffect(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice2W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>() -> IDirectInputDevice2W_Vtbl {
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDeviceW_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDirectInputDevice2W as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDeviceW as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7A_Impl: Sized + IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&self, param0: &::windows::core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&self, param0: &::windows::core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice7A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>() -> IDirectInputDevice7A_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffectsInFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteEffectToFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDevice2A_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDeviceA as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDevice2A as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7W_Impl: Sized + IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&self, param0: &::windows::core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&self, param0: &::windows::core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice7W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>() -> IDirectInputDevice7W_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffectsInFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteEffectToFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base__: IDirectInputDevice2W_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, Impl, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDeviceW as ::windows::core::ComInterface>::IID || iid == &<IDirectInputDevice2W as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8A_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CreateEffect(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &::windows::core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&self, param0: &::windows::core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &::windows::core::PCSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &::windows::core::PCSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice8A {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>() -> IDirectInputDevice8A_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffectsInFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteEffectToFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows::core::PCSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows::core::PCSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputDevice8A as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8W_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CreateEffect(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &::windows::core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&self, param0: &::windows::core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &::windows::core::PCWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &::windows::core::PCWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDevice8W {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>() -> IDirectInputDevice8W_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::windows::core::from_raw_borrowed(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumEffectsInFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteEffectToFile(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows::core::PCWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows::core::PCWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputDevice8W as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceA_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDeviceA {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>() -> IDirectInputDeviceA_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputDeviceA as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceW_Impl: Sized {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()>;
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputDeviceW {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>() -> IDirectInputDeviceW_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputDeviceW as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputEffect_Impl: Sized {
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetEffectGuid(&self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::Result<()>;
    fn SetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::Result<()>;
    fn Start(&self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn GetEffectStatus(&self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn Download(&self) -> ::windows::core::Result<()>;
    fn Unload(&self) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputEffect {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>() -> IDirectInputEffect_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectGuid(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Download().into()
        }
        unsafe extern "system" fn Unload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unload().into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputEffect as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"implement\"`*"]
pub trait IDirectInputEffectDriver_Impl: Sized {
    fn DeviceID(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetVersions(&self, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::Result<()>;
    fn Escape(&self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn SetGain(&self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&self, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::Result<()>;
    fn DownloadEffect(&self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::Result<()>;
    fn DestroyEffect(&self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn StartEffect(&self, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::Result<()>;
    fn StopEffect(&self, param0: u32, param1: u32) -> ::windows::core::Result<()>;
    fn GetEffectStatus(&self, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectInputEffectDriver {}
impl IDirectInputEffectDriver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>() -> IDirectInputEffectDriver_Vtbl {
        unsafe extern "system" fn DeviceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceID(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetVersions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVersions(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGain(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendForceFeedbackCommand(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForceFeedbackState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn DownloadEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DestroyEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn StopEffect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetEffectStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEffectStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputEffectDriver as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig_Impl: Sized {
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&self) -> ::windows::core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTypeInfo(&self, param0: &::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn SetTypeInfo(&self, param0: &::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteType(&self, param0: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> ::windows::core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OpenTypeKey(&self, param0: &::windows::core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn OpenConfigKey(&self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IDirectInputJoyConfig {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>() -> IDirectInputJoyConfig_Vtbl {
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypeInfo(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeInfo(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteType(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenTypeKey(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenConfigKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenConfigKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputJoyConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig8_Impl: Sized {
    fn Acquire(&self) -> ::windows::core::Result<()>;
    fn Unacquire(&self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&self) -> ::windows::core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTypeInfo(&self, param0: &::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::Result<()>;
    fn SetTypeInfo(&self, param0: &::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DeleteType(&self, param0: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> ::windows::core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OpenTypeKey(&self, param0: &::windows::core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn OpenAppStatusKey(&self, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IDirectInputJoyConfig8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfig8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>() -> IDirectInputJoyConfig8_Vtbl {
        unsafe extern "system" fn Acquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypeInfo(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeInfo(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn DeleteType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteType(::core::mem::transmute(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenTypeKey(::core::mem::transmute(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenAppStatusKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenAppStatusKey(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDirectInputJoyConfig8 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_HumanInterfaceDevice\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputW_Impl: Sized {
    fn CreateDevice(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectInputW {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>() -> IDirectInputW_Vtbl {
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::windows::core::from_raw_borrowed(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HMODULE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            EnumDevices: EnumDevices::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputW as ::windows::core::ComInterface>::IID
    }
}
