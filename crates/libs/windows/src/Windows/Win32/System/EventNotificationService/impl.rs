#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Logoff(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StartShell(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DisplayLock(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DisplayUnlock(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StartScreenSaver(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StopScreenSaver(&self, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISensLogon {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>() -> ISensLogon_Vtbl {
        unsafe extern "system" fn Logon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Logon(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn Logoff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Logoff(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartShell<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartShell(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayLock(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn DisplayUnlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUnlock(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StartScreenSaver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartScreenSaver(::core::mem::transmute(&bstrusername)).into()
        }
        unsafe extern "system" fn StopScreenSaver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopScreenSaver(::core::mem::transmute(&bstrusername)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<ISensLogon as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensLogon2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Logon(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn Logoff(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionDisconnect(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn SessionReconnect(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
    fn PostShell(&self, bstrusername: &::windows::core::BSTR, dwsessionid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISensLogon2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensLogon2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>() -> ISensLogon2_Vtbl {
        unsafe extern "system" fn Logon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Logon(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn Logoff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Logoff(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionDisconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SessionDisconnect(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn SessionReconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SessionReconnect(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        unsafe extern "system" fn PostShell<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensLogon2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwsessionid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostShell(::core::mem::transmute(&bstrusername), ::core::mem::transmute_copy(&dwsessionid)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Logon: Logon::<Identity, Impl, OFFSET>,
            Logoff: Logoff::<Identity, Impl, OFFSET>,
            SessionDisconnect: SessionDisconnect::<Identity, Impl, OFFSET>,
            SessionReconnect: SessionReconnect::<Identity, Impl, OFFSET>,
            PostShell: PostShell::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensLogon2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensNetwork_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionMade(&self, bstrconnection: &::windows::core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn ConnectionMadeNoQOCInfo(&self, bstrconnection: &::windows::core::BSTR, ultype: u32) -> ::windows::core::Result<()>;
    fn ConnectionLost(&self, bstrconnection: &::windows::core::BSTR, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::Result<()>;
    fn DestinationReachable(&self, bstrdestination: &::windows::core::BSTR, bstrconnection: &::windows::core::BSTR, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::Result<()>;
    fn DestinationReachableNoQOCInfo(&self, bstrdestination: &::windows::core::BSTR, bstrconnection: &::windows::core::BSTR, ultype: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISensNetwork {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>() -> ISensNetwork_Vtbl {
        unsafe extern "system" fn ConnectionMade<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows::core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectionMade(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn ConnectionMadeNoQOCInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows::core::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectionMadeNoQOCInfo(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn ConnectionLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnection: ::std::mem::MaybeUninit<::windows::core::BSTR>, ultype: SENS_CONNECTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectionLost(::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn DestinationReachable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrconnection: ::std::mem::MaybeUninit<::windows::core::BSTR>, ultype: u32, lpqocinfo: *const SENS_QOCINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestinationReachable(::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&lpqocinfo)).into()
        }
        unsafe extern "system" fn DestinationReachableNoQOCInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrconnection: ::std::mem::MaybeUninit<::windows::core::BSTR>, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestinationReachableNoQOCInfo(::core::mem::transmute(&bstrdestination), ::core::mem::transmute(&bstrconnection), ::core::mem::transmute_copy(&ultype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectionMade: ConnectionMade::<Identity, Impl, OFFSET>,
            ConnectionMadeNoQOCInfo: ConnectionMadeNoQOCInfo::<Identity, Impl, OFFSET>,
            ConnectionLost: ConnectionLost::<Identity, Impl, OFFSET>,
            DestinationReachable: DestinationReachable::<Identity, Impl, OFFSET>,
            DestinationReachableNoQOCInfo: DestinationReachableNoQOCInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensNetwork as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_EventNotificationService\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISensOnNow_Impl: Sized + super::Com::IDispatch_Impl {
    fn OnACPower(&self) -> ::windows::core::Result<()>;
    fn OnBatteryPower(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
    fn BatteryLow(&self, dwbatterylifepercent: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISensOnNow {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISensOnNow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>() -> ISensOnNow_Vtbl {
        unsafe extern "system" fn OnACPower<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnACPower().into()
        }
        unsafe extern "system" fn OnBatteryPower<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBatteryPower(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        unsafe extern "system" fn BatteryLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensOnNow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbatterylifepercent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BatteryLow(::core::mem::transmute_copy(&dwbatterylifepercent)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnACPower: OnACPower::<Identity, Impl, OFFSET>,
            OnBatteryPower: OnBatteryPower::<Identity, Impl, OFFSET>,
            BatteryLow: BatteryLow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensOnNow as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
