#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Shared(&self) -> ::windows::core::Result<i16>;
    fn SetShared(&self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>() -> IRDPSRAPIApplication_Vtbl {
        unsafe extern "system" fn Windows<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindowlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Windows: Windows::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Shared: Shared::<Identity, Impl, OFFSET>,
            SetShared: SetShared::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplication as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplicationFilter_Impl: Sized + super::Com::IDispatch_Impl {
    fn Applications(&self) -> ::windows::core::Result<IRDPSRAPIApplicationList>;
    fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>() -> IRDPSRAPIApplicationFilter_Vtbl {
        unsafe extern "system" fn Applications<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Applications() {
                ::core::result::Result::Ok(ok__) => {
                    *papplications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindows: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindows = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Applications: Applications::<Identity, Impl, OFFSET>,
            Windows: Windows::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationFilter as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplicationList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIApplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>() -> IRDPSRAPIApplicationList_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationList as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendee_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn RemoteName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ControlLevel(&self) -> ::windows::core::Result<CTRL_LEVEL>;
    fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> ::windows::core::Result<()>;
    fn Invitation(&self) -> ::windows::core::Result<IRDPSRAPIInvitation>;
    fn TerminateConnection(&self) -> ::windows::core::Result<()>;
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn ConnectivityInfo(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendee_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>() -> IRDPSRAPIAttendee_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteName<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlLevel<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ControlLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlLevel<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetControlLevel(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn Invitation<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Invitation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TerminateConnection().into()
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectivityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            RemoteName: RemoteName::<Identity, Impl, OFFSET>,
            ControlLevel: ControlLevel::<Identity, Impl, OFFSET>,
            SetControlLevel: SetControlLevel::<Identity, Impl, OFFSET>,
            Invitation: Invitation::<Identity, Impl, OFFSET>,
            TerminateConnection: TerminateConnection::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            ConnectivityInfo: ConnectivityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendee as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendeeDisconnectInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Attendee(&self) -> ::windows::core::Result<IRDPSRAPIAttendee>;
    fn Reason(&self) -> ::windows::core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>() -> IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
        unsafe extern "system" fn Attendee<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attendee() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *preason = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Attendee: Attendee::<Identity, Impl, OFFSET>,
            Reason: Reason::<Identity, Impl, OFFSET>,
            Code: Code::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeDisconnectInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendeeManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, id: i32) -> ::windows::core::Result<IRDPSRAPIAttendee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>() -> IRDPSRAPIAttendeeManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPIAudioStream_Impl: Sized {
    fn Initialize(&self) -> ::windows::core::Result<i64>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn FreeBuffer(&self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIAudioStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>() -> IRDPSRAPIAudioStream_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnperiodinhundrednsintervals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&ptimestamp)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeBuffer().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAudioStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIClipboardUseEvents_Impl: Sized {
    fn OnPasteFromClipboard(&self, clipboardformat: u32, pattendee: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIClipboardUseEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: isize>() -> IRDPSRAPIClipboardUseEvents_Vtbl {
        unsafe extern "system" fn OnPasteFromClipboard<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnPasteFromClipboard(::core::mem::transmute_copy(&clipboardformat), ::core::mem::transmute(&pattendee)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnPasteFromClipboard: OnPasteFromClipboard::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIClipboardUseEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRDPSRAPIDebug_Impl: Sized {
    fn SetCLXCmdLine(&self, clxcmdline: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CLXCmdLine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIDebug_Impl, const OFFSET: isize>() -> IRDPSRAPIDebug_Vtbl {
        unsafe extern "system" fn SetCLXCmdLine<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCLXCmdLine(::core::mem::transmute(&clxcmdline)).into()
        }
        unsafe extern "system" fn CLXCmdLine<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CLXCmdLine() {
                ::core::result::Result::Ok(ok__) => {
                    *pclxcmdline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetCLXCmdLine: SetCLXCmdLine::<Identity, Impl, OFFSET>,
            CLXCmdLine: CLXCmdLine::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIFrameBuffer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn Bpp(&self) -> ::windows::core::Result<i32>;
    fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIFrameBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>() -> IRDPSRAPIFrameBuffer_Vtbl {
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *plwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *plheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bpp<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Bpp() {
                ::core::result::Result::Ok(ok__) => {
                    *plbpp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameBufferBits(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&heigth)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Bpp: Bpp::<Identity, Impl, OFFSET>,
            GetFrameBufferBits: GetFrameBufferBits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIFrameBuffer as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIInvitation_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GroupName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Password(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AttendeeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetAttendeeLimit(&self, newval: i32) -> ::windows::core::Result<()>;
    fn Revoked(&self) -> ::windows::core::Result<i16>;
    fn SetRevoked(&self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>() -> IRDPSRAPIInvitation_Vtbl {
        unsafe extern "system" fn ConnectionString<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttendeeLimit<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttendeeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttendeeLimit(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Revoked<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Revoked() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevoked<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRevoked(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, Impl, OFFSET>,
            GroupName: GroupName::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            AttendeeLimit: AttendeeLimit::<Identity, Impl, OFFSET>,
            SetAttendeeLimit: SetAttendeeLimit::<Identity, Impl, OFFSET>,
            Revoked: Revoked::<Identity, Impl, OFFSET>,
            SetRevoked: SetRevoked::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitation as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIInvitationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, item: &super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIInvitation>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn CreateInvitation(&self, bstrauthstring: &super::super::Foundation::BSTR, bstrgroupname: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, attendeelimit: i32) -> ::windows::core::Result<IRDPSRAPIInvitation>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>() -> IRDPSRAPIInvitationManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinvitation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInvitation<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInvitation(::core::mem::transmute(&bstrauthstring), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&attendeelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinvitation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            CreateInvitation: CreateInvitation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitationManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPIPerfCounterLogger_Impl: Sized {
    fn LogValue(&self, lvalue: i64) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIPerfCounterLogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: isize>() -> IRDPSRAPIPerfCounterLogger_Vtbl {
        unsafe extern "system" fn LogValue<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogValue(::core::mem::transmute_copy(&lvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LogValue: LogValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLogger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRDPSRAPIPerfCounterLoggingManager_Impl: Sized {
    fn CreateLogger(&self, bstrcountername: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRDPSRAPIPerfCounterLogger>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: isize>() -> IRDPSRAPIPerfCounterLoggingManager_Vtbl {
        unsafe extern "system" fn CreateLogger<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLogger(::core::mem::transmute(&bstrcountername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplogger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateLogger: CreateLogger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLoggingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISessionProperties_Impl: Sized + super::Com::IDispatch_Impl {
    fn Property(&self, propertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetProperty(&self, propertyname: &super::super::Foundation::BSTR, newval: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISessionProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>() -> IRDPSRAPISessionProperties_Vtbl {
        unsafe extern "system" fn Property<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Property(::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute(&propertyname), ::core::mem::transmute(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISessionProperties as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISharingSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn SetColorDepth(&self, colordepth: i32) -> ::windows::core::Result<()>;
    fn ColorDepth(&self) -> ::windows::core::Result<i32>;
    fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties>;
    fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn ConnectToClient(&self, bstrconnectionstring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()>;
    fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISharingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>() -> IRDPSRAPISharingSession_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetColorDepth<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorDepth(::core::mem::transmute_copy(&colordepth)).into()
        }
        unsafe extern "system" fn ColorDepth<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColorDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolordepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ConnectToClient<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectToClient(::core::mem::transmute(&bstrconnectionstring)).into()
        }
        unsafe extern "system" fn SetDesktopSharedRect<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDesktopSharedRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn GetDesktopSharedRect<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesktopSharedRect(::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetColorDepth: SetColorDepth::<Identity, Impl, OFFSET>,
            ColorDepth: ColorDepth::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Attendees: Attendees::<Identity, Impl, OFFSET>,
            Invitations: Invitations::<Identity, Impl, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, Impl, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            ConnectToClient: ConnectToClient::<Identity, Impl, OFFSET>,
            SetDesktopSharedRect: SetDesktopSharedRect::<Identity, Impl, OFFSET>,
            GetDesktopSharedRect: GetDesktopSharedRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISharingSession2_Impl: Sized + super::Com::IDispatch_Impl + IRDPSRAPISharingSession_Impl {
    fn ConnectUsingTransportStream(&self, pstream: &::core::option::Option<IRDPSRAPITransportStream>, bstrgroup: &super::super::Foundation::BSTR, bstrauthenticatedattendeename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FrameBuffer(&self) -> ::windows::core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(&self, pattendee: &::core::option::Option<IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISharingSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>() -> IRDPSRAPISharingSession2_Vtbl {
        unsafe extern "system" fn ConnectUsingTransportStream<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectUsingTransportStream(::core::mem::transmute(&pstream), ::core::mem::transmute(&bstrgroup), ::core::mem::transmute(&bstrauthenticatedattendeename)).into()
        }
        unsafe extern "system" fn FrameBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattendee: ::windows::core::RawPtr, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendControlLevelChangeResponse(::core::mem::transmute(&pattendee), ::core::mem::transmute_copy(&requestedlevel), ::core::mem::transmute_copy(&reasoncode)).into()
        }
        Self {
            base: IRDPSRAPISharingSession_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectUsingTransportStream: ConnectUsingTransportStream::<Identity, Impl, OFFSET>,
            FrameBuffer: FrameBuffer::<Identity, Impl, OFFSET>,
            SendControlLevelChangeResponse: SendControlLevelChangeResponse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IRDPSRAPISharingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPITcpConnectionInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Protocol(&self) -> ::windows::core::Result<i32>;
    fn LocalPort(&self) -> ::windows::core::Result<i32>;
    fn LocalIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PeerPort(&self) -> ::windows::core::Result<i32>;
    fn PeerIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPITcpConnectionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>() -> IRDPSRAPITcpConnectionInfo_Vtbl {
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *plprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalIP<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalIP() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsrlocalip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerPort<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIP<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeerIP() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            LocalPort: LocalPort::<Identity, Impl, OFFSET>,
            LocalIP: LocalIP::<Identity, Impl, OFFSET>,
            PeerPort: PeerPort::<Identity, Impl, OFFSET>,
            PeerIP: PeerIP::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITcpConnectionInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStream_Impl: Sized {
    fn AllocBuffer(&self, maxpayload: i32) -> ::windows::core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(&self, pbuffer: &::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn WriteBuffer(&self, pbuffer: &::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn ReadBuffer(&self, pbuffer: &::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn Open(&self, pcallbacks: &::core::option::Option<IRDPSRAPITransportStreamEvents>) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPITransportStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>() -> IRDPSRAPITransportStream_Vtbl {
        unsafe extern "system" fn AllocBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocBuffer(::core::mem::transmute_copy(&maxpayload)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FreeBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn WriteBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn ReadBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReadBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute(&pcallbacks)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllocBuffer: AllocBuffer::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, Impl, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStream as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamBuffer_Impl: Sized {
    fn Storage(&self) -> ::windows::core::Result<*mut u8>;
    fn StorageSize(&self) -> ::windows::core::Result<i32>;
    fn PayloadSize(&self) -> ::windows::core::Result<i32>;
    fn SetPayloadSize(&self, lval: i32) -> ::windows::core::Result<()>;
    fn PayloadOffset(&self) -> ::windows::core::Result<i32>;
    fn SetPayloadOffset(&self, lretval: i32) -> ::windows::core::Result<()>;
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn SetFlags(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn Context(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetContext(&self, pcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IRDPSRAPITransportStreamBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>() -> IRDPSRAPITransportStreamBuffer_Vtbl {
        unsafe extern "system" fn Storage<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Storage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageSize<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadSize<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PayloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadSize<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPayloadSize(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PayloadOffset<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PayloadOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPayloadOffset(::core::mem::transmute_copy(&lretval)).into()
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Context<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Storage: Storage::<Identity, Impl, OFFSET>,
            StorageSize: StorageSize::<Identity, Impl, OFFSET>,
            PayloadSize: PayloadSize::<Identity, Impl, OFFSET>,
            SetPayloadSize: SetPayloadSize::<Identity, Impl, OFFSET>,
            PayloadOffset: PayloadOffset::<Identity, Impl, OFFSET>,
            SetPayloadOffset: SetPayloadOffset::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamEvents_Impl: Sized {
    fn OnWriteCompleted(&self, pbuffer: &::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(&self, pbuffer: &::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(&self, hrreason: ::windows::core::HRESULT);
}
impl IRDPSRAPITransportStreamEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>() -> IRDPSRAPITransportStreamEvents_Vtbl {
        unsafe extern "system" fn OnWriteCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWriteCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnReadCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnReadCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnStreamClosed<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStreamClosed(::core::mem::transmute_copy(&hrreason))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnWriteCompleted: OnWriteCompleted::<Identity, Impl, OFFSET>,
            OnReadCompleted: OnReadCompleted::<Identity, Impl, OFFSET>,
            OnStreamClosed: OnStreamClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIViewer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Connect(&self, bstrconnectionstring: &super::super::Foundation::BSTR, bstrname: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager>;
    fn SetSmartSizing(&self, vbsmartsizing: i16) -> ::windows::core::Result<()>;
    fn SmartSizing(&self) -> ::windows::core::Result<i16>;
    fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> ::windows::core::Result<()>;
    fn SetDisconnectedText(&self, bstrdisconnectedtext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisconnectedText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestColorDepthChange(&self, bpp: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties>;
    fn StartReverseConnectListener(&self, bstrconnectionstring: &super::super::Foundation::BSTR, bstrusername: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIViewer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>() -> IRDPSRAPIViewer_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&bstrconnectionstring), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Attendees<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartSizing<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbsmartsizing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSmartSizing(::core::mem::transmute_copy(&vbsmartsizing)).into()
        }
        unsafe extern "system" fn SmartSizing<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmartSizing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbsmartsizing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestControl<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestControl(::core::mem::transmute_copy(&ctrllevel)).into()
        }
        unsafe extern "system" fn SetDisconnectedText<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisconnectedText(::core::mem::transmute(&bstrdisconnectedtext)).into()
        }
        unsafe extern "system" fn DisconnectedText<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisconnectedText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisconnectedtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestColorDepthChange(::core::mem::transmute_copy(&bpp)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartReverseConnectListener(::core::mem::transmute(&bstrconnectionstring), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreverseconnectstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Attendees: Attendees::<Identity, Impl, OFFSET>,
            Invitations: Invitations::<Identity, Impl, OFFSET>,
            ApplicationFilter: ApplicationFilter::<Identity, Impl, OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Identity, Impl, OFFSET>,
            SetSmartSizing: SetSmartSizing::<Identity, Impl, OFFSET>,
            SmartSizing: SmartSizing::<Identity, Impl, OFFSET>,
            RequestControl: RequestControl::<Identity, Impl, OFFSET>,
            SetDisconnectedText: SetDisconnectedText::<Identity, Impl, OFFSET>,
            DisconnectedText: DisconnectedText::<Identity, Impl, OFFSET>,
            RequestColorDepthChange: RequestColorDepthChange::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            StartReverseConnectListener: StartReverseConnectListener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIViewer as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIVirtualChannel_Impl: Sized + super::Com::IDispatch_Impl {
    fn SendData(&self, bstrdata: &super::super::Foundation::BSTR, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::Result<()>;
    fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn Priority(&self) -> ::windows::core::Result<CHANNEL_PRIORITY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>() -> IRDPSRAPIVirtualChannel_Vtbl {
        unsafe extern "system" fn SendData<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendData(::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&channelsendflags)).into()
        }
        unsafe extern "system" fn SetAccess<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccess(::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&accesstype)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SendData: SendData::<Identity, Impl, OFFSET>,
            SetAccess: SetAccess::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannel as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIVirtualChannelManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, item: &super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(&self, bstrchannelname: &super::super::Foundation::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannelManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>() -> IRDPSRAPIVirtualChannelManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVirtualChannel(::core::mem::transmute(&bstrchannelname), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&channelflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannelManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIWindow_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Application(&self) -> ::windows::core::Result<IRDPSRAPIApplication>;
    fn Shared(&self) -> ::windows::core::Result<i16>;
    fn SetShared(&self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn Flags(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>() -> IRDPSRAPIWindow_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Application() {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Application: Application::<Identity, Impl, OFFSET>,
            Shared: Shared::<Identity, Impl, OFFSET>,
            SetShared: SetShared::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIWindow as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIWindowList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIWindow>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindowList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: isize>() -> IRDPSRAPIWindowList_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIWindowList as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRDPViewerInputSink_Impl: Sized {
    fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseWheelEvent(&self, wheelrotation: u16) -> ::windows::core::Result<()>;
    fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::Result<()>;
    fn SendSyncEvent(&self, syncflags: u32) -> ::windows::core::Result<()>;
    fn BeginTouchFrame(&self) -> ::windows::core::Result<()>;
    fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn EndTouchFrame(&self) -> ::windows::core::Result<()>;
}
impl IRDPViewerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>() -> IRDPViewerInputSink_Vtbl {
        unsafe extern "system" fn SendMouseButtonEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendMouseButtonEvent(::core::mem::transmute_copy(&buttontype), ::core::mem::transmute_copy(&vbbuttondown), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseMoveEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendMouseMoveEvent(::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseWheelEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendMouseWheelEvent(::core::mem::transmute_copy(&wheelrotation)).into()
        }
        unsafe extern "system" fn SendKeyboardEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendKeyboardEvent(::core::mem::transmute_copy(&codetype), ::core::mem::transmute_copy(&keycode), ::core::mem::transmute_copy(&vbkeyup), ::core::mem::transmute_copy(&vbrepeat), ::core::mem::transmute_copy(&vbextended)).into()
        }
        unsafe extern "system" fn SendSyncEvent<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendSyncEvent(::core::mem::transmute_copy(&syncflags)).into()
        }
        unsafe extern "system" fn BeginTouchFrame<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginTouchFrame().into()
        }
        unsafe extern "system" fn AddTouchInput<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTouchInput(::core::mem::transmute_copy(&contactid), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn EndTouchFrame<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndTouchFrame().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendMouseButtonEvent: SendMouseButtonEvent::<Identity, Impl, OFFSET>,
            SendMouseMoveEvent: SendMouseMoveEvent::<Identity, Impl, OFFSET>,
            SendMouseWheelEvent: SendMouseWheelEvent::<Identity, Impl, OFFSET>,
            SendKeyboardEvent: SendKeyboardEvent::<Identity, Impl, OFFSET>,
            SendSyncEvent: SendSyncEvent::<Identity, Impl, OFFSET>,
            BeginTouchFrame: BeginTouchFrame::<Identity, Impl, OFFSET>,
            AddTouchInput: AddTouchInput::<Identity, Impl, OFFSET>,
            EndTouchFrame: EndTouchFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPViewerInputSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IRDPSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IRDPSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IRDPSessionEvents_Impl, const OFFSET: isize>() -> _IRDPSessionEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IRDPSessionEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
