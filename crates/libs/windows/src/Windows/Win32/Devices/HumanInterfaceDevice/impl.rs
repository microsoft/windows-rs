#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2A_Impl: Sized + IDirectInputA_Impl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2A_Vtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputA_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FindDevice: FindDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2W_Impl: Sized + IDirectInputW_Impl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2W_Vtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputW_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FindDevice: FindDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7A_Impl: Sized + IDirectInputA_Impl + IDirectInput2A_Impl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7A_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2A_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7W_Impl: Sized + IDirectInputW_Impl + IDirectInput2W_Impl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7W_Vtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2W_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7W as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8A_Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            FindDevice: FindDevice::<Impl, IMPL_OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Impl, IMPL_OFFSET>,
            ConfigureDevices: ConfigureDevices::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8W_Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            FindDevice: FindDevice::<Impl, IMPL_OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Impl, IMPL_OFFSET>,
            ConfigureDevices: ConfigureDevices::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputA_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputA_Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2A_Vtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceA_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateEffect: CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects: EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo: GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            Poll: Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData: SendDeviceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2A as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2W_Vtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceW_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateEffect: CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects: EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo: GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            Poll: Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData: SendDeviceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7A_Impl: Sized + IDirectInputDeviceA_Impl + IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7A_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2A_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7W_Impl: Sized + IDirectInputDeviceW_Impl + IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PWSTR, param1: &LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7W_Vtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2W_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8A_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8A_Vtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState: GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData: GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat: SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification: SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect: CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects: EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo: GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            Poll: Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData: SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
            BuildActionMap: BuildActionMap::<Impl, IMPL_OFFSET>,
            SetActionMap: SetActionMap::<Impl, IMPL_OFFSET>,
            GetImageInfo: GetImageInfo::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8W_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8W_Vtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImageInfo(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState: GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData: GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat: SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification: SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect: CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects: EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo: GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            Poll: Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData: SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
            BuildActionMap: BuildActionMap::<Impl, IMPL_OFFSET>,
            SetActionMap: SetActionMap::<Impl, IMPL_OFFSET>,
            GetImageInfo: GetImageInfo::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceA_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceA_Vtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState: GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData: GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat: SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification: SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceW_Vtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects: EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState: GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData: GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat: SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification: SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo: GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffect_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectGuid<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectGuid(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetParameters<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetParameters<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Start<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Download<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn Unload<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unload().into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetEffectGuid: GetEffectGuid::<Impl, IMPL_OFFSET>,
            GetParameters: GetParameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            GetEffectStatus: GetEffectStatus::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            Unload: Unload::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffectDriver_Vtbl {
        unsafe extern "system" fn DeviceID<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceID(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetVersions<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersions(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetGain<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGain(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn DownloadEffect<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DestroyEffect<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartEffect<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn StopEffect<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectStatus(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DeviceID: DeviceID::<Impl, IMPL_OFFSET>,
            GetVersions: GetVersions::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            SetGain: SetGain::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            DownloadEffect: DownloadEffect::<Impl, IMPL_OFFSET>,
            DestroyEffect: DestroyEffect::<Impl, IMPL_OFFSET>,
            StartEffect: StartEffect::<Impl, IMPL_OFFSET>,
            StopEffect: StopEffect::<Impl, IMPL_OFFSET>,
            GetEffectStatus: GetEffectStatus::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfig_Vtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenConfigKey<Impl: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenConfigKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SendNotify: SendNotify::<Impl, IMPL_OFFSET>,
            EnumTypes: EnumTypes::<Impl, IMPL_OFFSET>,
            GetTypeInfo: GetTypeInfo::<Impl, IMPL_OFFSET>,
            SetTypeInfo: SetTypeInfo::<Impl, IMPL_OFFSET>,
            DeleteType: DeleteType::<Impl, IMPL_OFFSET>,
            GetConfig: GetConfig::<Impl, IMPL_OFFSET>,
            SetConfig: SetConfig::<Impl, IMPL_OFFSET>,
            DeleteConfig: DeleteConfig::<Impl, IMPL_OFFSET>,
            GetUserValues: GetUserValues::<Impl, IMPL_OFFSET>,
            SetUserValues: SetUserValues::<Impl, IMPL_OFFSET>,
            AddNewHardware: AddNewHardware::<Impl, IMPL_OFFSET>,
            OpenTypeKey: OpenTypeKey::<Impl, IMPL_OFFSET>,
            OpenConfigKey: OpenConfigKey::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfig8_Vtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenAppStatusKey<Impl: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenAppStatusKey(::core::mem::transmute_copy(&param0)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Acquire: Acquire::<Impl, IMPL_OFFSET>,
            Unacquire: Unacquire::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SendNotify: SendNotify::<Impl, IMPL_OFFSET>,
            EnumTypes: EnumTypes::<Impl, IMPL_OFFSET>,
            GetTypeInfo: GetTypeInfo::<Impl, IMPL_OFFSET>,
            SetTypeInfo: SetTypeInfo::<Impl, IMPL_OFFSET>,
            DeleteType: DeleteType::<Impl, IMPL_OFFSET>,
            GetConfig: GetConfig::<Impl, IMPL_OFFSET>,
            SetConfig: SetConfig::<Impl, IMPL_OFFSET>,
            DeleteConfig: DeleteConfig::<Impl, IMPL_OFFSET>,
            GetUserValues: GetUserValues::<Impl, IMPL_OFFSET>,
            SetUserValues: SetUserValues::<Impl, IMPL_OFFSET>,
            AddNewHardware: AddNewHardware::<Impl, IMPL_OFFSET>,
            OpenTypeKey: OpenTypeKey::<Impl, IMPL_OFFSET>,
            OpenAppStatusKey: OpenAppStatusKey::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputW_Vtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
            EnumDevices: EnumDevices::<Impl, IMPL_OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Impl, IMPL_OFFSET>,
            RunControlPanel: RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputW as ::windows::core::Interface>::IID
    }
}
