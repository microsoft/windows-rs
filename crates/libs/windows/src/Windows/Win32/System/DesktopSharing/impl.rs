#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationImpl: Sized + IDispatchImpl {
    fn Windows();
    fn Id();
    fn Shared();
    fn SetShared();
    fn Name();
    fn Flags();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIApplication {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationVtbl {
    pub const fn new<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIApplicationVtbl {
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Windows(::core::mem::transmute_copy(&pwindowlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shared(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetShared(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIApplication>, base.5, Windows::<Impl, OFFSET>, Id::<Impl, OFFSET>, Shared::<Impl, OFFSET>, SetShared::<Impl, OFFSET>, Name::<Impl, OFFSET>, Flags::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationFilterImpl: Sized + IDispatchImpl {
    fn Applications();
    fn Windows();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIApplicationFilter {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIApplicationFilter";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationFilterVtbl {
    pub const fn new<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIApplicationFilterVtbl {
        unsafe extern "system" fn Applications<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Applications(::core::mem::transmute_copy(&papplications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwindows: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Windows(::core::mem::transmute_copy(&pwindows)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IRDPSRAPIApplicationFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnabled(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIApplicationFilter>, base.5, Applications::<Impl, OFFSET>, Windows::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationListImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIApplicationList {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIApplicationList";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationListVtbl {
    pub const fn new<Impl: IRDPSRAPIApplicationListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIApplicationListVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIApplicationListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIApplicationListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(item, ::core::mem::transmute_copy(&papplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIApplicationList>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeImpl: Sized + IDispatchImpl {
    fn Id();
    fn RemoteName();
    fn ControlLevel();
    fn SetControlLevel();
    fn Invitation();
    fn TerminateConnection();
    fn Flags();
    fn ConnectivityInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIAttendee {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIAttendee";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeVtbl {
    pub const fn new<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIAttendeeVtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteName<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlLevel<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControlLevel(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlLevel<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetControlLevel(pnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitation<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invitation(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminateConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&plflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectivityInfo<Impl: IRDPSRAPIAttendeeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectivityInfo(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIAttendee>, base.5, Id::<Impl, OFFSET>, RemoteName::<Impl, OFFSET>, ControlLevel::<Impl, OFFSET>, SetControlLevel::<Impl, OFFSET>, Invitation::<Impl, OFFSET>, TerminateConnection::<Impl, OFFSET>, Flags::<Impl, OFFSET>, ConnectivityInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeDisconnectInfoImpl: Sized + IDispatchImpl {
    fn Attendee();
    fn Reason();
    fn Code();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIAttendeeDisconnectInfo {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIAttendeeDisconnectInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeDisconnectInfoVtbl {
    pub const fn new<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIAttendeeDisconnectInfoVtbl {
        unsafe extern "system" fn Attendee<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Attendee(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason(::core::mem::transmute_copy(&preason)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Code<Impl: IRDPSRAPIAttendeeDisconnectInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Code(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIAttendeeDisconnectInfo>, base.5, Attendee::<Impl, OFFSET>, Reason::<Impl, OFFSET>, Code::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIAttendeeManager {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIAttendeeManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeManagerVtbl {
    pub const fn new<Impl: IRDPSRAPIAttendeeManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIAttendeeManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIAttendeeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIAttendeeManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(id, ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIAttendeeManager>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPIAudioStreamImpl: Sized {
    fn Initialize();
    fn Start();
    fn Stop();
    fn GetBuffer();
    fn FreeBuffer();
}
impl ::windows::core::RuntimeName for IRDPSRAPIAudioStream {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIAudioStream";
}
impl IRDPSRAPIAudioStreamVtbl {
    pub const fn new<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIAudioStreamVtbl {
        unsafe extern "system" fn Initialize<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&pnperiodinhundrednsintervals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&ptimestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPIAudioStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIAudioStream>, base.5, Initialize::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, GetBuffer::<Impl, OFFSET>, FreeBuffer::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPIClipboardUseEventsImpl: Sized {
    fn OnPasteFromClipboard();
}
impl ::windows::core::RuntimeName for IRDPSRAPIClipboardUseEvents {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIClipboardUseEvents";
}
impl IRDPSRAPIClipboardUseEventsVtbl {
    pub const fn new<Impl: IRDPSRAPIClipboardUseEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIClipboardUseEventsVtbl {
        unsafe extern "system" fn OnPasteFromClipboard<Impl: IRDPSRAPIClipboardUseEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPasteFromClipboard(clipboardformat, &*(&pattendee as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIClipboardUseEvents>, base.5, OnPasteFromClipboard::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPIDebugImpl: Sized {
    fn SetCLXCmdLine();
    fn CLXCmdLine();
}
impl ::windows::core::RuntimeName for IRDPSRAPIDebug {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIDebug";
}
impl IRDPSRAPIDebugVtbl {
    pub const fn new<Impl: IRDPSRAPIDebugImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIDebugVtbl {
        unsafe extern "system" fn SetCLXCmdLine<Impl: IRDPSRAPIDebugImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCLXCmdLine(&*(&clxcmdline as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLXCmdLine<Impl: IRDPSRAPIDebugImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CLXCmdLine(::core::mem::transmute_copy(&pclxcmdline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIDebug>, base.5, SetCLXCmdLine::<Impl, OFFSET>, CLXCmdLine::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIFrameBufferImpl: Sized + IDispatchImpl {
    fn Width();
    fn Height();
    fn Bpp();
    fn GetFrameBufferBits();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIFrameBuffer {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIFrameBuffer";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIFrameBufferVtbl {
    pub const fn new<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIFrameBufferVtbl {
        unsafe extern "system" fn Width<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&plwidth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&plheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bpp<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bpp(::core::mem::transmute_copy(&plbpp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameBufferBits<Impl: IRDPSRAPIFrameBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrameBufferBits(x, y, width, heigth, ::core::mem::transmute_copy(&ppbits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIFrameBuffer>, base.5, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>, Bpp::<Impl, OFFSET>, GetFrameBufferBits::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitationImpl: Sized + IDispatchImpl {
    fn ConnectionString();
    fn GroupName();
    fn Password();
    fn AttendeeLimit();
    fn SetAttendeeLimit();
    fn Revoked();
    fn SetRevoked();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIInvitation {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIInvitation";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationVtbl {
    pub const fn new<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIInvitationVtbl {
        unsafe extern "system" fn ConnectionString<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionString(::core::mem::transmute_copy(&pbstrval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GroupName(::core::mem::transmute_copy(&pbstrval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Password(::core::mem::transmute_copy(&pbstrval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttendeeLimit<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttendeeLimit(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttendeeLimit<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttendeeLimit(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoked<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Revoked(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevoked<Impl: IRDPSRAPIInvitationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRevoked(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIInvitation>, base.5, ConnectionString::<Impl, OFFSET>, GroupName::<Impl, OFFSET>, Password::<Impl, OFFSET>, AttendeeLimit::<Impl, OFFSET>, SetAttendeeLimit::<Impl, OFFSET>, Revoked::<Impl, OFFSET>, SetRevoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitationManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn CreateInvitation();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIInvitationManager {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIInvitationManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationManagerVtbl {
    pub const fn new<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIInvitationManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&item as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppinvitation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInvitation<Impl: IRDPSRAPIInvitationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInvitation(
                &*(&bstrauthstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                attendeelimit,
                ::core::mem::transmute_copy(&ppinvitation),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIInvitationManager>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, CreateInvitation::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPIPerfCounterLoggerImpl: Sized {
    fn LogValue();
}
impl ::windows::core::RuntimeName for IRDPSRAPIPerfCounterLogger {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIPerfCounterLogger";
}
impl IRDPSRAPIPerfCounterLoggerVtbl {
    pub const fn new<Impl: IRDPSRAPIPerfCounterLoggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIPerfCounterLoggerVtbl {
        unsafe extern "system" fn LogValue<Impl: IRDPSRAPIPerfCounterLoggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LogValue(lvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIPerfCounterLogger>, base.5, LogValue::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPIPerfCounterLoggingManagerImpl: Sized {
    fn CreateLogger();
}
impl ::windows::core::RuntimeName for IRDPSRAPIPerfCounterLoggingManager {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIPerfCounterLoggingManager";
}
impl IRDPSRAPIPerfCounterLoggingManagerVtbl {
    pub const fn new<Impl: IRDPSRAPIPerfCounterLoggingManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIPerfCounterLoggingManagerVtbl {
        unsafe extern "system" fn CreateLogger<Impl: IRDPSRAPIPerfCounterLoggingManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLogger(&*(&bstrcountername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplogger)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIPerfCounterLoggingManager>, base.5, CreateLogger::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISessionPropertiesImpl: Sized + IDispatchImpl {
    fn Property();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPISessionProperties {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPISessionProperties";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISessionPropertiesVtbl {
    pub const fn new<Impl: IRDPSRAPISessionPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPISessionPropertiesVtbl {
        unsafe extern "system" fn Property<Impl: IRDPSRAPISessionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Property(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IRDPSRAPISessionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&propertyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&newval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPISessionProperties>, base.5, Property::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSessionImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn SetColorDepth();
    fn ColorDepth();
    fn Properties();
    fn Attendees();
    fn Invitations();
    fn ApplicationFilter();
    fn VirtualChannelManager();
    fn Pause();
    fn Resume();
    fn ConnectToClient();
    fn SetDesktopSharedRect();
    fn GetDesktopSharedRect();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPISharingSession {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPISharingSession";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSessionVtbl {
    pub const fn new<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPISharingSessionVtbl {
        unsafe extern "system" fn Open<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorDepth<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorDepth(colordepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorDepth<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorDepth(::core::mem::transmute_copy(&pcolordepth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Attendees(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invitations(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectToClient<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectToClient(&*(&bstrconnectionstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesktopSharedRect<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDesktopSharedRect(left, top, right, bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesktopSharedRect<Impl: IRDPSRAPISharingSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDesktopSharedRect(::core::mem::transmute_copy(&pleft), ::core::mem::transmute_copy(&ptop), ::core::mem::transmute_copy(&pright), ::core::mem::transmute_copy(&pbottom)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IRDPSRAPISharingSession>,
            base.5,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            SetColorDepth::<Impl, OFFSET>,
            ColorDepth::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Attendees::<Impl, OFFSET>,
            Invitations::<Impl, OFFSET>,
            ApplicationFilter::<Impl, OFFSET>,
            VirtualChannelManager::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            ConnectToClient::<Impl, OFFSET>,
            SetDesktopSharedRect::<Impl, OFFSET>,
            GetDesktopSharedRect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSession2Impl: Sized + IRDPSRAPISharingSessionImpl + IDispatchImpl {
    fn ConnectUsingTransportStream();
    fn FrameBuffer();
    fn SendControlLevelChangeResponse();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPISharingSession2 {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPISharingSession2";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession2Vtbl {
    pub const fn new<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPISharingSession2Vtbl {
        unsafe extern "system" fn ConnectUsingTransportStream<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectUsingTransportStream(
                &*(&pstream as *const <IRDPSRAPITransportStream as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStream as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrgroup as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrauthenticatedattendeename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameBuffer<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameBuffer(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendControlLevelChangeResponse<Impl: IRDPSRAPISharingSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattendee: ::windows::core::RawPtr, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendControlLevelChangeResponse(&*(&pattendee as *const <IRDPSRAPIAttendee as ::windows::core::Abi>::Abi as *const <IRDPSRAPIAttendee as ::windows::core::DefaultType>::DefaultType), requestedlevel, reasoncode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPISharingSession2>, base.5, ConnectUsingTransportStream::<Impl, OFFSET>, FrameBuffer::<Impl, OFFSET>, SendControlLevelChangeResponse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPITcpConnectionInfoImpl: Sized + IDispatchImpl {
    fn Protocol();
    fn LocalPort();
    fn LocalIP();
    fn PeerPort();
    fn PeerIP();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPITcpConnectionInfo {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPITcpConnectionInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPITcpConnectionInfoVtbl {
    pub const fn new<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPITcpConnectionInfoVtbl {
        unsafe extern "system" fn Protocol<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&plprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalPort(::core::mem::transmute_copy(&plport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalIP<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalIP(::core::mem::transmute_copy(&pbsrlocalip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerPort<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeerPort(::core::mem::transmute_copy(&plport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerIP<Impl: IRDPSRAPITcpConnectionInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeerIP(::core::mem::transmute_copy(&pbstrip)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPITcpConnectionInfo>, base.5, Protocol::<Impl, OFFSET>, LocalPort::<Impl, OFFSET>, LocalIP::<Impl, OFFSET>, PeerPort::<Impl, OFFSET>, PeerIP::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPITransportStreamImpl: Sized {
    fn AllocBuffer();
    fn FreeBuffer();
    fn WriteBuffer();
    fn ReadBuffer();
    fn Open();
    fn Close();
}
impl ::windows::core::RuntimeName for IRDPSRAPITransportStream {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPITransportStream";
}
impl IRDPSRAPITransportStreamVtbl {
    pub const fn new<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPITransportStreamVtbl {
        unsafe extern "system" fn AllocBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllocBuffer(maxpayload, ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeBuffer(&*(&pbuffer as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteBuffer(&*(&pbuffer as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBuffer<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadBuffer(&*(&pbuffer as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallbacks: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(&*(&pcallbacks as *const <IRDPSRAPITransportStreamEvents as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamEvents as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IRDPSRAPITransportStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPITransportStream>, base.5, AllocBuffer::<Impl, OFFSET>, FreeBuffer::<Impl, OFFSET>, WriteBuffer::<Impl, OFFSET>, ReadBuffer::<Impl, OFFSET>, Open::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPITransportStreamBufferImpl: Sized {
    fn Storage();
    fn StorageSize();
    fn PayloadSize();
    fn SetPayloadSize();
    fn PayloadOffset();
    fn SetPayloadOffset();
    fn Flags();
    fn SetFlags();
    fn Context();
    fn SetContext();
}
impl ::windows::core::RuntimeName for IRDPSRAPITransportStreamBuffer {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPITransportStreamBuffer";
}
impl IRDPSRAPITransportStreamBufferVtbl {
    pub const fn new<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPITransportStreamBufferVtbl {
        unsafe extern "system" fn Storage<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Storage(::core::mem::transmute_copy(&ppbstorage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StorageSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StorageSize(::core::mem::transmute_copy(&plmaxstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PayloadSize(::core::mem::transmute_copy(&plretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadSize<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPayloadSize(lval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PayloadOffset<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PayloadOffset(::core::mem::transmute_copy(&plretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadOffset<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPayloadOffset(lretval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&plflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFlags(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Context(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IRDPSRAPITransportStreamBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContext(&*(&pcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPITransportStreamBuffer>, base.5, Storage::<Impl, OFFSET>, StorageSize::<Impl, OFFSET>, PayloadSize::<Impl, OFFSET>, SetPayloadSize::<Impl, OFFSET>, PayloadOffset::<Impl, OFFSET>, SetPayloadOffset::<Impl, OFFSET>, Flags::<Impl, OFFSET>, SetFlags::<Impl, OFFSET>, Context::<Impl, OFFSET>, SetContext::<Impl, OFFSET>)
    }
}
pub trait IRDPSRAPITransportStreamEventsImpl: Sized {
    fn OnWriteCompleted();
    fn OnReadCompleted();
    fn OnStreamClosed();
}
impl ::windows::core::RuntimeName for IRDPSRAPITransportStreamEvents {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPITransportStreamEvents";
}
impl IRDPSRAPITransportStreamEventsVtbl {
    pub const fn new<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPITransportStreamEventsVtbl {
        unsafe extern "system" fn OnWriteCompleted<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnWriteCompleted(&*(&pbuffer as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnReadCompleted<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnReadCompleted(&*(&pbuffer as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi as *const <IRDPSRAPITransportStreamBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnStreamClosed<Impl: IRDPSRAPITransportStreamEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnStreamClosed(hrreason).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPITransportStreamEvents>, base.5, OnWriteCompleted::<Impl, OFFSET>, OnReadCompleted::<Impl, OFFSET>, OnStreamClosed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIViewerImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Disconnect();
    fn Attendees();
    fn Invitations();
    fn ApplicationFilter();
    fn VirtualChannelManager();
    fn SetSmartSizing();
    fn SmartSizing();
    fn RequestControl();
    fn SetDisconnectedText();
    fn DisconnectedText();
    fn RequestColorDepthChange();
    fn Properties();
    fn StartReverseConnectListener();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIViewer {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIViewer";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIViewerVtbl {
    pub const fn new<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIViewerVtbl {
        unsafe extern "system" fn Connect<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(
                &*(&bstrconnectionstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attendees<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Attendees(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invitations<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invitations(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationFilter<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationFilter(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualChannelManager<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VirtualChannelManager(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmartSizing<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbsmartsizing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSmartSizing(vbsmartsizing) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmartSizing<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmartSizing(::core::mem::transmute_copy(&pvbsmartsizing)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestControl<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestControl(ctrllevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisconnectedText<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisconnectedText(&*(&bstrdisconnectedtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectedText<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectedText(::core::mem::transmute_copy(&pbstrdisconnectedtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestColorDepthChange<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestColorDepthChange(bpp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartReverseConnectListener<Impl: IRDPSRAPIViewerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartReverseConnectListener(
                &*(&bstrconnectionstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pbstrreverseconnectstring),
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IRDPSRAPIViewer>,
            base.5,
            Connect::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            Attendees::<Impl, OFFSET>,
            Invitations::<Impl, OFFSET>,
            ApplicationFilter::<Impl, OFFSET>,
            VirtualChannelManager::<Impl, OFFSET>,
            SetSmartSizing::<Impl, OFFSET>,
            SmartSizing::<Impl, OFFSET>,
            RequestControl::<Impl, OFFSET>,
            SetDisconnectedText::<Impl, OFFSET>,
            DisconnectedText::<Impl, OFFSET>,
            RequestColorDepthChange::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            StartReverseConnectListener::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannelImpl: Sized + IDispatchImpl {
    fn SendData();
    fn SetAccess();
    fn Name();
    fn Flags();
    fn Priority();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIVirtualChannel {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIVirtualChannel";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelVtbl {
    pub const fn new<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIVirtualChannelVtbl {
        unsafe extern "system" fn SendData<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendData(&*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lattendeeid, channelsendflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccess<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAccess(lattendeeid, accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&plflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IRDPSRAPIVirtualChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&ppriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIVirtualChannel>, base.5, SendData::<Impl, OFFSET>, SetAccess::<Impl, OFFSET>, Name::<Impl, OFFSET>, Flags::<Impl, OFFSET>, Priority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannelManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn CreateVirtualChannel();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIVirtualChannelManager {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIVirtualChannelManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelManagerVtbl {
    pub const fn new<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIVirtualChannelManagerVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&item as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Impl: IRDPSRAPIVirtualChannelManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVirtualChannel(&*(&bstrchannelname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), priority, channelflags, ::core::mem::transmute_copy(&ppchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIVirtualChannelManager>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, CreateVirtualChannel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindowImpl: Sized + IDispatchImpl {
    fn Id();
    fn Application();
    fn Shared();
    fn SetShared();
    fn Name();
    fn Show();
    fn Flags();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIWindow {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIWindow";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowVtbl {
    pub const fn new<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIWindowVtbl {
        unsafe extern "system" fn Id<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Application<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Application(::core::mem::transmute_copy(&papplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shared<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shared(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShared<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetShared(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Show() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRDPSRAPIWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIWindow>, base.5, Id::<Impl, OFFSET>, Application::<Impl, OFFSET>, Shared::<Impl, OFFSET>, SetShared::<Impl, OFFSET>, Name::<Impl, OFFSET>, Show::<Impl, OFFSET>, Flags::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindowListImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRDPSRAPIWindowList {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPSRAPIWindowList";
}
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowListVtbl {
    pub const fn new<Impl: IRDPSRAPIWindowListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPSRAPIWindowListVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IRDPSRAPIWindowListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRDPSRAPIWindowListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(item, ::core::mem::transmute_copy(&pwindow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPSRAPIWindowList>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IRDPViewerInputSinkImpl: Sized {
    fn SendMouseButtonEvent();
    fn SendMouseMoveEvent();
    fn SendMouseWheelEvent();
    fn SendKeyboardEvent();
    fn SendSyncEvent();
    fn BeginTouchFrame();
    fn AddTouchInput();
    fn EndTouchFrame();
}
impl ::windows::core::RuntimeName for IRDPViewerInputSink {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing.IRDPViewerInputSink";
}
impl IRDPViewerInputSinkVtbl {
    pub const fn new<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRDPViewerInputSinkVtbl {
        unsafe extern "system" fn SendMouseButtonEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendMouseButtonEvent(buttontype, vbbuttondown, xpos, ypos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMouseMoveEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendMouseMoveEvent(xpos, ypos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMouseWheelEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendMouseWheelEvent(wheelrotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendKeyboardEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendKeyboardEvent(codetype, keycode, vbkeyup, vbrepeat, vbextended) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendSyncEvent<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendSyncEvent(syncflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginTouchFrame<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginTouchFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTouchInput<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddTouchInput(contactid, event, x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndTouchFrame<Impl: IRDPViewerInputSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndTouchFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRDPViewerInputSink>, base.5, SendMouseButtonEvent::<Impl, OFFSET>, SendMouseMoveEvent::<Impl, OFFSET>, SendMouseWheelEvent::<Impl, OFFSET>, SendKeyboardEvent::<Impl, OFFSET>, SendSyncEvent::<Impl, OFFSET>, BeginTouchFrame::<Impl, OFFSET>, AddTouchInput::<Impl, OFFSET>, EndTouchFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IRDPSessionEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IRDPSessionEvents {
    const NAME: &'static str = "Windows.Win32.System.DesktopSharing._IRDPSessionEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _IRDPSessionEventsVtbl {
    pub const fn new<Impl: _IRDPSessionEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> _IRDPSessionEventsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<_IRDPSessionEvents>, base.5)
    }
}
