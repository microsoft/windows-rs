#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Logoff(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartShell(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayLock(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayUnlock(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartScreenSaver(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopScreenSaver(&self, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>() -> ISensLogon_Vtbl {
        unsafe extern "system" fn Logon<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logon(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn Logoff<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logoff(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartShell<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartShell(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayLock<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayLock(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayUnlock<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayUnlock(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartScreenSaver<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartScreenSaver(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StopScreenSaver<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopScreenSaver(::core::mem::transmute(&bstrusername)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            StartShell: StartShell::<Identity, Impl, OFFSET>,
            DisplayLock: DisplayLock::<Identity, Impl, OFFSET>,
            DisplayUnlock: DisplayUnlock::<Identity, Impl, OFFSET>,
            StartScreenSaver: StartScreenSaver::<Identity, Impl, OFFSET>,
            StopScreenSaver: StopScreenSaver::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn Logoff(&self, bstrusername: &super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionDisconnect(&self, bstrusername: &super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionReconnect(&self, bstrusername: &super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn PostShell(&self, bstrusername: &super::super::Foundation::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>() -> ISensLogon2_Vtbl {
        unsafe extern "system" fn Logon<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logon(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn Logoff<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Logoff(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionDisconnect<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SessionDisconnect(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionReconnect<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SessionReconnect(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn PostShell<Identity: ::windows::core::IUnknownImpl, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PostShell(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            SessionDisconnect: SessionDisconnect::<Identity, Impl, OFFSET>,
            SessionReconnect: SessionReconnect::<Identity, Impl, OFFSET>,
            PostShell: PostShell::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensNetwork_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionMade(&self, bstrconnection: &super::super::Foundation::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &super::super::Foundation::BSTR, ultype: u32) -> ::windows::core::Result<()>;
    fn ConnectionLost(&self, bstrconnection: &super::super::Foundation::BSTR, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::Result<()>;
    fn DestinationReachable(&self, bstrdestination: &super::super::Foundation::BSTR, bstrconnection: &super::super::Foundation::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn DestinationReachableNoQOCInfo(&self, bstrdestination: &super::super::Foundation::BSTR, bstrconnection: &super::super::Foundation::BSTR, ultype: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>() -> ISensNetwork_Vtbl {
        unsafe extern "system" fn ConnectionMade<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectionMade(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectionMadeNoQOCInfo(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn ConnectionLost<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectionLost(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn DestinationReachable<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestinationReachable(::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Identity: ::windows::core::IUnknownImpl, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrconnection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DestinationReachableNoQOCInfo(::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectionMade: ConnectionMade::<Identity, Impl, OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Identity, Impl, OFFSET>,
            ConnectionLost: ConnectionLost::<Identity, Impl, OFFSET>,
            DestinationReachable: DestinationReachable::<Identity, Impl, OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensNetwork as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensOnNow_Impl: Sized + super::Com::IDispatch_Impl {
    fn OnACPower(&self) -> ::windows::core::Result<()>;
    fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
    fn BatteryLow(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensOnNow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNow_Impl, const OFFSET: isize>() -> ISensOnNow_Vtbl {
        unsafe extern "system" fn OnACPower<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnACPower().into()
        }
        unsafe extern "system" fn OnBatteryPower<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBatteryPower(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        unsafe extern "system" fn BatteryLow<Identity: ::windows::core::IUnknownImpl, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BatteryLow(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnACPower: OnACPower::<Identity, Impl, OFFSET>,
            OnBatteryPower: OnBatteryPower::<Identity, Impl, OFFSET>,
            BatteryLow: BatteryLow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensOnNow as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
