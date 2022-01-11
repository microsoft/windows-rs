#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2AImpl: Sized + IDirectInputAImpl {
    fn FindDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2AVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2WImpl: Sized + IDirectInputWImpl {
    fn FindDevice();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput2WVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7AImpl: Sized + IDirectInput2AImpl + IDirectInputAImpl {
    fn CreateDeviceEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7AVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>, CreateDeviceEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7WImpl: Sized + IDirectInput2WImpl + IDirectInputWImpl {
    fn CreateDeviceEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput7WVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>, CreateDeviceEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput7W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8AImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
    fn FindDevice();
    fn EnumDevicesBySemantics();
    fn ConfigureDevices();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8AVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>, EnumDevicesBySemantics::<Impl, IMPL_OFFSET>, ConfigureDevices::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput8A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput8WImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
    fn FindDevice();
    fn EnumDevicesBySemantics();
    fn ConfigureDevices();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInput8WVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, FindDevice::<Impl, IMPL_OFFSET>, EnumDevicesBySemantics::<Impl, IMPL_OFFSET>, ConfigureDevices::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInput8W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputAImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputAVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2AImpl: Sized + IDirectInputDeviceAImpl {
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2AVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice2WImpl: Sized + IDirectInputDeviceWImpl {
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice2WVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice2W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7AImpl: Sized + IDirectInputDevice2AImpl + IDirectInputDeviceAImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7AVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7WImpl: Sized + IDirectInputDevice2WImpl + IDirectInputDeviceWImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice7WVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8AImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
    fn BuildActionMap();
    fn SetActionMap();
    fn GetImageInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8AImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8AVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile::<Impl, IMPL_OFFSET>,
            BuildActionMap::<Impl, IMPL_OFFSET>,
            SetActionMap::<Impl, IMPL_OFFSET>,
            GetImageInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice8A as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice8WImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
    fn CreateEffect();
    fn EnumEffects();
    fn GetEffectInfo();
    fn GetForceFeedbackState();
    fn SendForceFeedbackCommand();
    fn EnumCreatedEffectObjects();
    fn Escape();
    fn Poll();
    fn SendDeviceData();
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
    fn BuildActionMap();
    fn SetActionMap();
    fn GetImageInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8WImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDevice8WVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            CreateEffect::<Impl, IMPL_OFFSET>,
            EnumEffects::<Impl, IMPL_OFFSET>,
            GetEffectInfo::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            EnumCreatedEffectObjects::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            Poll::<Impl, IMPL_OFFSET>,
            SendDeviceData::<Impl, IMPL_OFFSET>,
            EnumEffectsInFile::<Impl, IMPL_OFFSET>,
            WriteEffectToFile::<Impl, IMPL_OFFSET>,
            BuildActionMap::<Impl, IMPL_OFFSET>,
            SetActionMap::<Impl, IMPL_OFFSET>,
            GetImageInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDevice8W as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceAImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceAVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDeviceA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDeviceWImpl: Sized {
    fn GetCapabilities();
    fn EnumObjects();
    fn GetProperty();
    fn SetProperty();
    fn Acquire();
    fn Unacquire();
    fn GetDeviceState();
    fn GetDeviceData();
    fn SetDataFormat();
    fn SetEventNotification();
    fn SetCooperativeLevel();
    fn GetObjectInfo();
    fn GetDeviceInfo();
    fn RunControlPanel();
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDeviceWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputDeviceWVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCapabilities::<Impl, IMPL_OFFSET>,
            EnumObjects::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty::<Impl, IMPL_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            GetDeviceState::<Impl, IMPL_OFFSET>,
            GetDeviceData::<Impl, IMPL_OFFSET>,
            SetDataFormat::<Impl, IMPL_OFFSET>,
            SetEventNotification::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            GetObjectInfo::<Impl, IMPL_OFFSET>,
            GetDeviceInfo::<Impl, IMPL_OFFSET>,
            RunControlPanel::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputDeviceW as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputEffectImpl: Sized {
    fn Initialize();
    fn GetEffectGuid();
    fn GetParameters();
    fn SetParameters();
    fn Start();
    fn Stop();
    fn GetEffectStatus();
    fn Download();
    fn Unload();
    fn Escape();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffectVtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectGuid<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Download<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unload<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetEffectGuid::<Impl, IMPL_OFFSET>, GetParameters::<Impl, IMPL_OFFSET>, SetParameters::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, GetEffectStatus::<Impl, IMPL_OFFSET>, Download::<Impl, IMPL_OFFSET>, Unload::<Impl, IMPL_OFFSET>, Escape::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputEffect as ::windows::core::Interface>::IID
    }
}
pub trait IDirectInputEffectDriverImpl: Sized {
    fn DeviceID();
    fn GetVersions();
    fn Escape();
    fn SetGain();
    fn SendForceFeedbackCommand();
    fn GetForceFeedbackState();
    fn DownloadEffect();
    fn DestroyEffect();
    fn StartEffect();
    fn StopEffect();
    fn GetEffectStatus();
}
impl IDirectInputEffectDriverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputEffectDriverVtbl {
        unsafe extern "system" fn DeviceID<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersions<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGain<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            DeviceID::<Impl, IMPL_OFFSET>,
            GetVersions::<Impl, IMPL_OFFSET>,
            Escape::<Impl, IMPL_OFFSET>,
            SetGain::<Impl, IMPL_OFFSET>,
            SendForceFeedbackCommand::<Impl, IMPL_OFFSET>,
            GetForceFeedbackState::<Impl, IMPL_OFFSET>,
            DownloadEffect::<Impl, IMPL_OFFSET>,
            DestroyEffect::<Impl, IMPL_OFFSET>,
            StartEffect::<Impl, IMPL_OFFSET>,
            StopEffect::<Impl, IMPL_OFFSET>,
            GetEffectStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputEffectDriver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfigImpl: Sized {
    fn Acquire();
    fn Unacquire();
    fn SetCooperativeLevel();
    fn SendNotify();
    fn EnumTypes();
    fn GetTypeInfo();
    fn SetTypeInfo();
    fn DeleteType();
    fn GetConfig();
    fn SetConfig();
    fn DeleteConfig();
    fn GetUserValues();
    fn SetUserValues();
    fn AddNewHardware();
    fn OpenTypeKey();
    fn OpenConfigKey();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfigVtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenConfigKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SendNotify::<Impl, IMPL_OFFSET>,
            EnumTypes::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            SetTypeInfo::<Impl, IMPL_OFFSET>,
            DeleteType::<Impl, IMPL_OFFSET>,
            GetConfig::<Impl, IMPL_OFFSET>,
            SetConfig::<Impl, IMPL_OFFSET>,
            DeleteConfig::<Impl, IMPL_OFFSET>,
            GetUserValues::<Impl, IMPL_OFFSET>,
            SetUserValues::<Impl, IMPL_OFFSET>,
            AddNewHardware::<Impl, IMPL_OFFSET>,
            OpenTypeKey::<Impl, IMPL_OFFSET>,
            OpenConfigKey::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IDirectInputJoyConfig8Impl: Sized {
    fn Acquire();
    fn Unacquire();
    fn SetCooperativeLevel();
    fn SendNotify();
    fn EnumTypes();
    fn GetTypeInfo();
    fn SetTypeInfo();
    fn DeleteType();
    fn GetConfig();
    fn SetConfig();
    fn DeleteConfig();
    fn GetUserValues();
    fn SetUserValues();
    fn AddNewHardware();
    fn OpenTypeKey();
    fn OpenAppStatusKey();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IDirectInputJoyConfig8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputJoyConfig8Vtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenAppStatusKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Acquire::<Impl, IMPL_OFFSET>,
            Unacquire::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            SendNotify::<Impl, IMPL_OFFSET>,
            EnumTypes::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            SetTypeInfo::<Impl, IMPL_OFFSET>,
            DeleteType::<Impl, IMPL_OFFSET>,
            GetConfig::<Impl, IMPL_OFFSET>,
            SetConfig::<Impl, IMPL_OFFSET>,
            DeleteConfig::<Impl, IMPL_OFFSET>,
            GetUserValues::<Impl, IMPL_OFFSET>,
            SetUserValues::<Impl, IMPL_OFFSET>,
            AddNewHardware::<Impl, IMPL_OFFSET>,
            OpenTypeKey::<Impl, IMPL_OFFSET>,
            OpenAppStatusKey::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputWImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputWImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectInputWVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateDevice::<Impl, IMPL_OFFSET>, EnumDevices::<Impl, IMPL_OFFSET>, GetDeviceStatus::<Impl, IMPL_OFFSET>, RunControlPanel::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectInputW as ::windows::core::Interface>::IID
    }
}
