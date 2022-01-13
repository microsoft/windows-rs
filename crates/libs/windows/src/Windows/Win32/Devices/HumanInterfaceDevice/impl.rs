#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2AImpl: Sized + IDirectInputAImpl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2AVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputAVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FindDevice: FindDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2WImpl: Sized + IDirectInputWImpl {
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2WVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        Self { base: IDirectInputWVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FindDevice: FindDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7AImpl: Sized + IDirectInputAImpl + IDirectInput2AImpl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7AVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2AVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7WImpl: Sized + IDirectInputWImpl + IDirectInput2WImpl {
    fn CreateDeviceEx(&mut self, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7WVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDeviceEx(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        Self { base: IDirectInput2WVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8AImpl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8A>, param2: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&mut self, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&mut self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8AVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IDirectInput8WImpl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8W>, param2: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
    fn FindDevice(&mut self, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumDevicesBySemantics(&mut self, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::Result<()>;
    fn ConfigureDevices(&mut self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8WVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevicesBySemantics(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IDirectInputAImpl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputAVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
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
pub trait IDirectInputDevice2AImpl: Sized + IDirectInputDeviceAImpl {
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2AVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceAVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDirectInputDevice2WImpl: Sized + IDirectInputDeviceWImpl {
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2WVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDeviceWVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDirectInputDevice7AImpl: Sized + IDirectInputDeviceAImpl + IDirectInputDevice2AImpl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7AVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2AVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7WImpl: Sized + IDirectInputDeviceWImpl + IDirectInputDevice2WImpl {
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7WVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        Self {
            base: IDirectInputDevice2WVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8AImpl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
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
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&mut self, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&mut self, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&mut self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8AVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
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
pub trait IDirectInputDevice8WImpl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
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
    fn CreateEffect(&mut self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumEffects(&mut self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn GetEffectInfo(&mut self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetForceFeedbackState(&mut self, param0: *mut u32) -> ::windows::core::Result<()>;
    fn SendForceFeedbackCommand(&mut self, param0: u32) -> ::windows::core::Result<()>;
    fn EnumCreatedEffectObjects(&mut self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
    fn Escape(&mut self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()>;
    fn Poll(&mut self) -> ::windows::core::Result<()>;
    fn SendDeviceData(&mut self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()>;
    fn EnumEffectsInFile(&mut self, param0: super::super::Foundation::PWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn WriteEffectToFile(&mut self, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::Result<()>;
    fn BuildActionMap(&mut self, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn SetActionMap(&mut self, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::Result<()>;
    fn GetImageInfo(&mut self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8WVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute(&param3)).into()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumCreatedEffectObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Poll().into()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumEffectsInFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteEffectToFile(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BuildActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionMap(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
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
pub trait IDirectInputDeviceAImpl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
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
impl IDirectInputDeviceAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceAVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IDirectInputDeviceWImpl: Sized {
    fn GetCapabilities(&mut self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()>;
    fn EnumObjects(&mut self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()>;
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
impl IDirectInputDeviceWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceWVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumObjects(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceData(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataFormat(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventNotification(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IDirectInputEffectImpl: Sized {
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
impl IDirectInputEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffectVtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn GetEffectGuid<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectGuid(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Start<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Download<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Download().into()
        }
        unsafe extern "system" fn Unload<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unload().into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
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
pub trait IDirectInputEffectDriverImpl: Sized {
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
impl IDirectInputEffectDriverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffectDriverVtbl {
        unsafe extern "system" fn DeviceID<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceID(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn GetVersions<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVersions(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetGain<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGain(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendForceFeedbackCommand(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForceFeedbackState(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn DownloadEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3), ::core::mem::transmute_copy(&param4)).into()
        }
        unsafe extern "system" fn DestroyEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn StartEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn StopEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopEffect(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IDirectInputJoyConfigImpl: Sized {
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&mut self) -> ::windows::core::Result<()>;
    fn EnumTypes(&mut self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
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
impl IDirectInputJoyConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfigVtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenConfigKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
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
pub trait IDirectInputJoyConfig8Impl: Sized {
    fn Acquire(&mut self) -> ::windows::core::Result<()>;
    fn Unacquire(&mut self) -> ::windows::core::Result<()>;
    fn SetCooperativeLevel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn SendNotify(&mut self) -> ::windows::core::Result<()>;
    fn EnumTypes(&mut self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
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
impl IDirectInputJoyConfig8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfig8Vtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Acquire().into()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unacquire().into()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendNotify().into()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumTypes(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeInfo(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteType(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfig(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteConfig(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserValues(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNewHardware(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenTypeKey(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2)).into()
        }
        unsafe extern "system" fn OpenAppStatusKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
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
pub trait IDirectInputWImpl: Sized {
    fn CreateDevice(&mut self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn EnumDevices(&mut self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()>;
    fn GetDeviceStatus(&mut self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RunControlPanel(&mut self, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputWVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute(&param2)).into()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumDevices(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1), ::core::mem::transmute_copy(&param2), ::core::mem::transmute_copy(&param3)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceStatus(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunControlPanel(::core::mem::transmute_copy(&param0), ::core::mem::transmute_copy(&param1)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
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
