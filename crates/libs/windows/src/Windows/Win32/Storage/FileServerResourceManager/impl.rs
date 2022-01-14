#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DIFsrmClassificationEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DIFsrmClassificationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DIFsrmClassificationEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DIFsrmClassificationEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DIFsrmClassificationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Show(&mut self, parentwnd: usize, accesspath: &super::super::Foundation::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &super::super::Foundation::BSTR, windowmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAccessDeniedRemediationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmAccessDeniedRemediationClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmAccessDeniedRemediationClient_Vtbl {
        unsafe extern "system" fn Show<Impl: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(::core::mem::transmute_copy(&parentwnd), ::core::mem::transmute_copy(&accesspath), ::core::mem::transmute_copy(&errortype), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&windowtitle), ::core::mem::transmute_copy(&windowmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Show: Show::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAction_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ActionType(&mut self) -> ::windows::core::Result<FsrmActionType>;
    fn RunLimitInterval(&mut self) -> ::windows::core::Result<i32>;
    fn SetRunLimitInterval(&mut self, minutes: i32) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmAction_Vtbl {
        unsafe extern "system" fn Id<Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionType<Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionType() {
                ::core::result::Result::Ok(ok__) => {
                    *actiontype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunLimitInterval<Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunLimitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRunLimitInterval<Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunLimitInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            ActionType: ActionType::<Impl, IMPL_OFFSET>,
            RunLimitInterval: RunLimitInterval::<Impl, IMPL_OFFSET>,
            SetRunLimitInterval: SetRunLimitInterval::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionCommand_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmAction_Impl {
    fn ExecutablePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetExecutablePath(&mut self, executablepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Arguments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetArguments(&mut self, arguments: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Account(&mut self) -> ::windows::core::Result<FsrmAccountType>;
    fn SetAccount(&mut self, account: FsrmAccountType) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetWorkingDirectory(&mut self, workingdirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MonitorCommand(&mut self) -> ::windows::core::Result<i16>;
    fn SetMonitorCommand(&mut self, monitorcommand: i16) -> ::windows::core::Result<()>;
    fn KillTimeOut(&mut self) -> ::windows::core::Result<i32>;
    fn SetKillTimeOut(&mut self, minutes: i32) -> ::windows::core::Result<()>;
    fn LogResult(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogResult(&mut self, logresults: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionCommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionCommand_Vtbl {
        unsafe extern "system" fn ExecutablePath<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutablePath() {
                ::core::result::Result::Ok(ok__) => {
                    *executablepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExecutablePath<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExecutablePath(::core::mem::transmute_copy(&executablepath)).into()
        }
        unsafe extern "system" fn Arguments<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    *arguments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArguments<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArguments(::core::mem::transmute_copy(&arguments)).into()
        }
        unsafe extern "system" fn Account<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Account() {
                ::core::result::Result::Ok(ok__) => {
                    *account = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccount<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccount(::core::mem::transmute_copy(&account)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *workingdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&workingdirectory)).into()
        }
        unsafe extern "system" fn MonitorCommand<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitorCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *monitorcommand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorCommand<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonitorCommand(::core::mem::transmute_copy(&monitorcommand)).into()
        }
        unsafe extern "system" fn KillTimeOut<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KillTimeOut() {
                ::core::result::Result::Ok(ok__) => {
                    *minutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKillTimeOut<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKillTimeOut(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LogResult<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogResult() {
                ::core::result::Result::Ok(ok__) => {
                    *logresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogResult<Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogResult(::core::mem::transmute_copy(&logresults)).into()
        }
        Self {
            base: IFsrmAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExecutablePath: ExecutablePath::<Impl, IMPL_OFFSET>,
            SetExecutablePath: SetExecutablePath::<Impl, IMPL_OFFSET>,
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            SetArguments: SetArguments::<Impl, IMPL_OFFSET>,
            Account: Account::<Impl, IMPL_OFFSET>,
            SetAccount: SetAccount::<Impl, IMPL_OFFSET>,
            WorkingDirectory: WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            MonitorCommand: MonitorCommand::<Impl, IMPL_OFFSET>,
            SetMonitorCommand: SetMonitorCommand::<Impl, IMPL_OFFSET>,
            KillTimeOut: KillTimeOut::<Impl, IMPL_OFFSET>,
            SetKillTimeOut: SetKillTimeOut::<Impl, IMPL_OFFSET>,
            LogResult: LogResult::<Impl, IMPL_OFFSET>,
            SetLogResult: SetLogResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmail_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmAction_Impl {
    fn MailFrom(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailFrom(&mut self, mailfrom: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailReplyTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailReplyTo(&mut self, mailreplyto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailTo(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailCc(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailCc(&mut self, mailcc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailBcc(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailBcc(&mut self, mailbcc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailSubject(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailSubject(&mut self, mailsubject: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MessageText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMessageText(&mut self, messagetext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmail_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEmail_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEmail_Vtbl {
        unsafe extern "system" fn MailFrom<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *mailfrom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailFrom<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailFrom(::core::mem::transmute_copy(&mailfrom)).into()
        }
        unsafe extern "system" fn MailReplyTo<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailReplyTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailreplyto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailReplyTo<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailReplyTo(::core::mem::transmute_copy(&mailreplyto)).into()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailTo(::core::mem::transmute_copy(&mailto)).into()
        }
        unsafe extern "system" fn MailCc<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailCc() {
                ::core::result::Result::Ok(ok__) => {
                    *mailcc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailCc<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailCc(::core::mem::transmute_copy(&mailcc)).into()
        }
        unsafe extern "system" fn MailBcc<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailBcc() {
                ::core::result::Result::Ok(ok__) => {
                    *mailbcc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailBcc<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailBcc(::core::mem::transmute_copy(&mailbcc)).into()
        }
        unsafe extern "system" fn MailSubject<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailSubject() {
                ::core::result::Result::Ok(ok__) => {
                    *mailsubject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailSubject<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailSubject(::core::mem::transmute_copy(&mailsubject)).into()
        }
        unsafe extern "system" fn MessageText<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageText() {
                ::core::result::Result::Ok(ok__) => {
                    *messagetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageText<Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageText(::core::mem::transmute_copy(&messagetext)).into()
        }
        Self {
            base: IFsrmAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MailFrom: MailFrom::<Impl, IMPL_OFFSET>,
            SetMailFrom: SetMailFrom::<Impl, IMPL_OFFSET>,
            MailReplyTo: MailReplyTo::<Impl, IMPL_OFFSET>,
            SetMailReplyTo: SetMailReplyTo::<Impl, IMPL_OFFSET>,
            MailTo: MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo: SetMailTo::<Impl, IMPL_OFFSET>,
            MailCc: MailCc::<Impl, IMPL_OFFSET>,
            SetMailCc: SetMailCc::<Impl, IMPL_OFFSET>,
            MailBcc: MailBcc::<Impl, IMPL_OFFSET>,
            SetMailBcc: SetMailBcc::<Impl, IMPL_OFFSET>,
            MailSubject: MailSubject::<Impl, IMPL_OFFSET>,
            SetMailSubject: SetMailSubject::<Impl, IMPL_OFFSET>,
            MessageText: MessageText::<Impl, IMPL_OFFSET>,
            SetMessageText: SetMessageText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmail2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmAction_Impl + IFsrmActionEmail_Impl {
    fn AttachmentFileListSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetAttachmentFileListSize(&mut self, attachmentfilelistsize: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmail2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEmail2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEmail2_Vtbl {
        unsafe extern "system" fn AttachmentFileListSize<Impl: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachmentFileListSize() {
                ::core::result::Result::Ok(ok__) => {
                    *attachmentfilelistsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Impl: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttachmentFileListSize(::core::mem::transmute_copy(&attachmentfilelistsize)).into()
        }
        Self {
            base: IFsrmActionEmail_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AttachmentFileListSize: AttachmentFileListSize::<Impl, IMPL_OFFSET>,
            SetAttachmentFileListSize: SetAttachmentFileListSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEventLog_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmAction_Impl {
    fn EventType(&mut self) -> ::windows::core::Result<FsrmEventType>;
    fn SetEventType(&mut self, eventtype: FsrmEventType) -> ::windows::core::Result<()>;
    fn MessageText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMessageText(&mut self, messagetext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEventLog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionEventLog_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionEventLog_Vtbl {
        unsafe extern "system" fn EventType<Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *eventtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventType<Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventType(::core::mem::transmute_copy(&eventtype)).into()
        }
        unsafe extern "system" fn MessageText<Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageText() {
                ::core::result::Result::Ok(ok__) => {
                    *messagetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageText<Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageText(::core::mem::transmute_copy(&messagetext)).into()
        }
        Self {
            base: IFsrmAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            SetEventType: SetEventType::<Impl, IMPL_OFFSET>,
            MessageText: MessageText::<Impl, IMPL_OFFSET>,
            SetMessageText: SetMessageText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionReport_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmAction_Impl {
    fn ReportTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetReportTypes(&mut self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailTo(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmActionReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmActionReport_Vtbl {
        unsafe extern "system" fn ReportTypes<Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *reporttypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportTypes<Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportTypes(::core::mem::transmute_copy(&reporttypes)).into()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailTo(::core::mem::transmute_copy(&mailto)).into()
        }
        Self {
            base: IFsrmAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReportTypes: ReportTypes::<Impl, IMPL_OFFSET>,
            SetReportTypes: SetReportTypes::<Impl, IMPL_OFFSET>,
            MailTo: MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo: SetMailTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAutoApplyQuota_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmQuotaBase_Impl + IFsrmQuotaObject_Impl {
    fn ExcludeFolders(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetExcludeFolders(&mut self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&mut self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAutoApplyQuota_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmAutoApplyQuota_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmAutoApplyQuota_Vtbl {
        unsafe extern "system" fn ExcludeFolders<Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcludeFolders() {
                ::core::result::Result::Ok(ok__) => {
                    *folders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeFolders<Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExcludeFolders(::core::mem::transmute_copy(&folders)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *derivedobjectsresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmQuotaObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExcludeFolders: ExcludeFolders::<Impl, IMPL_OFFSET>,
            SetExcludeFolders: SetExcludeFolders::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAutoApplyQuota as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ClassificationReportFormats(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetClassificationReportFormats(&mut self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Logging(&mut self) -> ::windows::core::Result<i32>;
    fn SetLogging(&mut self, logging: i32) -> ::windows::core::Result<()>;
    fn ClassificationReportMailTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClassificationReportMailTo(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClassificationReportEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetClassificationReportEnabled(&mut self, reportenabled: i16) -> ::windows::core::Result<()>;
    fn ClassificationLastReportPathWithoutExtension(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClassificationLastError(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ClassificationRunningStatus(&mut self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn EnumPropertyDefinitions(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreatePropertyDefinition(&mut self) -> ::windows::core::Result<IFsrmPropertyDefinition>;
    fn GetPropertyDefinition(&mut self, propertyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmPropertyDefinition>;
    fn EnumRules(&mut self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateRule(&mut self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule>;
    fn GetRule(&mut self, rulename: &super::super::Foundation::BSTR, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule>;
    fn EnumModuleDefinitions(&mut self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateModuleDefinition(&mut self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition>;
    fn GetModuleDefinition(&mut self, modulename: &super::super::Foundation::BSTR, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition>;
    fn RunClassification(&mut self, context: FsrmReportGenerationContext, reserved: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WaitForClassificationCompletion(&mut self, waitseconds: i32) -> ::windows::core::Result<i16>;
    fn CancelClassification(&mut self) -> ::windows::core::Result<()>;
    fn EnumFileProperties(&mut self, filepath: &super::super::Foundation::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn GetFileProperty(&mut self, filepath: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty>;
    fn SetFileProperty(&mut self, filepath: &super::super::Foundation::BSTR, propertyname: &super::super::Foundation::BSTR, propertyvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClearFileProperty(&mut self, filepath: &super::super::Foundation::BSTR, property: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationManager_Vtbl {
        unsafe extern "system" fn ClassificationReportFormats<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationReportFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *formats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportFormats<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassificationReportFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn Logging<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logging() {
                ::core::result::Result::Ok(ok__) => {
                    *logging = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogging<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogging(::core::mem::transmute_copy(&logging)).into()
        }
        unsafe extern "system" fn ClassificationReportMailTo<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationReportMailTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassificationReportMailTo(::core::mem::transmute_copy(&mailto)).into()
        }
        unsafe extern "system" fn ClassificationReportEnabled<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationReportEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *reportenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassificationReportEnabled(::core::mem::transmute_copy(&reportenabled)).into()
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationLastReportPathWithoutExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *lastreportpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassificationLastError<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationLastError() {
                ::core::result::Result::Ok(ok__) => {
                    *lasterror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassificationRunningStatus<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassificationRunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *runningstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPropertyDefinitions(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertydefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyDefinition<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    *propertydefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDefinition<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyDefinition(::core::mem::transmute_copy(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertydefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRules<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumRules(::core::mem::transmute_copy(&ruletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *rules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRule<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRule(::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRule<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRule(::core::mem::transmute_copy(&rulename), ::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *rule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumModuleDefinitions<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumModuleDefinitions(::core::mem::transmute_copy(&moduletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *moduledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateModuleDefinition<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateModuleDefinition(::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *moduledefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModuleDefinition<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModuleDefinition(::core::mem::transmute_copy(&modulename), ::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *moduledefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunClassification<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunClassification(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForClassificationCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelClassification<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelClassification().into()
        }
        unsafe extern "system" fn EnumFileProperties<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFileProperties(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *fileproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileProperty<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileProperty(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileProperty<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileProperty(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&propertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn ClearFileProperty<Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearFileProperty(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&property)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClassificationReportFormats: ClassificationReportFormats::<Impl, IMPL_OFFSET>,
            SetClassificationReportFormats: SetClassificationReportFormats::<Impl, IMPL_OFFSET>,
            Logging: Logging::<Impl, IMPL_OFFSET>,
            SetLogging: SetLogging::<Impl, IMPL_OFFSET>,
            ClassificationReportMailTo: ClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            SetClassificationReportMailTo: SetClassificationReportMailTo::<Impl, IMPL_OFFSET>,
            ClassificationReportEnabled: ClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            SetClassificationReportEnabled: SetClassificationReportEnabled::<Impl, IMPL_OFFSET>,
            ClassificationLastReportPathWithoutExtension: ClassificationLastReportPathWithoutExtension::<Impl, IMPL_OFFSET>,
            ClassificationLastError: ClassificationLastError::<Impl, IMPL_OFFSET>,
            ClassificationRunningStatus: ClassificationRunningStatus::<Impl, IMPL_OFFSET>,
            EnumPropertyDefinitions: EnumPropertyDefinitions::<Impl, IMPL_OFFSET>,
            CreatePropertyDefinition: CreatePropertyDefinition::<Impl, IMPL_OFFSET>,
            GetPropertyDefinition: GetPropertyDefinition::<Impl, IMPL_OFFSET>,
            EnumRules: EnumRules::<Impl, IMPL_OFFSET>,
            CreateRule: CreateRule::<Impl, IMPL_OFFSET>,
            GetRule: GetRule::<Impl, IMPL_OFFSET>,
            EnumModuleDefinitions: EnumModuleDefinitions::<Impl, IMPL_OFFSET>,
            CreateModuleDefinition: CreateModuleDefinition::<Impl, IMPL_OFFSET>,
            GetModuleDefinition: GetModuleDefinition::<Impl, IMPL_OFFSET>,
            RunClassification: RunClassification::<Impl, IMPL_OFFSET>,
            WaitForClassificationCompletion: WaitForClassificationCompletion::<Impl, IMPL_OFFSET>,
            CancelClassification: CancelClassification::<Impl, IMPL_OFFSET>,
            EnumFileProperties: EnumFileProperties::<Impl, IMPL_OFFSET>,
            GetFileProperty: GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty: SetFileProperty::<Impl, IMPL_OFFSET>,
            ClearFileProperty: ClearFileProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManager2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmClassificationManager_Impl {
    fn ClassifyFiles(&mut self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationManager2_Vtbl {
        unsafe extern "system" fn ClassifyFiles<Impl: IFsrmClassificationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClassifyFiles(::core::mem::transmute_copy(&filepaths), ::core::mem::transmute_copy(&propertynames), ::core::mem::transmute_copy(&propertyvalues), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base: IFsrmClassificationManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ClassifyFiles: ClassifyFiles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationRule_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmRule_Impl {
    fn ExecutionOption(&mut self) -> ::windows::core::Result<FsrmExecutionOption>;
    fn SetExecutionOption(&mut self, executionoption: FsrmExecutionOption) -> ::windows::core::Result<()>;
    fn PropertyAffected(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPropertyAffected(&mut self, property: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetValue(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassificationRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassificationRule_Vtbl {
        unsafe extern "system" fn ExecutionOption<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutionOption() {
                ::core::result::Result::Ok(ok__) => {
                    *executionoption = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExecutionOption<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExecutionOption(::core::mem::transmute_copy(&executionoption)).into()
        }
        unsafe extern "system" fn PropertyAffected<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyAffected() {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyAffected<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyAffected(::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn Value<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IFsrmRule_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExecutionOption: ExecutionOption::<Impl, IMPL_OFFSET>,
            SetExecutionOption: SetExecutionOption::<Impl, IMPL_OFFSET>,
            PropertyAffected: PropertyAffected::<Impl, IMPL_OFFSET>,
            SetPropertyAffected: SetPropertyAffected::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmPipelineModuleDefinition_Impl {
    fn PropertiesAffected(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesAffected(&mut self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PropertiesUsed(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesUsed(&mut self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn NeedsExplicitValue(&mut self) -> ::windows::core::Result<i16>;
    fn SetNeedsExplicitValue(&mut self, needsexplicitvalue: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassifierModuleDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassifierModuleDefinition_Vtbl {
        unsafe extern "system" fn PropertiesAffected<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesAffected() {
                ::core::result::Result::Ok(ok__) => {
                    *propertiesaffected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertiesAffected<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertiesAffected(::core::mem::transmute_copy(&propertiesaffected)).into()
        }
        unsafe extern "system" fn PropertiesUsed<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertiesUsed() {
                ::core::result::Result::Ok(ok__) => {
                    *propertiesused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertiesUsed<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertiesUsed(::core::mem::transmute_copy(&propertiesused)).into()
        }
        unsafe extern "system" fn NeedsExplicitValue<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsExplicitValue() {
                ::core::result::Result::Ok(ok__) => {
                    *needsexplicitvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNeedsExplicitValue(::core::mem::transmute_copy(&needsexplicitvalue)).into()
        }
        Self {
            base: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PropertiesAffected: PropertiesAffected::<Impl, IMPL_OFFSET>,
            SetPropertiesAffected: SetPropertiesAffected::<Impl, IMPL_OFFSET>,
            PropertiesUsed: PropertiesUsed::<Impl, IMPL_OFFSET>,
            SetPropertiesUsed: SetPropertiesUsed::<Impl, IMPL_OFFSET>,
            NeedsExplicitValue: NeedsExplicitValue::<Impl, IMPL_OFFSET>,
            SetNeedsExplicitValue: SetNeedsExplicitValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleImplementation_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmPipelineModuleImplementation_Impl {
    fn LastModified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UseRulesAndDefinitions(&mut self, rules: &::core::option::Option<IFsrmCollection>, propertydefinitions: &::core::option::Option<IFsrmCollection>) -> ::windows::core::Result<()>;
    fn OnBeginFile(&mut self, propertybag: &::core::option::Option<IFsrmPropertyBag>, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn DoesPropertyValueApply(&mut self, property: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR, applyvalue: *mut i16, idrule: &::windows::core::GUID, idpropdef: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPropertyValueToApply(&mut self, property: &super::super::Foundation::BSTR, value: *mut super::super::Foundation::BSTR, idrule: &::windows::core::GUID, idpropdef: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnEndFile(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmClassifierModuleImplementation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmClassifierModuleImplementation_Vtbl {
        unsafe extern "system" fn LastModified<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModified() {
                ::core::result::Result::Ok(ok__) => {
                    *lastmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: ::windows::core::RawPtr, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseRulesAndDefinitions(::core::mem::transmute(&rules), ::core::mem::transmute(&propertydefinitions)).into()
        }
        unsafe extern "system" fn OnBeginFile<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBeginFile(::core::mem::transmute(&propertybag), ::core::mem::transmute_copy(&arrayruleids)).into()
        }
        unsafe extern "system" fn DoesPropertyValueApply<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoesPropertyValueApply(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&applyvalue), ::core::mem::transmute_copy(&idrule), ::core::mem::transmute_copy(&idpropdef)).into()
        }
        unsafe extern "system" fn GetPropertyValueToApply<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyValueToApply(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&idrule), ::core::mem::transmute_copy(&idpropdef)).into()
        }
        unsafe extern "system" fn OnEndFile<Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndFile().into()
        }
        Self {
            base: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LastModified: LastModified::<Impl, IMPL_OFFSET>,
            UseRulesAndDefinitions: UseRulesAndDefinitions::<Impl, IMPL_OFFSET>,
            OnBeginFile: OnBeginFile::<Impl, IMPL_OFFSET>,
            DoesPropertyValueApply: DoesPropertyValueApply::<Impl, IMPL_OFFSET>,
            GetPropertyValueToApply: GetPropertyValueToApply::<Impl, IMPL_OFFSET>,
            OnEndFile: OnEndFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleImplementation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<FsrmCollectionState>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&mut self, waitseconds: i32) -> ::windows::core::Result<i16>;
    fn GetById(&mut self, id: &::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *unknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *state = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetById<Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetById(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *entry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            WaitForCompletion: WaitForCompletion::<Impl, IMPL_OFFSET>,
            GetById: GetById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCommittableCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmCollection_Impl + IFsrmMutableCollection_Impl {
    fn Commit(&mut self, options: FsrmCommitOptions) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCommittableCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmCommittableCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmCommittableCollection_Vtbl {
        unsafe extern "system" fn Commit<Impl: IFsrmCommittableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *results = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFsrmMutableCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Commit: Commit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCommittableCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmDerivedObjectsResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DerivedObjects(&mut self) -> ::windows::core::Result<IFsrmCollection>;
    fn Results(&mut self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmDerivedObjectsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmDerivedObjectsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmDerivedObjectsResult_Vtbl {
        unsafe extern "system" fn DerivedObjects<Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedobjects: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DerivedObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *derivedobjects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *results = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DerivedObjects: DerivedObjects::<Impl, IMPL_OFFSET>,
            Results: Results::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmExportImport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExportFileGroups(&mut self, filepath: &super::super::Foundation::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportFileGroups(&mut self, filepath: &super::super::Foundation::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(&mut self, filepath: &super::super::Foundation::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportFileScreenTemplates(&mut self, filepath: &super::super::Foundation::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(&mut self, filepath: &super::super::Foundation::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportQuotaTemplates(&mut self, filepath: &super::super::Foundation::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmExportImport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmExportImport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmExportImport_Vtbl {
        unsafe extern "system" fn ExportFileGroups<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportFileGroups(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute_copy(&remotehost)).into()
        }
        unsafe extern "system" fn ImportFileGroups<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportFileGroups(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute_copy(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    *filegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportFileScreenTemplates(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute_copy(&remotehost)).into()
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportFileScreenTemplates(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute_copy(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    *templates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportQuotaTemplates<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportQuotaTemplates(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute_copy(&remotehost)).into()
        }
        unsafe extern "system" fn ImportQuotaTemplates<Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportQuotaTemplates(::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute_copy(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    *templates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ExportFileGroups: ExportFileGroups::<Impl, IMPL_OFFSET>,
            ImportFileGroups: ImportFileGroups::<Impl, IMPL_OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Impl, IMPL_OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Impl, IMPL_OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Impl, IMPL_OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmExportImport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileCondition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<FsrmFileConditionType>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileCondition_Vtbl {
        unsafe extern "system" fn Type<Impl: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileConditionProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmFileCondition_Impl {
    fn PropertyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPropertyName(&mut self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PropertyId(&mut self) -> ::windows::core::Result<FsrmFileSystemPropertyId>;
    fn SetPropertyId(&mut self, newval: FsrmFileSystemPropertyId) -> ::windows::core::Result<()>;
    fn Operator(&mut self) -> ::windows::core::Result<FsrmPropertyConditionType>;
    fn SetOperator(&mut self, newval: FsrmPropertyConditionType) -> ::windows::core::Result<()>;
    fn ValueType(&mut self) -> ::windows::core::Result<FsrmPropertyValueType>;
    fn SetValueType(&mut self, newval: FsrmPropertyValueType) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&mut self, newval: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileConditionProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileConditionProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileConditionProperty_Vtbl {
        unsafe extern "system" fn PropertyName<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyName<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn PropertyId<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyId<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyId(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Operator<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operator() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperator<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperator(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ValueType<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueType<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueType(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Value<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IFsrmFileCondition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PropertyName: PropertyName::<Impl, IMPL_OFFSET>,
            SetPropertyName: SetPropertyName::<Impl, IMPL_OFFSET>,
            PropertyId: PropertyId::<Impl, IMPL_OFFSET>,
            SetPropertyId: SetPropertyId::<Impl, IMPL_OFFSET>,
            Operator: Operator::<Impl, IMPL_OFFSET>,
            SetOperator: SetOperator::<Impl, IMPL_OFFSET>,
            ValueType: ValueType::<Impl, IMPL_OFFSET>,
            SetValueType: SetValueType::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileConditionProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Members(&mut self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetMembers(&mut self, members: &::core::option::Option<IFsrmMutableCollection>) -> ::windows::core::Result<()>;
    fn NonMembers(&mut self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetNonMembers(&mut self, nonmembers: &::core::option::Option<IFsrmMutableCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Members<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *members = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMembers<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMembers(::core::mem::transmute(&members)).into()
        }
        unsafe extern "system" fn NonMembers<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *nonmembers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonMembers<Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNonMembers(::core::mem::transmute(&nonmembers)).into()
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Members: Members::<Impl, IMPL_OFFSET>,
            SetMembers: SetMembers::<Impl, IMPL_OFFSET>,
            NonMembers: NonMembers::<Impl, IMPL_OFFSET>,
            SetNonMembers: SetNonMembers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupImported_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmFileGroup_Impl {
    fn OverwriteOnCommit(&mut self) -> ::windows::core::Result<i16>;
    fn SetOverwriteOnCommit(&mut self, overwrite: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroupImported_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroupImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *overwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base: IFsrmFileGroup_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateFileGroup(&mut self) -> ::windows::core::Result<IFsrmFileGroup>;
    fn GetFileGroup(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileGroup>;
    fn EnumFileGroups(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportFileGroups(&mut self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImportFileGroups(&mut self, serializedfilegroups: &super::super::Foundation::BSTR, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileGroupManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileGroupManager_Vtbl {
        unsafe extern "system" fn CreateFileGroup<Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *filegroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileGroup<Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileGroup(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *filegroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileGroups<Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFileGroups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *filegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportFileGroups<Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportFileGroups(::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *serializedfilegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileGroups<Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilegroups: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportFileGroups(::core::mem::transmute_copy(&serializedfilegroups), ::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *filegroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateFileGroup: CreateFileGroup::<Impl, IMPL_OFFSET>,
            GetFileGroup: GetFileGroup::<Impl, IMPL_OFFSET>,
            EnumFileGroups: EnumFileGroups::<Impl, IMPL_OFFSET>,
            ExportFileGroups: ExportFileGroups::<Impl, IMPL_OFFSET>,
            ImportFileGroups: ImportFileGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJob_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&mut self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn OperationType(&mut self) -> ::windows::core::Result<FsrmFileManagementType>;
    fn SetOperationType(&mut self, operationtype: FsrmFileManagementType) -> ::windows::core::Result<()>;
    fn ExpirationDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetExpirationDirectory(&mut self, expirationdirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CustomAction(&mut self) -> ::windows::core::Result<IFsrmActionCommand>;
    fn Notifications(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Logging(&mut self) -> ::windows::core::Result<i32>;
    fn SetLogging(&mut self, loggingflags: i32) -> ::windows::core::Result<()>;
    fn ReportEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetReportEnabled(&mut self, reportenabled: i16) -> ::windows::core::Result<()>;
    fn Formats(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&mut self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailTo(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DaysSinceFileCreated(&mut self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileCreated(&mut self, dayssincecreation: i32) -> ::windows::core::Result<()>;
    fn DaysSinceFileLastAccessed(&mut self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileLastAccessed(&mut self, dayssinceaccess: i32) -> ::windows::core::Result<()>;
    fn DaysSinceFileLastModified(&mut self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileLastModified(&mut self, dayssincemodify: i32) -> ::windows::core::Result<()>;
    fn PropertyConditions(&mut self) -> ::windows::core::Result<IFsrmCollection>;
    fn FromDate(&mut self) -> ::windows::core::Result<f64>;
    fn SetFromDate(&mut self, fromdate: f64) -> ::windows::core::Result<()>;
    fn Task(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTask(&mut self, taskname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&mut self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RunningStatus(&mut self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn LastError(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastReportPathWithoutExtension(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastRun(&mut self) -> ::windows::core::Result<f64>;
    fn FileNamePattern(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFileNamePattern(&mut self, filenamepattern: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Run(&mut self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&mut self, waitseconds: i32) -> ::windows::core::Result<i16>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn AddNotification(&mut self, days: i32) -> ::windows::core::Result<()>;
    fn DeleteNotification(&mut self, days: i32) -> ::windows::core::Result<()>;
    fn ModifyNotification(&mut self, days: i32, newdays: i32) -> ::windows::core::Result<()>;
    fn CreateNotificationAction(&mut self, days: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumNotificationActions(&mut self, days: i32) -> ::windows::core::Result<IFsrmCollection>;
    fn CreatePropertyCondition(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmPropertyCondition>;
    fn CreateCustomAction(&mut self) -> ::windows::core::Result<IFsrmActionCommand>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileManagementJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileManagementJob_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceroots = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn Enabled<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn OperationType<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationType() {
                ::core::result::Result::Ok(ok__) => {
                    *operationtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationType<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperationType(::core::mem::transmute_copy(&operationtype)).into()
        }
        unsafe extern "system" fn ExpirationDirectory<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *expirationdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationDirectory<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationDirectory(::core::mem::transmute_copy(&expirationdirectory)).into()
        }
        unsafe extern "system" fn CustomAction<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomAction() {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notifications<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Notifications() {
                ::core::result::Result::Ok(ok__) => {
                    *notifications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logging<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logging() {
                ::core::result::Result::Ok(ok__) => {
                    *loggingflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogging<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogging(::core::mem::transmute_copy(&loggingflags)).into()
        }
        unsafe extern "system" fn ReportEnabled<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *reportenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportEnabled<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportEnabled(::core::mem::transmute_copy(&reportenabled)).into()
        }
        unsafe extern "system" fn Formats<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Formats() {
                ::core::result::Result::Ok(ok__) => {
                    *formats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormats<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailTo(::core::mem::transmute_copy(&mailto)).into()
        }
        unsafe extern "system" fn DaysSinceFileCreated<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaysSinceFileCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *dayssincecreation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileCreated<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysSinceFileCreated(::core::mem::transmute_copy(&dayssincecreation)).into()
        }
        unsafe extern "system" fn DaysSinceFileLastAccessed<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaysSinceFileLastAccessed() {
                ::core::result::Result::Ok(ok__) => {
                    *dayssinceaccess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastAccessed<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysSinceFileLastAccessed(::core::mem::transmute_copy(&dayssinceaccess)).into()
        }
        unsafe extern "system" fn DaysSinceFileLastModified<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaysSinceFileLastModified() {
                ::core::result::Result::Ok(ok__) => {
                    *dayssincemodify = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastModified<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysSinceFileLastModified(::core::mem::transmute_copy(&dayssincemodify)).into()
        }
        unsafe extern "system" fn PropertyConditions<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyconditions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyConditions() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyconditions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDate<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromDate() {
                ::core::result::Result::Ok(ok__) => {
                    *fromdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromDate<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromDate(::core::mem::transmute_copy(&fromdate)).into()
        }
        unsafe extern "system" fn Task<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *taskname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&taskname)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *parameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn RunningStatus<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *runningstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastError<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastError() {
                ::core::result::Result::Ok(ok__) => {
                    *lasterror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReportPathWithoutExtension<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastReportPathWithoutExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRun<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastRun() {
                ::core::result::Result::Ok(ok__) => {
                    *lastrun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileNamePattern<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileNamePattern() {
                ::core::result::Result::Ok(ok__) => {
                    *filenamepattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNamePattern<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileNamePattern(::core::mem::transmute_copy(&filenamepattern)).into()
        }
        unsafe extern "system" fn Run<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn AddNotification<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNotification(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn DeleteNotification<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteNotification(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn ModifyNotification<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyNotification(::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&newdays)).into()
        }
        unsafe extern "system" fn CreateNotificationAction<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNotificationAction(::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNotificationActions<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNotificationActions(::core::mem::transmute_copy(&days)) {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyCondition(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertycondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomAction<Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomAction() {
                ::core::result::Result::Ok(ok__) => {
                    *customaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            NamespaceRoots: NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            OperationType: OperationType::<Impl, IMPL_OFFSET>,
            SetOperationType: SetOperationType::<Impl, IMPL_OFFSET>,
            ExpirationDirectory: ExpirationDirectory::<Impl, IMPL_OFFSET>,
            SetExpirationDirectory: SetExpirationDirectory::<Impl, IMPL_OFFSET>,
            CustomAction: CustomAction::<Impl, IMPL_OFFSET>,
            Notifications: Notifications::<Impl, IMPL_OFFSET>,
            Logging: Logging::<Impl, IMPL_OFFSET>,
            SetLogging: SetLogging::<Impl, IMPL_OFFSET>,
            ReportEnabled: ReportEnabled::<Impl, IMPL_OFFSET>,
            SetReportEnabled: SetReportEnabled::<Impl, IMPL_OFFSET>,
            Formats: Formats::<Impl, IMPL_OFFSET>,
            SetFormats: SetFormats::<Impl, IMPL_OFFSET>,
            MailTo: MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo: SetMailTo::<Impl, IMPL_OFFSET>,
            DaysSinceFileCreated: DaysSinceFileCreated::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileCreated: SetDaysSinceFileCreated::<Impl, IMPL_OFFSET>,
            DaysSinceFileLastAccessed: DaysSinceFileLastAccessed::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileLastAccessed: SetDaysSinceFileLastAccessed::<Impl, IMPL_OFFSET>,
            DaysSinceFileLastModified: DaysSinceFileLastModified::<Impl, IMPL_OFFSET>,
            SetDaysSinceFileLastModified: SetDaysSinceFileLastModified::<Impl, IMPL_OFFSET>,
            PropertyConditions: PropertyConditions::<Impl, IMPL_OFFSET>,
            FromDate: FromDate::<Impl, IMPL_OFFSET>,
            SetFromDate: SetFromDate::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            SetTask: SetTask::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            RunningStatus: RunningStatus::<Impl, IMPL_OFFSET>,
            LastError: LastError::<Impl, IMPL_OFFSET>,
            LastReportPathWithoutExtension: LastReportPathWithoutExtension::<Impl, IMPL_OFFSET>,
            LastRun: LastRun::<Impl, IMPL_OFFSET>,
            FileNamePattern: FileNamePattern::<Impl, IMPL_OFFSET>,
            SetFileNamePattern: SetFileNamePattern::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            WaitForCompletion: WaitForCompletion::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            AddNotification: AddNotification::<Impl, IMPL_OFFSET>,
            DeleteNotification: DeleteNotification::<Impl, IMPL_OFFSET>,
            ModifyNotification: ModifyNotification::<Impl, IMPL_OFFSET>,
            CreateNotificationAction: CreateNotificationAction::<Impl, IMPL_OFFSET>,
            EnumNotificationActions: EnumNotificationActions::<Impl, IMPL_OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Impl, IMPL_OFFSET>,
            CreateCustomAction: CreateCustomAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJobManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn EnumFileManagementJobs(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateFileManagementJob(&mut self) -> ::windows::core::Result<IFsrmFileManagementJob>;
    fn GetFileManagementJob(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileManagementJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJobManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileManagementJobManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileManagementJobManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    *variables = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *descriptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileManagementJobs<Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFileManagementJobs(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *filemanagementjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileManagementJob<Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileManagementJob() {
                ::core::result::Result::Ok(ok__) => {
                    *filemanagementjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileManagementJob<Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileManagementJob(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *filemanagementjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActionVariables: ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            EnumFileManagementJobs: EnumFileManagementJobs::<Impl, IMPL_OFFSET>,
            CreateFileManagementJob: CreateFileManagementJob::<Impl, IMPL_OFFSET>,
            GetFileManagementJob: GetFileManagementJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJobManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreen_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmFileScreenBase_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SourceTemplateName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MatchesSourceTemplate(&mut self) -> ::windows::core::Result<i16>;
    fn UserSid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ApplyTemplate(&mut self, filescreentemplatename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreen_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreen_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreen_Vtbl {
        unsafe extern "system" fn Path<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceTemplateName<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceTemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    *filescreentemplatename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesSourceTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *matches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSid<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSid() {
                ::core::result::Result::Ok(ok__) => {
                    *usersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *useraccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTemplate<Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyTemplate(::core::mem::transmute_copy(&filescreentemplatename)).into()
        }
        Self {
            base: IFsrmFileScreenBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SourceTemplateName: SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            UserSid: UserSid::<Impl, IMPL_OFFSET>,
            UserAccount: UserAccount::<Impl, IMPL_OFFSET>,
            ApplyTemplate: ApplyTemplate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreen as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenBase_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn BlockedFileGroups(&mut self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetBlockedFileGroups(&mut self, blocklist: &::core::option::Option<IFsrmMutableCollection>) -> ::windows::core::Result<()>;
    fn FileScreenFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetFileScreenFlags(&mut self, filescreenflags: i32) -> ::windows::core::Result<()>;
    fn CreateAction(&mut self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumActions(&mut self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenBase_Vtbl {
        unsafe extern "system" fn BlockedFileGroups<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockedFileGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *blocklist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockedFileGroups<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockedFileGroups(::core::mem::transmute(&blocklist)).into()
        }
        unsafe extern "system" fn FileScreenFlags<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileScreenFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *filescreenflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileScreenFlags<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileScreenFlags(::core::mem::transmute_copy(&filescreenflags)).into()
        }
        unsafe extern "system" fn CreateAction<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAction(::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumActions<Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumActions() {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BlockedFileGroups: BlockedFileGroups::<Impl, IMPL_OFFSET>,
            SetBlockedFileGroups: SetBlockedFileGroups::<Impl, IMPL_OFFSET>,
            FileScreenFlags: FileScreenFlags::<Impl, IMPL_OFFSET>,
            SetFileScreenFlags: SetFileScreenFlags::<Impl, IMPL_OFFSET>,
            CreateAction: CreateAction::<Impl, IMPL_OFFSET>,
            EnumActions: EnumActions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenException_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AllowedFileGroups(&mut self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetAllowedFileGroups(&mut self, allowlist: &::core::option::Option<IFsrmMutableCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenException_Vtbl {
        unsafe extern "system" fn Path<Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedFileGroups<Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedFileGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *allowlist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedFileGroups<Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedFileGroups(::core::mem::transmute(&allowlist)).into()
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            AllowedFileGroups: AllowedFileGroups::<Impl, IMPL_OFFSET>,
            SetAllowedFileGroups: SetAllowedFileGroups::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenException as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateFileScreen(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileScreen>;
    fn GetFileScreen(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileScreen>;
    fn EnumFileScreens(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenException(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileScreenException>;
    fn GetFileScreenException(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileScreenException>;
    fn EnumFileScreenExceptions(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenCollection(&mut self) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    *variables = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *descriptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreen<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileScreen(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileScreen<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileScreen(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileScreens<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFileScreens(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreens = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreenException<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileScreenException(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreenexception = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileScreenException<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileScreenException(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreenexception = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileScreenExceptions<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFileScreenExceptions(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreenexceptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreenCollection<Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileScreenCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActionVariables: ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            CreateFileScreen: CreateFileScreen::<Impl, IMPL_OFFSET>,
            GetFileScreen: GetFileScreen::<Impl, IMPL_OFFSET>,
            EnumFileScreens: EnumFileScreens::<Impl, IMPL_OFFSET>,
            CreateFileScreenException: CreateFileScreenException::<Impl, IMPL_OFFSET>,
            GetFileScreenException: GetFileScreenException::<Impl, IMPL_OFFSET>,
            EnumFileScreenExceptions: EnumFileScreenExceptions::<Impl, IMPL_OFFSET>,
            CreateFileScreenCollection: CreateFileScreenCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplate_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmFileScreenBase_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyTemplate(&mut self, filescreentemplatename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&mut self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplate_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn CopyTemplate<Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTemplate(::core::mem::transmute_copy(&filescreentemplatename)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *derivedobjectsresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmFileScreenBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate: CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateImported_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmFileScreenBase_Impl + IFsrmFileScreenTemplate_Impl {
    fn OverwriteOnCommit(&mut self) -> ::windows::core::Result<i16>;
    fn SetOverwriteOnCommit(&mut self, overwrite: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplateImported_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplateImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *overwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base: IFsrmFileScreenTemplate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&mut self) -> ::windows::core::Result<IFsrmFileScreenTemplate>;
    fn GetTemplate(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmFileScreenTemplate>;
    fn EnumTemplates(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&mut self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImportTemplates(&mut self, serializedfilescreentemplates: &super::super::Foundation::BSTR, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmFileScreenTemplateManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmFileScreenTemplateManager_Vtbl {
        unsafe extern "system" fn CreateTemplate<Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *filescreentemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplate<Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplate(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreentemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTemplates<Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTemplates(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreentemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportTemplates<Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportTemplates(::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *serializedfilescreentemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportTemplates<Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportTemplates(::core::mem::transmute_copy(&serializedfilescreentemplates), ::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *filescreentemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateTemplate: CreateTemplate::<Impl, IMPL_OFFSET>,
            GetTemplate: GetTemplate::<Impl, IMPL_OFFSET>,
            EnumTemplates: EnumTemplates::<Impl, IMPL_OFFSET>,
            ExportTemplates: ExportTemplates::<Impl, IMPL_OFFSET>,
            ImportTemplates: ImportTemplates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmMutableCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmCollection_Impl {
    fn Add(&mut self, item: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn RemoveById(&mut self, id: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmMutableCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmMutableCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmMutableCollection_Vtbl {
        unsafe extern "system" fn Add<Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&item)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn RemoveById<Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveById(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Clone<Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveById: RemoveById::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmMutableCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmObject_Vtbl {
        unsafe extern "system" fn Id<Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Commit<Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPathMapper_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetSharePathsForLocalPath(&mut self, localpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPathMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPathMapper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPathMapper_Vtbl {
        unsafe extern "system" fn GetSharePathsForLocalPath<Impl: IFsrmPathMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharePathsForLocalPath(::core::mem::transmute_copy(&localpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *sharepaths = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPathMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleConnector_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ModuleImplementation(&mut self) -> ::windows::core::Result<IFsrmPipelineModuleImplementation>;
    fn ModuleName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HostingUserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HostingProcessPid(&mut self) -> ::windows::core::Result<i32>;
    fn Bind(&mut self, moduledefinition: &::core::option::Option<IFsrmPipelineModuleDefinition>, moduleimplementation: &::core::option::Option<IFsrmPipelineModuleImplementation>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleConnector_Vtbl {
        unsafe extern "system" fn ModuleImplementation<Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleImplementation() {
                ::core::result::Result::Ok(ok__) => {
                    *pipelinemoduleimplementation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleName<Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleName() {
                ::core::result::Result::Ok(ok__) => {
                    *username = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostingUserAccount<Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostingUserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *useraccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostingProcessPid<Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostingProcessPid() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bind<Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleimplementation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(::core::mem::transmute(&moduledefinition), ::core::mem::transmute(&moduleimplementation)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ModuleImplementation: ModuleImplementation::<Impl, IMPL_OFFSET>,
            ModuleName: ModuleName::<Impl, IMPL_OFFSET>,
            HostingUserAccount: HostingUserAccount::<Impl, IMPL_OFFSET>,
            HostingProcessPid: HostingProcessPid::<Impl, IMPL_OFFSET>,
            Bind: Bind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn ModuleClsid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModuleClsid(&mut self, moduleclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Company(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCompany(&mut self, company: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&mut self, version: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ModuleType(&mut self) -> ::windows::core::Result<FsrmPipelineModuleType>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn NeedsFileContent(&mut self) -> ::windows::core::Result<i16>;
    fn SetNeedsFileContent(&mut self, needsfilecontent: i16) -> ::windows::core::Result<()>;
    fn Account(&mut self) -> ::windows::core::Result<FsrmAccountType>;
    fn SetAccount(&mut self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()>;
    fn SupportedExtensions(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetSupportedExtensions(&mut self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&mut self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleDefinition_Vtbl {
        unsafe extern "system" fn ModuleClsid<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleClsid() {
                ::core::result::Result::Ok(ok__) => {
                    *moduleclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModuleClsid<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModuleClsid(::core::mem::transmute_copy(&moduleclsid)).into()
        }
        unsafe extern "system" fn Name<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Company<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Company() {
                ::core::result::Result::Ok(ok__) => {
                    *company = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompany(::core::mem::transmute_copy(&company)).into()
        }
        unsafe extern "system" fn Version<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *version = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&version)).into()
        }
        unsafe extern "system" fn ModuleType<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleType() {
                ::core::result::Result::Ok(ok__) => {
                    *moduletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn NeedsFileContent<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsFileContent() {
                ::core::result::Result::Ok(ok__) => {
                    *needsfilecontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeedsFileContent<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNeedsFileContent(::core::mem::transmute_copy(&needsfilecontent)).into()
        }
        unsafe extern "system" fn Account<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Account() {
                ::core::result::Result::Ok(ok__) => {
                    *retrievalaccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccount<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccount(::core::mem::transmute_copy(&retrievalaccount)).into()
        }
        unsafe extern "system" fn SupportedExtensions<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedExtensions<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportedExtensions(::core::mem::transmute_copy(&supportedextensions)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *parameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ModuleClsid: ModuleClsid::<Impl, IMPL_OFFSET>,
            SetModuleClsid: SetModuleClsid::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Company: Company::<Impl, IMPL_OFFSET>,
            SetCompany: SetCompany::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            ModuleType: ModuleType::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            NeedsFileContent: NeedsFileContent::<Impl, IMPL_OFFSET>,
            SetNeedsFileContent: SetNeedsFileContent::<Impl, IMPL_OFFSET>,
            Account: Account::<Impl, IMPL_OFFSET>,
            SetAccount: SetAccount::<Impl, IMPL_OFFSET>,
            SupportedExtensions: SupportedExtensions::<Impl, IMPL_OFFSET>,
            SetSupportedExtensions: SetSupportedExtensions::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleImplementation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnLoad(&mut self, moduledefinition: &::core::option::Option<IFsrmPipelineModuleDefinition>) -> ::windows::core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPipelineModuleImplementation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPipelineModuleImplementation_Vtbl {
        unsafe extern "system" fn OnLoad<Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLoad(::core::mem::transmute(&moduledefinition)) {
                ::core::result::Result::Ok(ok__) => {
                    *moduleconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUnload<Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUnload().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnLoad: OnLoad::<Impl, IMPL_OFFSET>,
            OnUnload: OnUnload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleImplementation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Sources(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyFlags(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmProperty_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sources<Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sources() {
                ::core::result::Result::Ok(ok__) => {
                    *sources = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyFlags<Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            Sources: Sources::<Impl, IMPL_OFFSET>,
            PropertyFlags: PropertyFlags::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RelativePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RelativeNamespaceRoot(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeIndex(&mut self) -> ::windows::core::Result<u32>;
    fn FileId(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ParentDirectoryId(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SizeAllocated(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreationTime(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LastAccessTime(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LastModificationTime(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Attributes(&mut self) -> ::windows::core::Result<u32>;
    fn OwnerSid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FilePropertyNames(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Messages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyBagFlags(&mut self) -> ::windows::core::Result<u32>;
    fn GetFileProperty(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmProperty>;
    fn SetFileProperty(&mut self, name: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddMessage(&mut self, message: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetFileStreamInterface(&mut self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyBag_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyBag_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativePath<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativePath() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *volumename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelativeNamespaceRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *relativenamespaceroot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeIndex<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *volumeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileId<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileId() {
                ::core::result::Result::Ok(ok__) => {
                    *fileid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDirectoryId<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentDirectoryId() {
                ::core::result::Result::Ok(ok__) => {
                    *parentdirectoryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeAllocated<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeAllocated() {
                ::core::result::Result::Ok(ok__) => {
                    *sizeallocated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *creationtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastAccessTime<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAccessTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastaccesstime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastModificationTime<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *lastmodificationtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *attributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerSid<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerSid() {
                ::core::result::Result::Ok(ok__) => {
                    *ownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePropertyNames<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilePropertyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *filepropertynames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Messages<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Messages() {
                ::core::result::Result::Ok(ok__) => {
                    *messages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyBagFlags<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyBagFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileProperty<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileProperty(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *fileproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileProperty<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AddMessage<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&message)).into()
        }
        unsafe extern "system" fn GetFileStreamInterface<Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileStreamInterface(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&interfacetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstreaminterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            RelativePath: RelativePath::<Impl, IMPL_OFFSET>,
            VolumeName: VolumeName::<Impl, IMPL_OFFSET>,
            RelativeNamespaceRoot: RelativeNamespaceRoot::<Impl, IMPL_OFFSET>,
            VolumeIndex: VolumeIndex::<Impl, IMPL_OFFSET>,
            FileId: FileId::<Impl, IMPL_OFFSET>,
            ParentDirectoryId: ParentDirectoryId::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SizeAllocated: SizeAllocated::<Impl, IMPL_OFFSET>,
            CreationTime: CreationTime::<Impl, IMPL_OFFSET>,
            LastAccessTime: LastAccessTime::<Impl, IMPL_OFFSET>,
            LastModificationTime: LastModificationTime::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
            OwnerSid: OwnerSid::<Impl, IMPL_OFFSET>,
            FilePropertyNames: FilePropertyNames::<Impl, IMPL_OFFSET>,
            Messages: Messages::<Impl, IMPL_OFFSET>,
            PropertyBagFlags: PropertyBagFlags::<Impl, IMPL_OFFSET>,
            GetFileProperty: GetFileProperty::<Impl, IMPL_OFFSET>,
            SetFileProperty: SetFileProperty::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
            GetFileStreamInterface: GetFileStreamInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBag2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmPropertyBag_Impl {
    fn GetFieldValue(&mut self, field: FsrmPropertyBagField) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetUntrustedInFileProperties(&mut self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyBag2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyBag2_Vtbl {
        unsafe extern "system" fn GetFieldValue<Impl: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFieldValue(::core::mem::transmute_copy(&field)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Impl: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUntrustedInFileProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *props = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmPropertyBag_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFieldValue: GetFieldValue::<Impl, IMPL_OFFSET>,
            GetUntrustedInFileProperties: GetUntrustedInFileProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyCondition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<FsrmPropertyConditionType>;
    fn SetType(&mut self, r#type: FsrmPropertyConditionType) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetValue(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyCondition_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Type<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Value<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<FsrmPropertyDefinitionType>;
    fn SetType(&mut self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()>;
    fn PossibleValues(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPossibleValues(&mut self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ValueDescriptions(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetValueDescriptions(&mut self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&mut self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinition_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Type<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn PossibleValues<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PossibleValues() {
                ::core::result::Result::Ok(ok__) => {
                    *possiblevalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleValues<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPossibleValues(::core::mem::transmute_copy(&possiblevalues)).into()
        }
        unsafe extern "system" fn ValueDescriptions<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *valuedescriptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueDescriptions<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueDescriptions(::core::mem::transmute_copy(&valuedescriptions)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *parameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            PossibleValues: PossibleValues::<Impl, IMPL_OFFSET>,
            SetPossibleValues: SetPossibleValues::<Impl, IMPL_OFFSET>,
            ValueDescriptions: ValueDescriptions::<Impl, IMPL_OFFSET>,
            SetValueDescriptions: SetValueDescriptions::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinition2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmPropertyDefinition_Impl {
    fn PropertyDefinitionFlags(&mut self) -> ::windows::core::Result<i32>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppliesTo(&mut self) -> ::windows::core::Result<i32>;
    fn ValueDefinitions(&mut self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinition2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinition2_Vtbl {
        unsafe extern "system" fn PropertyDefinitionFlags<Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyDefinitionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *propertydefinitionflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn AppliesTo<Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppliesTo() {
                ::core::result::Result::Ok(ok__) => {
                    *appliesto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueDefinitions<Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *valuedefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmPropertyDefinition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PropertyDefinitionFlags: PropertyDefinitionFlags::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            AppliesTo: AppliesTo::<Impl, IMPL_OFFSET>,
            ValueDefinitions: ValueDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinitionValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UniqueID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinitionValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmPropertyDefinitionValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmPropertyDefinitionValue_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueID<Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniqueid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueID() {
                ::core::result::Result::Ok(ok__) => {
                    *uniqueid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            UniqueID: UniqueID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuota_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmQuotaBase_Impl + IFsrmQuotaObject_Impl {
    fn QuotaUsed(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn QuotaPeakUsage(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn QuotaPeakUsageTime(&mut self) -> ::windows::core::Result<f64>;
    fn ResetPeakUsage(&mut self) -> ::windows::core::Result<()>;
    fn RefreshUsageProperties(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuota_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuota_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuota_Vtbl {
        unsafe extern "system" fn QuotaUsed<Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuotaUsed() {
                ::core::result::Result::Ok(ok__) => {
                    *used = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuotaPeakUsage<Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuotaPeakUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *peakusage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuotaPeakUsageTime<Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuotaPeakUsageTime() {
                ::core::result::Result::Ok(ok__) => {
                    *peakusagedatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetPeakUsage<Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetPeakUsage().into()
        }
        unsafe extern "system" fn RefreshUsageProperties<Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshUsageProperties().into()
        }
        Self {
            base: IFsrmQuotaObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QuotaUsed: QuotaUsed::<Impl, IMPL_OFFSET>,
            QuotaPeakUsage: QuotaPeakUsage::<Impl, IMPL_OFFSET>,
            QuotaPeakUsageTime: QuotaPeakUsageTime::<Impl, IMPL_OFFSET>,
            ResetPeakUsage: ResetPeakUsage::<Impl, IMPL_OFFSET>,
            RefreshUsageProperties: RefreshUsageProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuota as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaBase_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn QuotaLimit(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetQuotaLimit(&mut self, quotalimit: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn QuotaFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetQuotaFlags(&mut self, quotaflags: i32) -> ::windows::core::Result<()>;
    fn Thresholds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddThreshold(&mut self, threshold: i32) -> ::windows::core::Result<()>;
    fn DeleteThreshold(&mut self, threshold: i32) -> ::windows::core::Result<()>;
    fn ModifyThreshold(&mut self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()>;
    fn CreateThresholdAction(&mut self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumThresholdActions(&mut self, threshold: i32) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaBase_Vtbl {
        unsafe extern "system" fn QuotaLimit<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuotaLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *quotalimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaLimit<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaLimit(::core::mem::transmute_copy(&quotalimit)).into()
        }
        unsafe extern "system" fn QuotaFlags<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuotaFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *quotaflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaFlags<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaFlags(::core::mem::transmute_copy(&quotaflags)).into()
        }
        unsafe extern "system" fn Thresholds<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thresholds() {
                ::core::result::Result::Ok(ok__) => {
                    *thresholds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddThreshold<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddThreshold(::core::mem::transmute_copy(&threshold)).into()
        }
        unsafe extern "system" fn DeleteThreshold<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteThreshold(::core::mem::transmute_copy(&threshold)).into()
        }
        unsafe extern "system" fn ModifyThreshold<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyThreshold(::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&newthreshold)).into()
        }
        unsafe extern "system" fn CreateThresholdAction<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateThresholdAction(::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumThresholdActions<Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumThresholdActions(::core::mem::transmute_copy(&threshold)) {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QuotaLimit: QuotaLimit::<Impl, IMPL_OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Impl, IMPL_OFFSET>,
            QuotaFlags: QuotaFlags::<Impl, IMPL_OFFSET>,
            SetQuotaFlags: SetQuotaFlags::<Impl, IMPL_OFFSET>,
            Thresholds: Thresholds::<Impl, IMPL_OFFSET>,
            AddThreshold: AddThreshold::<Impl, IMPL_OFFSET>,
            DeleteThreshold: DeleteThreshold::<Impl, IMPL_OFFSET>,
            ModifyThreshold: ModifyThreshold::<Impl, IMPL_OFFSET>,
            CreateThresholdAction: CreateThresholdAction::<Impl, IMPL_OFFSET>,
            EnumThresholdActions: EnumThresholdActions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateQuota(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn CreateAutoApplyQuota(&mut self, quotatemplatename: &super::super::Foundation::BSTR, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota>;
    fn GetQuota(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn GetAutoApplyQuota(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota>;
    fn GetRestrictiveQuota(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn EnumQuotas(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn EnumAutoApplyQuotas(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn EnumEffectiveQuotas(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn Scan(&mut self, strpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CreateQuotaCollection(&mut self) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    *variables = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *descriptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuota<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuota(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *quota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAutoApplyQuota<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAutoApplyQuota(::core::mem::transmute_copy(&quotatemplatename), ::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *quota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuota<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuota(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *quota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoApplyQuota<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoApplyQuota(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *quota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictiveQuota<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictiveQuota(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *quota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumQuotas<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumQuotas(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAutoApplyQuotas<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAutoApplyQuotas(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffectiveQuotas<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumEffectiveQuotas(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotas = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scan<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Scan(::core::mem::transmute_copy(&strpath)).into()
        }
        unsafe extern "system" fn CreateQuotaCollection<Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateQuotaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActionVariables: ActionVariables::<Impl, IMPL_OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Impl, IMPL_OFFSET>,
            CreateQuota: CreateQuota::<Impl, IMPL_OFFSET>,
            CreateAutoApplyQuota: CreateAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetQuota: GetQuota::<Impl, IMPL_OFFSET>,
            GetAutoApplyQuota: GetAutoApplyQuota::<Impl, IMPL_OFFSET>,
            GetRestrictiveQuota: GetRestrictiveQuota::<Impl, IMPL_OFFSET>,
            EnumQuotas: EnumQuotas::<Impl, IMPL_OFFSET>,
            EnumAutoApplyQuotas: EnumAutoApplyQuotas::<Impl, IMPL_OFFSET>,
            EnumEffectiveQuotas: EnumEffectiveQuotas::<Impl, IMPL_OFFSET>,
            Scan: Scan::<Impl, IMPL_OFFSET>,
            CreateQuotaCollection: CreateQuotaCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManagerEx_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmQuotaManager_Impl {
    fn IsAffectedByQuota(&mut self, path: &super::super::Foundation::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaManagerEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaManagerEx_Vtbl {
        unsafe extern "system" fn IsAffectedByQuota<Impl: IFsrmQuotaManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAffectedByQuota(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *affected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFsrmQuotaManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), IsAffectedByQuota: IsAffectedByQuota::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaObject_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmQuotaBase_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserSid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SourceTemplateName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MatchesSourceTemplate(&mut self) -> ::windows::core::Result<i16>;
    fn ApplyTemplate(&mut self, quotatemplatename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaObject_Vtbl {
        unsafe extern "system" fn Path<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSid<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSid() {
                ::core::result::Result::Ok(ok__) => {
                    *usersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *useraccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceTemplateName<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceTemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    *quotatemplatename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchesSourceTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *matches = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTemplate<Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyTemplate(::core::mem::transmute_copy(&quotatemplatename)).into()
        }
        Self {
            base: IFsrmQuotaBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            UserSid: UserSid::<Impl, IMPL_OFFSET>,
            UserAccount: UserAccount::<Impl, IMPL_OFFSET>,
            SourceTemplateName: SourceTemplateName::<Impl, IMPL_OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Impl, IMPL_OFFSET>,
            ApplyTemplate: ApplyTemplate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplate_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmQuotaBase_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyTemplate(&mut self, quotatemplatename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&mut self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplate_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn CopyTemplate<Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTemplate(::core::mem::transmute_copy(&quotatemplatename)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *derivedobjectsresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmQuotaBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            CopyTemplate: CopyTemplate::<Impl, IMPL_OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateImported_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmQuotaBase_Impl + IFsrmQuotaTemplate_Impl {
    fn OverwriteOnCommit(&mut self) -> ::windows::core::Result<i16>;
    fn SetOverwriteOnCommit(&mut self, overwrite: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplateImported_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplateImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *overwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base: IFsrmQuotaTemplate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Impl, IMPL_OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateImported as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&mut self) -> ::windows::core::Result<IFsrmQuotaTemplate>;
    fn GetTemplate(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmQuotaTemplate>;
    fn EnumTemplates(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&mut self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImportTemplates(&mut self, serializedquotatemplates: &super::super::Foundation::BSTR, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmQuotaTemplateManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmQuotaTemplateManager_Vtbl {
        unsafe extern "system" fn CreateTemplate<Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *quotatemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplate<Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplate(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotatemplate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTemplates<Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTemplates(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotatemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportTemplates<Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportTemplates(::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *serializedquotatemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportTemplates<Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportTemplates(::core::mem::transmute_copy(&serializedquotatemplates), ::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *quotatemplates = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateTemplate: CreateTemplate::<Impl, IMPL_OFFSET>,
            GetTemplate: GetTemplate::<Impl, IMPL_OFFSET>,
            EnumTemplates: EnumTemplates::<Impl, IMPL_OFFSET>,
            ExportTemplates: ExportTemplates::<Impl, IMPL_OFFSET>,
            ImportTemplates: ImportTemplates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&mut self) -> ::windows::core::Result<FsrmReportType>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LastGeneratedFileNamePrefix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFilter(&mut self, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&mut self, filter: FsrmReportFilter, filtervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReport_Vtbl {
        unsafe extern "system" fn Type<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *reporttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Description<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn LastGeneratedFileNamePrefix<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastGeneratedFileNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *prefix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilter<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilter(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *filtervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilter(::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&filtervalue)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            LastGeneratedFileNamePrefix: LastGeneratedFileNamePrefix::<Impl, IMPL_OFFSET>,
            GetFilter: GetFilter::<Impl, IMPL_OFFSET>,
            SetFilter: SetFilter::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportJob_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Task(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTask(&mut self, taskname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&mut self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Formats(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&mut self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailTo(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RunningStatus(&mut self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn LastRun(&mut self) -> ::windows::core::Result<f64>;
    fn LastError(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LastGeneratedInDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumReports(&mut self) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateReport(&mut self, reporttype: FsrmReportType) -> ::windows::core::Result<IFsrmReport>;
    fn Run(&mut self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&mut self, waitseconds: i32) -> ::windows::core::Result<i16>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportJob_Vtbl {
        unsafe extern "system" fn Task<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *taskname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&taskname)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceroots = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn Formats<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Formats() {
                ::core::result::Result::Ok(ok__) => {
                    *formats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormats<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn MailTo<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    *mailto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailTo(::core::mem::transmute_copy(&mailto)).into()
        }
        unsafe extern "system" fn RunningStatus<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *runningstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRun<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastRun() {
                ::core::result::Result::Ok(ok__) => {
                    *lastrun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastError<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastError() {
                ::core::result::Result::Ok(ok__) => {
                    *lasterror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastGeneratedInDirectory<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastGeneratedInDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumReports<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumReports() {
                ::core::result::Result::Ok(ok__) => {
                    *reports = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReport<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *report = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *completed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Task: Task::<Impl, IMPL_OFFSET>,
            SetTask: SetTask::<Impl, IMPL_OFFSET>,
            NamespaceRoots: NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            Formats: Formats::<Impl, IMPL_OFFSET>,
            SetFormats: SetFormats::<Impl, IMPL_OFFSET>,
            MailTo: MailTo::<Impl, IMPL_OFFSET>,
            SetMailTo: SetMailTo::<Impl, IMPL_OFFSET>,
            RunningStatus: RunningStatus::<Impl, IMPL_OFFSET>,
            LastRun: LastRun::<Impl, IMPL_OFFSET>,
            LastError: LastError::<Impl, IMPL_OFFSET>,
            LastGeneratedInDirectory: LastGeneratedInDirectory::<Impl, IMPL_OFFSET>,
            EnumReports: EnumReports::<Impl, IMPL_OFFSET>,
            CreateReport: CreateReport::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            WaitForCompletion: WaitForCompletion::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumReportJobs(&mut self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateReportJob(&mut self) -> ::windows::core::Result<IFsrmReportJob>;
    fn GetReportJob(&mut self, taskname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IFsrmReportJob>;
    fn GetOutputDirectory(&mut self, context: FsrmReportGenerationContext) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOutputDirectory(&mut self, context: FsrmReportGenerationContext, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsFilterValidForReportType(&mut self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<i16>;
    fn GetDefaultFilter(&mut self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDefaultFilter(&mut self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetReportSizeLimit(&mut self, limit: FsrmReportLimit) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetReportSizeLimit(&mut self, limit: FsrmReportLimit, limitvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportManager_Vtbl {
        unsafe extern "system" fn EnumReportJobs<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumReportJobs(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    *reportjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReportJob<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReportJob() {
                ::core::result::Result::Ok(ok__) => {
                    *reportjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportJob<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportJob(::core::mem::transmute_copy(&taskname)) {
                ::core::result::Result::Ok(ok__) => {
                    *reportjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputDirectory<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputDirectory(::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputDirectory<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputDirectory(::core::mem::transmute_copy(&context), ::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn IsFilterValidForReportType<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFilterValidForReportType(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *valid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFilter(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *filtervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultFilter<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultFilter(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&filtervalue)).into()
        }
        unsafe extern "system" fn GetReportSizeLimit<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportSizeLimit(::core::mem::transmute_copy(&limit)) {
                ::core::result::Result::Ok(ok__) => {
                    *limitvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportSizeLimit<Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportSizeLimit(::core::mem::transmute_copy(&limit), ::core::mem::transmute_copy(&limitvalue)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumReportJobs: EnumReportJobs::<Impl, IMPL_OFFSET>,
            CreateReportJob: CreateReportJob::<Impl, IMPL_OFFSET>,
            GetReportJob: GetReportJob::<Impl, IMPL_OFFSET>,
            GetOutputDirectory: GetOutputDirectory::<Impl, IMPL_OFFSET>,
            SetOutputDirectory: SetOutputDirectory::<Impl, IMPL_OFFSET>,
            IsFilterValidForReportType: IsFilterValidForReportType::<Impl, IMPL_OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Impl, IMPL_OFFSET>,
            SetDefaultFilter: SetDefaultFilter::<Impl, IMPL_OFFSET>,
            GetReportSizeLimit: GetReportSizeLimit::<Impl, IMPL_OFFSET>,
            SetReportSizeLimit: SetReportSizeLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportScheduler_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn VerifyNamespaces(&mut self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CreateScheduleTask(&mut self, taskname: &super::super::Foundation::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ModifyScheduleTask(&mut self, taskname: &super::super::Foundation::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteScheduleTask(&mut self, taskname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportScheduler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmReportScheduler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmReportScheduler_Vtbl {
        unsafe extern "system" fn VerifyNamespaces<Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VerifyNamespaces(::core::mem::transmute_copy(&namespacessafearray)).into()
        }
        unsafe extern "system" fn CreateScheduleTask<Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateScheduleTask(::core::mem::transmute_copy(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute_copy(&serializedtask)).into()
        }
        unsafe extern "system" fn ModifyScheduleTask<Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyScheduleTask(::core::mem::transmute_copy(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute_copy(&serializedtask)).into()
        }
        unsafe extern "system" fn DeleteScheduleTask<Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteScheduleTask(::core::mem::transmute_copy(&taskname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            VerifyNamespaces: VerifyNamespaces::<Impl, IMPL_OFFSET>,
            CreateScheduleTask: CreateScheduleTask::<Impl, IMPL_OFFSET>,
            ModifyScheduleTask: ModifyScheduleTask::<Impl, IMPL_OFFSET>,
            DeleteScheduleTask: DeleteScheduleTask::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportScheduler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmRule_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RuleType(&mut self) -> ::windows::core::Result<FsrmRuleType>;
    fn ModuleDefinitionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetModuleDefinitionName(&mut self, moduledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&mut self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RuleFlags(&mut self) -> ::windows::core::Result<i32>;
    fn SetRuleFlags(&mut self, ruleflags: i32) -> ::windows::core::Result<()>;
    fn Parameters(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&mut self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn LastModified(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmRule_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn RuleType<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleType() {
                ::core::result::Result::Ok(ok__) => {
                    *ruletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleDefinitionName<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModuleDefinitionName() {
                ::core::result::Result::Ok(ok__) => {
                    *moduledefinitionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModuleDefinitionName<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModuleDefinitionName(::core::mem::transmute_copy(&moduledefinitionname)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *namespaceroots = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn RuleFlags<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *ruleflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleFlags<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleFlags(::core::mem::transmute_copy(&ruleflags)).into()
        }
        unsafe extern "system" fn Parameters<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    *parameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn LastModified<Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModified() {
                ::core::result::Result::Ok(ok__) => {
                    *lastmodified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFsrmObject_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            RuleType: RuleType::<Impl, IMPL_OFFSET>,
            ModuleDefinitionName: ModuleDefinitionName::<Impl, IMPL_OFFSET>,
            SetModuleDefinitionName: SetModuleDefinitionName::<Impl, IMPL_OFFSET>,
            NamespaceRoots: NamespaceRoots::<Impl, IMPL_OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Impl, IMPL_OFFSET>,
            RuleFlags: RuleFlags::<Impl, IMPL_OFFSET>,
            SetRuleFlags: SetRuleFlags::<Impl, IMPL_OFFSET>,
            Parameters: Parameters::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            LastModified: LastModified::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmSetting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SmtpServer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSmtpServer(&mut self, smtpserver: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MailFrom(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMailFrom(&mut self, mailfrom: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AdminEmail(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAdminEmail(&mut self, adminemail: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisableCommandLine(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisableCommandLine(&mut self, disablecommandline: i16) -> ::windows::core::Result<()>;
    fn EnableScreeningAudit(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnableScreeningAudit(&mut self, enablescreeningaudit: i16) -> ::windows::core::Result<()>;
    fn EmailTest(&mut self, mailto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetActionRunLimitInterval(&mut self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::Result<()>;
    fn GetActionRunLimitInterval(&mut self, actiontype: FsrmActionType) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmSetting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmSetting_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmSetting_Vtbl {
        unsafe extern "system" fn SmtpServer<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmtpServer() {
                ::core::result::Result::Ok(ok__) => {
                    *smtpserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmtpServer<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmtpServer(::core::mem::transmute_copy(&smtpserver)).into()
        }
        unsafe extern "system" fn MailFrom<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *mailfrom = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailFrom<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailFrom(::core::mem::transmute_copy(&mailfrom)).into()
        }
        unsafe extern "system" fn AdminEmail<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminEmail() {
                ::core::result::Result::Ok(ok__) => {
                    *adminemail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdminEmail<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdminEmail(::core::mem::transmute_copy(&adminemail)).into()
        }
        unsafe extern "system" fn DisableCommandLine<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableCommandLine() {
                ::core::result::Result::Ok(ok__) => {
                    *disablecommandline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableCommandLine<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisableCommandLine(::core::mem::transmute_copy(&disablecommandline)).into()
        }
        unsafe extern "system" fn EnableScreeningAudit<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableScreeningAudit() {
                ::core::result::Result::Ok(ok__) => {
                    *enablescreeningaudit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableScreeningAudit(::core::mem::transmute_copy(&enablescreeningaudit)).into()
        }
        unsafe extern "system" fn EmailTest<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EmailTest(::core::mem::transmute_copy(&mailto)).into()
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActionRunLimitInterval(::core::mem::transmute_copy(&actiontype), ::core::mem::transmute_copy(&delaytimeminutes)).into()
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActionRunLimitInterval(::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *delaytimeminutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SmtpServer: SmtpServer::<Impl, IMPL_OFFSET>,
            SetSmtpServer: SetSmtpServer::<Impl, IMPL_OFFSET>,
            MailFrom: MailFrom::<Impl, IMPL_OFFSET>,
            SetMailFrom: SetMailFrom::<Impl, IMPL_OFFSET>,
            AdminEmail: AdminEmail::<Impl, IMPL_OFFSET>,
            SetAdminEmail: SetAdminEmail::<Impl, IMPL_OFFSET>,
            DisableCommandLine: DisableCommandLine::<Impl, IMPL_OFFSET>,
            SetDisableCommandLine: SetDisableCommandLine::<Impl, IMPL_OFFSET>,
            EnableScreeningAudit: EnableScreeningAudit::<Impl, IMPL_OFFSET>,
            SetEnableScreeningAudit: SetEnableScreeningAudit::<Impl, IMPL_OFFSET>,
            EmailTest: EmailTest::<Impl, IMPL_OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Impl, IMPL_OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmObject_Impl + IFsrmPipelineModuleDefinition_Impl {
    fn Capabilities(&mut self) -> ::windows::core::Result<FsrmStorageModuleCaps>;
    fn SetCapabilities(&mut self, capabilities: FsrmStorageModuleCaps) -> ::windows::core::Result<()>;
    fn StorageType(&mut self) -> ::windows::core::Result<FsrmStorageModuleType>;
    fn SetStorageType(&mut self, storagetype: FsrmStorageModuleType) -> ::windows::core::Result<()>;
    fn UpdatesFileContent(&mut self) -> ::windows::core::Result<i16>;
    fn SetUpdatesFileContent(&mut self, updatesfilecontent: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmStorageModuleDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmStorageModuleDefinition_Vtbl {
        unsafe extern "system" fn Capabilities<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *capabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapabilities<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCapabilities(::core::mem::transmute_copy(&capabilities)).into()
        }
        unsafe extern "system" fn StorageType<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StorageType() {
                ::core::result::Result::Ok(ok__) => {
                    *storagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageType<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageType(::core::mem::transmute_copy(&storagetype)).into()
        }
        unsafe extern "system" fn UpdatesFileContent<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatesFileContent() {
                ::core::result::Result::Ok(ok__) => {
                    *updatesfilecontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdatesFileContent<Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdatesFileContent(::core::mem::transmute_copy(&updatesfilecontent)).into()
        }
        Self {
            base: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Capabilities: Capabilities::<Impl, IMPL_OFFSET>,
            SetCapabilities: SetCapabilities::<Impl, IMPL_OFFSET>,
            StorageType: StorageType::<Impl, IMPL_OFFSET>,
            SetStorageType: SetStorageType::<Impl, IMPL_OFFSET>,
            UpdatesFileContent: UpdatesFileContent::<Impl, IMPL_OFFSET>,
            SetUpdatesFileContent: SetUpdatesFileContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleImplementation_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(&mut self, propertydefinitions: &::core::option::Option<IFsrmCollection>) -> ::windows::core::Result<()>;
    fn LoadProperties(&mut self, propertybag: &::core::option::Option<IFsrmPropertyBag>) -> ::windows::core::Result<()>;
    fn SaveProperties(&mut self, propertybag: &::core::option::Option<IFsrmPropertyBag>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsrmStorageModuleImplementation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsrmStorageModuleImplementation_Vtbl {
        unsafe extern "system" fn UseDefinitions<Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseDefinitions(::core::mem::transmute(&propertydefinitions)).into()
        }
        unsafe extern "system" fn LoadProperties<Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadProperties(::core::mem::transmute(&propertybag)).into()
        }
        unsafe extern "system" fn SaveProperties<Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveProperties(::core::mem::transmute(&propertybag)).into()
        }
        Self {
            base: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UseDefinitions: UseDefinitions::<Impl, IMPL_OFFSET>,
            LoadProperties: LoadProperties::<Impl, IMPL_OFFSET>,
            SaveProperties: SaveProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleImplementation as ::windows::core::Interface>::IID
    }
}
