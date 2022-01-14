#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn Windows(&mut self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Shared(&mut self) -> ::windows::core::Result<i16>;
    fn SetShared(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplication_Vtbl {
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindowlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Windows: Windows::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Shared: Shared::<Impl, IMPL_OFFSET>,
            SetShared: SetShared::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplicationFilter_Impl: Sized + super::Com::IDispatch_Impl {
    fn Applications(&mut self) -> ::windows::core::Result<IRDPSRAPIApplicationList>;
    fn Windows(&mut self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplicationFilter_Vtbl {
        unsafe extern "system" fn Applications<Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Applications() {
                ::core::result::Result::Ok(ok__) => {
                    *papplications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindows: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindows = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IRDPSRAPIApplicationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Applications: Applications::<Impl, IMPL_OFFSET>,
            Windows: Windows::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplicationList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IRDPSRAPIApplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplicationList_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIApplicationList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendee_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn RemoteName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ControlLevel(&mut self) -> ::windows::core::Result<CTRL_LEVEL>;
    fn SetControlLevel(&mut self, pnewval: CTRL_LEVEL) -> ::windows::core::Result<()>;
    fn Invitation(&mut self) -> ::windows::core::Result<IRDPSRAPIInvitation>;
    fn TerminateConnection(&mut self) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn ConnectivityInfo(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendee_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendee_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendee_Vtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteName<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlLevel<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlLevel<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlLevel(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn Invitation<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateConnection().into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Impl: IRDPSRAPIAttendee_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectivityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            RemoteName: RemoteName::<Impl, IMPL_OFFSET>,
            ControlLevel: ControlLevel::<Impl, IMPL_OFFSET>,
            SetControlLevel: SetControlLevel::<Impl, IMPL_OFFSET>,
            Invitation: Invitation::<Impl, IMPL_OFFSET>,
            TerminateConnection: TerminateConnection::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            ConnectivityInfo: ConnectivityInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendee as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendeeDisconnectInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Attendee(&mut self) -> ::windows::core::Result<IRDPSRAPIAttendee>;
    fn Reason(&mut self) -> ::windows::core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
        unsafe extern "system" fn Attendee<Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendee() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *preason = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Impl: IRDPSRAPIAttendeeDisconnectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Attendee: Attendee::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
            Code: Code::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeDisconnectInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendeeManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, id: i32) -> ::windows::core::Result<IRDPSRAPIAttendee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendeeManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIAttendeeManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeManager as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPIAudioStream_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<i64>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn GetBuffer(&mut self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn FreeBuffer(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIAudioStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAudioStream_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnperiodinhundrednsintervals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetBuffer<Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&ptimestamp)).into()
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPIAudioStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeBuffer().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            FreeBuffer: FreeBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAudioStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIClipboardUseEvents_Impl: Sized {
    fn OnPasteFromClipboard(&mut self, clipboardformat: u32, pattendee: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIClipboardUseEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIClipboardUseEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIClipboardUseEvents_Vtbl {
        unsafe extern "system" fn OnPasteFromClipboard<Impl: IRDPSRAPIClipboardUseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPasteFromClipboard(::core::mem::transmute_copy(&clipboardformat), ::core::mem::transmute(&pattendee)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnPasteFromClipboard: OnPasteFromClipboard::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIClipboardUseEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRDPSRAPIDebug_Impl: Sized {
    fn SetCLXCmdLine(&mut self, clxcmdline: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CLXCmdLine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIDebug_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIDebug_Vtbl {
        unsafe extern "system" fn SetCLXCmdLine<Impl: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCLXCmdLine(::core::mem::transmute_copy(&clxcmdline)).into()
        }
        unsafe extern "system" fn CLXCmdLine<Impl: IRDPSRAPIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLXCmdLine() {
                ::core::result::Result::Ok(ok__) => {
                    *pclxcmdline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCLXCmdLine: SetCLXCmdLine::<Impl, IMPL_OFFSET>,
            CLXCmdLine: CLXCmdLine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIFrameBuffer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn Bpp(&mut self) -> ::windows::core::Result<i32>;
    fn GetFrameBufferBits(&mut self, x: i32, y: i32, width: i32, heigth: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIFrameBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIFrameBuffer_Vtbl {
        unsafe extern "system" fn Width<Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *plwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *plheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bpp<Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bpp() {
                ::core::result::Result::Ok(ok__) => {
                    *plbpp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Impl: IRDPSRAPIFrameBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameBufferBits(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&heigth)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Bpp: Bpp::<Impl, IMPL_OFFSET>,
            GetFrameBufferBits: GetFrameBufferBits::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIFrameBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIInvitation_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectionString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GroupName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AttendeeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetAttendeeLimit(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn Revoked(&mut self) -> ::windows::core::Result<i16>;
    fn SetRevoked(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIInvitation_Vtbl {
        unsafe extern "system" fn ConnectionString<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttendeeLimit<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttendeeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttendeeLimit(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Revoked<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoked() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevoked<Impl: IRDPSRAPIInvitation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevoked(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectionString: ConnectionString::<Impl, IMPL_OFFSET>,
            GroupName: GroupName::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            AttendeeLimit: AttendeeLimit::<Impl, IMPL_OFFSET>,
            SetAttendeeLimit: SetAttendeeLimit::<Impl, IMPL_OFFSET>,
            Revoked: Revoked::<Impl, IMPL_OFFSET>,
            SetRevoked: SetRevoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIInvitationManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIInvitation>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn CreateInvitation(&mut self, bstrauthstring: super::super::Foundation::BSTR, bstrgroupname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, attendeelimit: i32) -> ::windows::core::Result<IRDPSRAPIInvitation>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIInvitationManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinvitation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInvitation<Impl: IRDPSRAPIInvitationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInvitation(::core::mem::transmute_copy(&bstrauthstring), ::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&attendeelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinvitation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            CreateInvitation: CreateInvitation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIInvitationManager as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPIPerfCounterLogger_Impl: Sized {
    fn LogValue(&mut self, lvalue: i64) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIPerfCounterLogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLogger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIPerfCounterLogger_Vtbl {
        unsafe extern "system" fn LogValue<Impl: IRDPSRAPIPerfCounterLogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogValue(::core::mem::transmute_copy(&lvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LogValue: LogValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLogger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRDPSRAPIPerfCounterLoggingManager_Impl: Sized {
    fn CreateLogger(&mut self, bstrcountername: super::super::Foundation::BSTR) -> ::windows::core::Result<IRDPSRAPIPerfCounterLogger>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIPerfCounterLoggingManager_Vtbl {
        unsafe extern "system" fn CreateLogger<Impl: IRDPSRAPIPerfCounterLoggingManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLogger(::core::mem::transmute_copy(&bstrcountername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplogger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateLogger: CreateLogger::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIPerfCounterLoggingManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISessionProperties_Impl: Sized + super::Com::IDispatch_Impl {
    fn Property(&mut self, propertyname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetProperty(&mut self, propertyname: super::super::Foundation::BSTR, newval: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISessionProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISessionProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISessionProperties_Vtbl {
        unsafe extern "system" fn Property<Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IRDPSRAPISessionProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Property: Property::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISessionProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISharingSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn SetColorDepth(&mut self, colordepth: i32) -> ::windows::core::Result<()>;
    fn ColorDepth(&mut self) -> ::windows::core::Result<i32>;
    fn Properties(&mut self) -> ::windows::core::Result<IRDPSRAPISessionProperties>;
    fn Attendees(&mut self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&mut self) -> ::windows::core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&mut self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&mut self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn ConnectToClient(&mut self, bstrconnectionstring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDesktopSharedRect(&mut self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()>;
    fn GetDesktopSharedRect(&mut self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISharingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISharingSession_Vtbl {
        unsafe extern "system" fn Open<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetColorDepth<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorDepth(::core::mem::transmute_copy(&colordepth)).into()
        }
        unsafe extern "system" fn ColorDepth<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolordepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ConnectToClient<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectToClient(::core::mem::transmute_copy(&bstrconnectionstring)).into()
        }
        unsafe extern "system" fn SetDesktopSharedRect<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesktopSharedRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn GetDesktopSharedRect<Impl: IRDPSRAPISharingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesktopSharedRect(::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            SetColorDepth: SetColorDepth::<Impl, IMPL_OFFSET>,
            ColorDepth: ColorDepth::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Attendees: Attendees::<Impl, IMPL_OFFSET>,
            Invitations: Invitations::<Impl, IMPL_OFFSET>,
            ApplicationFilter: ApplicationFilter::<Impl, IMPL_OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            ConnectToClient: ConnectToClient::<Impl, IMPL_OFFSET>,
            SetDesktopSharedRect: SetDesktopSharedRect::<Impl, IMPL_OFFSET>,
            GetDesktopSharedRect: GetDesktopSharedRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISharingSession2_Impl: Sized + super::Com::IDispatch_Impl + IRDPSRAPISharingSession_Impl {
    fn ConnectUsingTransportStream(&mut self, pstream: ::core::option::Option<IRDPSRAPITransportStream>, bstrgroup: super::super::Foundation::BSTR, bstrauthenticatedattendeename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FrameBuffer(&mut self) -> ::windows::core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(&mut self, pattendee: ::core::option::Option<IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISharingSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISharingSession2_Vtbl {
        unsafe extern "system" fn ConnectUsingTransportStream<Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectUsingTransportStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&bstrgroup), ::core::mem::transmute_copy(&bstrauthenticatedattendeename)).into()
        }
        unsafe extern "system" fn FrameBuffer<Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Impl: IRDPSRAPISharingSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattendee: ::windows::core::RawPtr, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendControlLevelChangeResponse(::core::mem::transmute(&pattendee), ::core::mem::transmute_copy(&requestedlevel), ::core::mem::transmute_copy(&reasoncode)).into()
        }
        Self {
            base: IRDPSRAPISharingSession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectUsingTransportStream: ConnectUsingTransportStream::<Impl, IMPL_OFFSET>,
            FrameBuffer: FrameBuffer::<Impl, IMPL_OFFSET>,
            SendControlLevelChangeResponse: SendControlLevelChangeResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISharingSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPITcpConnectionInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Protocol(&mut self) -> ::windows::core::Result<i32>;
    fn LocalPort(&mut self) -> ::windows::core::Result<i32>;
    fn LocalIP(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PeerPort(&mut self) -> ::windows::core::Result<i32>;
    fn PeerIP(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPITcpConnectionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITcpConnectionInfo_Vtbl {
        unsafe extern "system" fn Protocol<Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *plprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalIP<Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalIP() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsrlocalip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerPort<Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIP<Impl: IRDPSRAPITcpConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerIP() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            LocalPort: LocalPort::<Impl, IMPL_OFFSET>,
            LocalIP: LocalIP::<Impl, IMPL_OFFSET>,
            PeerPort: PeerPort::<Impl, IMPL_OFFSET>,
            PeerIP: PeerIP::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITcpConnectionInfo as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStream_Impl: Sized {
    fn AllocBuffer(&mut self, maxpayload: i32) -> ::windows::core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn WriteBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn ReadBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn Open(&mut self, pcallbacks: ::core::option::Option<IRDPSRAPITransportStreamEvents>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPITransportStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStream_Vtbl {
        unsafe extern "system" fn AllocBuffer<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocBuffer(::core::mem::transmute_copy(&maxpayload)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn WriteBuffer<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn ReadBuffer<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn Open<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&pcallbacks)).into()
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPITransportStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllocBuffer: AllocBuffer::<Impl, IMPL_OFFSET>,
            FreeBuffer: FreeBuffer::<Impl, IMPL_OFFSET>,
            WriteBuffer: WriteBuffer::<Impl, IMPL_OFFSET>,
            ReadBuffer: ReadBuffer::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStream as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamBuffer_Impl: Sized {
    fn Storage(&mut self) -> ::windows::core::Result<*mut u8>;
    fn StorageSize(&mut self) -> ::windows::core::Result<i32>;
    fn PayloadSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetPayloadSize(&mut self, lval: i32) -> ::windows::core::Result<()>;
    fn PayloadOffset(&mut self) -> ::windows::core::Result<i32>;
    fn SetPayloadOffset(&mut self, lretval: i32) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn SetFlags(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn Context(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetContext(&mut self, pcontext: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IRDPSRAPITransportStreamBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStreamBuffer_Vtbl {
        unsafe extern "system" fn Storage<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Storage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageSize<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadSize<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadSize<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadSize(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PayloadOffset<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayloadOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadOffset(::core::mem::transmute_copy(&lretval)).into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Context<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IRDPSRAPITransportStreamBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Storage: Storage::<Impl, IMPL_OFFSET>,
            StorageSize: StorageSize::<Impl, IMPL_OFFSET>,
            PayloadSize: PayloadSize::<Impl, IMPL_OFFSET>,
            SetPayloadSize: SetPayloadSize::<Impl, IMPL_OFFSET>,
            PayloadOffset: PayloadOffset::<Impl, IMPL_OFFSET>,
            SetPayloadOffset: SetPayloadOffset::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPITransportStreamEvents_Impl: Sized {
    fn OnWriteCompleted(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(&mut self, hrreason: ::windows::core::HRESULT);
}
impl IRDPSRAPITransportStreamEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStreamEvents_Vtbl {
        unsafe extern "system" fn OnWriteCompleted<Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWriteCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnReadCompleted<Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReadCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnStreamClosed<Impl: IRDPSRAPITransportStreamEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamClosed(::core::mem::transmute_copy(&hrreason))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnWriteCompleted: OnWriteCompleted::<Impl, IMPL_OFFSET>,
            OnReadCompleted: OnReadCompleted::<Impl, IMPL_OFFSET>,
            OnStreamClosed: OnStreamClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPITransportStreamEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIViewer_Impl: Sized + super::Com::IDispatch_Impl {
    fn Connect(&mut self, bstrconnectionstring: super::super::Foundation::BSTR, bstrname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Attendees(&mut self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager>;
    fn Invitations(&mut self) -> ::windows::core::Result<IRDPSRAPIInvitationManager>;
    fn ApplicationFilter(&mut self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter>;
    fn VirtualChannelManager(&mut self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager>;
    fn SetSmartSizing(&mut self, vbsmartsizing: i16) -> ::windows::core::Result<()>;
    fn SmartSizing(&mut self) -> ::windows::core::Result<i16>;
    fn RequestControl(&mut self, ctrllevel: CTRL_LEVEL) -> ::windows::core::Result<()>;
    fn SetDisconnectedText(&mut self, bstrdisconnectedtext: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisconnectedText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestColorDepthChange(&mut self, bpp: i32) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<IRDPSRAPISessionProperties>;
    fn StartReverseConnectListener(&mut self, bstrconnectionstring: super::super::Foundation::BSTR, bstrusername: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIViewer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIViewer_Vtbl {
        unsafe extern "system" fn Connect<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&bstrconnectionstring), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartSizing<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbsmartsizing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmartSizing(::core::mem::transmute_copy(&vbsmartsizing)).into()
        }
        unsafe extern "system" fn SmartSizing<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartSizing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbsmartsizing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestControl<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestControl(::core::mem::transmute_copy(&ctrllevel)).into()
        }
        unsafe extern "system" fn SetDisconnectedText<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisconnectedText(::core::mem::transmute_copy(&bstrdisconnectedtext)).into()
        }
        unsafe extern "system" fn DisconnectedText<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectedText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisconnectedtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestColorDepthChange(::core::mem::transmute_copy(&bpp)).into()
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Impl: IRDPSRAPIViewer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartReverseConnectListener(::core::mem::transmute_copy(&bstrconnectionstring), ::core::mem::transmute_copy(&bstrusername), ::core::mem::transmute_copy(&bstrpassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreverseconnectstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Attendees: Attendees::<Impl, IMPL_OFFSET>,
            Invitations: Invitations::<Impl, IMPL_OFFSET>,
            ApplicationFilter: ApplicationFilter::<Impl, IMPL_OFFSET>,
            VirtualChannelManager: VirtualChannelManager::<Impl, IMPL_OFFSET>,
            SetSmartSizing: SetSmartSizing::<Impl, IMPL_OFFSET>,
            SmartSizing: SmartSizing::<Impl, IMPL_OFFSET>,
            RequestControl: RequestControl::<Impl, IMPL_OFFSET>,
            SetDisconnectedText: SetDisconnectedText::<Impl, IMPL_OFFSET>,
            DisconnectedText: DisconnectedText::<Impl, IMPL_OFFSET>,
            RequestColorDepthChange: RequestColorDepthChange::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            StartReverseConnectListener: StartReverseConnectListener::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIViewer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIVirtualChannel_Impl: Sized + super::Com::IDispatch_Impl {
    fn SendData(&mut self, bstrdata: super::super::Foundation::BSTR, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::Result<()>;
    fn SetAccess(&mut self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn Priority(&mut self) -> ::windows::core::Result<CHANNEL_PRIORITY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIVirtualChannel_Vtbl {
        unsafe extern "system" fn SendData<Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendData(::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&channelsendflags)).into()
        }
        unsafe extern "system" fn SetAccess<Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccess(::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&accesstype)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IRDPSRAPIVirtualChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SendData: SendData::<Impl, IMPL_OFFSET>,
            SetAccess: SetAccess::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIVirtualChannelManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(&mut self, bstrchannelname: super::super::Foundation::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannelManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIVirtualChannelManager_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IRDPSRAPIVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(::core::mem::transmute_copy(&bstrchannelname), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&channelflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIVirtualChannelManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIWindow_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Application(&mut self) -> ::windows::core::Result<IRDPSRAPIApplication>;
    fn Shared(&mut self) -> ::windows::core::Result<i16>;
    fn SetShared(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Show(&mut self) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIWindow_Vtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Application() {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Application: Application::<Impl, IMPL_OFFSET>,
            Shared: Shared::<Impl, IMPL_OFFSET>,
            SetShared: SetShared::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIWindowList_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IRDPSRAPIWindow>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindowList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIWindowList_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIWindowList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIWindowList as ::windows::core::Interface>::IID
    }
}
pub trait IRDPViewerInputSink_Impl: Sized {
    fn SendMouseButtonEvent(&mut self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseMoveEvent(&mut self, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseWheelEvent(&mut self, wheelrotation: u16) -> ::windows::core::Result<()>;
    fn SendKeyboardEvent(&mut self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::Result<()>;
    fn SendSyncEvent(&mut self, syncflags: u32) -> ::windows::core::Result<()>;
    fn BeginTouchFrame(&mut self) -> ::windows::core::Result<()>;
    fn AddTouchInput(&mut self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn EndTouchFrame(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPViewerInputSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPViewerInputSink_Vtbl {
        unsafe extern "system" fn SendMouseButtonEvent<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseButtonEvent(::core::mem::transmute_copy(&buttontype), ::core::mem::transmute_copy(&vbbuttondown), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseMoveEvent<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseMoveEvent(::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseWheelEvent<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseWheelEvent(::core::mem::transmute_copy(&wheelrotation)).into()
        }
        unsafe extern "system" fn SendKeyboardEvent<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendKeyboardEvent(::core::mem::transmute_copy(&codetype), ::core::mem::transmute_copy(&keycode), ::core::mem::transmute_copy(&vbkeyup), ::core::mem::transmute_copy(&vbrepeat), ::core::mem::transmute_copy(&vbextended)).into()
        }
        unsafe extern "system" fn SendSyncEvent<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendSyncEvent(::core::mem::transmute_copy(&syncflags)).into()
        }
        unsafe extern "system" fn BeginTouchFrame<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginTouchFrame().into()
        }
        unsafe extern "system" fn AddTouchInput<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTouchInput(::core::mem::transmute_copy(&contactid), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn EndTouchFrame<Impl: IRDPViewerInputSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndTouchFrame().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendMouseButtonEvent: SendMouseButtonEvent::<Impl, IMPL_OFFSET>,
            SendMouseMoveEvent: SendMouseMoveEvent::<Impl, IMPL_OFFSET>,
            SendMouseWheelEvent: SendMouseWheelEvent::<Impl, IMPL_OFFSET>,
            SendKeyboardEvent: SendKeyboardEvent::<Impl, IMPL_OFFSET>,
            SendSyncEvent: SendSyncEvent::<Impl, IMPL_OFFSET>,
            BeginTouchFrame: BeginTouchFrame::<Impl, IMPL_OFFSET>,
            AddTouchInput: AddTouchInput::<Impl, IMPL_OFFSET>,
            EndTouchFrame: EndTouchFrame::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IRDPSessionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IRDPSessionEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IRDPSessionEvents as ::windows::core::Interface>::IID
    }
}
