#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIApplicationImpl: Sized + IDispatchImpl {
    fn Windows(&mut self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Shared(&mut self) -> ::windows::core::Result<i16>;
    fn SetShared(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplicationVtbl {
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindowlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIApplicationFilterImpl: Sized + IDispatchImpl {
    fn Applications(&mut self) -> ::windows::core::Result<IRDPSRAPIApplicationList>;
    fn Windows(&mut self) -> ::windows::core::Result<IRDPSRAPIWindowList>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplicationFilterVtbl {
        unsafe extern "system" fn Applications<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Applications() {
                ::core::result::Result::Ok(ok__) => {
                    *papplications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindows: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindows = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIApplicationListImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IRDPSRAPIApplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIApplicationListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIApplicationListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIApplicationListVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIApplicationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIApplicationListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIApplicationList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPIAttendeeImpl: Sized + IDispatchImpl {
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
impl IRDPSRAPIAttendeeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendeeVtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteName<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlLevel<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlLevel<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlLevel(::core::mem::transmute_copy(&pnewval)).into()
        }
        unsafe extern "system" fn Invitation<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateConnection().into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIAttendeeDisconnectInfoImpl: Sized + IDispatchImpl {
    fn Attendee(&mut self) -> ::windows::core::Result<IRDPSRAPIAttendee>;
    fn Reason(&mut self) -> ::windows::core::Result<ATTENDEE_DISCONNECT_REASON>;
    fn Code(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeDisconnectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendeeDisconnectInfoVtbl {
        unsafe extern "system" fn Attendee<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendee() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *preason = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIAttendeeManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, id: i32) -> ::windows::core::Result<IRDPSRAPIAttendee>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIAttendeeManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAttendeeManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAttendeeManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIAttendeeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIAttendeeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIAttendeeManager as ::windows::core::Interface>::IID
    }
}
pub trait IRDPSRAPIAudioStreamImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<i64>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn GetBuffer(&mut self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::Result<()>;
    fn FreeBuffer(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIAudioStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIAudioStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIAudioStreamVtbl {
        unsafe extern "system" fn Initialize<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnperiodinhundrednsintervals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn GetBuffer<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&ptimestamp)).into()
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPIClipboardUseEventsImpl: Sized {
    fn OnPasteFromClipboard(&mut self, clipboardformat: u32, pattendee: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIClipboardUseEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIClipboardUseEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIClipboardUseEventsVtbl {
        unsafe extern "system" fn OnPasteFromClipboard<Impl: IRDPSRAPIClipboardUseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPIDebugImpl: Sized {
    fn SetCLXCmdLine(&mut self, clxcmdline: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CLXCmdLine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIDebugVtbl {
        unsafe extern "system" fn SetCLXCmdLine<Impl: IRDPSRAPIDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCLXCmdLine(::core::mem::transmute_copy(&clxcmdline)).into()
        }
        unsafe extern "system" fn CLXCmdLine<Impl: IRDPSRAPIDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPIFrameBufferImpl: Sized + IDispatchImpl {
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn Bpp(&mut self) -> ::windows::core::Result<i32>;
    fn GetFrameBufferBits(&mut self, x: i32, y: i32, width: i32, heigth: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIFrameBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIFrameBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIFrameBufferVtbl {
        unsafe extern "system" fn Width<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *plwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *plheight = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bpp<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bpp() {
                ::core::result::Result::Ok(ok__) => {
                    *plbpp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIInvitationImpl: Sized + IDispatchImpl {
    fn ConnectionString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GroupName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Password(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AttendeeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetAttendeeLimit(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn Revoked(&mut self) -> ::windows::core::Result<i16>;
    fn SetRevoked(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIInvitationVtbl {
        unsafe extern "system" fn ConnectionString<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttendeeLimit<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttendeeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttendeeLimit(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Revoked<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoked() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevoked<Impl: IRDPSRAPIInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevoked(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIInvitationManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIInvitation>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn CreateInvitation(&mut self, bstrauthstring: super::super::Foundation::BSTR, bstrgroupname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, attendeelimit: i32) -> ::windows::core::Result<IRDPSRAPIInvitation>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIInvitationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIInvitationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIInvitationManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinvitation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInvitation<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIPerfCounterLoggerImpl: Sized {
    fn LogValue(&mut self, lvalue: i64) -> ::windows::core::Result<()>;
}
impl IRDPSRAPIPerfCounterLoggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLoggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIPerfCounterLoggerVtbl {
        unsafe extern "system" fn LogValue<Impl: IRDPSRAPIPerfCounterLoggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPIPerfCounterLoggingManagerImpl: Sized {
    fn CreateLogger(&mut self, bstrcountername: super::super::Foundation::BSTR) -> ::windows::core::Result<IRDPSRAPIPerfCounterLogger>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRDPSRAPIPerfCounterLoggingManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIPerfCounterLoggingManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIPerfCounterLoggingManagerVtbl {
        unsafe extern "system" fn CreateLogger<Impl: IRDPSRAPIPerfCounterLoggingManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPISessionPropertiesImpl: Sized + IDispatchImpl {
    fn Property(&mut self, propertyname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetProperty(&mut self, propertyname: super::super::Foundation::BSTR, newval: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISessionPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISessionPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISessionPropertiesVtbl {
        unsafe extern "system" fn Property<Impl: IRDPSRAPISessionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IRDPSRAPISessionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Property: Property::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPISessionProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRDPSRAPISharingSessionImpl: Sized + IDispatchImpl {
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
impl IRDPSRAPISharingSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISharingSessionVtbl {
        unsafe extern "system" fn Open<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetColorDepth<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorDepth(::core::mem::transmute_copy(&colordepth)).into()
        }
        unsafe extern "system" fn ColorDepth<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolordepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ConnectToClient<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectToClient(::core::mem::transmute_copy(&bstrconnectionstring)).into()
        }
        unsafe extern "system" fn SetDesktopSharedRect<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesktopSharedRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom)).into()
        }
        unsafe extern "system" fn GetDesktopSharedRect<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesktopSharedRect(::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPISharingSession2Impl: Sized + IDispatchImpl + IRDPSRAPISharingSessionImpl {
    fn ConnectUsingTransportStream(&mut self, pstream: ::core::option::Option<IRDPSRAPITransportStream>, bstrgroup: super::super::Foundation::BSTR, bstrauthenticatedattendeename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FrameBuffer(&mut self) -> ::windows::core::Result<IRDPSRAPIFrameBuffer>;
    fn SendControlLevelChangeResponse(&mut self, pattendee: ::core::option::Option<IRDPSRAPIAttendee>, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPISharingSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPISharingSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPISharingSession2Vtbl {
        unsafe extern "system" fn ConnectUsingTransportStream<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectUsingTransportStream(::core::mem::transmute(&pstream), ::core::mem::transmute_copy(&bstrgroup), ::core::mem::transmute_copy(&bstrauthenticatedattendeename)).into()
        }
        unsafe extern "system" fn FrameBuffer<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattendee: ::windows::core::RawPtr, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendControlLevelChangeResponse(::core::mem::transmute(&pattendee), ::core::mem::transmute_copy(&requestedlevel), ::core::mem::transmute_copy(&reasoncode)).into()
        }
        Self {
            base: IRDPSRAPISharingSessionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPITcpConnectionInfoImpl: Sized + IDispatchImpl {
    fn Protocol(&mut self) -> ::windows::core::Result<i32>;
    fn LocalPort(&mut self) -> ::windows::core::Result<i32>;
    fn LocalIP(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PeerPort(&mut self) -> ::windows::core::Result<i32>;
    fn PeerIP(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPITcpConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITcpConnectionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITcpConnectionInfoVtbl {
        unsafe extern "system" fn Protocol<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *plprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalIP<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalIP() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsrlocalip = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerPort<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIP<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPITransportStreamImpl: Sized {
    fn AllocBuffer(&mut self, maxpayload: i32) -> ::windows::core::Result<IRDPSRAPITransportStreamBuffer>;
    fn FreeBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn WriteBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn ReadBuffer(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>) -> ::windows::core::Result<()>;
    fn Open(&mut self, pcallbacks: ::core::option::Option<IRDPSRAPITransportStreamEvents>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPSRAPITransportStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStreamVtbl {
        unsafe extern "system" fn AllocBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocBuffer(::core::mem::transmute_copy(&maxpayload)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn WriteBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn ReadBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadBuffer(::core::mem::transmute(&pbuffer)).into()
        }
        unsafe extern "system" fn Open<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&pcallbacks)).into()
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPITransportStreamBufferImpl: Sized {
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
impl IRDPSRAPITransportStreamBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStreamBufferVtbl {
        unsafe extern "system" fn Storage<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Storage() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbstorage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadSize(::core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PayloadOffset<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayloadOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *plretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadOffset(::core::mem::transmute_copy(&lretval)).into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Context<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRDPSRAPITransportStreamEventsImpl: Sized {
    fn OnWriteCompleted(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnReadCompleted(&mut self, pbuffer: ::core::option::Option<IRDPSRAPITransportStreamBuffer>);
    fn OnStreamClosed(&mut self, hrreason: ::windows::core::HRESULT);
}
impl IRDPSRAPITransportStreamEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPITransportStreamEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPITransportStreamEventsVtbl {
        unsafe extern "system" fn OnWriteCompleted<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWriteCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnReadCompleted<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReadCompleted(::core::mem::transmute(&pbuffer))
        }
        unsafe extern "system" fn OnStreamClosed<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT) {
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
pub trait IRDPSRAPIViewerImpl: Sized + IDispatchImpl {
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
impl IRDPSRAPIViewerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIViewerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIViewerVtbl {
        unsafe extern "system" fn Connect<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&bstrconnectionstring), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attendees() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartSizing<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbsmartsizing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmartSizing(::core::mem::transmute_copy(&vbsmartsizing)).into()
        }
        unsafe extern "system" fn SmartSizing<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmartSizing() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbsmartsizing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestControl<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestControl(::core::mem::transmute_copy(&ctrllevel)).into()
        }
        unsafe extern "system" fn SetDisconnectedText<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisconnectedText(::core::mem::transmute_copy(&bstrdisconnectedtext)).into()
        }
        unsafe extern "system" fn DisconnectedText<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectedText() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisconnectedtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestColorDepthChange(::core::mem::transmute_copy(&bpp)).into()
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Impl: IRDPSRAPIViewerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIVirtualChannelImpl: Sized + IDispatchImpl {
    fn SendData(&mut self, bstrdata: super::super::Foundation::BSTR, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::Result<()>;
    fn SetAccess(&mut self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn Priority(&mut self) -> ::windows::core::Result<CHANNEL_PRIORITY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIVirtualChannelVtbl {
        unsafe extern "system" fn SendData<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendData(::core::mem::transmute_copy(&bstrdata), ::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&channelsendflags)).into()
        }
        unsafe extern "system" fn SetAccess<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccess(::core::mem::transmute_copy(&lattendeeid), ::core::mem::transmute_copy(&accesstype)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *plflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIVirtualChannelManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: super::Com::VARIANT) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
    fn CreateVirtualChannel(&mut self, bstrchannelname: super::super::Foundation::BSTR, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIVirtualChannelManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIVirtualChannelManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIVirtualChannelManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pchannel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIWindowImpl: Sized + IDispatchImpl {
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Application(&mut self) -> ::windows::core::Result<IRDPSRAPIApplication>;
    fn Shared(&mut self) -> ::windows::core::Result<i16>;
    fn SetShared(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Show(&mut self) -> ::windows::core::Result<()>;
    fn Flags(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIWindowVtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Application() {
                ::core::result::Result::Ok(ok__) => {
                    *papplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shared() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShared(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
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
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRDPSRAPIWindowListImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, item: i32) -> ::windows::core::Result<IRDPSRAPIWindow>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRDPSRAPIWindowListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPSRAPIWindowListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPSRAPIWindowListVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIWindowListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIWindowListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRDPSRAPIWindowList as ::windows::core::Interface>::IID
    }
}
pub trait IRDPViewerInputSinkImpl: Sized {
    fn SendMouseButtonEvent(&mut self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseMoveEvent(&mut self, xpos: u32, ypos: u32) -> ::windows::core::Result<()>;
    fn SendMouseWheelEvent(&mut self, wheelrotation: u16) -> ::windows::core::Result<()>;
    fn SendKeyboardEvent(&mut self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::Result<()>;
    fn SendSyncEvent(&mut self, syncflags: u32) -> ::windows::core::Result<()>;
    fn BeginTouchFrame(&mut self) -> ::windows::core::Result<()>;
    fn AddTouchInput(&mut self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn EndTouchFrame(&mut self) -> ::windows::core::Result<()>;
}
impl IRDPViewerInputSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRDPViewerInputSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRDPViewerInputSinkVtbl {
        unsafe extern "system" fn SendMouseButtonEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseButtonEvent(::core::mem::transmute_copy(&buttontype), ::core::mem::transmute_copy(&vbbuttondown), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseMoveEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseMoveEvent(::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)).into()
        }
        unsafe extern "system" fn SendMouseWheelEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMouseWheelEvent(::core::mem::transmute_copy(&wheelrotation)).into()
        }
        unsafe extern "system" fn SendKeyboardEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendKeyboardEvent(::core::mem::transmute_copy(&codetype), ::core::mem::transmute_copy(&keycode), ::core::mem::transmute_copy(&vbkeyup), ::core::mem::transmute_copy(&vbrepeat), ::core::mem::transmute_copy(&vbextended)).into()
        }
        unsafe extern "system" fn SendSyncEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendSyncEvent(::core::mem::transmute_copy(&syncflags)).into()
        }
        unsafe extern "system" fn BeginTouchFrame<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginTouchFrame().into()
        }
        unsafe extern "system" fn AddTouchInput<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTouchInput(::core::mem::transmute_copy(&contactid), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn EndTouchFrame<Impl: IRDPViewerInputSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait _IRDPSessionEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IRDPSessionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IRDPSessionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IRDPSessionEventsVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IRDPSessionEvents as ::windows::core::Interface>::IID
    }
}
