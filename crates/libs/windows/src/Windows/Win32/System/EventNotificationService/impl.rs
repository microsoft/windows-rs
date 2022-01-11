#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogonImpl: Sized + IDispatchImpl {
    fn Logon();
    fn Logoff();
    fn StartShell();
    fn DisplayLock();
    fn DisplayUnlock();
    fn StartScreenSaver();
    fn StopScreenSaver();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensLogonVtbl {
        unsafe extern "system" fn Logon<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Logoff<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartShell<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayLock<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayUnlock<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartScreenSaver<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopScreenSaver<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Logon::<Impl, IMPL_OFFSET>,
            Logoff::<Impl, IMPL_OFFSET>,
            StartShell::<Impl, IMPL_OFFSET>,
            DisplayLock::<Impl, IMPL_OFFSET>,
            DisplayUnlock::<Impl, IMPL_OFFSET>,
            StartScreenSaver::<Impl, IMPL_OFFSET>,
            StopScreenSaver::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon2Impl: Sized + IDispatchImpl {
    fn Logon();
    fn Logoff();
    fn SessionDisconnect();
    fn SessionReconnect();
    fn PostShell();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensLogon2Vtbl {
        unsafe extern "system" fn Logon<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Logoff<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionDisconnect<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionReconnect<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostShell<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Logon::<Impl, IMPL_OFFSET>, Logoff::<Impl, IMPL_OFFSET>, SessionDisconnect::<Impl, IMPL_OFFSET>, SessionReconnect::<Impl, IMPL_OFFSET>, PostShell::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensNetworkImpl: Sized + IDispatchImpl {
    fn ConnectionMade();
    fn ConnectionMadeNoQOCInfo();
    fn ConnectionLost();
    fn DestinationReachable();
    fn DestinationReachableNoQOCInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensNetworkVtbl {
        unsafe extern "system" fn ConnectionMade<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionLost<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationReachable<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ConnectionMade::<Impl, IMPL_OFFSET>,
            ConnectionMadeNoQOCInfo::<Impl, IMPL_OFFSET>,
            ConnectionLost::<Impl, IMPL_OFFSET>,
            DestinationReachable::<Impl, IMPL_OFFSET>,
            DestinationReachableNoQOCInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensOnNowImpl: Sized + IDispatchImpl {
    fn OnACPower();
    fn OnBatteryPower();
    fn BatteryLow();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensOnNowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensOnNowVtbl {
        unsafe extern "system" fn OnACPower<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnBatteryPower<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BatteryLow<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnACPower::<Impl, IMPL_OFFSET>, OnBatteryPower::<Impl, IMPL_OFFSET>, BatteryLow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensOnNow as ::windows::core::Interface>::IID
    }
}
