pub trait IDirectInput2AImpl: Sized + IDirectInputAImpl {
    fn FindDevice();
}
impl ::windows::core::RuntimeName for IDirectInput2A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput2A";
}
impl IDirectInput2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2AImpl, const OFFSET: isize>() -> IDirectInput2AVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindDevice(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInput2A>, ::windows::core::GetTrustLevel, FindDevice::<Impl, OFFSET>)
    }
}
pub trait IDirectInput2WImpl: Sized + IDirectInputWImpl {
    fn FindDevice();
}
impl ::windows::core::RuntimeName for IDirectInput2W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput2W";
}
impl IDirectInput2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput2WImpl, const OFFSET: isize>() -> IDirectInput2WVtbl {
        unsafe extern "system" fn FindDevice<Impl: IDirectInput2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindDevice(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInput2W>, ::windows::core::GetTrustLevel, FindDevice::<Impl, OFFSET>)
    }
}
pub trait IDirectInput7AImpl: Sized + IDirectInput2AImpl + IDirectInputAImpl {
    fn CreateDeviceEx();
}
impl ::windows::core::RuntimeName for IDirectInput7A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput7A";
}
impl IDirectInput7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7AImpl, const OFFSET: isize>() -> IDirectInput7AVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: &::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeviceEx(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInput7A>, ::windows::core::GetTrustLevel, CreateDeviceEx::<Impl, OFFSET>)
    }
}
pub trait IDirectInput7WImpl: Sized + IDirectInput2WImpl + IDirectInputWImpl {
    fn CreateDeviceEx();
}
impl ::windows::core::RuntimeName for IDirectInput7W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput7W";
}
impl IDirectInput7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput7WImpl, const OFFSET: isize>() -> IDirectInput7WVtbl {
        unsafe extern "system" fn CreateDeviceEx<Impl: IDirectInput7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: &::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeviceEx(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInput7W>, ::windows::core::GetTrustLevel, CreateDeviceEx::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectInput8A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput8A";
}
impl IDirectInput8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8AImpl, const OFFSET: isize>() -> IDirectInput8AVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(param0, &*(&param1 as *const <LPDIENUMDEVICESCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: super::super::Foundation::PSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindDevice(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: *mut DIACTIONFORMATA, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevicesBySemantics(
                &*(&param0 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIACTIONFORMATA as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATA as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <LPDIENUMDEVICESBYSEMANTICSCBA as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESBYSEMANTICSCBA as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param4,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureDevices(
                &*(&param0 as *const <LPDICONFIGUREDEVICESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDICONFIGUREDEVICESCALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DICONFIGUREDEVICESPARAMSA as ::windows::core::Abi>::Abi as *const <DICONFIGUREDEVICESPARAMSA as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInput8A>,
            ::windows::core::GetTrustLevel,
            CreateDevice::<Impl, OFFSET>,
            EnumDevices::<Impl, OFFSET>,
            GetDeviceStatus::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            FindDevice::<Impl, OFFSET>,
            EnumDevicesBySemantics::<Impl, OFFSET>,
            ConfigureDevices::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInput8W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInput8W";
}
impl IDirectInput8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInput8WImpl, const OFFSET: isize>() -> IDirectInput8WVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(param0, &*(&param1 as *const <LPDIENUMDEVICESCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindDevice<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: super::super::Foundation::PWSTR, param2: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindDevice(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIACTIONFORMATW, param2: ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevicesBySemantics(
                &*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIACTIONFORMATW as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATW as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <LPDIENUMDEVICESBYSEMANTICSCBW as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESBYSEMANTICSCBW as ::windows::core::DefaultType>::DefaultType),
                &*(&param3 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param4,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureDevices<Impl: IDirectInput8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureDevices(
                &*(&param0 as *const <LPDICONFIGUREDEVICESCALLBACK as ::windows::core::Abi>::Abi as *const <LPDICONFIGUREDEVICESCALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DICONFIGUREDEVICESPARAMSW as ::windows::core::Abi>::Abi as *const <DICONFIGUREDEVICESPARAMSW as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInput8W>,
            ::windows::core::GetTrustLevel,
            CreateDevice::<Impl, OFFSET>,
            EnumDevices::<Impl, OFFSET>,
            GetDeviceStatus::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            FindDevice::<Impl, OFFSET>,
            EnumDevicesBySemantics::<Impl, OFFSET>,
            ConfigureDevices::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectInputAImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
impl ::windows::core::RuntimeName for IDirectInputA {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputA";
}
impl IDirectInputAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputAImpl, const OFFSET: isize>() -> IDirectInputAVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(param0, &*(&param1 as *const <LPDIENUMDEVICESCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInputA>, ::windows::core::GetTrustLevel, CreateDevice::<Impl, OFFSET>, EnumDevices::<Impl, OFFSET>, GetDeviceStatus::<Impl, OFFSET>, RunControlPanel::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDevice2A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice2A";
}
impl IDirectInputDevice2AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2AImpl, const OFFSET: isize>() -> IDirectInputDevice2AVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffect(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&param2),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffects(&*(&param0 as *const <LPDIENUMEFFECTSCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectInfo(&*(&param0 as *const <DIEFFECTINFOA as ::windows::core::Abi>::Abi as *const <DIEFFECTINFOA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceFeedbackState(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendForceFeedbackCommand(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCreatedEffectObjects(&*(&param0 as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&param0 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Poll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDevice2A>,
            ::windows::core::GetTrustLevel,
            CreateEffect::<Impl, OFFSET>,
            EnumEffects::<Impl, OFFSET>,
            GetEffectInfo::<Impl, OFFSET>,
            GetForceFeedbackState::<Impl, OFFSET>,
            SendForceFeedbackCommand::<Impl, OFFSET>,
            EnumCreatedEffectObjects::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            Poll::<Impl, OFFSET>,
            SendDeviceData::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDevice2W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice2W";
}
impl IDirectInputDevice2WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice2WImpl, const OFFSET: isize>() -> IDirectInputDevice2WVtbl {
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffect(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&param2),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffects(&*(&param0 as *const <LPDIENUMEFFECTSCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectInfo(&*(&param0 as *const <DIEFFECTINFOW as ::windows::core::Abi>::Abi as *const <DIEFFECTINFOW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceFeedbackState(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendForceFeedbackCommand(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCreatedEffectObjects(&*(&param0 as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&param0 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Poll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice2WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDevice2W>,
            ::windows::core::GetTrustLevel,
            CreateEffect::<Impl, OFFSET>,
            EnumEffects::<Impl, OFFSET>,
            GetEffectInfo::<Impl, OFFSET>,
            GetForceFeedbackState::<Impl, OFFSET>,
            SendForceFeedbackCommand::<Impl, OFFSET>,
            EnumCreatedEffectObjects::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            Poll::<Impl, OFFSET>,
            SendDeviceData::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectInputDevice7AImpl: Sized + IDirectInputDevice2AImpl + IDirectInputDeviceAImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
impl ::windows::core::RuntimeName for IDirectInputDevice7A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice7A";
}
impl IDirectInputDevice7AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7AImpl, const OFFSET: isize>() -> IDirectInputDevice7AVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffectsInFile(
                &*(&param0 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param3,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteEffectToFile(&*(&param0 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <DIFILEEFFECT as ::windows::core::Abi>::Abi as *const <DIFILEEFFECT as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInputDevice7A>, ::windows::core::GetTrustLevel, EnumEffectsInFile::<Impl, OFFSET>, WriteEffectToFile::<Impl, OFFSET>)
    }
}
pub trait IDirectInputDevice7WImpl: Sized + IDirectInputDevice2WImpl + IDirectInputDeviceWImpl {
    fn EnumEffectsInFile();
    fn WriteEffectToFile();
}
impl ::windows::core::RuntimeName for IDirectInputDevice7W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice7W";
}
impl IDirectInputDevice7WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice7WImpl, const OFFSET: isize>() -> IDirectInputDevice7WVtbl {
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffectsInFile(
                &*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param3,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice7WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteEffectToFile(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <DIFILEEFFECT as ::windows::core::Abi>::Abi as *const <DIFILEEFFECT as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInputDevice7W>, ::windows::core::GetTrustLevel, EnumEffectsInFile::<Impl, OFFSET>, WriteEffectToFile::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDevice8A {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice8A";
}
impl IDirectInputDevice8AVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8AImpl, const OFFSET: isize>() -> IDirectInputDevice8AVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&param0 as *const <DIDEVCAPS as ::windows::core::Abi>::Abi as *const <DIDEVCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(&*(&param0 as *const <LPDIENUMDEVICEOBJECTSCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICEOBJECTSCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceState(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDataFormat(&*(&param0 as *const <DIDATAFORMAT as ::windows::core::Abi>::Abi as *const <DIDATAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(&*(&param0 as *const <DIDEVICEOBJECTINSTANCEA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTINSTANCEA as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceInfo(&*(&param0 as *const <DIDEVICEINSTANCEA as ::windows::core::Abi>::Abi as *const <DIDEVICEINSTANCEA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffect(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&param2),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffects(&*(&param0 as *const <LPDIENUMEFFECTSCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectInfo(&*(&param0 as *const <DIEFFECTINFOA as ::windows::core::Abi>::Abi as *const <DIEFFECTINFOA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceFeedbackState(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendForceFeedbackCommand(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCreatedEffectObjects(&*(&param0 as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&param0 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Poll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffectsInFile(
                &*(&param0 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param3,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteEffectToFile(&*(&param0 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <DIFILEEFFECT as ::windows::core::Abi>::Abi as *const <DIFILEEFFECT as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildActionMap(&*(&param0 as *const <DIACTIONFORMATA as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: super::super::Foundation::PSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActionMap(&*(&param0 as *const <DIACTIONFORMATA as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8AImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageInfo(&*(&param0 as *const <DIDEVICEIMAGEINFOHEADERA as ::windows::core::Abi>::Abi as *const <DIDEVICEIMAGEINFOHEADERA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDevice8A>,
            ::windows::core::GetTrustLevel,
            GetCapabilities::<Impl, OFFSET>,
            EnumObjects::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            GetDeviceState::<Impl, OFFSET>,
            GetDeviceData::<Impl, OFFSET>,
            SetDataFormat::<Impl, OFFSET>,
            SetEventNotification::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            GetObjectInfo::<Impl, OFFSET>,
            GetDeviceInfo::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            CreateEffect::<Impl, OFFSET>,
            EnumEffects::<Impl, OFFSET>,
            GetEffectInfo::<Impl, OFFSET>,
            GetForceFeedbackState::<Impl, OFFSET>,
            SendForceFeedbackCommand::<Impl, OFFSET>,
            EnumCreatedEffectObjects::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            Poll::<Impl, OFFSET>,
            SendDeviceData::<Impl, OFFSET>,
            EnumEffectsInFile::<Impl, OFFSET>,
            WriteEffectToFile::<Impl, OFFSET>,
            BuildActionMap::<Impl, OFFSET>,
            SetActionMap::<Impl, OFFSET>,
            GetImageInfo::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDevice8W {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDevice8W";
}
impl IDirectInputDevice8WVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDevice8WImpl, const OFFSET: isize>() -> IDirectInputDevice8WVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&param0 as *const <DIDEVCAPS as ::windows::core::Abi>::Abi as *const <DIDEVCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(&*(&param0 as *const <LPDIENUMDEVICEOBJECTSCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICEOBJECTSCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceState(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDataFormat(&*(&param0 as *const <DIDATAFORMAT as ::windows::core::Abi>::Abi as *const <DIDATAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(&*(&param0 as *const <DIDEVICEOBJECTINSTANCEW as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTINSTANCEW as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceInfo(&*(&param0 as *const <DIDEVICEINSTANCEW as ::windows::core::Abi>::Abi as *const <DIDEVICEINSTANCEW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffect<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::windows::core::RawPtr, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEffect(
                &*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&param2),
                &*(&param3 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffects(&*(&param0 as *const <LPDIENUMEFFECTSCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectInfo(&*(&param0 as *const <DIEFFECTINFOW as ::windows::core::Abi>::Abi as *const <DIEFFECTINFOW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceFeedbackState(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendForceFeedbackCommand(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumCreatedEffectObjects(&*(&param0 as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMCREATEDEFFECTOBJECTSCALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&param0 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Poll<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Poll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDeviceData<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffectsInFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffectsInFile(
                &*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIENUMEFFECTSINFILECALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                param3,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteEffectToFile(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <DIFILEEFFECT as ::windows::core::Abi>::Abi as *const <DIFILEEFFECT as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildActionMap(&*(&param0 as *const <DIACTIONFORMATW as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActionMap<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: super::super::Foundation::PWSTR, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActionMap(&*(&param0 as *const <DIACTIONFORMATW as ::windows::core::Abi>::Abi as *const <DIACTIONFORMATW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Impl: IDirectInputDevice8WImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageInfo(&*(&param0 as *const <DIDEVICEIMAGEINFOHEADERW as ::windows::core::Abi>::Abi as *const <DIDEVICEIMAGEINFOHEADERW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDevice8W>,
            ::windows::core::GetTrustLevel,
            GetCapabilities::<Impl, OFFSET>,
            EnumObjects::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            GetDeviceState::<Impl, OFFSET>,
            GetDeviceData::<Impl, OFFSET>,
            SetDataFormat::<Impl, OFFSET>,
            SetEventNotification::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            GetObjectInfo::<Impl, OFFSET>,
            GetDeviceInfo::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            CreateEffect::<Impl, OFFSET>,
            EnumEffects::<Impl, OFFSET>,
            GetEffectInfo::<Impl, OFFSET>,
            GetForceFeedbackState::<Impl, OFFSET>,
            SendForceFeedbackCommand::<Impl, OFFSET>,
            EnumCreatedEffectObjects::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            Poll::<Impl, OFFSET>,
            SendDeviceData::<Impl, OFFSET>,
            EnumEffectsInFile::<Impl, OFFSET>,
            WriteEffectToFile::<Impl, OFFSET>,
            BuildActionMap::<Impl, OFFSET>,
            SetActionMap::<Impl, OFFSET>,
            GetImageInfo::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDeviceA {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDeviceA";
}
impl IDirectInputDeviceAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceAImpl, const OFFSET: isize>() -> IDirectInputDeviceAVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&param0 as *const <DIDEVCAPS as ::windows::core::Abi>::Abi as *const <DIDEVCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(&*(&param0 as *const <LPDIENUMDEVICEOBJECTSCALLBACKA as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICEOBJECTSCALLBACKA as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceState(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDataFormat(&*(&param0 as *const <DIDATAFORMAT as ::windows::core::Abi>::Abi as *const <DIDATAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(&*(&param0 as *const <DIDEVICEOBJECTINSTANCEA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTINSTANCEA as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceInfo(&*(&param0 as *const <DIDEVICEINSTANCEA as ::windows::core::Abi>::Abi as *const <DIDEVICEINSTANCEA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDeviceA>,
            ::windows::core::GetTrustLevel,
            GetCapabilities::<Impl, OFFSET>,
            EnumObjects::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            GetDeviceState::<Impl, OFFSET>,
            GetDeviceData::<Impl, OFFSET>,
            SetDataFormat::<Impl, OFFSET>,
            SetEventNotification::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            GetObjectInfo::<Impl, OFFSET>,
            GetDeviceInfo::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputDeviceW {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputDeviceW";
}
impl IDirectInputDeviceWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputDeviceWImpl, const OFFSET: isize>() -> IDirectInputDeviceWVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&param0 as *const <DIDEVCAPS as ::windows::core::Abi>::Abi as *const <DIDEVCAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjects<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumObjects(&*(&param0 as *const <LPDIENUMDEVICEOBJECTSCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICEOBJECTSCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIPROPHEADER as ::windows::core::Abi>::Abi as *const <DIPROPHEADER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Acquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceState<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceState(param0, &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceData<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceData(param0, &*(&param1 as *const <DIDEVICEOBJECTDATA as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTDATA as ::windows::core::DefaultType>::DefaultType), param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataFormat<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDataFormat(&*(&param0 as *const <DIDATAFORMAT as ::windows::core::Abi>::Abi as *const <DIDATAFORMAT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventNotification<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventNotification(&*(&param0 as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectInfo(&*(&param0 as *const <DIDEVICEOBJECTINSTANCEW as ::windows::core::Abi>::Abi as *const <DIDEVICEOBJECTINSTANCEW as ::windows::core::DefaultType>::DefaultType), param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceInfo(&*(&param0 as *const <DIDEVICEINSTANCEW as ::windows::core::Abi>::Abi as *const <DIDEVICEINSTANCEW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputDeviceWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputDeviceW>,
            ::windows::core::GetTrustLevel,
            GetCapabilities::<Impl, OFFSET>,
            EnumObjects::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            GetDeviceState::<Impl, OFFSET>,
            GetDeviceData::<Impl, OFFSET>,
            SetDataFormat::<Impl, OFFSET>,
            SetEventNotification::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            GetObjectInfo::<Impl, OFFSET>,
            GetDeviceInfo::<Impl, OFFSET>,
            RunControlPanel::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputEffect {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputEffect";
}
impl IDirectInputEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectImpl, const OFFSET: isize>() -> IDirectInputEffectVtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectGuid<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectGuid(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters(&*(&param0 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetParameters(&*(&param0 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectStatus(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unload<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(&*(&param0 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputEffect>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetEffectGuid::<Impl, OFFSET>,
            GetParameters::<Impl, OFFSET>,
            SetParameters::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            GetEffectStatus::<Impl, OFFSET>,
            Download::<Impl, OFFSET>,
            Unload::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IDirectInputEffectDriver {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputEffectDriver";
}
impl IDirectInputEffectDriverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>() -> IDirectInputEffectDriverVtbl {
        unsafe extern "system" fn DeviceID<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceID(param0, param1, param2, param3, &*(&param4 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersions<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersions(&*(&param0 as *const <DIDRIVERVERSIONS as ::windows::core::Abi>::Abi as *const <DIDRIVERVERSIONS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(param0, param1, &*(&param2 as *const <DIEFFESCAPE as ::windows::core::Abi>::Abi as *const <DIEFFESCAPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGain(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendForceFeedbackCommand(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceFeedbackState(param0, &*(&param1 as *const <DIDEVICESTATE as ::windows::core::Abi>::Abi as *const <DIDEVICESTATE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadEffect(param0, param1, param2, &*(&param3 as *const <DIEFFECT as ::windows::core::Abi>::Abi as *const <DIEFFECT as ::windows::core::DefaultType>::DefaultType), param4) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyEffect(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartEffect(param0, param1, param2, param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopEffect<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopEffect(param0, param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectStatus<Impl: IDirectInputEffectDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectStatus(param0, param1, param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputEffectDriver>,
            ::windows::core::GetTrustLevel,
            DeviceID::<Impl, OFFSET>,
            GetVersions::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            SetGain::<Impl, OFFSET>,
            SendForceFeedbackCommand::<Impl, OFFSET>,
            GetForceFeedbackState::<Impl, OFFSET>,
            DownloadEffect::<Impl, OFFSET>,
            DestroyEffect::<Impl, OFFSET>,
            StartEffect::<Impl, OFFSET>,
            StopEffect::<Impl, OFFSET>,
            GetEffectStatus::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputJoyConfig {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputJoyConfig";
}
impl IDirectInputJoyConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>() -> IDirectInputJoyConfigVtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTypes(&*(&param0 as *const <LPDIJOYTYPECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIJOYTYPECALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIJOYTYPEINFO as ::windows::core::Abi>::Abi as *const <DIJOYTYPEINFO as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypeInfo(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIJOYTYPEINFO as ::windows::core::Abi>::Abi as *const <DIJOYTYPEINFO as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteType(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfig(param0, &*(&param1 as *const <DIJOYCONFIG as ::windows::core::Abi>::Abi as *const <DIJOYCONFIG as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfig(param0, &*(&param1 as *const <DIJOYCONFIG as ::windows::core::Abi>::Abi as *const <DIJOYCONFIG as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteConfig(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserValues(&*(&param0 as *const <DIJOYUSERVALUES as ::windows::core::Abi>::Abi as *const <DIJOYUSERVALUES as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUserValues(&*(&param0 as *const <DIJOYUSERVALUES as ::windows::core::Abi>::Abi as *const <DIJOYUSERVALUES as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNewHardware(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTypeKey(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenConfigKey<Impl: IDirectInputJoyConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenConfigKey(param0, param1, &*(&param2 as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputJoyConfig>,
            ::windows::core::GetTrustLevel,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SendNotify::<Impl, OFFSET>,
            EnumTypes::<Impl, OFFSET>,
            GetTypeInfo::<Impl, OFFSET>,
            SetTypeInfo::<Impl, OFFSET>,
            DeleteType::<Impl, OFFSET>,
            GetConfig::<Impl, OFFSET>,
            SetConfig::<Impl, OFFSET>,
            DeleteConfig::<Impl, OFFSET>,
            GetUserValues::<Impl, OFFSET>,
            SetUserValues::<Impl, OFFSET>,
            AddNewHardware::<Impl, OFFSET>,
            OpenTypeKey::<Impl, OFFSET>,
            OpenConfigKey::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectInputJoyConfig8 {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputJoyConfig8";
}
impl IDirectInputJoyConfig8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>() -> IDirectInputJoyConfig8Vtbl {
        unsafe extern "system" fn Acquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Acquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unacquire<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unacquire() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendNotify<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendNotify() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTypes<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: ::windows::core::RawPtr, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTypes(&*(&param0 as *const <LPDIJOYTYPECALLBACK as ::windows::core::Abi>::Abi as *const <LPDIJOYTYPECALLBACK as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypeInfo(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <DIJOYTYPEINFO as ::windows::core::Abi>::Abi as *const <DIJOYTYPEINFO as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeInfo<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypeInfo(
                &*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&param1 as *const <DIJOYTYPEINFO as ::windows::core::Abi>::Abi as *const <DIJOYTYPEINFO as ::windows::core::DefaultType>::DefaultType),
                param2,
                &*(&param3 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteType<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteType(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfig(param0, &*(&param1 as *const <DIJOYCONFIG as ::windows::core::Abi>::Abi as *const <DIJOYCONFIG as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfig(param0, &*(&param1 as *const <DIJOYCONFIG as ::windows::core::Abi>::Abi as *const <DIJOYCONFIG as ::windows::core::DefaultType>::DefaultType), param2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteConfig<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteConfig(param0) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserValues(&*(&param0 as *const <DIJOYUSERVALUES as ::windows::core::Abi>::Abi as *const <DIJOYUSERVALUES as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserValues<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUserValues(&*(&param0 as *const <DIJOYUSERVALUES as ::windows::core::Abi>::Abi as *const <DIJOYUSERVALUES as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNewHardware<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNewHardware(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&param1 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTypeKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTypeKey(&*(&param0 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), param1, &*(&param2 as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAppStatusKey<Impl: IDirectInputJoyConfig8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAppStatusKey(&*(&param0 as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDirectInputJoyConfig8>,
            ::windows::core::GetTrustLevel,
            Acquire::<Impl, OFFSET>,
            Unacquire::<Impl, OFFSET>,
            SetCooperativeLevel::<Impl, OFFSET>,
            SendNotify::<Impl, OFFSET>,
            EnumTypes::<Impl, OFFSET>,
            GetTypeInfo::<Impl, OFFSET>,
            SetTypeInfo::<Impl, OFFSET>,
            DeleteType::<Impl, OFFSET>,
            GetConfig::<Impl, OFFSET>,
            SetConfig::<Impl, OFFSET>,
            DeleteConfig::<Impl, OFFSET>,
            GetUserValues::<Impl, OFFSET>,
            SetUserValues::<Impl, OFFSET>,
            AddNewHardware::<Impl, OFFSET>,
            OpenTypeKey::<Impl, OFFSET>,
            OpenAppStatusKey::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectInputWImpl: Sized {
    fn CreateDevice();
    fn EnumDevices();
    fn GetDeviceStatus();
    fn RunControlPanel();
    fn Initialize();
}
impl ::windows::core::RuntimeName for IDirectInputW {
    const NAME: &'static str = "Windows.Win32.Devices.HumanInterfaceDevice.IDirectInputW";
}
impl IDirectInputWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectInputWImpl, const OFFSET: isize>() -> IDirectInputWVtbl {
        unsafe extern "system" fn CreateDevice<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID, param1: *mut ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&param1), &*(&param2 as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDevices<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: u32, param1: ::windows::core::RawPtr, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDevices(param0, &*(&param1 as *const <LPDIENUMDEVICESCALLBACKW as ::windows::core::Abi>::Abi as *const <LPDIENUMDEVICESCALLBACKW as ::windows::core::DefaultType>::DefaultType), &*(&param2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), param3) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus(&*(&param0 as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunControlPanel<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunControlPanel(&*(&param0 as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectInputWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&param0 as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), param1) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDirectInputW>, ::windows::core::GetTrustLevel, CreateDevice::<Impl, OFFSET>, EnumDevices::<Impl, OFFSET>, GetDeviceStatus::<Impl, OFFSET>, RunControlPanel::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
    }
}
