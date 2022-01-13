#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogonImpl: Sized + IDispatchImpl {
    fn Logon(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Logoff(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartShell(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayLock(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayUnlock(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartScreenSaver(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopScreenSaver(&mut self, bstrusername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensLogonVtbl {
        unsafe extern "system" fn Logon<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logon(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn Logoff<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logoff(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn StartShell<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartShell(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayLock<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayLock(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayUnlock<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayUnlock(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn StartScreenSaver<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartScreenSaver(::core::mem::transmute_copy(&bstrusername)).into()
        }
        unsafe extern "system" fn StopScreenSaver<Impl: ISensLogonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopScreenSaver(::core::mem::transmute_copy(&bstrusername)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Logon: Logon::<Impl, IMPL_OFFSET>,
            Logoff: Logoff::<Impl, IMPL_OFFSET>,
            StartShell: StartShell::<Impl, IMPL_OFFSET>,
            DisplayLock: DisplayLock::<Impl, IMPL_OFFSET>,
            DisplayUnlock: DisplayUnlock::<Impl, IMPL_OFFSET>,
            StartScreenSaver: StartScreenSaver::<Impl, IMPL_OFFSET>,
            StopScreenSaver: StopScreenSaver::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon2Impl: Sized + IDispatchImpl {
    fn Logon(&mut self, bstrusername: super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn Logoff(&mut self, bstrusername: super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionDisconnect(&mut self, bstrusername: super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionReconnect(&mut self, bstrusername: super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn PostShell(&mut self, bstrusername: super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensLogon2Vtbl {
        unsafe extern "system" fn Logon<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logon(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn Logoff<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Logoff(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionDisconnect<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionDisconnect(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionReconnect<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SessionReconnect(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn PostShell<Impl: ISensLogon2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostShell(::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Logon: Logon::<Impl, IMPL_OFFSET>,
            Logoff: Logoff::<Impl, IMPL_OFFSET>,
            SessionDisconnect: SessionDisconnect::<Impl, IMPL_OFFSET>,
            SessionReconnect: SessionReconnect::<Impl, IMPL_OFFSET>,
            PostShell: PostShell::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensNetworkImpl: Sized + IDispatchImpl {
    fn ConnectionMade(&mut self, bstrconnection: super::super::Foundation::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn ConnectionMadeNoQOCInfo(&mut self, bstrconnection: super::super::Foundation::BSTR, ultype: u32) -> ::windows::core::Result<()>;
    fn ConnectionLost(&mut self, bstrconnection: super::super::Foundation::BSTR, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::Result<()>;
    fn DestinationReachable(&mut self, bstrdestination: super::super::Foundation::BSTR, bstrconnection: super::super::Foundation::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn DestinationReachableNoQOCInfo(&mut self, bstrdestination: super::super::Foundation::BSTR, bstrconnection: super::super::Foundation::BSTR, ultype: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensNetworkVtbl {
        unsafe extern "system" fn ConnectionMade<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionMade(::core::mem::transmute_copy(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionMadeNoQOCInfo(::core::mem::transmute_copy(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn ConnectionLost<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectionLost(::core::mem::transmute_copy(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn DestinationReachable<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestinationReachable(::core::mem::transmute_copy(&bstrdestination), ::core::mem::transmute_copy(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Impl: ISensNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestinationReachableNoQOCInfo(::core::mem::transmute_copy(&bstrdestination), ::core::mem::transmute_copy(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectionMade: ConnectionMade::<Impl, IMPL_OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Impl, IMPL_OFFSET>,
            ConnectionLost: ConnectionLost::<Impl, IMPL_OFFSET>,
            DestinationReachable: DestinationReachable::<Impl, IMPL_OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensOnNowImpl: Sized + IDispatchImpl {
    fn OnACPower(&mut self) -> ::windows::core::Result<()>;
    fn OnBatteryPower(&mut self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
    fn BatteryLow(&mut self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensOnNowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensOnNowVtbl {
        unsafe extern "system" fn OnACPower<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnACPower().into()
        }
        unsafe extern "system" fn OnBatteryPower<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBatteryPower(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        unsafe extern "system" fn BatteryLow<Impl: ISensOnNowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BatteryLow(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnACPower: OnACPower::<Impl, IMPL_OFFSET>,
            OnBatteryPower: OnBatteryPower::<Impl, IMPL_OFFSET>,
            BatteryLow: BatteryLow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensOnNow as ::windows::core::Interface>::IID
    }
}
