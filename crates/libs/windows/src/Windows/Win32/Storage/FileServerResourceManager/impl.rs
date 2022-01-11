#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DIFsrmClassificationEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DIFsrmClassificationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DIFsrmClassificationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DIFsrmClassificationEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DIFsrmClassificationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAccessDeniedRemediationClientImpl: Sized + IDispatchImpl {
    fn Show();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAccessDeniedRemediationClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmAccessDeniedRemediationClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmAccessDeniedRemediationClientVtbl {
        unsafe extern "system" fn Show<Impl: IFsrmAccessDeniedRemediationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Show::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionImpl: Sized + IDispatchImpl {
    fn Id();
    fn ActionType();
    fn RunLimitInterval();
    fn SetRunLimitInterval();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionVtbl {
        unsafe extern "system" fn Id<Impl: IFsrmActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActionType<Impl: IFsrmActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunLimitInterval<Impl: IFsrmActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunLimitInterval<Impl: IFsrmActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, ActionType::<Impl, IMPL_OFFSET>, RunLimitInterval::<Impl, IMPL_OFFSET>, SetRunLimitInterval::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionCommandImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn ExecutablePath();
    fn SetExecutablePath();
    fn Arguments();
    fn SetArguments();
    fn Account();
    fn SetAccount();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
    fn MonitorCommand();
    fn SetMonitorCommand();
    fn KillTimeOut();
    fn SetKillTimeOut();
    fn LogResult();
    fn SetLogResult();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionCommandVtbl {
        unsafe extern "system" fn ExecutablePath<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExecutablePath<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Arguments<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetArguments<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Account<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccount<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonitorCommand<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMonitorCommand<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KillTimeOut<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKillTimeOut<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LogResult<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLogResult<Impl: IFsrmActionCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            ExecutablePath::<Impl, IMPL_OFFSET>,
            SetExecutablePath::<Impl, IMPL_OFFSET>,
            Arguments::<Impl, IMPL_OFFSET>,
            SetArguments::<Impl, IMPL_OFFSET>,
            Account::<Impl, IMPL_OFFSET>,
            SetAccount::<Impl, IMPL_OFFSET>,
            WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            MonitorCommand::<Impl, IMPL_OFFSET>,
            SetMonitorCommand::<Impl, IMPL_OFFSET>,
            KillTimeOut::<Impl, IMPL_OFFSET>,
            SetKillTimeOut::<Impl, IMPL_OFFSET>,
            LogResult::<Impl, IMPL_OFFSET>,
            SetLogResult::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmailImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn MailFrom();
    fn SetMailFrom();
    fn MailReplyTo();
    fn SetMailReplyTo();
    fn MailTo();
    fn SetMailTo();
    fn MailCc();
    fn SetMailCc();
    fn MailBcc();
    fn SetMailBcc();
    fn MailSubject();
    fn SetMailSubject();
    fn MessageText();
    fn SetMessageText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmailVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEmailImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEmailVtbl {
        unsafe extern "system" fn MailFrom<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailFrom<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailReplyTo<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailReplyTo<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailCc<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailCc<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailBcc<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailBcc<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailSubject<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailSubject<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageText<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMessageText<Impl: IFsrmActionEmailImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            MailFrom::<Impl, IMPL_OFFSET>,
            SetMailFrom::<Impl, IMPL_OFFSET>,
            MailReplyTo::<Impl, IMPL_OFFSET>,
            SetMailReplyTo::<Impl, IMPL_OFFSET>,
            MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo::<Impl, IMPL_OFFSET>,
            MailCc::<Impl, IMPL_OFFSET>,
            SetMailCc::<Impl, IMPL_OFFSET>,
            MailBcc::<Impl, IMPL_OFFSET>,
            SetMailBcc::<Impl, IMPL_OFFSET>,
            MailSubject::<Impl, IMPL_OFFSET>,
            SetMailSubject::<Impl, IMPL_OFFSET>,
            MessageText::<Impl, IMPL_OFFSET>,
            SetMessageText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmail2Impl: Sized + IFsrmActionEmailImpl + IFsrmActionImpl + IDispatchImpl {
    fn AttachmentFileListSize();
    fn SetAttachmentFileListSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmail2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEmail2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEmail2Vtbl {
        unsafe extern "system" fn AttachmentFileListSize<Impl: IFsrmActionEmail2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Impl: IFsrmActionEmail2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            MailFrom::<Impl, IMPL_OFFSET>,
            SetMailFrom::<Impl, IMPL_OFFSET>,
            MailReplyTo::<Impl, IMPL_OFFSET>,
            SetMailReplyTo::<Impl, IMPL_OFFSET>,
            MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo::<Impl, IMPL_OFFSET>,
            MailCc::<Impl, IMPL_OFFSET>,
            SetMailCc::<Impl, IMPL_OFFSET>,
            MailBcc::<Impl, IMPL_OFFSET>,
            SetMailBcc::<Impl, IMPL_OFFSET>,
            MailSubject::<Impl, IMPL_OFFSET>,
            SetMailSubject::<Impl, IMPL_OFFSET>,
            MessageText::<Impl, IMPL_OFFSET>,
            SetMessageText::<Impl, IMPL_OFFSET>,
            AttachmentFileListSize::<Impl, IMPL_OFFSET>,
            SetAttachmentFileListSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEventLogImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn EventType();
    fn SetEventType();
    fn MessageText();
    fn SetMessageText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEventLogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEventLogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEventLogVtbl {
        unsafe extern "system" fn EventType<Impl: IFsrmActionEventLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventType<Impl: IFsrmActionEventLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageText<Impl: IFsrmActionEventLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMessageText<Impl: IFsrmActionEventLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            EventType::<Impl, IMPL_OFFSET>,
            SetEventType::<Impl, IMPL_OFFSET>,
            MessageText::<Impl, IMPL_OFFSET>,
            SetMessageText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionReportImpl: Sized + IFsrmActionImpl + IDispatchImpl {
    fn ReportTypes();
    fn SetReportTypes();
    fn MailTo();
    fn SetMailTo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionReportVtbl {
        unsafe extern "system" fn ReportTypes<Impl: IFsrmActionReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReportTypes<Impl: IFsrmActionReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmActionReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmActionReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            ReportTypes::<Impl, IMPL_OFFSET>,
            SetReportTypes::<Impl, IMPL_OFFSET>,
            MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAutoApplyQuotaImpl: Sized + IFsrmQuotaObjectImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn ExcludeFolders();
    fn SetExcludeFolders();
    fn CommitAndUpdateDerived();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAutoApplyQuotaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmAutoApplyQuotaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmAutoApplyQuotaVtbl {
        unsafe extern "system" fn ExcludeFolders<Impl: IFsrmAutoApplyQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExcludeFolders<Impl: IFsrmAutoApplyQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmAutoApplyQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            UserSid::<Impl, IMPL_OFFSET>,
            UserAccount::<Impl, IMPL_OFFSET>,
            SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            ApplyTemplate::<Impl, IMPL_OFFSET>,
            ExcludeFolders::<Impl, IMPL_OFFSET>,
            SetExcludeFolders::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAutoApplyQuota as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManagerImpl: Sized + IDispatchImpl {
    fn ClassificationReportFormats();
    fn SetClassificationReportFormats();
    fn Logging();
    fn SetLogging();
    fn ClassificationReportMailTo();
    fn SetClassificationReportMailTo();
    fn ClassificationReportEnabled();
    fn SetClassificationReportEnabled();
    fn ClassificationLastReportPathWithoutExtension();
    fn ClassificationLastError();
    fn ClassificationRunningStatus();
    fn EnumPropertyDefinitions();
    fn CreatePropertyDefinition();
    fn GetPropertyDefinition();
    fn EnumRules();
    fn CreateRule();
    fn GetRule();
    fn EnumModuleDefinitions();
    fn CreateModuleDefinition();
    fn GetModuleDefinition();
    fn RunClassification();
    fn WaitForClassificationCompletion();
    fn CancelClassification();
    fn EnumFileProperties();
    fn GetFileProperty();
    fn SetFileProperty();
    fn ClearFileProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationManagerVtbl {
        unsafe extern "system" fn ClassificationReportFormats<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassificationReportFormats<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Logging<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLogging<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassificationReportMailTo<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassificationReportEnabled<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassificationLastError<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassificationRunningStatus<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePropertyDefinition<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyDefinition<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRules<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRule<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRule<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumModuleDefinitions<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateModuleDefinition<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModuleDefinition<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunClassification<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelClassification<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFileProperties<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileProperty<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileProperty<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearFileProperty<Impl: IFsrmClassificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            ClassificationReportFormats::<Impl, IMPL_OFFSET>,
            SetClassificationReportFormats::<Impl, IMPL_OFFSET>,
            Logging::<Impl, IMPL_OFFSET>,
            SetLogging::<Impl, IMPL_OFFSET>,
            ClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            SetClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            ClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            SetClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            ClassificationLastReportPathWithoutExtension::<Impl, IMPL_OFFSET>,
            ClassificationLastError::<Impl, IMPL_OFFSET>,
            ClassificationRunningStatus::<Impl, IMPL_OFFSET>,
            EnumPropertyDefinitions::<Impl, IMPL_OFFSET>,
            CreatePropertyDefinition::<Impl, IMPL_OFFSET>,
            GetPropertyDefinition::<Impl, IMPL_OFFSET>,
            EnumRules::<Impl, IMPL_OFFSET>,
            CreateRule::<Impl, IMPL_OFFSET>,
            GetRule::<Impl, IMPL_OFFSET>,
            EnumModuleDefinitions::<Impl, IMPL_OFFSET>,
            CreateModuleDefinition::<Impl, IMPL_OFFSET>,
            GetModuleDefinition::<Impl, IMPL_OFFSET>,
            RunClassification::<Impl, IMPL_OFFSET>,
            WaitForClassificationCompletion::<Impl, IMPL_OFFSET>,
            CancelClassification::<Impl, IMPL_OFFSET>,
            EnumFileProperties::<Impl, IMPL_OFFSET>,
            GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty::<Impl, IMPL_OFFSET>,
            ClearFileProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManager2Impl: Sized + IFsrmClassificationManagerImpl + IDispatchImpl {
    fn ClassifyFiles();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationManager2Vtbl {
        unsafe extern "system" fn ClassifyFiles<Impl: IFsrmClassificationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT {
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
            ClassificationReportFormats::<Impl, IMPL_OFFSET>,
            SetClassificationReportFormats::<Impl, IMPL_OFFSET>,
            Logging::<Impl, IMPL_OFFSET>,
            SetLogging::<Impl, IMPL_OFFSET>,
            ClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            SetClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            ClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            SetClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            ClassificationLastReportPathWithoutExtension::<Impl, IMPL_OFFSET>,
            ClassificationLastError::<Impl, IMPL_OFFSET>,
            ClassificationRunningStatus::<Impl, IMPL_OFFSET>,
            EnumPropertyDefinitions::<Impl, IMPL_OFFSET>,
            CreatePropertyDefinition::<Impl, IMPL_OFFSET>,
            GetPropertyDefinition::<Impl, IMPL_OFFSET>,
            EnumRules::<Impl, IMPL_OFFSET>,
            CreateRule::<Impl, IMPL_OFFSET>,
            GetRule::<Impl, IMPL_OFFSET>,
            EnumModuleDefinitions::<Impl, IMPL_OFFSET>,
            CreateModuleDefinition::<Impl, IMPL_OFFSET>,
            GetModuleDefinition::<Impl, IMPL_OFFSET>,
            RunClassification::<Impl, IMPL_OFFSET>,
            WaitForClassificationCompletion::<Impl, IMPL_OFFSET>,
            CancelClassification::<Impl, IMPL_OFFSET>,
            EnumFileProperties::<Impl, IMPL_OFFSET>,
            GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty::<Impl, IMPL_OFFSET>,
            ClearFileProperty::<Impl, IMPL_OFFSET>,
            ClassifyFiles::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationRuleImpl: Sized + IFsrmRuleImpl + IFsrmObjectImpl + IDispatchImpl {
    fn ExecutionOption();
    fn SetExecutionOption();
    fn PropertyAffected();
    fn SetPropertyAffected();
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationRuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationRuleVtbl {
        unsafe extern "system" fn ExecutionOption<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExecutionOption<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyAffected<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyAffected<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmClassificationRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            RuleType::<Impl, IMPL_OFFSET>,
            ModuleDefinitionName::<Impl, IMPL_OFFSET>,
            SetModuleDefinitionName::<Impl, IMPL_OFFSET>,
            NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            RuleFlags::<Impl, IMPL_OFFSET>,
            SetRuleFlags::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            LastModified::<Impl, IMPL_OFFSET>,
            ExecutionOption::<Impl, IMPL_OFFSET>,
            SetExecutionOption::<Impl, IMPL_OFFSET>,
            PropertyAffected::<Impl, IMPL_OFFSET>,
            SetPropertyAffected::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleDefinitionImpl: Sized + IFsrmPipelineModuleDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn PropertiesAffected();
    fn SetPropertiesAffected();
    fn PropertiesUsed();
    fn SetPropertiesUsed();
    fn NeedsExplicitValue();
    fn SetNeedsExplicitValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassifierModuleDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassifierModuleDefinitionVtbl {
        unsafe extern "system" fn PropertiesAffected<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertiesAffected<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertiesUsed<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertiesUsed<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NeedsExplicitValue<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Impl: IFsrmClassifierModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            ModuleClsid::<Impl, IMPL_OFFSET>,
            SetModuleClsid::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Company::<Impl, IMPL_OFFSET>,
            SetCompany::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            ModuleType::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            NeedsFileContent::<Impl, IMPL_OFFSET>,
            SetNeedsFileContent::<Impl, IMPL_OFFSET>,
            Account::<Impl, IMPL_OFFSET>,
            SetAccount::<Impl, IMPL_OFFSET>,
            SupportedExtensions::<Impl, IMPL_OFFSET>,
            SetSupportedExtensions::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            PropertiesAffected::<Impl, IMPL_OFFSET>,
            SetPropertiesAffected::<Impl, IMPL_OFFSET>,
            PropertiesUsed::<Impl, IMPL_OFFSET>,
            SetPropertiesUsed::<Impl, IMPL_OFFSET>,
            NeedsExplicitValue::<Impl, IMPL_OFFSET>,
            SetNeedsExplicitValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleImplementationImpl: Sized + IFsrmPipelineModuleImplementationImpl + IDispatchImpl {
    fn LastModified();
    fn UseRulesAndDefinitions();
    fn OnBeginFile();
    fn DoesPropertyValueApply();
    fn GetPropertyValueToApply();
    fn OnEndFile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleImplementationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassifierModuleImplementationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassifierModuleImplementationVtbl {
        unsafe extern "system" fn LastModified<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: ::windows::core::RawPtr, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnBeginFile<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoesPropertyValueApply<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValueToApply<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEndFile<Impl: IFsrmClassifierModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            OnLoad::<Impl, IMPL_OFFSET>,
            OnUnload::<Impl, IMPL_OFFSET>,
            LastModified::<Impl, IMPL_OFFSET>,
            UseRulesAndDefinitions::<Impl, IMPL_OFFSET>,
            OnBeginFile::<Impl, IMPL_OFFSET>,
            DoesPropertyValueApply::<Impl, IMPL_OFFSET>,
            GetPropertyValueToApply::<Impl, IMPL_OFFSET>,
            OnEndFile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleImplementation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn State();
    fn Cancel();
    fn WaitForCompletion();
    fn GetById();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetById<Impl: IFsrmCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            WaitForCompletion::<Impl, IMPL_OFFSET>,
            GetById::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCommittableCollectionImpl: Sized + IFsrmMutableCollectionImpl + IFsrmCollectionImpl + IDispatchImpl {
    fn Commit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCommittableCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmCommittableCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmCommittableCollectionVtbl {
        unsafe extern "system" fn Commit<Impl: IFsrmCommittableCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            WaitForCompletion::<Impl, IMPL_OFFSET>,
            GetById::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            RemoveById::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCommittableCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmDerivedObjectsResultImpl: Sized + IDispatchImpl {
    fn DerivedObjects();
    fn Results();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmDerivedObjectsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmDerivedObjectsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmDerivedObjectsResultVtbl {
        unsafe extern "system" fn DerivedObjects<Impl: IFsrmDerivedObjectsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedobjects: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Results<Impl: IFsrmDerivedObjectsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, DerivedObjects::<Impl, IMPL_OFFSET>, Results::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmExportImportImpl: Sized + IDispatchImpl {
    fn ExportFileGroups();
    fn ImportFileGroups();
    fn ExportFileScreenTemplates();
    fn ImportFileScreenTemplates();
    fn ExportQuotaTemplates();
    fn ImportQuotaTemplates();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmExportImportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmExportImportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmExportImportVtbl {
        unsafe extern "system" fn ExportFileGroups<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportFileGroups<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportQuotaTemplates<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportQuotaTemplates<Impl: IFsrmExportImportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ExportFileGroups::<Impl, IMPL_OFFSET>,
            ImportFileGroups::<Impl, IMPL_OFFSET>,
            ExportFileScreenTemplates::<Impl, IMPL_OFFSET>,
            ImportFileScreenTemplates::<Impl, IMPL_OFFSET>,
            ExportQuotaTemplates::<Impl, IMPL_OFFSET>,
            ImportQuotaTemplates::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmExportImport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileConditionImpl: Sized + IDispatchImpl {
    fn Type();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileConditionVtbl {
        unsafe extern "system" fn Type<Impl: IFsrmFileConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmFileConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileConditionPropertyImpl: Sized + IFsrmFileConditionImpl + IDispatchImpl {
    fn PropertyName();
    fn SetPropertyName();
    fn PropertyId();
    fn SetPropertyId();
    fn Operator();
    fn SetOperator();
    fn ValueType();
    fn SetValueType();
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileConditionPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileConditionPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileConditionPropertyVtbl {
        unsafe extern "system" fn PropertyName<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyName<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyId<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyId<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Operator<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOperator<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValueType<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueType<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmFileConditionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            Type::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            PropertyName::<Impl, IMPL_OFFSET>,
            SetPropertyName::<Impl, IMPL_OFFSET>,
            PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId::<Impl, IMPL_OFFSET>,
            Operator::<Impl, IMPL_OFFSET>,
            SetOperator::<Impl, IMPL_OFFSET>,
            ValueType::<Impl, IMPL_OFFSET>,
            SetValueType::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileConditionProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Members();
    fn SetMembers();
    fn NonMembers();
    fn SetNonMembers();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroupVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Members<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMembers<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NonMembers<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNonMembers<Impl: IFsrmFileGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Members::<Impl, IMPL_OFFSET>,
            SetMembers::<Impl, IMPL_OFFSET>,
            NonMembers::<Impl, IMPL_OFFSET>,
            SetNonMembers::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupImportedImpl: Sized + IFsrmFileGroupImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupImportedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroupImportedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroupImportedVtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmFileGroupImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmFileGroupImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Members::<Impl, IMPL_OFFSET>,
            SetMembers::<Impl, IMPL_OFFSET>,
            NonMembers::<Impl, IMPL_OFFSET>,
            SetNonMembers::<Impl, IMPL_OFFSET>,
            OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupManagerImpl: Sized + IDispatchImpl {
    fn CreateFileGroup();
    fn GetFileGroup();
    fn EnumFileGroups();
    fn ExportFileGroups();
    fn ImportFileGroups();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroupManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroupManagerVtbl {
        unsafe extern "system" fn CreateFileGroup<Impl: IFsrmFileGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileGroup<Impl: IFsrmFileGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFileGroups<Impl: IFsrmFileGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportFileGroups<Impl: IFsrmFileGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportFileGroups<Impl: IFsrmFileGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilegroups: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            CreateFileGroup::<Impl, IMPL_OFFSET>,
            GetFileGroup::<Impl, IMPL_OFFSET>,
            EnumFileGroups::<Impl, IMPL_OFFSET>,
            ExportFileGroups::<Impl, IMPL_OFFSET>,
            ImportFileGroups::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJobImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn Enabled();
    fn SetEnabled();
    fn OperationType();
    fn SetOperationType();
    fn ExpirationDirectory();
    fn SetExpirationDirectory();
    fn CustomAction();
    fn Notifications();
    fn Logging();
    fn SetLogging();
    fn ReportEnabled();
    fn SetReportEnabled();
    fn Formats();
    fn SetFormats();
    fn MailTo();
    fn SetMailTo();
    fn DaysSinceFileCreated();
    fn SetDaysSinceFileCreated();
    fn DaysSinceFileLastAccessed();
    fn SetDaysSinceFileLastAccessed();
    fn DaysSinceFileLastModified();
    fn SetDaysSinceFileLastModified();
    fn PropertyConditions();
    fn FromDate();
    fn SetFromDate();
    fn Task();
    fn SetTask();
    fn Parameters();
    fn SetParameters();
    fn RunningStatus();
    fn LastError();
    fn LastReportPathWithoutExtension();
    fn LastRun();
    fn FileNamePattern();
    fn SetFileNamePattern();
    fn Run();
    fn WaitForCompletion();
    fn Cancel();
    fn AddNotification();
    fn DeleteNotification();
    fn ModifyNotification();
    fn CreateNotificationAction();
    fn EnumNotificationActions();
    fn CreatePropertyCondition();
    fn CreateCustomAction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileManagementJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileManagementJobVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OperationType<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOperationType<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpirationDirectory<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExpirationDirectory<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CustomAction<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notifications<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Logging<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLogging<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReportEnabled<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReportEnabled<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Formats<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormats<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DaysSinceFileCreated<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysSinceFileCreated<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DaysSinceFileLastAccessed<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysSinceFileLastAccessed<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DaysSinceFileLastModified<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysSinceFileLastModified<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyConditions<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyconditions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FromDate<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFromDate<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Task<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTask<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunningStatus<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastError<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastReportPathWithoutExtension<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastRun<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileNamePattern<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileNamePattern<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Run<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNotification<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteNotification<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyNotification<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNotificationAction<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumNotificationActions<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePropertyCondition<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCustomAction<Impl: IFsrmFileManagementJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            OperationType::<Impl, IMPL_OFFSET>,
            SetOperationType::<Impl, IMPL_OFFSET>,
            ExpirationDirectory::<Impl, IMPL_OFFSET>,
            SetExpirationDirectory::<Impl, IMPL_OFFSET>,
            CustomAction::<Impl, IMPL_OFFSET>,
            Notifications::<Impl, IMPL_OFFSET>,
            Logging::<Impl, IMPL_OFFSET>,
            SetLogging::<Impl, IMPL_OFFSET>,
            ReportEnabled::<Impl, IMPL_OFFSET>,
            SetReportEnabled::<Impl, IMPL_OFFSET>,
            Formats::<Impl, IMPL_OFFSET>,
            SetFormats::<Impl, IMPL_OFFSET>,
            MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo::<Impl, IMPL_OFFSET>,
            DaysSinceFileCreated::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileCreated::<Impl, IMPL_OFFSET>,
            DaysSinceFileLastAccessed::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileLastAccessed::<Impl, IMPL_OFFSET>,
            DaysSinceFileLastModified::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileLastModified::<Impl, IMPL_OFFSET>,
            PropertyConditions::<Impl, IMPL_OFFSET>,
            FromDate::<Impl, IMPL_OFFSET>,
            SetFromDate::<Impl, IMPL_OFFSET>,
            Task::<Impl, IMPL_OFFSET>,
            SetTask::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            RunningStatus::<Impl, IMPL_OFFSET>,
            LastError::<Impl, IMPL_OFFSET>,
            LastReportPathWithoutExtension::<Impl, IMPL_OFFSET>,
            LastRun::<Impl, IMPL_OFFSET>,
            FileNamePattern::<Impl, IMPL_OFFSET>,
            SetFileNamePattern::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            WaitForCompletion::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            AddNotification::<Impl, IMPL_OFFSET>,
            DeleteNotification::<Impl, IMPL_OFFSET>,
            ModifyNotification::<Impl, IMPL_OFFSET>,
            CreateNotificationAction::<Impl, IMPL_OFFSET>,
            EnumNotificationActions::<Impl, IMPL_OFFSET>,
            CreatePropertyCondition::<Impl, IMPL_OFFSET>,
            CreateCustomAction::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJobManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn EnumFileManagementJobs();
    fn CreateFileManagementJob();
    fn GetFileManagementJob();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJobManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileManagementJobManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileManagementJobManagerVtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmFileManagementJobManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmFileManagementJobManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFileManagementJobs<Impl: IFsrmFileManagementJobManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFileManagementJob<Impl: IFsrmFileManagementJobManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileManagementJob<Impl: IFsrmFileManagementJobManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            EnumFileManagementJobs::<Impl, IMPL_OFFSET>,
            CreateFileManagementJob::<Impl, IMPL_OFFSET>,
            GetFileManagementJob::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJobManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenImpl: Sized + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn SourceTemplateName();
    fn MatchesSourceTemplate();
    fn UserSid();
    fn UserAccount();
    fn ApplyTemplate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenVtbl {
        unsafe extern "system" fn Path<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceTemplateName<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MatchesSourceTemplate<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSid<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserAccount<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyTemplate<Impl: IFsrmFileScreenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            BlockedFileGroups::<Impl, IMPL_OFFSET>,
            SetBlockedFileGroups::<Impl, IMPL_OFFSET>,
            FileScreenFlags::<Impl, IMPL_OFFSET>,
            SetFileScreenFlags::<Impl, IMPL_OFFSET>,
            CreateAction::<Impl, IMPL_OFFSET>,
            EnumActions::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            UserSid::<Impl, IMPL_OFFSET>,
            UserAccount::<Impl, IMPL_OFFSET>,
            ApplyTemplate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreen as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenBaseImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn BlockedFileGroups();
    fn SetBlockedFileGroups();
    fn FileScreenFlags();
    fn SetFileScreenFlags();
    fn CreateAction();
    fn EnumActions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenBaseVtbl {
        unsafe extern "system" fn BlockedFileGroups<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlockedFileGroups<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileScreenFlags<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileScreenFlags<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAction<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumActions<Impl: IFsrmFileScreenBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            BlockedFileGroups::<Impl, IMPL_OFFSET>,
            SetBlockedFileGroups::<Impl, IMPL_OFFSET>,
            FileScreenFlags::<Impl, IMPL_OFFSET>,
            SetFileScreenFlags::<Impl, IMPL_OFFSET>,
            CreateAction::<Impl, IMPL_OFFSET>,
            EnumActions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenExceptionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn AllowedFileGroups();
    fn SetAllowedFileGroups();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenExceptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenExceptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenExceptionVtbl {
        unsafe extern "system" fn Path<Impl: IFsrmFileScreenExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowedFileGroups<Impl: IFsrmFileScreenExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowedFileGroups<Impl: IFsrmFileScreenExceptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            AllowedFileGroups::<Impl, IMPL_OFFSET>,
            SetAllowedFileGroups::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenException as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn CreateFileScreen();
    fn GetFileScreen();
    fn EnumFileScreens();
    fn CreateFileScreenException();
    fn GetFileScreenException();
    fn EnumFileScreenExceptions();
    fn CreateFileScreenCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenManagerVtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFileScreen<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileScreen<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFileScreens<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFileScreenException<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileScreenException<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFileScreenExceptions<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFileScreenCollection<Impl: IFsrmFileScreenManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            CreateFileScreen::<Impl, IMPL_OFFSET>,
            GetFileScreen::<Impl, IMPL_OFFSET>,
            EnumFileScreens::<Impl, IMPL_OFFSET>,
            CreateFileScreenException::<Impl, IMPL_OFFSET>,
            GetFileScreenException::<Impl, IMPL_OFFSET>,
            EnumFileScreenExceptions::<Impl, IMPL_OFFSET>,
            CreateFileScreenCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateImpl: Sized + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn CopyTemplate();
    fn CommitAndUpdateDerived();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplateVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileScreenTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileScreenTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTemplate<Impl: IFsrmFileScreenTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmFileScreenTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            BlockedFileGroups::<Impl, IMPL_OFFSET>,
            SetBlockedFileGroups::<Impl, IMPL_OFFSET>,
            FileScreenFlags::<Impl, IMPL_OFFSET>,
            SetFileScreenFlags::<Impl, IMPL_OFFSET>,
            CreateAction::<Impl, IMPL_OFFSET>,
            EnumActions::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateImportedImpl: Sized + IFsrmFileScreenTemplateImpl + IFsrmFileScreenBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateImportedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplateImportedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplateImportedVtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmFileScreenTemplateImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmFileScreenTemplateImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            BlockedFileGroups::<Impl, IMPL_OFFSET>,
            SetBlockedFileGroups::<Impl, IMPL_OFFSET>,
            FileScreenFlags::<Impl, IMPL_OFFSET>,
            SetFileScreenFlags::<Impl, IMPL_OFFSET>,
            CreateAction::<Impl, IMPL_OFFSET>,
            EnumActions::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
            OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateManagerImpl: Sized + IDispatchImpl {
    fn CreateTemplate();
    fn GetTemplate();
    fn EnumTemplates();
    fn ExportTemplates();
    fn ImportTemplates();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplateManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplateManagerVtbl {
        unsafe extern "system" fn CreateTemplate<Impl: IFsrmFileScreenTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTemplate<Impl: IFsrmFileScreenTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTemplates<Impl: IFsrmFileScreenTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportTemplates<Impl: IFsrmFileScreenTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportTemplates<Impl: IFsrmFileScreenTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateTemplate::<Impl, IMPL_OFFSET>, GetTemplate::<Impl, IMPL_OFFSET>, EnumTemplates::<Impl, IMPL_OFFSET>, ExportTemplates::<Impl, IMPL_OFFSET>, ImportTemplates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmMutableCollectionImpl: Sized + IFsrmCollectionImpl + IDispatchImpl {
    fn Add();
    fn Remove();
    fn RemoveById();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmMutableCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmMutableCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmMutableCollectionVtbl {
        unsafe extern "system" fn Add<Impl: IFsrmMutableCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IFsrmMutableCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveById<Impl: IFsrmMutableCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IFsrmMutableCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            WaitForCompletion::<Impl, IMPL_OFFSET>,
            GetById::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            RemoveById::<Impl, IMPL_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmMutableCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmObjectImpl: Sized + IDispatchImpl {
    fn Id();
    fn Description();
    fn SetDescription();
    fn Delete();
    fn Commit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmObjectVtbl {
        unsafe extern "system" fn Id<Impl: IFsrmObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IFsrmObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IFsrmObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IFsrmObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, Description::<Impl, IMPL_OFFSET>, SetDescription::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPathMapperImpl: Sized + IDispatchImpl {
    fn GetSharePathsForLocalPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPathMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPathMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPathMapperVtbl {
        unsafe extern "system" fn GetSharePathsForLocalPath<Impl: IFsrmPathMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetSharePathsForLocalPath::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPathMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleConnectorImpl: Sized + IDispatchImpl {
    fn ModuleImplementation();
    fn ModuleName();
    fn HostingUserAccount();
    fn HostingProcessPid();
    fn Bind();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleConnectorVtbl {
        unsafe extern "system" fn ModuleImplementation<Impl: IFsrmPipelineModuleConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModuleName<Impl: IFsrmPipelineModuleConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HostingUserAccount<Impl: IFsrmPipelineModuleConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HostingProcessPid<Impl: IFsrmPipelineModuleConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bind<Impl: IFsrmPipelineModuleConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleimplementation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ModuleImplementation::<Impl, IMPL_OFFSET>, ModuleName::<Impl, IMPL_OFFSET>, HostingUserAccount::<Impl, IMPL_OFFSET>, HostingProcessPid::<Impl, IMPL_OFFSET>, Bind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleDefinitionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn ModuleClsid();
    fn SetModuleClsid();
    fn Name();
    fn SetName();
    fn Company();
    fn SetCompany();
    fn Version();
    fn SetVersion();
    fn ModuleType();
    fn Enabled();
    fn SetEnabled();
    fn NeedsFileContent();
    fn SetNeedsFileContent();
    fn Account();
    fn SetAccount();
    fn SupportedExtensions();
    fn SetSupportedExtensions();
    fn Parameters();
    fn SetParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleDefinitionVtbl {
        unsafe extern "system" fn ModuleClsid<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModuleClsid<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Company<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompany<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModuleType<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NeedsFileContent<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNeedsFileContent<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Account<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccount<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedExtensions<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSupportedExtensions<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmPipelineModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            ModuleClsid::<Impl, IMPL_OFFSET>,
            SetModuleClsid::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Company::<Impl, IMPL_OFFSET>,
            SetCompany::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            ModuleType::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            NeedsFileContent::<Impl, IMPL_OFFSET>,
            SetNeedsFileContent::<Impl, IMPL_OFFSET>,
            Account::<Impl, IMPL_OFFSET>,
            SetAccount::<Impl, IMPL_OFFSET>,
            SupportedExtensions::<Impl, IMPL_OFFSET>,
            SetSupportedExtensions::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleImplementationImpl: Sized + IDispatchImpl {
    fn OnLoad();
    fn OnUnload();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleImplementationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleImplementationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleImplementationVtbl {
        unsafe extern "system" fn OnLoad<Impl: IFsrmPipelineModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUnload<Impl: IFsrmPipelineModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnLoad::<Impl, IMPL_OFFSET>, OnUnload::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleImplementation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
    fn Sources();
    fn PropertyFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IFsrmPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Sources<Impl: IFsrmPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyFlags<Impl: IFsrmPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, Sources::<Impl, IMPL_OFFSET>, PropertyFlags::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBagImpl: Sized + IDispatchImpl {
    fn Name();
    fn RelativePath();
    fn VolumeName();
    fn RelativeNamespaceRoot();
    fn VolumeIndex();
    fn FileId();
    fn ParentDirectoryId();
    fn Size();
    fn SizeAllocated();
    fn CreationTime();
    fn LastAccessTime();
    fn LastModificationTime();
    fn Attributes();
    fn OwnerSid();
    fn FilePropertyNames();
    fn Messages();
    fn PropertyBagFlags();
    fn GetFileProperty();
    fn SetFileProperty();
    fn AddMessage();
    fn GetFileStreamInterface();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBagVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyBagImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyBagVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelativePath<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeName<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeIndex<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileId<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentDirectoryId<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Size<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SizeAllocated<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreationTime<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastAccessTime<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastModificationTime<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attributes<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OwnerSid<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FilePropertyNames<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Messages<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyBagFlags<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileProperty<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileProperty<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMessage<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileStreamInterface<Impl: IFsrmPropertyBagImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Name::<Impl, IMPL_OFFSET>,
            RelativePath::<Impl, IMPL_OFFSET>,
            VolumeName::<Impl, IMPL_OFFSET>,
            RelativeNamespaceRoot::<Impl, IMPL_OFFSET>,
            VolumeIndex::<Impl, IMPL_OFFSET>,
            FileId::<Impl, IMPL_OFFSET>,
            ParentDirectoryId::<Impl, IMPL_OFFSET>,
            Size::<Impl, IMPL_OFFSET>,
            SizeAllocated::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            LastAccessTime::<Impl, IMPL_OFFSET>,
            LastModificationTime::<Impl, IMPL_OFFSET>,
            Attributes::<Impl, IMPL_OFFSET>,
            OwnerSid::<Impl, IMPL_OFFSET>,
            FilePropertyNames::<Impl, IMPL_OFFSET>,
            Messages::<Impl, IMPL_OFFSET>,
            PropertyBagFlags::<Impl, IMPL_OFFSET>,
            GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty::<Impl, IMPL_OFFSET>,
            AddMessage::<Impl, IMPL_OFFSET>,
            GetFileStreamInterface::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBag2Impl: Sized + IFsrmPropertyBagImpl + IDispatchImpl {
    fn GetFieldValue();
    fn GetUntrustedInFileProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBag2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyBag2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyBag2Vtbl {
        unsafe extern "system" fn GetFieldValue<Impl: IFsrmPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Impl: IFsrmPropertyBag2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Name::<Impl, IMPL_OFFSET>,
            RelativePath::<Impl, IMPL_OFFSET>,
            VolumeName::<Impl, IMPL_OFFSET>,
            RelativeNamespaceRoot::<Impl, IMPL_OFFSET>,
            VolumeIndex::<Impl, IMPL_OFFSET>,
            FileId::<Impl, IMPL_OFFSET>,
            ParentDirectoryId::<Impl, IMPL_OFFSET>,
            Size::<Impl, IMPL_OFFSET>,
            SizeAllocated::<Impl, IMPL_OFFSET>,
            CreationTime::<Impl, IMPL_OFFSET>,
            LastAccessTime::<Impl, IMPL_OFFSET>,
            LastModificationTime::<Impl, IMPL_OFFSET>,
            Attributes::<Impl, IMPL_OFFSET>,
            OwnerSid::<Impl, IMPL_OFFSET>,
            FilePropertyNames::<Impl, IMPL_OFFSET>,
            Messages::<Impl, IMPL_OFFSET>,
            PropertyBagFlags::<Impl, IMPL_OFFSET>,
            GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty::<Impl, IMPL_OFFSET>,
            AddMessage::<Impl, IMPL_OFFSET>,
            GetFileStreamInterface::<Impl, IMPL_OFFSET>,
            GetFieldValue::<Impl, IMPL_OFFSET>,
            GetUntrustedInFileProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyConditionImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn Value();
    fn SetValue();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyConditionVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinitionImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn PossibleValues();
    fn SetPossibleValues();
    fn ValueDescriptions();
    fn SetValueDescriptions();
    fn Parameters();
    fn SetParameters();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinitionVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PossibleValues<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPossibleValues<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValueDescriptions<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueDescriptions<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmPropertyDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            PossibleValues::<Impl, IMPL_OFFSET>,
            SetPossibleValues::<Impl, IMPL_OFFSET>,
            ValueDescriptions::<Impl, IMPL_OFFSET>,
            SetValueDescriptions::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinition2Impl: Sized + IFsrmPropertyDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn PropertyDefinitionFlags();
    fn DisplayName();
    fn SetDisplayName();
    fn AppliesTo();
    fn ValueDefinitions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinition2Vtbl {
        unsafe extern "system" fn PropertyDefinitionFlags<Impl: IFsrmPropertyDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IFsrmPropertyDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IFsrmPropertyDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AppliesTo<Impl: IFsrmPropertyDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValueDefinitions<Impl: IFsrmPropertyDefinition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            SetType::<Impl, IMPL_OFFSET>,
            PossibleValues::<Impl, IMPL_OFFSET>,
            SetPossibleValues::<Impl, IMPL_OFFSET>,
            ValueDescriptions::<Impl, IMPL_OFFSET>,
            SetValueDescriptions::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            PropertyDefinitionFlags::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            AppliesTo::<Impl, IMPL_OFFSET>,
            ValueDefinitions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinitionValueImpl: Sized + IDispatchImpl {
    fn Name();
    fn DisplayName();
    fn Description();
    fn UniqueID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinitionValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinitionValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinitionValueVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyDefinitionValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IFsrmPropertyDefinitionValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IFsrmPropertyDefinitionValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UniqueID<Impl: IFsrmPropertyDefinitionValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniqueid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, DisplayName::<Impl, IMPL_OFFSET>, Description::<Impl, IMPL_OFFSET>, UniqueID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaImpl: Sized + IFsrmQuotaObjectImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn QuotaUsed();
    fn QuotaPeakUsage();
    fn QuotaPeakUsageTime();
    fn ResetPeakUsage();
    fn RefreshUsageProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaVtbl {
        unsafe extern "system" fn QuotaUsed<Impl: IFsrmQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuotaPeakUsage<Impl: IFsrmQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuotaPeakUsageTime<Impl: IFsrmQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetPeakUsage<Impl: IFsrmQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshUsageProperties<Impl: IFsrmQuotaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            UserSid::<Impl, IMPL_OFFSET>,
            UserAccount::<Impl, IMPL_OFFSET>,
            SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            ApplyTemplate::<Impl, IMPL_OFFSET>,
            QuotaUsed::<Impl, IMPL_OFFSET>,
            QuotaPeakUsage::<Impl, IMPL_OFFSET>,
            QuotaPeakUsageTime::<Impl, IMPL_OFFSET>,
            ResetPeakUsage::<Impl, IMPL_OFFSET>,
            RefreshUsageProperties::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuota as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaBaseImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn QuotaLimit();
    fn SetQuotaLimit();
    fn QuotaFlags();
    fn SetQuotaFlags();
    fn Thresholds();
    fn AddThreshold();
    fn DeleteThreshold();
    fn ModifyThreshold();
    fn CreateThresholdAction();
    fn EnumThresholdActions();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaBaseVtbl {
        unsafe extern "system" fn QuotaLimit<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaLimit<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QuotaFlags<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaFlags<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Thresholds<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddThreshold<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteThreshold<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyThreshold<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateThresholdAction<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumThresholdActions<Impl: IFsrmQuotaBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManagerImpl: Sized + IDispatchImpl {
    fn ActionVariables();
    fn ActionVariableDescriptions();
    fn CreateQuota();
    fn CreateAutoApplyQuota();
    fn GetQuota();
    fn GetAutoApplyQuota();
    fn GetRestrictiveQuota();
    fn EnumQuotas();
    fn EnumAutoApplyQuotas();
    fn EnumEffectiveQuotas();
    fn Scan();
    fn CreateQuotaCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaManagerVtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuota<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAutoApplyQuota<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuota<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoApplyQuota<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictiveQuota<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumQuotas<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAutoApplyQuotas<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumEffectiveQuotas<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Scan<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateQuotaCollection<Impl: IFsrmQuotaManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            CreateQuota::<Impl, IMPL_OFFSET>,
            CreateAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetQuota::<Impl, IMPL_OFFSET>,
            GetAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetRestrictiveQuota::<Impl, IMPL_OFFSET>,
            EnumQuotas::<Impl, IMPL_OFFSET>,
            EnumAutoApplyQuotas::<Impl, IMPL_OFFSET>,
            EnumEffectiveQuotas::<Impl, IMPL_OFFSET>,
            Scan::<Impl, IMPL_OFFSET>,
            CreateQuotaCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManagerExImpl: Sized + IFsrmQuotaManagerImpl + IDispatchImpl {
    fn IsAffectedByQuota();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManagerExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaManagerExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaManagerExVtbl {
        unsafe extern "system" fn IsAffectedByQuota<Impl: IFsrmQuotaManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows::core::HRESULT {
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
            ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            CreateQuota::<Impl, IMPL_OFFSET>,
            CreateAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetQuota::<Impl, IMPL_OFFSET>,
            GetAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetRestrictiveQuota::<Impl, IMPL_OFFSET>,
            EnumQuotas::<Impl, IMPL_OFFSET>,
            EnumAutoApplyQuotas::<Impl, IMPL_OFFSET>,
            EnumEffectiveQuotas::<Impl, IMPL_OFFSET>,
            Scan::<Impl, IMPL_OFFSET>,
            CreateQuotaCollection::<Impl, IMPL_OFFSET>,
            IsAffectedByQuota::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaObjectImpl: Sized + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Path();
    fn UserSid();
    fn UserAccount();
    fn SourceTemplateName();
    fn MatchesSourceTemplate();
    fn ApplyTemplate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaObjectVtbl {
        unsafe extern "system" fn Path<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserSid<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserAccount<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceTemplateName<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MatchesSourceTemplate<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyTemplate<Impl: IFsrmQuotaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            UserSid::<Impl, IMPL_OFFSET>,
            UserAccount::<Impl, IMPL_OFFSET>,
            SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            ApplyTemplate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateImpl: Sized + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn CopyTemplate();
    fn CommitAndUpdateDerived();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplateVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmQuotaTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmQuotaTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTemplate<Impl: IFsrmQuotaTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmQuotaTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateImportedImpl: Sized + IFsrmQuotaTemplateImpl + IFsrmQuotaBaseImpl + IFsrmObjectImpl + IDispatchImpl {
    fn OverwriteOnCommit();
    fn SetOverwriteOnCommit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateImportedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplateImportedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplateImportedVtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmQuotaTemplateImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmQuotaTemplateImportedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
            OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateManagerImpl: Sized + IDispatchImpl {
    fn CreateTemplate();
    fn GetTemplate();
    fn EnumTemplates();
    fn ExportTemplates();
    fn ImportTemplates();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplateManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplateManagerVtbl {
        unsafe extern "system" fn CreateTemplate<Impl: IFsrmQuotaTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTemplate<Impl: IFsrmQuotaTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTemplates<Impl: IFsrmQuotaTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportTemplates<Impl: IFsrmQuotaTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportTemplates<Impl: IFsrmQuotaTemplateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateTemplate::<Impl, IMPL_OFFSET>, GetTemplate::<Impl, IMPL_OFFSET>, EnumTemplates::<Impl, IMPL_OFFSET>, ExportTemplates::<Impl, IMPL_OFFSET>, ImportTemplates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportImpl: Sized + IDispatchImpl {
    fn Type();
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn LastGeneratedFileNamePrefix();
    fn GetFilter();
    fn SetFilter();
    fn Delete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportVtbl {
        unsafe extern "system" fn Type<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastGeneratedFileNamePrefix<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFilter<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFilter<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Type::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            LastGeneratedFileNamePrefix::<Impl, IMPL_OFFSET>,
            GetFilter::<Impl, IMPL_OFFSET>,
            SetFilter::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportJobImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Task();
    fn SetTask();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn Formats();
    fn SetFormats();
    fn MailTo();
    fn SetMailTo();
    fn RunningStatus();
    fn LastRun();
    fn LastError();
    fn LastGeneratedInDirectory();
    fn EnumReports();
    fn CreateReport();
    fn Run();
    fn WaitForCompletion();
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportJobImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportJobVtbl {
        unsafe extern "system" fn Task<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTask<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Formats<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormats<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunningStatus<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastRun<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastError<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastGeneratedInDirectory<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumReports<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateReport<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Run<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmReportJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Task::<Impl, IMPL_OFFSET>,
            SetTask::<Impl, IMPL_OFFSET>,
            NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            Formats::<Impl, IMPL_OFFSET>,
            SetFormats::<Impl, IMPL_OFFSET>,
            MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo::<Impl, IMPL_OFFSET>,
            RunningStatus::<Impl, IMPL_OFFSET>,
            LastRun::<Impl, IMPL_OFFSET>,
            LastError::<Impl, IMPL_OFFSET>,
            LastGeneratedInDirectory::<Impl, IMPL_OFFSET>,
            EnumReports::<Impl, IMPL_OFFSET>,
            CreateReport::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            WaitForCompletion::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportManagerImpl: Sized + IDispatchImpl {
    fn EnumReportJobs();
    fn CreateReportJob();
    fn GetReportJob();
    fn GetOutputDirectory();
    fn SetOutputDirectory();
    fn IsFilterValidForReportType();
    fn GetDefaultFilter();
    fn SetDefaultFilter();
    fn GetReportSizeLimit();
    fn SetReportSizeLimit();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportManagerVtbl {
        unsafe extern "system" fn EnumReportJobs<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateReportJob<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReportJob<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputDirectory<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputDirectory<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsFilterValidForReportType<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultFilter<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultFilter<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReportSizeLimit<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReportSizeLimit<Impl: IFsrmReportManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
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
            EnumReportJobs::<Impl, IMPL_OFFSET>,
            CreateReportJob::<Impl, IMPL_OFFSET>,
            GetReportJob::<Impl, IMPL_OFFSET>,
            GetOutputDirectory::<Impl, IMPL_OFFSET>,
            SetOutputDirectory::<Impl, IMPL_OFFSET>,
            IsFilterValidForReportType::<Impl, IMPL_OFFSET>,
            GetDefaultFilter::<Impl, IMPL_OFFSET>,
            SetDefaultFilter::<Impl, IMPL_OFFSET>,
            GetReportSizeLimit::<Impl, IMPL_OFFSET>,
            SetReportSizeLimit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportSchedulerImpl: Sized + IDispatchImpl {
    fn VerifyNamespaces();
    fn CreateScheduleTask();
    fn ModifyScheduleTask();
    fn DeleteScheduleTask();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportSchedulerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportSchedulerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportSchedulerVtbl {
        unsafe extern "system" fn VerifyNamespaces<Impl: IFsrmReportSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateScheduleTask<Impl: IFsrmReportSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyScheduleTask<Impl: IFsrmReportSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteScheduleTask<Impl: IFsrmReportSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, VerifyNamespaces::<Impl, IMPL_OFFSET>, CreateScheduleTask::<Impl, IMPL_OFFSET>, ModifyScheduleTask::<Impl, IMPL_OFFSET>, DeleteScheduleTask::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportScheduler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmRuleImpl: Sized + IFsrmObjectImpl + IDispatchImpl {
    fn Name();
    fn SetName();
    fn RuleType();
    fn ModuleDefinitionName();
    fn SetModuleDefinitionName();
    fn NamespaceRoots();
    fn SetNamespaceRoots();
    fn RuleFlags();
    fn SetRuleFlags();
    fn Parameters();
    fn SetParameters();
    fn LastModified();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmRuleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmRuleVtbl {
        unsafe extern "system" fn Name<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RuleType<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModuleDefinitionName<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModuleDefinitionName<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RuleFlags<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRuleFlags<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastModified<Impl: IFsrmRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            RuleType::<Impl, IMPL_OFFSET>,
            ModuleDefinitionName::<Impl, IMPL_OFFSET>,
            SetModuleDefinitionName::<Impl, IMPL_OFFSET>,
            NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            RuleFlags::<Impl, IMPL_OFFSET>,
            SetRuleFlags::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            LastModified::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmSettingImpl: Sized + IDispatchImpl {
    fn SmtpServer();
    fn SetSmtpServer();
    fn MailFrom();
    fn SetMailFrom();
    fn AdminEmail();
    fn SetAdminEmail();
    fn DisableCommandLine();
    fn SetDisableCommandLine();
    fn EnableScreeningAudit();
    fn SetEnableScreeningAudit();
    fn EmailTest();
    fn SetActionRunLimitInterval();
    fn GetActionRunLimitInterval();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmSettingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmSettingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmSettingVtbl {
        unsafe extern "system" fn SmtpServer<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSmtpServer<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MailFrom<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMailFrom<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdminEmail<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAdminEmail<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableCommandLine<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisableCommandLine<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableScreeningAudit<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EmailTest<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Impl: IFsrmSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT {
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
            SmtpServer::<Impl, IMPL_OFFSET>,
            SetSmtpServer::<Impl, IMPL_OFFSET>,
            MailFrom::<Impl, IMPL_OFFSET>,
            SetMailFrom::<Impl, IMPL_OFFSET>,
            AdminEmail::<Impl, IMPL_OFFSET>,
            SetAdminEmail::<Impl, IMPL_OFFSET>,
            DisableCommandLine::<Impl, IMPL_OFFSET>,
            SetDisableCommandLine::<Impl, IMPL_OFFSET>,
            EnableScreeningAudit::<Impl, IMPL_OFFSET>,
            SetEnableScreeningAudit::<Impl, IMPL_OFFSET>,
            EmailTest::<Impl, IMPL_OFFSET>,
            SetActionRunLimitInterval::<Impl, IMPL_OFFSET>,
            GetActionRunLimitInterval::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleDefinitionImpl: Sized + IFsrmPipelineModuleDefinitionImpl + IFsrmObjectImpl + IDispatchImpl {
    fn Capabilities();
    fn SetCapabilities();
    fn StorageType();
    fn SetStorageType();
    fn UpdatesFileContent();
    fn SetUpdatesFileContent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmStorageModuleDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmStorageModuleDefinitionVtbl {
        unsafe extern "system" fn Capabilities<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCapabilities<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StorageType<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStorageType<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdatesFileContent<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpdatesFileContent<Impl: IFsrmStorageModuleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            Commit::<Impl, IMPL_OFFSET>,
            ModuleClsid::<Impl, IMPL_OFFSET>,
            SetModuleClsid::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SetName::<Impl, IMPL_OFFSET>,
            Company::<Impl, IMPL_OFFSET>,
            SetCompany::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            ModuleType::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            NeedsFileContent::<Impl, IMPL_OFFSET>,
            SetNeedsFileContent::<Impl, IMPL_OFFSET>,
            Account::<Impl, IMPL_OFFSET>,
            SetAccount::<Impl, IMPL_OFFSET>,
            SupportedExtensions::<Impl, IMPL_OFFSET>,
            SetSupportedExtensions::<Impl, IMPL_OFFSET>,
            Parameters::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            Capabilities::<Impl, IMPL_OFFSET>,
            SetCapabilities::<Impl, IMPL_OFFSET>,
            StorageType::<Impl, IMPL_OFFSET>,
            SetStorageType::<Impl, IMPL_OFFSET>,
            UpdatesFileContent::<Impl, IMPL_OFFSET>,
            SetUpdatesFileContent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleImplementationImpl: Sized + IFsrmPipelineModuleImplementationImpl + IDispatchImpl {
    fn UseDefinitions();
    fn LoadProperties();
    fn SaveProperties();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleImplementationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmStorageModuleImplementationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmStorageModuleImplementationVtbl {
        unsafe extern "system" fn UseDefinitions<Impl: IFsrmStorageModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadProperties<Impl: IFsrmStorageModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveProperties<Impl: IFsrmStorageModuleImplementationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, OnLoad::<Impl, IMPL_OFFSET>, OnUnload::<Impl, IMPL_OFFSET>, UseDefinitions::<Impl, IMPL_OFFSET>, LoadProperties::<Impl, IMPL_OFFSET>, SaveProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleImplementation as ::windows::core::Interface>::IID
    }
}
