#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DIFsrmClassificationEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for DIFsrmClassificationEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DIFsrmClassificationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DIFsrmClassificationEvents_Impl, const OFFSET: isize>() -> DIFsrmClassificationEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DIFsrmClassificationEvents as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAccessDeniedRemediationClient_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Show(&self, parentwnd: usize, accesspath: &::windows::core::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &::windows::core::BSTR, windowmessage: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmAccessDeniedRemediationClient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAccessDeniedRemediationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>() -> IFsrmAccessDeniedRemediationClient_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAccessDeniedRemediationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::std::mem::MaybeUninit<::windows::core::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::std::mem::MaybeUninit<::windows::core::BSTR>, windowmessage: ::std::mem::MaybeUninit<::windows::core::BSTR>, result: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Show(::core::mem::transmute_copy(&parentwnd), ::core::mem::transmute(&accesspath), ::core::mem::transmute_copy(&errortype), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&windowtitle), ::core::mem::transmute(&windowmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Show: Show::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAccessDeniedRemediationClient as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAction_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ActionType(&self) -> ::windows::core::Result<FsrmActionType>;
    fn RunLimitInterval(&self) -> ::windows::core::Result<i32>;
    fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>() -> IFsrmAction_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actiontype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunLimitInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunLimitInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRunLimitInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunLimitInterval(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            ActionType: ActionType::<Identity, Impl, OFFSET>,
            RunLimitInterval: RunLimitInterval::<Identity, Impl, OFFSET>,
            SetRunLimitInterval: SetRunLimitInterval::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAction as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionCommand_Impl: Sized + IFsrmAction_Impl {
    fn ExecutablePath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetExecutablePath(&self, executablepath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetArguments(&self, arguments: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Account(&self) -> ::windows::core::Result<FsrmAccountType>;
    fn SetAccount(&self, account: FsrmAccountType) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetWorkingDirectory(&self, workingdirectory: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MonitorCommand(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMonitorCommand(&self, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn KillTimeOut(&self) -> ::windows::core::Result<i32>;
    fn SetKillTimeOut(&self, minutes: i32) -> ::windows::core::Result<()>;
    fn LogResult(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogResult(&self, logresults: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmActionCommand {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>() -> IFsrmActionCommand_Vtbl {
        unsafe extern "system" fn ExecutablePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecutablePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(executablepath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExecutablePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executablepath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExecutablePath(::core::mem::transmute(&executablepath)).into()
        }
        unsafe extern "system" fn Arguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Arguments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(arguments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arguments: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArguments(::core::mem::transmute(&arguments)).into()
        }
        unsafe extern "system" fn Account<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Account() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(account, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccount(::core::mem::transmute_copy(&account)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(workingdirectory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkingDirectory(::core::mem::transmute(&workingdirectory)).into()
        }
        unsafe extern "system" fn MonitorCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MonitorCommand() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(monitorcommand, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMonitorCommand(::core::mem::transmute_copy(&monitorcommand)).into()
        }
        unsafe extern "system" fn KillTimeOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KillTimeOut() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKillTimeOut<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKillTimeOut(::core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LogResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LogResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(logresults, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logresults: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogResult(::core::mem::transmute_copy(&logresults)).into()
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExecutablePath: ExecutablePath::<Identity, Impl, OFFSET>,
            SetExecutablePath: SetExecutablePath::<Identity, Impl, OFFSET>,
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            SetArguments: SetArguments::<Identity, Impl, OFFSET>,
            Account: Account::<Identity, Impl, OFFSET>,
            SetAccount: SetAccount::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            MonitorCommand: MonitorCommand::<Identity, Impl, OFFSET>,
            SetMonitorCommand: SetMonitorCommand::<Identity, Impl, OFFSET>,
            KillTimeOut: KillTimeOut::<Identity, Impl, OFFSET>,
            SetKillTimeOut: SetKillTimeOut::<Identity, Impl, OFFSET>,
            LogResult: LogResult::<Identity, Impl, OFFSET>,
            SetLogResult: SetLogResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionCommand as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmAction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmail_Impl: Sized + IFsrmAction_Impl {
    fn MailFrom(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailReplyTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailReplyTo(&self, mailreplyto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailCc(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailCc(&self, mailcc: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailBcc(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailBcc(&self, mailbcc: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailSubject(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailSubject(&self, mailsubject: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MessageText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMessageText(&self, messagetext: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmActionEmail {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmail_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>() -> IFsrmActionEmail_Vtbl {
        unsafe extern "system" fn MailFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailfrom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailFrom(::core::mem::transmute(&mailfrom)).into()
        }
        unsafe extern "system" fn MailReplyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailReplyTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailreplyto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailReplyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailreplyto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailReplyTo(::core::mem::transmute(&mailreplyto)).into()
        }
        unsafe extern "system" fn MailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailTo(::core::mem::transmute(&mailto)).into()
        }
        unsafe extern "system" fn MailCc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailCc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailcc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailCc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailcc: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailCc(::core::mem::transmute(&mailcc)).into()
        }
        unsafe extern "system" fn MailBcc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailBcc() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailbcc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailBcc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailbcc: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailBcc(::core::mem::transmute(&mailbcc)).into()
        }
        unsafe extern "system" fn MailSubject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailSubject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailsubject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailSubject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailsubject: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailSubject(::core::mem::transmute(&mailsubject)).into()
        }
        unsafe extern "system" fn MessageText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageText(::core::mem::transmute(&messagetext)).into()
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            MailFrom: MailFrom::<Identity, Impl, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, Impl, OFFSET>,
            MailReplyTo: MailReplyTo::<Identity, Impl, OFFSET>,
            SetMailReplyTo: SetMailReplyTo::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            MailCc: MailCc::<Identity, Impl, OFFSET>,
            SetMailCc: SetMailCc::<Identity, Impl, OFFSET>,
            MailBcc: MailBcc::<Identity, Impl, OFFSET>,
            SetMailBcc: SetMailBcc::<Identity, Impl, OFFSET>,
            MailSubject: MailSubject::<Identity, Impl, OFFSET>,
            SetMailSubject: SetMailSubject::<Identity, Impl, OFFSET>,
            MessageText: MessageText::<Identity, Impl, OFFSET>,
            SetMessageText: SetMessageText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmAction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEmail2_Impl: Sized + IFsrmActionEmail_Impl {
    fn AttachmentFileListSize(&self) -> ::windows::core::Result<i32>;
    fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmActionEmail2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEmail2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: isize>() -> IFsrmActionEmail2_Vtbl {
        unsafe extern "system" fn AttachmentFileListSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachmentFileListSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attachmentfilelistsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachmentFileListSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEmail2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachmentFileListSize(::core::mem::transmute_copy(&attachmentfilelistsize)).into()
        }
        Self {
            base__: IFsrmActionEmail_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttachmentFileListSize: AttachmentFileListSize::<Identity, Impl, OFFSET>,
            SetAttachmentFileListSize: SetAttachmentFileListSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEmail2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmAction as ::windows::core::ComInterface>::IID || iid == &<IFsrmActionEmail as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionEventLog_Impl: Sized + IFsrmAction_Impl {
    fn EventType(&self) -> ::windows::core::Result<FsrmEventType>;
    fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows::core::Result<()>;
    fn MessageText(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMessageText(&self, messagetext: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmActionEventLog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionEventLog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>() -> IFsrmActionEventLog_Vtbl {
        unsafe extern "system" fn EventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventType(::core::mem::transmute_copy(&eventtype)).into()
        }
        unsafe extern "system" fn MessageText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionEventLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageText(::core::mem::transmute(&messagetext)).into()
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventType: EventType::<Identity, Impl, OFFSET>,
            SetEventType: SetEventType::<Identity, Impl, OFFSET>,
            MessageText: MessageText::<Identity, Impl, OFFSET>,
            SetMessageText: SetMessageText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionEventLog as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmAction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmActionReport_Impl: Sized + IFsrmAction_Impl {
    fn ReportTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmActionReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmActionReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: isize>() -> IFsrmActionReport_Vtbl {
        unsafe extern "system" fn ReportTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reporttypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportTypes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReportTypes(::core::mem::transmute_copy(&reporttypes)).into()
        }
        unsafe extern "system" fn MailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmActionReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailTo(::core::mem::transmute(&mailto)).into()
        }
        Self {
            base__: IFsrmAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReportTypes: ReportTypes::<Identity, Impl, OFFSET>,
            SetReportTypes: SetReportTypes::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmActionReport as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmAction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmAutoApplyQuota_Impl: Sized + IFsrmQuotaObject_Impl {
    fn ExcludeFolders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmAutoApplyQuota {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmAutoApplyQuota_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>() -> IFsrmAutoApplyQuota_Vtbl {
        unsafe extern "system" fn ExcludeFolders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExcludeFolders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(folders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeFolders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExcludeFolders(::core::mem::transmute_copy(&folders)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmAutoApplyQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmQuotaObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExcludeFolders: ExcludeFolders::<Identity, Impl, OFFSET>,
            SetExcludeFolders: SetExcludeFolders::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmAutoApplyQuota as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Logging(&self) -> ::windows::core::Result<i32>;
    fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()>;
    fn ClassificationReportMailTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetClassificationReportMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ClassificationReportEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetClassificationReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClassificationLastError(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition>;
    fn GetPropertyDefinition(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmPropertyDefinition>;
    fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule>;
    fn GetRule(&self, rulename: &::windows::core::BSTR, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule>;
    fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition>;
    fn GetModuleDefinition(&self, modulename: &::windows::core::BSTR, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition>;
    fn RunClassification(&self, context: FsrmReportGenerationContext, reserved: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CancelClassification(&self) -> ::windows::core::Result<()>;
    fn EnumFileProperties(&self, filepath: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn GetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, propertyvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ClearFileProperty(&self, filepath: &::windows::core::BSTR, property: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmClassificationManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>() -> IFsrmClassificationManager_Vtbl {
        unsafe extern "system" fn ClassificationReportFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationReportFormats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassificationReportFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn Logging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Logging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(logging, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogging(::core::mem::transmute_copy(&logging)).into()
        }
        unsafe extern "system" fn ClassificationReportMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationReportMailTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassificationReportMailTo(::core::mem::transmute(&mailto)).into()
        }
        unsafe extern "system" fn ClassificationReportEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationReportEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassificationReportEnabled(::core::mem::transmute_copy(&reportenabled)).into()
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastreportpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationLastReportPathWithoutExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastreportpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassificationLastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationLastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassificationRunningStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassificationRunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumPropertyDefinitions(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyDefinition(::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRules<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumRules(::core::mem::transmute_copy(&ruletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRule(::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::std::mem::MaybeUninit<::windows::core::BSTR>, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRule(::core::mem::transmute(&rulename), ::core::mem::transmute_copy(&ruletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumModuleDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumModuleDefinitions(::core::mem::transmute_copy(&moduletype), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateModuleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateModuleDefinition(::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModuleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modulename: ::std::mem::MaybeUninit<::windows::core::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModuleDefinition(::core::mem::transmute(&modulename), ::core::mem::transmute_copy(&moduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunClassification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunClassification(::core::mem::transmute_copy(&context), ::core::mem::transmute(&reserved)).into()
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForClassificationCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelClassification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelClassification().into()
        }
        unsafe extern "system" fn EnumFileProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFileProperties(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileProperty(::core::mem::transmute(&filepath), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileProperty(::core::mem::transmute(&filepath), ::core::mem::transmute(&propertyname), ::core::mem::transmute(&propertyvalue)).into()
        }
        unsafe extern "system" fn ClearFileProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, property: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearFileProperty(::core::mem::transmute(&filepath), ::core::mem::transmute(&property)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClassificationReportFormats: ClassificationReportFormats::<Identity, Impl, OFFSET>,
            SetClassificationReportFormats: SetClassificationReportFormats::<Identity, Impl, OFFSET>,
            Logging: Logging::<Identity, Impl, OFFSET>,
            SetLogging: SetLogging::<Identity, Impl, OFFSET>,
            ClassificationReportMailTo: ClassificationReportMailTo::<Identity, Impl, OFFSET>,
            SetClassificationReportMailTo: SetClassificationReportMailTo::<Identity, Impl, OFFSET>,
            ClassificationReportEnabled: ClassificationReportEnabled::<Identity, Impl, OFFSET>,
            SetClassificationReportEnabled: SetClassificationReportEnabled::<Identity, Impl, OFFSET>,
            ClassificationLastReportPathWithoutExtension: ClassificationLastReportPathWithoutExtension::<Identity, Impl, OFFSET>,
            ClassificationLastError: ClassificationLastError::<Identity, Impl, OFFSET>,
            ClassificationRunningStatus: ClassificationRunningStatus::<Identity, Impl, OFFSET>,
            EnumPropertyDefinitions: EnumPropertyDefinitions::<Identity, Impl, OFFSET>,
            CreatePropertyDefinition: CreatePropertyDefinition::<Identity, Impl, OFFSET>,
            GetPropertyDefinition: GetPropertyDefinition::<Identity, Impl, OFFSET>,
            EnumRules: EnumRules::<Identity, Impl, OFFSET>,
            CreateRule: CreateRule::<Identity, Impl, OFFSET>,
            GetRule: GetRule::<Identity, Impl, OFFSET>,
            EnumModuleDefinitions: EnumModuleDefinitions::<Identity, Impl, OFFSET>,
            CreateModuleDefinition: CreateModuleDefinition::<Identity, Impl, OFFSET>,
            GetModuleDefinition: GetModuleDefinition::<Identity, Impl, OFFSET>,
            RunClassification: RunClassification::<Identity, Impl, OFFSET>,
            WaitForClassificationCompletion: WaitForClassificationCompletion::<Identity, Impl, OFFSET>,
            CancelClassification: CancelClassification::<Identity, Impl, OFFSET>,
            EnumFileProperties: EnumFileProperties::<Identity, Impl, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, Impl, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, Impl, OFFSET>,
            ClearFileProperty: ClearFileProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationManager2_Impl: Sized + IFsrmClassificationManager_Impl {
    fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmClassificationManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager2_Impl, const OFFSET: isize>() -> IFsrmClassificationManager2_Vtbl {
        unsafe extern "system" fn ClassifyFiles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassifyFiles(::core::mem::transmute_copy(&filepaths), ::core::mem::transmute_copy(&propertynames), ::core::mem::transmute_copy(&propertyvalues), ::core::mem::transmute_copy(&options)).into()
        }
        Self { base__: IFsrmClassificationManager_Vtbl::new::<Identity, Impl, OFFSET>(), ClassifyFiles: ClassifyFiles::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationManager2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmClassificationManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassificationRule_Impl: Sized + IFsrmRule_Impl {
    fn ExecutionOption(&self) -> ::windows::core::Result<FsrmExecutionOption>;
    fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows::core::Result<()>;
    fn PropertyAffected(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPropertyAffected(&self, property: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetValue(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmClassificationRule {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassificationRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>() -> IFsrmClassificationRule_Vtbl {
        unsafe extern "system" fn ExecutionOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecutionOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(executionoption, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExecutionOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExecutionOption(::core::mem::transmute_copy(&executionoption)).into()
        }
        unsafe extern "system" fn PropertyAffected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyAffected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyAffected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyAffected(::core::mem::transmute(&property)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: IFsrmRule_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExecutionOption: ExecutionOption::<Identity, Impl, OFFSET>,
            SetExecutionOption: SetExecutionOption::<Identity, Impl, OFFSET>,
            PropertyAffected: PropertyAffected::<Identity, Impl, OFFSET>,
            SetPropertyAffected: SetPropertyAffected::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmRule as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleDefinition_Impl: Sized + IFsrmPipelineModuleDefinition_Impl {
    fn PropertiesAffected(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PropertiesUsed(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn NeedsExplicitValue(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsExplicitValue(&self, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmClassifierModuleDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>() -> IFsrmClassifierModuleDefinition_Vtbl {
        unsafe extern "system" fn PropertiesAffected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertiesAffected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertiesaffected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertiesAffected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertiesAffected(::core::mem::transmute_copy(&propertiesaffected)).into()
        }
        unsafe extern "system" fn PropertiesUsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertiesUsed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertiesused, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertiesUsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertiesUsed(::core::mem::transmute_copy(&propertiesused)).into()
        }
        unsafe extern "system" fn NeedsExplicitValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NeedsExplicitValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(needsexplicitvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNeedsExplicitValue(::core::mem::transmute_copy(&needsexplicitvalue)).into()
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertiesAffected: PropertiesAffected::<Identity, Impl, OFFSET>,
            SetPropertiesAffected: SetPropertiesAffected::<Identity, Impl, OFFSET>,
            PropertiesUsed: PropertiesUsed::<Identity, Impl, OFFSET>,
            SetPropertiesUsed: SetPropertiesUsed::<Identity, Impl, OFFSET>,
            NeedsExplicitValue: NeedsExplicitValue::<Identity, Impl, OFFSET>,
            SetNeedsExplicitValue: SetNeedsExplicitValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmPipelineModuleDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmClassifierModuleImplementation_Impl: Sized + IFsrmPipelineModuleImplementation_Impl {
    fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UseRulesAndDefinitions(&self, rules: ::core::option::Option<&IFsrmCollection>, propertydefinitions: ::core::option::Option<&IFsrmCollection>) -> ::windows::core::Result<()>;
    fn OnBeginFile(&self, propertybag: ::core::option::Option<&IFsrmPropertyBag>, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn DoesPropertyValueApply(&self, property: &::windows::core::BSTR, value: &::windows::core::BSTR, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: &::windows::core::GUID, idpropdef: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetPropertyValueToApply(&self, property: &::windows::core::BSTR, value: *mut ::windows::core::BSTR, idrule: &::windows::core::GUID, idpropdef: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnEndFile(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmClassifierModuleImplementation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmClassifierModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>() -> IFsrmClassifierModuleImplementation_Vtbl {
        unsafe extern "system" fn LastModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseRulesAndDefinitions(::windows::core::from_raw_borrowed(&rules), ::windows::core::from_raw_borrowed(&propertydefinitions)).into()
        }
        unsafe extern "system" fn OnBeginFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBeginFile(::windows::core::from_raw_borrowed(&propertybag), ::core::mem::transmute_copy(&arrayruleids)).into()
        }
        unsafe extern "system" fn DoesPropertyValueApply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: ::std::mem::MaybeUninit<::windows::core::BSTR>, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoesPropertyValueApply(::core::mem::transmute(&property), ::core::mem::transmute(&value), ::core::mem::transmute_copy(&applyvalue), ::core::mem::transmute(&idrule), ::core::mem::transmute(&idpropdef)).into()
        }
        unsafe extern "system" fn GetPropertyValueToApply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyValueToApply(::core::mem::transmute(&property), ::core::mem::transmute_copy(&value), ::core::mem::transmute(&idrule), ::core::mem::transmute(&idpropdef)).into()
        }
        unsafe extern "system" fn OnEndFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndFile().into()
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, Impl, OFFSET>(),
            LastModified: LastModified::<Identity, Impl, OFFSET>,
            UseRulesAndDefinitions: UseRulesAndDefinitions::<Identity, Impl, OFFSET>,
            OnBeginFile: OnBeginFile::<Identity, Impl, OFFSET>,
            DoesPropertyValueApply: DoesPropertyValueApply::<Identity, Impl, OFFSET>,
            GetPropertyValueToApply: GetPropertyValueToApply::<Identity, Impl, OFFSET>,
            OnEndFile: OnEndFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleImplementation as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmPipelineModuleImplementation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<FsrmCollectionState>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetById(&self, id: &::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>() -> IFsrmCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unknown, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetById(::core::mem::transmute(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            GetById: GetById::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmCommittableCollection_Impl: Sized + IFsrmMutableCollection_Impl {
    fn Commit(&self, options: FsrmCommitOptions) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmCommittableCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmCommittableCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCommittableCollection_Impl, const OFFSET: isize>() -> IFsrmCommittableCollection_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmCommittableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Commit(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IFsrmMutableCollection_Vtbl::new::<Identity, Impl, OFFSET>(), Commit: Commit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmCommittableCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmCollection as ::windows::core::ComInterface>::IID || iid == &<IFsrmMutableCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmDerivedObjectsResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DerivedObjects(&self) -> ::windows::core::Result<IFsrmCollection>;
    fn Results(&self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmDerivedObjectsResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmDerivedObjectsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>() -> IFsrmDerivedObjectsResult_Vtbl {
        unsafe extern "system" fn DerivedObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, derivedobjects: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DerivedObjects() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjects, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Results<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmDerivedObjectsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, results: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Results() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DerivedObjects: DerivedObjects::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmDerivedObjectsResult as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmExportImport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExportFileGroups(&self, filepath: &::windows::core::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImportFileGroups(&self, filepath: &::windows::core::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportFileScreenTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImportFileScreenTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportQuotaTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ImportQuotaTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmExportImport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmExportImport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>() -> IFsrmExportImport_Vtbl {
        unsafe extern "system" fn ExportFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportFileGroups(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute(&remotehost)).into()
        }
        unsafe extern "system" fn ImportFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportFileGroups(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&filegroupnamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportFileScreenTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportFileScreenTemplates(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)).into()
        }
        unsafe extern "system" fn ImportFileScreenTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportFileScreenTemplates(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(templates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportQuotaTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportQuotaTemplates(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)).into()
        }
        unsafe extern "system" fn ImportQuotaTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmExportImport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::std::mem::MaybeUninit<::windows::core::BSTR>, templates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportQuotaTemplates(::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&templatenamessafearray), ::core::mem::transmute(&remotehost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(templates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExportFileGroups: ExportFileGroups::<Identity, Impl, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, Impl, OFFSET>,
            ExportFileScreenTemplates: ExportFileScreenTemplates::<Identity, Impl, OFFSET>,
            ImportFileScreenTemplates: ImportFileScreenTemplates::<Identity, Impl, OFFSET>,
            ExportQuotaTemplates: ExportQuotaTemplates::<Identity, Impl, OFFSET>,
            ImportQuotaTemplates: ImportQuotaTemplates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmExportImport as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileCondition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileCondition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: isize>() -> IFsrmFileCondition_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileCondition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileConditionProperty_Impl: Sized + IFsrmFileCondition_Impl {
    fn PropertyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPropertyName(&self, newval: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PropertyId(&self) -> ::windows::core::Result<FsrmFileSystemPropertyId>;
    fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows::core::Result<()>;
    fn Operator(&self) -> ::windows::core::Result<FsrmPropertyConditionType>;
    fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows::core::Result<()>;
    fn ValueType(&self) -> ::windows::core::Result<FsrmPropertyValueType>;
    fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetValue(&self, newval: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileConditionProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileConditionProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>() -> IFsrmFileConditionProperty_Vtbl {
        unsafe extern "system" fn PropertyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyName(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn PropertyId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyId(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Operator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperator(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ValueType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueType(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileConditionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&newval)).into()
        }
        Self {
            base__: IFsrmFileCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyName: PropertyName::<Identity, Impl, OFFSET>,
            SetPropertyName: SetPropertyName::<Identity, Impl, OFFSET>,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            SetPropertyId: SetPropertyId::<Identity, Impl, OFFSET>,
            Operator: Operator::<Identity, Impl, OFFSET>,
            SetOperator: SetOperator::<Identity, Impl, OFFSET>,
            ValueType: ValueType::<Identity, Impl, OFFSET>,
            SetValueType: SetValueType::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileConditionProperty as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileCondition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroup_Impl: Sized + IFsrmObject_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetMembers(&self, members: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows::core::Result<()>;
    fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetNonMembers(&self, nonmembers: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>() -> IFsrmFileGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(members, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMembers(::windows::core::from_raw_borrowed(&members)).into()
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonmembers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonmembers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNonMembers(::windows::core::from_raw_borrowed(&nonmembers)).into()
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            SetMembers: SetMembers::<Identity, Impl, OFFSET>,
            NonMembers: NonMembers::<Identity, Impl, OFFSET>,
            SetNonMembers: SetNonMembers::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroup as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupImported_Impl: Sized + IFsrmFileGroup_Impl {
    fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileGroupImported {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: isize>() -> IFsrmFileGroupImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base__: IFsrmFileGroup_Vtbl::new::<Identity, Impl, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupImported as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileGroup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileGroupManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateFileGroup(&self) -> ::windows::core::Result<IFsrmFileGroup>;
    fn GetFileGroup(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileGroup>;
    fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ImportFileGroups(&self, serializedfilegroups: &::windows::core::BSTR, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileGroupManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileGroupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>() -> IFsrmFileGroupManager_Vtbl {
        unsafe extern "system" fn CreateFileGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileGroup(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFileGroups(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExportFileGroups(::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedfilegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilegroups: ::std::mem::MaybeUninit<::windows::core::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportFileGroups(::core::mem::transmute(&serializedfilegroups), ::core::mem::transmute_copy(&filegroupnamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filegroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateFileGroup: CreateFileGroup::<Identity, Impl, OFFSET>,
            GetFileGroup: GetFileGroup::<Identity, Impl, OFFSET>,
            EnumFileGroups: EnumFileGroups::<Identity, Impl, OFFSET>,
            ExportFileGroups: ExportFileGroups::<Identity, Impl, OFFSET>,
            ImportFileGroups: ImportFileGroups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileGroupManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJob_Impl: Sized + IFsrmObject_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OperationType(&self) -> ::windows::core::Result<FsrmFileManagementType>;
    fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows::core::Result<()>;
    fn ExpirationDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetExpirationDirectory(&self, expirationdirectory: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand>;
    fn Notifications(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Logging(&self) -> ::windows::core::Result<i32>;
    fn SetLogging(&self, loggingflags: i32) -> ::windows::core::Result<()>;
    fn ReportEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetReportEnabled(&self, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DaysSinceFileCreated(&self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows::core::Result<()>;
    fn DaysSinceFileLastAccessed(&self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows::core::Result<()>;
    fn DaysSinceFileLastModified(&self) -> ::windows::core::Result<i32>;
    fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows::core::Result<()>;
    fn PropertyConditions(&self) -> ::windows::core::Result<IFsrmCollection>;
    fn FromDate(&self) -> ::windows::core::Result<f64>;
    fn SetFromDate(&self, fromdate: f64) -> ::windows::core::Result<()>;
    fn Task(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn LastError(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn LastReportPathWithoutExtension(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn LastRun(&self) -> ::windows::core::Result<f64>;
    fn FileNamePattern(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFileNamePattern(&self, filenamepattern: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn AddNotification(&self, days: i32) -> ::windows::core::Result<()>;
    fn DeleteNotification(&self, days: i32) -> ::windows::core::Result<()>;
    fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows::core::Result<()>;
    fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumNotificationActions(&self, days: i32) -> ::windows::core::Result<IFsrmCollection>;
    fn CreatePropertyCondition(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmPropertyCondition>;
    fn CreateCustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileManagementJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>() -> IFsrmFileManagementJob_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn OperationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OperationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(operationtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperationType(::core::mem::transmute_copy(&operationtype)).into()
        }
        unsafe extern "system" fn ExpirationDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpirationDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expirationdirectory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdirectory: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExpirationDirectory(::core::mem::transmute(&expirationdirectory)).into()
        }
        unsafe extern "system" fn CustomAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CustomAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notifications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Notifications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notifications, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Logging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(loggingflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogging(::core::mem::transmute_copy(&loggingflags)).into()
        }
        unsafe extern "system" fn ReportEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReportEnabled(::core::mem::transmute_copy(&reportenabled)).into()
        }
        unsafe extern "system" fn Formats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Formats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn MailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailTo(::core::mem::transmute(&mailto)).into()
        }
        unsafe extern "system" fn DaysSinceFileCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DaysSinceFileCreated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssincecreation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysSinceFileCreated(::core::mem::transmute_copy(&dayssincecreation)).into()
        }
        unsafe extern "system" fn DaysSinceFileLastAccessed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DaysSinceFileLastAccessed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssinceaccess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastAccessed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysSinceFileLastAccessed(::core::mem::transmute_copy(&dayssinceaccess)).into()
        }
        unsafe extern "system" fn DaysSinceFileLastModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DaysSinceFileLastModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dayssincemodify, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysSinceFileLastModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysSinceFileLastModified(::core::mem::transmute_copy(&dayssincemodify)).into()
        }
        unsafe extern "system" fn PropertyConditions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyconditions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyConditions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyconditions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FromDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fromdate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFromDate(::core::mem::transmute_copy(&fromdate)).into()
        }
        unsafe extern "system" fn Task<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Task() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(taskname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTask(::core::mem::transmute(&taskname)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn RunningStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReportPathWithoutExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastReportPathWithoutExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastRun() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileNamePattern<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileNamePattern() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filenamepattern, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNamePattern<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filenamepattern: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileNamePattern(::core::mem::transmute(&filenamepattern)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn AddNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNotification(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn DeleteNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteNotification(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn ModifyNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyNotification(::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&newdays)).into()
        }
        unsafe extern "system" fn CreateNotificationAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNotificationAction(::core::mem::transmute_copy(&days), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNotificationActions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNotificationActions(::core::mem::transmute_copy(&days)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertycondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyCondition(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertycondition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCustomAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            OperationType: OperationType::<Identity, Impl, OFFSET>,
            SetOperationType: SetOperationType::<Identity, Impl, OFFSET>,
            ExpirationDirectory: ExpirationDirectory::<Identity, Impl, OFFSET>,
            SetExpirationDirectory: SetExpirationDirectory::<Identity, Impl, OFFSET>,
            CustomAction: CustomAction::<Identity, Impl, OFFSET>,
            Notifications: Notifications::<Identity, Impl, OFFSET>,
            Logging: Logging::<Identity, Impl, OFFSET>,
            SetLogging: SetLogging::<Identity, Impl, OFFSET>,
            ReportEnabled: ReportEnabled::<Identity, Impl, OFFSET>,
            SetReportEnabled: SetReportEnabled::<Identity, Impl, OFFSET>,
            Formats: Formats::<Identity, Impl, OFFSET>,
            SetFormats: SetFormats::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            DaysSinceFileCreated: DaysSinceFileCreated::<Identity, Impl, OFFSET>,
            SetDaysSinceFileCreated: SetDaysSinceFileCreated::<Identity, Impl, OFFSET>,
            DaysSinceFileLastAccessed: DaysSinceFileLastAccessed::<Identity, Impl, OFFSET>,
            SetDaysSinceFileLastAccessed: SetDaysSinceFileLastAccessed::<Identity, Impl, OFFSET>,
            DaysSinceFileLastModified: DaysSinceFileLastModified::<Identity, Impl, OFFSET>,
            SetDaysSinceFileLastModified: SetDaysSinceFileLastModified::<Identity, Impl, OFFSET>,
            PropertyConditions: PropertyConditions::<Identity, Impl, OFFSET>,
            FromDate: FromDate::<Identity, Impl, OFFSET>,
            SetFromDate: SetFromDate::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            RunningStatus: RunningStatus::<Identity, Impl, OFFSET>,
            LastError: LastError::<Identity, Impl, OFFSET>,
            LastReportPathWithoutExtension: LastReportPathWithoutExtension::<Identity, Impl, OFFSET>,
            LastRun: LastRun::<Identity, Impl, OFFSET>,
            FileNamePattern: FileNamePattern::<Identity, Impl, OFFSET>,
            SetFileNamePattern: SetFileNamePattern::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            AddNotification: AddNotification::<Identity, Impl, OFFSET>,
            DeleteNotification: DeleteNotification::<Identity, Impl, OFFSET>,
            ModifyNotification: ModifyNotification::<Identity, Impl, OFFSET>,
            CreateNotificationAction: CreateNotificationAction::<Identity, Impl, OFFSET>,
            EnumNotificationActions: EnumNotificationActions::<Identity, Impl, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, Impl, OFFSET>,
            CreateCustomAction: CreateCustomAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJob as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileManagementJobManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateFileManagementJob(&self) -> ::windows::core::Result<IFsrmFileManagementJob>;
    fn GetFileManagementJob(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileManagementJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileManagementJobManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileManagementJobManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>() -> IFsrmFileManagementJobManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileManagementJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFileManagementJobs(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileManagementJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileManagementJob() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileManagementJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileManagementJobManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileManagementJob(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filemanagementjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            EnumFileManagementJobs: EnumFileManagementJobs::<Identity, Impl, OFFSET>,
            CreateFileManagementJob: CreateFileManagementJob::<Identity, Impl, OFFSET>,
            GetFileManagementJob: GetFileManagementJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileManagementJobManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreen_Impl: Sized + IFsrmFileScreenBase_Impl {
    fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ApplyTemplate(&self, filescreentemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreen {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreen_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>() -> IFsrmFileScreen_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceTemplateName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceTemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplatename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesSourceTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserSid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreen_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyTemplate(::core::mem::transmute(&filescreentemplatename)).into()
        }
        Self {
            base__: IFsrmFileScreenBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, Impl, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, Impl, OFFSET>,
            UserSid: UserSid::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreen as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileScreenBase as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenBase_Impl: Sized + IFsrmObject_Impl {
    fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetBlockedFileGroups(&self, blocklist: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows::core::Result<()>;
    fn FileScreenFlags(&self) -> ::windows::core::Result<i32>;
    fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()>;
    fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenBase {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>() -> IFsrmFileScreenBase_Vtbl {
        unsafe extern "system" fn BlockedFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockedFileGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocklist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockedFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocklist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBlockedFileGroups(::windows::core::from_raw_borrowed(&blocklist)).into()
        }
        unsafe extern "system" fn FileScreenFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileScreenFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileScreenFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileScreenFlags(::core::mem::transmute_copy(&filescreenflags)).into()
        }
        unsafe extern "system" fn CreateAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAction(::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumActions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumActions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            BlockedFileGroups: BlockedFileGroups::<Identity, Impl, OFFSET>,
            SetBlockedFileGroups: SetBlockedFileGroups::<Identity, Impl, OFFSET>,
            FileScreenFlags: FileScreenFlags::<Identity, Impl, OFFSET>,
            SetFileScreenFlags: SetFileScreenFlags::<Identity, Impl, OFFSET>,
            CreateAction: CreateAction::<Identity, Impl, OFFSET>,
            EnumActions: EnumActions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenBase as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenException_Impl: Sized + IFsrmObject_Impl {
    fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AllowedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection>;
    fn SetAllowedFileGroups(&self, allowlist: ::core::option::Option<&IFsrmMutableCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenException {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>() -> IFsrmFileScreenException_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowedFileGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allowlist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedFileGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowlist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowedFileGroups(::windows::core::from_raw_borrowed(&allowlist)).into()
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            AllowedFileGroups: AllowedFileGroups::<Identity, Impl, OFFSET>,
            SetAllowedFileGroups: SetAllowedFileGroups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenException as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateFileScreen(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreen>;
    fn GetFileScreen(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreen>;
    fn EnumFileScreens(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenException(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenException>;
    fn GetFileScreenException(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenException>;
    fn EnumFileScreenExceptions(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn CreateFileScreenCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>() -> IFsrmFileScreenManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileScreen(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileScreen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileScreen(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileScreens<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, filescreens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFileScreens(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreens, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreenException<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileScreenException(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexception, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileScreenException<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileScreenException(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexception, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFileScreenExceptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFileScreenExceptions(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreenexceptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileScreenCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileScreenCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            CreateFileScreen: CreateFileScreen::<Identity, Impl, OFFSET>,
            GetFileScreen: GetFileScreen::<Identity, Impl, OFFSET>,
            EnumFileScreens: EnumFileScreens::<Identity, Impl, OFFSET>,
            CreateFileScreenException: CreateFileScreenException::<Identity, Impl, OFFSET>,
            GetFileScreenException: GetFileScreenException::<Identity, Impl, OFFSET>,
            EnumFileScreenExceptions: EnumFileScreenExceptions::<Identity, Impl, OFFSET>,
            CreateFileScreenCollection: CreateFileScreenCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplate_Impl: Sized + IFsrmFileScreenBase_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CopyTemplate(&self, filescreentemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenTemplate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>() -> IFsrmFileScreenTemplate_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn CopyTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTemplate(::core::mem::transmute(&filescreentemplatename)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmFileScreenBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplate as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileScreenBase as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateImported_Impl: Sized + IFsrmFileScreenTemplate_Impl {
    fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenTemplateImported {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>() -> IFsrmFileScreenTemplateImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base__: IFsrmFileScreenTemplate_Vtbl::new::<Identity, Impl, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateImported as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileScreenBase as ::windows::core::ComInterface>::IID || iid == &<IFsrmFileScreenTemplate as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmFileScreenTemplateManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmFileScreenTemplate>;
    fn GetTemplate(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenTemplate>;
    fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ImportTemplates(&self, serializedfilescreentemplates: &::windows::core::BSTR, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmFileScreenTemplateManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmFileScreenTemplateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>() -> IFsrmFileScreenTemplateManager_Vtbl {
        unsafe extern "system" fn CreateTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTemplate(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumTemplates(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExportTemplates(::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedfilescreentemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmFileScreenTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::std::mem::MaybeUninit<::windows::core::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportTemplates(::core::mem::transmute(&serializedfilescreentemplates), ::core::mem::transmute_copy(&filescreentemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filescreentemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTemplate: CreateTemplate::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, Impl, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, Impl, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmFileScreenTemplateManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmMutableCollection_Impl: Sized + IFsrmCollection_Impl {
    fn Add(&self, item: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&self, index: i32) -> ::windows::core::Result<()>;
    fn RemoveById(&self, id: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmMutableCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmMutableCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>() -> IFsrmMutableCollection_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn RemoveById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveById(::core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmMutableCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmCollection_Vtbl::new::<Identity, Impl, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveById: RemoveById::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmMutableCollection as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmCollection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>() -> IFsrmObject_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPathMapper_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetSharePathsForLocalPath(&self, localpath: &::windows::core::BSTR) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPathMapper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPathMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPathMapper_Impl, const OFFSET: isize>() -> IFsrmPathMapper_Vtbl {
        unsafe extern "system" fn GetSharePathsForLocalPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPathMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSharePathsForLocalPath(::core::mem::transmute(&localpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharepaths, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSharePathsForLocalPath: GetSharePathsForLocalPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPathMapper as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleConnector_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ModuleImplementation(&self) -> ::windows::core::Result<IFsrmPipelineModuleImplementation>;
    fn ModuleName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn HostingUserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn HostingProcessPid(&self) -> ::windows::core::Result<i32>;
    fn Bind(&self, moduledefinition: ::core::option::Option<&IFsrmPipelineModuleDefinition>, moduleimplementation: ::core::option::Option<&IFsrmPipelineModuleImplementation>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPipelineModuleConnector {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>() -> IFsrmPipelineModuleConnector_Vtbl {
        unsafe extern "system" fn ModuleImplementation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleImplementation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pipelinemoduleimplementation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(username, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostingUserAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostingUserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostingProcessPid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HostingProcessPid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleimplementation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bind(::windows::core::from_raw_borrowed(&moduledefinition), ::windows::core::from_raw_borrowed(&moduleimplementation)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ModuleImplementation: ModuleImplementation::<Identity, Impl, OFFSET>,
            ModuleName: ModuleName::<Identity, Impl, OFFSET>,
            HostingUserAccount: HostingUserAccount::<Identity, Impl, OFFSET>,
            HostingProcessPid: HostingProcessPid::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleDefinition_Impl: Sized + IFsrmObject_Impl {
    fn ModuleClsid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetModuleClsid(&self, moduleclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCompany(&self, company: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetVersion(&self, version: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType>;
    fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn NeedsFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNeedsFileContent(&self, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Account(&self) -> ::windows::core::Result<FsrmAccountType>;
    fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()>;
    fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPipelineModuleDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>() -> IFsrmPipelineModuleDefinition_Vtbl {
        unsafe extern "system" fn ModuleClsid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleClsid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduleclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModuleClsid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduleclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModuleClsid(::core::mem::transmute(&moduleclsid)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Company<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Company() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(company, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, company: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompany(::core::mem::transmute(&company)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn ModuleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn NeedsFileContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NeedsFileContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(needsfilecontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeedsFileContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNeedsFileContent(::core::mem::transmute_copy(&needsfilecontent)).into()
        }
        unsafe extern "system" fn Account<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Account() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retrievalaccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccount(::core::mem::transmute_copy(&retrievalaccount)).into()
        }
        unsafe extern "system" fn SupportedExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedextensions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportedExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSupportedExtensions(::core::mem::transmute_copy(&supportedextensions)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            ModuleClsid: ModuleClsid::<Identity, Impl, OFFSET>,
            SetModuleClsid: SetModuleClsid::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            SetCompany: SetCompany::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            ModuleType: ModuleType::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            NeedsFileContent: NeedsFileContent::<Identity, Impl, OFFSET>,
            SetNeedsFileContent: SetNeedsFileContent::<Identity, Impl, OFFSET>,
            Account: Account::<Identity, Impl, OFFSET>,
            SetAccount: SetAccount::<Identity, Impl, OFFSET>,
            SupportedExtensions: SupportedExtensions::<Identity, Impl, OFFSET>,
            SetSupportedExtensions: SetSupportedExtensions::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPipelineModuleImplementation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnLoad(&self, moduledefinition: ::core::option::Option<&IFsrmPipelineModuleDefinition>) -> ::windows::core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPipelineModuleImplementation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPipelineModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>() -> IFsrmPipelineModuleImplementation_Vtbl {
        unsafe extern "system" fn OnLoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnLoad(::windows::core::from_raw_borrowed(&moduledefinition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduleconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnUnload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUnload().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnLoad: OnLoad::<Identity, Impl, OFFSET>,
            OnUnload: OnUnload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleImplementation as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Sources(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyFlags(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: isize>() -> IFsrmProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sources() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sources, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            Sources: Sources::<Identity, Impl, OFFSET>,
            PropertyFlags: PropertyFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmProperty as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBag_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RelativePath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn VolumeName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn VolumeIndex(&self) -> ::windows::core::Result<u32>;
    fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Attributes(&self) -> ::windows::core::Result<u32>;
    fn OwnerSid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn PropertyBagFlags(&self) -> ::windows::core::Result<u32>;
    fn GetFileProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, name: &::windows::core::BSTR, value: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddMessage(&self, message: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyBag {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBag_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>() -> IFsrmPropertyBag_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RelativePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volumename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RelativeNamespaceRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(relativenamespaceroot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volumeid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentDirectoryId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentDirectoryId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parentdirectoryid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeAllocated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeAllocated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sizeallocated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(creationtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastAccessTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastAccessTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastaccesstime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastModificationTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastModificationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodificationtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ownersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerSid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ownersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilePropertyNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FilePropertyNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepropertynames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Messages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Messages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyBagFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyBagFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, fileproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileProperty(::core::mem::transmute(&name), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMessage(::core::mem::transmute(&message)).into()
        }
        unsafe extern "system" fn GetFileStreamInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileStreamInterface(::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&interfacetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstreaminterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            RelativePath: RelativePath::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            RelativeNamespaceRoot: RelativeNamespaceRoot::<Identity, Impl, OFFSET>,
            VolumeIndex: VolumeIndex::<Identity, Impl, OFFSET>,
            FileId: FileId::<Identity, Impl, OFFSET>,
            ParentDirectoryId: ParentDirectoryId::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SizeAllocated: SizeAllocated::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            LastAccessTime: LastAccessTime::<Identity, Impl, OFFSET>,
            LastModificationTime: LastModificationTime::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            OwnerSid: OwnerSid::<Identity, Impl, OFFSET>,
            FilePropertyNames: FilePropertyNames::<Identity, Impl, OFFSET>,
            Messages: Messages::<Identity, Impl, OFFSET>,
            PropertyBagFlags: PropertyBagFlags::<Identity, Impl, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, Impl, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            GetFileStreamInterface: GetFileStreamInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyBag2_Impl: Sized + IFsrmPropertyBag_Impl {
    fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetUntrustedInFileProperties(&self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyBag2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyBag2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: isize>() -> IFsrmPropertyBag2_Vtbl {
        unsafe extern "system" fn GetFieldValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFieldValue(::core::mem::transmute_copy(&field)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUntrustedInFileProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(props, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmPropertyBag_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFieldValue: GetFieldValue::<Identity, Impl, OFFSET>,
            GetUntrustedInFileProperties: GetUntrustedInFileProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyBag2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmPropertyBag as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyCondition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<FsrmPropertyConditionType>;
    fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetValue(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyCondition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>() -> IFsrmPropertyCondition_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyCondition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinition_Impl: Sized + IFsrmObject_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType>;
    fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()>;
    fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>() -> IFsrmPropertyDefinition_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn PossibleValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PossibleValues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(possiblevalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPossibleValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPossibleValues(::core::mem::transmute_copy(&possiblevalues)).into()
        }
        unsafe extern "system" fn ValueDescriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuedescriptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueDescriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueDescriptions(::core::mem::transmute_copy(&valuedescriptions)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            PossibleValues: PossibleValues::<Identity, Impl, OFFSET>,
            SetPossibleValues: SetPossibleValues::<Identity, Impl, OFFSET>,
            ValueDescriptions: ValueDescriptions::<Identity, Impl, OFFSET>,
            SetValueDescriptions: SetValueDescriptions::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinition2_Impl: Sized + IFsrmPropertyDefinition_Impl {
    fn PropertyDefinitionFlags(&self) -> ::windows::core::Result<i32>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDisplayName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AppliesTo(&self) -> ::windows::core::Result<i32>;
    fn ValueDefinitions(&self) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyDefinition2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinition2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>() -> IFsrmPropertyDefinition2_Vtbl {
        unsafe extern "system" fn PropertyDefinitionFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyDefinitionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertydefinitionflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn AppliesTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppliesTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(appliesto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuedefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuedefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmPropertyDefinition_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyDefinitionFlags: PropertyDefinitionFlags::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            AppliesTo: AppliesTo::<Identity, Impl, OFFSET>,
            ValueDefinitions: ValueDefinitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmPropertyDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmPropertyDefinitionValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UniqueID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmPropertyDefinitionValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmPropertyDefinitionValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>() -> IFsrmPropertyDefinitionValue_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniqueid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UniqueID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uniqueid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            UniqueID: UniqueID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuota_Impl: Sized + IFsrmQuotaObject_Impl {
    fn QuotaUsed(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn QuotaPeakUsage(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn QuotaPeakUsageTime(&self) -> ::windows::core::Result<f64>;
    fn ResetPeakUsage(&self) -> ::windows::core::Result<()>;
    fn RefreshUsageProperties(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuota {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuota_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>() -> IFsrmQuota_Vtbl {
        unsafe extern "system" fn QuotaUsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuotaUsed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(used, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuotaPeakUsage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuotaPeakUsage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peakusage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuotaPeakUsageTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuotaPeakUsageTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peakusagedatetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetPeakUsage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetPeakUsage().into()
        }
        unsafe extern "system" fn RefreshUsageProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuota_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshUsageProperties().into()
        }
        Self {
            base__: IFsrmQuotaObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            QuotaUsed: QuotaUsed::<Identity, Impl, OFFSET>,
            QuotaPeakUsage: QuotaPeakUsage::<Identity, Impl, OFFSET>,
            QuotaPeakUsageTime: QuotaPeakUsageTime::<Identity, Impl, OFFSET>,
            ResetPeakUsage: ResetPeakUsage::<Identity, Impl, OFFSET>,
            RefreshUsageProperties: RefreshUsageProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuota as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaBase_Impl: Sized + IFsrmObject_Impl {
    fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetQuotaLimit(&self, quotalimit: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn QuotaFlags(&self) -> ::windows::core::Result<i32>;
    fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()>;
    fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()>;
    fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()>;
    fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()>;
    fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction>;
    fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaBase {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>() -> IFsrmQuotaBase_Vtbl {
        unsafe extern "system" fn QuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuotaLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotalimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaLimit(::core::mem::transmute(&quotalimit)).into()
        }
        unsafe extern "system" fn QuotaFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuotaFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotaflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuotaFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaFlags(::core::mem::transmute_copy(&quotaflags)).into()
        }
        unsafe extern "system" fn Thresholds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Thresholds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thresholds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddThreshold(::core::mem::transmute_copy(&threshold)).into()
        }
        unsafe extern "system" fn DeleteThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteThreshold(::core::mem::transmute_copy(&threshold)).into()
        }
        unsafe extern "system" fn ModifyThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyThreshold(::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&newthreshold)).into()
        }
        unsafe extern "system" fn CreateThresholdAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateThresholdAction(::core::mem::transmute_copy(&threshold), ::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(action, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumThresholdActions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumThresholdActions(::core::mem::transmute_copy(&threshold)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            QuotaLimit: QuotaLimit::<Identity, Impl, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, Impl, OFFSET>,
            QuotaFlags: QuotaFlags::<Identity, Impl, OFFSET>,
            SetQuotaFlags: SetQuotaFlags::<Identity, Impl, OFFSET>,
            Thresholds: Thresholds::<Identity, Impl, OFFSET>,
            AddThreshold: AddThreshold::<Identity, Impl, OFFSET>,
            DeleteThreshold: DeleteThreshold::<Identity, Impl, OFFSET>,
            ModifyThreshold: ModifyThreshold::<Identity, Impl, OFFSET>,
            CreateThresholdAction: CreateThresholdAction::<Identity, Impl, OFFSET>,
            EnumThresholdActions: EnumThresholdActions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn CreateAutoApplyQuota(&self, quotatemplatename: &::windows::core::BSTR, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota>;
    fn GetQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn GetAutoApplyQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota>;
    fn GetRestrictiveQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota>;
    fn EnumQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn EnumAutoApplyQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn EnumEffectiveQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn Scan(&self, strpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>() -> IFsrmQuotaManager_Vtbl {
        unsafe extern "system" fn ActionVariables<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariables() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variables, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActionVariableDescriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActionVariableDescriptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(descriptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQuota(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAutoApplyQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows::core::BSTR>, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAutoApplyQuota(::core::mem::transmute(&quotatemplatename), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQuota(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutoApplyQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutoApplyQuota(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictiveQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictiveQuota(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumQuotas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumQuotas(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAutoApplyQuotas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAutoApplyQuotas(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEffectiveQuotas<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumEffectiveQuotas(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotas, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scan<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Scan(::core::mem::transmute(&strpath)).into()
        }
        unsafe extern "system" fn CreateQuotaCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateQuotaCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActionVariables: ActionVariables::<Identity, Impl, OFFSET>,
            ActionVariableDescriptions: ActionVariableDescriptions::<Identity, Impl, OFFSET>,
            CreateQuota: CreateQuota::<Identity, Impl, OFFSET>,
            CreateAutoApplyQuota: CreateAutoApplyQuota::<Identity, Impl, OFFSET>,
            GetQuota: GetQuota::<Identity, Impl, OFFSET>,
            GetAutoApplyQuota: GetAutoApplyQuota::<Identity, Impl, OFFSET>,
            GetRestrictiveQuota: GetRestrictiveQuota::<Identity, Impl, OFFSET>,
            EnumQuotas: EnumQuotas::<Identity, Impl, OFFSET>,
            EnumAutoApplyQuotas: EnumAutoApplyQuotas::<Identity, Impl, OFFSET>,
            EnumEffectiveQuotas: EnumEffectiveQuotas::<Identity, Impl, OFFSET>,
            Scan: Scan::<Identity, Impl, OFFSET>,
            CreateQuotaCollection: CreateQuotaCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaManagerEx_Impl: Sized + IFsrmQuotaManager_Impl {
    fn IsAffectedByQuota(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaManagerEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManagerEx_Impl, const OFFSET: isize>() -> IFsrmQuotaManagerEx_Vtbl {
        unsafe extern "system" fn IsAffectedByQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows::core::BSTR>, options: FsrmEnumOptions, affected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAffectedByQuota(::core::mem::transmute(&path), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(affected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IFsrmQuotaManager_Vtbl::new::<Identity, Impl, OFFSET>(), IsAffectedByQuota: IsAffectedByQuota::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaManagerEx as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaObject_Impl: Sized + IFsrmQuotaBase_Impl {
    fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ApplyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>() -> IFsrmQuotaObject_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserSid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useraccount: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(useraccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceTemplateName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceTemplateName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplatename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchesSourceTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesSourceTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyTemplate(::core::mem::transmute(&quotatemplatename)).into()
        }
        Self {
            base__: IFsrmQuotaBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            UserSid: UserSid::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            SourceTemplateName: SourceTemplateName::<Identity, Impl, OFFSET>,
            MatchesSourceTemplate: MatchesSourceTemplate::<Identity, Impl, OFFSET>,
            ApplyTemplate: ApplyTemplate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaObject as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplate_Impl: Sized + IFsrmQuotaBase_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CopyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaTemplate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>() -> IFsrmQuotaTemplate_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn CopyTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTemplate(::core::mem::transmute(&quotatemplatename)).into()
        }
        unsafe extern "system" fn CommitAndUpdateDerived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitAndUpdateDerived(::core::mem::transmute_copy(&commitoptions), ::core::mem::transmute_copy(&applyoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedobjectsresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmQuotaBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            CopyTemplate: CopyTemplate::<Identity, Impl, OFFSET>,
            CommitAndUpdateDerived: CommitAndUpdateDerived::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplate as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateImported_Impl: Sized + IFsrmQuotaTemplate_Impl {
    fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOverwriteOnCommit(&self, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaTemplateImported {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateImported_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>() -> IFsrmQuotaTemplateImported_Vtbl {
        unsafe extern "system" fn OverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverwriteOnCommit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overwrite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverwriteOnCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateImported_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverwriteOnCommit(::core::mem::transmute_copy(&overwrite)).into()
        }
        Self {
            base__: IFsrmQuotaTemplate_Vtbl::new::<Identity, Impl, OFFSET>(),
            OverwriteOnCommit: OverwriteOnCommit::<Identity, Impl, OFFSET>,
            SetOverwriteOnCommit: SetOverwriteOnCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateImported as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaBase as ::windows::core::ComInterface>::IID || iid == &<IFsrmQuotaTemplate as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmQuotaTemplateManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmQuotaTemplate>;
    fn GetTemplate(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuotaTemplate>;
    fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection>;
    fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ImportTemplates(&self, serializedquotatemplates: &::windows::core::BSTR, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmQuotaTemplateManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmQuotaTemplateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>() -> IFsrmQuotaTemplateManager_Vtbl {
        unsafe extern "system" fn CreateTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTemplate(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumTemplates(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExportTemplates(::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(serializedquotatemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmQuotaTemplateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::std::mem::MaybeUninit<::windows::core::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportTemplates(::core::mem::transmute(&serializedquotatemplates), ::core::mem::transmute_copy(&quotatemplatenamesarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(quotatemplates, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTemplate: CreateTemplate::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            EnumTemplates: EnumTemplates::<Identity, Impl, OFFSET>,
            ExportTemplates: ExportTemplates::<Identity, Impl, OFFSET>,
            ImportTemplates: ImportTemplates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmQuotaTemplateManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows::core::Result<FsrmReportType>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn LastGeneratedFileNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetFilter(&self, filter: FsrmReportFilter, filtervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>() -> IFsrmReport_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reporttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn LastGeneratedFileNamePrefix<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefix: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastGeneratedFileNamePrefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFilter(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filtervalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilter(::core::mem::transmute_copy(&filter), ::core::mem::transmute(&filtervalue)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            LastGeneratedFileNamePrefix: LastGeneratedFileNamePrefix::<Identity, Impl, OFFSET>,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReport as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportJob_Impl: Sized + IFsrmObject_Impl {
    fn Task(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus>;
    fn LastRun(&self) -> ::windows::core::Result<f64>;
    fn LastError(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn LastGeneratedInDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn EnumReports(&self) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows::core::Result<IFsrmReport>;
    fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()>;
    fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmReportJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>() -> IFsrmReportJob_Vtbl {
        unsafe extern "system" fn Task<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Task() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(taskname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTask(::core::mem::transmute(&taskname)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn Formats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Formats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormats(::core::mem::transmute_copy(&formats)).into()
        }
        unsafe extern "system" fn MailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailTo(::core::mem::transmute(&mailto)).into()
        }
        unsafe extern "system" fn RunningStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunningStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runningstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastRun() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lasterror: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lasterror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastGeneratedInDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastGeneratedInDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumReports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reports: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumReports() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reports, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(report, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run(::core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn WaitForCompletion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitForCompletion(::core::mem::transmute_copy(&waitseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(completed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            Formats: Formats::<Identity, Impl, OFFSET>,
            SetFormats: SetFormats::<Identity, Impl, OFFSET>,
            MailTo: MailTo::<Identity, Impl, OFFSET>,
            SetMailTo: SetMailTo::<Identity, Impl, OFFSET>,
            RunningStatus: RunningStatus::<Identity, Impl, OFFSET>,
            LastRun: LastRun::<Identity, Impl, OFFSET>,
            LastError: LastError::<Identity, Impl, OFFSET>,
            LastGeneratedInDirectory: LastGeneratedInDirectory::<Identity, Impl, OFFSET>,
            EnumReports: EnumReports::<Identity, Impl, OFFSET>,
            CreateReport: CreateReport::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            WaitForCompletion: WaitForCompletion::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportJob as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection>;
    fn CreateReportJob(&self) -> ::windows::core::Result<IFsrmReportJob>;
    fn GetReportJob(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmReportJob>;
    fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOutputDirectory(&self, context: FsrmReportGenerationContext, path: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetReportSizeLimit(&self, limit: FsrmReportLimit, limitvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmReportManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>() -> IFsrmReportManager_Vtbl {
        unsafe extern "system" fn EnumReportJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumReportJobs(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReportJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateReportJob() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReportJob(::core::mem::transmute(&taskname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reportjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputDirectory(::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputDirectory(::core::mem::transmute_copy(&context), ::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn IsFilterValidForReportType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFilterValidForReportType(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultFilter(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filtervalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultFilter(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&filter), ::core::mem::transmute(&filtervalue)).into()
        }
        unsafe extern "system" fn GetReportSizeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReportSizeLimit(::core::mem::transmute_copy(&limit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(limitvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportSizeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReportSizeLimit(::core::mem::transmute_copy(&limit), ::core::mem::transmute(&limitvalue)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumReportJobs: EnumReportJobs::<Identity, Impl, OFFSET>,
            CreateReportJob: CreateReportJob::<Identity, Impl, OFFSET>,
            GetReportJob: GetReportJob::<Identity, Impl, OFFSET>,
            GetOutputDirectory: GetOutputDirectory::<Identity, Impl, OFFSET>,
            SetOutputDirectory: SetOutputDirectory::<Identity, Impl, OFFSET>,
            IsFilterValidForReportType: IsFilterValidForReportType::<Identity, Impl, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, Impl, OFFSET>,
            SetDefaultFilter: SetDefaultFilter::<Identity, Impl, OFFSET>,
            GetReportSizeLimit: GetReportSizeLimit::<Identity, Impl, OFFSET>,
            SetReportSizeLimit: SetReportSizeLimit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportManager as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmReportScheduler_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CreateScheduleTask(&self, taskname: &::windows::core::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ModifyScheduleTask(&self, taskname: &::windows::core::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DeleteScheduleTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmReportScheduler {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmReportScheduler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>() -> IFsrmReportScheduler_Vtbl {
        unsafe extern "system" fn VerifyNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VerifyNamespaces(::core::mem::transmute_copy(&namespacessafearray)).into()
        }
        unsafe extern "system" fn CreateScheduleTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateScheduleTask(::core::mem::transmute(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute(&serializedtask)).into()
        }
        unsafe extern "system" fn ModifyScheduleTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyScheduleTask(::core::mem::transmute(&taskname), ::core::mem::transmute_copy(&namespacessafearray), ::core::mem::transmute(&serializedtask)).into()
        }
        unsafe extern "system" fn DeleteScheduleTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmReportScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScheduleTask(::core::mem::transmute(&taskname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            VerifyNamespaces: VerifyNamespaces::<Identity, Impl, OFFSET>,
            CreateScheduleTask: CreateScheduleTask::<Identity, Impl, OFFSET>,
            ModifyScheduleTask: ModifyScheduleTask::<Identity, Impl, OFFSET>,
            DeleteScheduleTask: DeleteScheduleTask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmReportScheduler as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmRule_Impl: Sized + IFsrmObject_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType>;
    fn ModuleDefinitionName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetModuleDefinitionName(&self, moduledefinitionname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RuleFlags(&self) -> ::windows::core::Result<i32>;
    fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()>;
    fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmRule {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>() -> IFsrmRule_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn RuleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RuleType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ruletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModuleDefinitionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModuleDefinitionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moduledefinitionname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModuleDefinitionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moduledefinitionname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModuleDefinitionName(::core::mem::transmute(&moduledefinitionname)).into()
        }
        unsafe extern "system" fn NamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NamespaceRoots() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceroots, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespaceRoots(::core::mem::transmute_copy(&namespaceroots)).into()
        }
        unsafe extern "system" fn RuleFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RuleFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ruleflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRuleFlags(::core::mem::transmute_copy(&ruleflags)).into()
        }
        unsafe extern "system" fn Parameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute_copy(&parameters)).into()
        }
        unsafe extern "system" fn LastModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFsrmObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            RuleType: RuleType::<Identity, Impl, OFFSET>,
            ModuleDefinitionName: ModuleDefinitionName::<Identity, Impl, OFFSET>,
            SetModuleDefinitionName: SetModuleDefinitionName::<Identity, Impl, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, Impl, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, Impl, OFFSET>,
            RuleFlags: RuleFlags::<Identity, Impl, OFFSET>,
            SetRuleFlags: SetRuleFlags::<Identity, Impl, OFFSET>,
            Parameters: Parameters::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            LastModified: LastModified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmRule as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmSetting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SmtpServer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSmtpServer(&self, smtpserver: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MailFrom(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMailFrom(&self, mailfrom: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AdminEmail(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetAdminEmail(&self, adminemail: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DisableCommandLine(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableCommandLine(&self, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn EnableScreeningAudit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableScreeningAudit(&self, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn EmailTest(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::Result<()>;
    fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmSetting {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmSetting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>() -> IFsrmSetting_Vtbl {
        unsafe extern "system" fn SmtpServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SmtpServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smtpserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmtpServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, smtpserver: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSmtpServer(::core::mem::transmute(&smtpserver)).into()
        }
        unsafe extern "system" fn MailFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MailFrom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mailfrom, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailfrom: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMailFrom(::core::mem::transmute(&mailfrom)).into()
        }
        unsafe extern "system" fn AdminEmail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminEmail() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(adminemail, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdminEmail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adminemail: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdminEmail(::core::mem::transmute(&adminemail)).into()
        }
        unsafe extern "system" fn DisableCommandLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisableCommandLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disablecommandline, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableCommandLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisableCommandLine(::core::mem::transmute_copy(&disablecommandline)).into()
        }
        unsafe extern "system" fn EnableScreeningAudit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnableScreeningAudit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enablescreeningaudit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableScreeningAudit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableScreeningAudit(::core::mem::transmute_copy(&enablescreeningaudit)).into()
        }
        unsafe extern "system" fn EmailTest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mailto: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EmailTest(::core::mem::transmute(&mailto)).into()
        }
        unsafe extern "system" fn SetActionRunLimitInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActionRunLimitInterval(::core::mem::transmute_copy(&actiontype), ::core::mem::transmute_copy(&delaytimeminutes)).into()
        }
        unsafe extern "system" fn GetActionRunLimitInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActionRunLimitInterval(::core::mem::transmute_copy(&actiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(delaytimeminutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SmtpServer: SmtpServer::<Identity, Impl, OFFSET>,
            SetSmtpServer: SetSmtpServer::<Identity, Impl, OFFSET>,
            MailFrom: MailFrom::<Identity, Impl, OFFSET>,
            SetMailFrom: SetMailFrom::<Identity, Impl, OFFSET>,
            AdminEmail: AdminEmail::<Identity, Impl, OFFSET>,
            SetAdminEmail: SetAdminEmail::<Identity, Impl, OFFSET>,
            DisableCommandLine: DisableCommandLine::<Identity, Impl, OFFSET>,
            SetDisableCommandLine: SetDisableCommandLine::<Identity, Impl, OFFSET>,
            EnableScreeningAudit: EnableScreeningAudit::<Identity, Impl, OFFSET>,
            SetEnableScreeningAudit: SetEnableScreeningAudit::<Identity, Impl, OFFSET>,
            EmailTest: EmailTest::<Identity, Impl, OFFSET>,
            SetActionRunLimitInterval: SetActionRunLimitInterval::<Identity, Impl, OFFSET>,
            GetActionRunLimitInterval: GetActionRunLimitInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmSetting as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleDefinition_Impl: Sized + IFsrmPipelineModuleDefinition_Impl {
    fn Capabilities(&self) -> ::windows::core::Result<FsrmStorageModuleCaps>;
    fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows::core::Result<()>;
    fn StorageType(&self) -> ::windows::core::Result<FsrmStorageModuleType>;
    fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows::core::Result<()>;
    fn UpdatesFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUpdatesFileContent(&self, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmStorageModuleDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>() -> IFsrmStorageModuleDefinition_Vtbl {
        unsafe extern "system" fn Capabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Capabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCapabilities(::core::mem::transmute_copy(&capabilities)).into()
        }
        unsafe extern "system" fn StorageType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StorageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storagetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStorageType(::core::mem::transmute_copy(&storagetype)).into()
        }
        unsafe extern "system" fn UpdatesFileContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UpdatesFileContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(updatesfilecontent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdatesFileContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdatesFileContent(::core::mem::transmute_copy(&updatesfilecontent)).into()
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, Impl, OFFSET>(),
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            SetCapabilities: SetCapabilities::<Identity, Impl, OFFSET>,
            StorageType: StorageType::<Identity, Impl, OFFSET>,
            SetStorageType: SetStorageType::<Identity, Impl, OFFSET>,
            UpdatesFileContent: UpdatesFileContent::<Identity, Impl, OFFSET>,
            SetUpdatesFileContent: SetUpdatesFileContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmObject as ::windows::core::ComInterface>::IID || iid == &<IFsrmPipelineModuleDefinition as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsrmStorageModuleImplementation_Impl: Sized + IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(&self, propertydefinitions: ::core::option::Option<&IFsrmCollection>) -> ::windows::core::Result<()>;
    fn LoadProperties(&self, propertybag: ::core::option::Option<&IFsrmPropertyBag>) -> ::windows::core::Result<()>;
    fn SaveProperties(&self, propertybag: ::core::option::Option<&IFsrmPropertyBag>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFsrmStorageModuleImplementation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsrmStorageModuleImplementation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>() -> IFsrmStorageModuleImplementation_Vtbl {
        unsafe extern "system" fn UseDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseDefinitions(::windows::core::from_raw_borrowed(&propertydefinitions)).into()
        }
        unsafe extern "system" fn LoadProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadProperties(::windows::core::from_raw_borrowed(&propertybag)).into()
        }
        unsafe extern "system" fn SaveProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveProperties(::windows::core::from_raw_borrowed(&propertybag)).into()
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseDefinitions: UseDefinitions::<Identity, Impl, OFFSET>,
            LoadProperties: LoadProperties::<Identity, Impl, OFFSET>,
            SaveProperties: SaveProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsrmStorageModuleImplementation as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFsrmPipelineModuleImplementation as ::windows::core::ComInterface>::IID
    }
}
