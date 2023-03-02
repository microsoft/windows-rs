#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccount_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Folders(&self) -> ::windows::core::Result<IFaxAccountFolders>;
    fn ListenToAccountEvents(&self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn RegisteredEvents(&self) -> ::windows::core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccount {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccount_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>() -> IFaxAccount_Vtbl {
        unsafe extern "system" fn AccountName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraccountname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraccountname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Folders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenToAccountEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ListenToAccountEvents(::core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisteredEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pregisteredevents, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccountName: AccountName::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            ListenToAccountEvents: ListenToAccountEvents::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccount as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> ::windows::core::Result<IFaxAccountOutgoingQueue>;
    fn IncomingQueue(&self) -> ::windows::core::Result<IFaxAccountIncomingQueue>;
    fn IncomingArchive(&self) -> ::windows::core::Result<IFaxAccountIncomingArchive>;
    fn OutgoingArchive(&self) -> ::windows::core::Result<IFaxAccountOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountFolders {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountFolders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>() -> IFaxAccountFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingarchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingarchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountFolders as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&self) -> ::windows::core::Result<i32>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountIncomingArchive {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountIncomingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>() -> IFaxAccountIncomingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessage(::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountIncomingArchive as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> ::windows::core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountIncomingQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountIncomingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>() -> IFaxAccountIncomingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJob(::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountIncomingQueue as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrjobid: &::windows::core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows::core::BSTR, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows::core::BSTR, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxaccount: ::core::option::Option<&IFaxAccount>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountNotify {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>() -> IFaxAccountNotify_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobAdded(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobRemoved(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobChanged(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid), ::windows::core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobAdded(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobRemoved(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobChanged(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrjobid), ::windows::core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, faddedtoreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingMessageAdded(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid), ::core::mem::transmute_copy(&faddedtoreceivefolder)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, fremovedfromreceivefolder: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingMessageRemoved(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid), ::core::mem::transmute_copy(&fremovedfromreceivefolder)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingMessageAdded(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingMessageRemoved(::windows::core::from_raw_borrowed(&pfaxaccount), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnServerShutDown(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountNotify as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&self) -> ::windows::core::Result<i32>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountOutgoingArchive {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountOutgoingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>() -> IFaxAccountOutgoingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessage(::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingArchive as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&self) -> ::windows::core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountOutgoingQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountOutgoingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>() -> IFaxAccountOutgoingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJob(::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingQueue as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountSet_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetAccounts(&self) -> ::windows::core::Result<IFaxAccounts>;
    fn GetAccount(&self, bstraccountname: &::windows::core::BSTR) -> ::windows::core::Result<IFaxAccount>;
    fn AddAccount(&self, bstraccountname: &::windows::core::BSTR) -> ::windows::core::Result<IFaxAccount>;
    fn RemoveAccount(&self, bstraccountname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccountSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>() -> IFaxAccountSet_Vtbl {
        unsafe extern "system" fn GetAccounts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxaccounts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccount(::core::mem::transmute(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddAccount(::core::mem::transmute(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAccount(::core::mem::transmute(&bstraccountname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAccounts: GetAccounts::<Identity, Impl, OFFSET>,
            GetAccount: GetAccount::<Identity, Impl, OFFSET>,
            AddAccount: AddAccount::<Identity, Impl, OFFSET>,
            RemoveAccount: RemoveAccount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountSet as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccounts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxAccount>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxAccounts {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccounts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>() -> IFaxAccounts_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxaccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxaccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccounts as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxActivity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IncomingMessages(&self) -> ::windows::core::Result<i32>;
    fn RoutingMessages(&self) -> ::windows::core::Result<i32>;
    fn OutgoingMessages(&self) -> ::windows::core::Result<i32>;
    fn QueuedMessages(&self) -> ::windows::core::Result<i32>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxActivity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>() -> IFaxActivity_Vtbl {
        unsafe extern "system" fn IncomingMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plincomingmessages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoutingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plroutingmessages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploutgoingmessages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueuedMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueuedMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plqueuedmessages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IncomingMessages: IncomingMessages::<Identity, Impl, OFFSET>,
            RoutingMessages: RoutingMessages::<Identity, Impl, OFFSET>,
            OutgoingMessages: OutgoingMessages::<Identity, Impl, OFFSET>,
            QueuedMessages: QueuedMessages::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxActivity as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxActivityLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LogIncoming(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogIncoming(&self, blogincoming: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn LogOutgoing(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetLogOutgoing(&self, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn DatabasePath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDatabasePath(&self, bstrdatabasepath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxActivityLogging {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxActivityLogging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>() -> IFaxActivityLogging_Vtbl {
        unsafe extern "system" fn LogIncoming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogincoming: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LogIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblogincoming, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogIncoming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogincoming: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogIncoming(::core::mem::transmute_copy(&blogincoming)).into()
        }
        unsafe extern "system" fn LogOutgoing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LogOutgoing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblogoutgoing, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOutgoing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogoutgoing: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogOutgoing(::core::mem::transmute_copy(&blogoutgoing)).into()
        }
        unsafe extern "system" fn DatabasePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DatabasePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdatabasepath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatabasePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDatabasePath(::core::mem::transmute(&bstrdatabasepath)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LogIncoming: LogIncoming::<Identity, Impl, OFFSET>,
            SetLogIncoming: SetLogIncoming::<Identity, Impl, OFFSET>,
            LogOutgoing: LogOutgoing::<Identity, Impl, OFFSET>,
            SetLogOutgoing: SetLogOutgoing::<Identity, Impl, OFFSET>,
            DatabasePath: DatabasePath::<Identity, Impl, OFFSET>,
            SetDatabasePath: SetDatabasePath::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxActivityLogging as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ArchiveLocation(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetArchiveLocation(&self, bstrarchivelocation: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn ArchiveAgeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetArchiveAgeLimit(&self, larchiveagelimit: i32) -> ::windows::core::Result<()>;
    fn ArchiveSizeLow(&self) -> ::windows::core::Result<i32>;
    fn ArchiveSizeHigh(&self) -> ::windows::core::Result<i32>;
    fn OutgoingQueueBlocked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueueBlocked(&self, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OutgoingQueuePaused(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOutgoingQueuePaused(&self, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn UseDeviceTSID(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> ::windows::core::Result<()>;
    fn RetryDelay(&self) -> ::windows::core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::core::Result<()>;
    fn DiscountRateStart(&self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::core::Result<()>;
    fn DiscountRateEnd(&self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::core::Result<()>;
    fn OutgoingQueueAgeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetOutgoingQueueAgeLimit(&self, loutgoingqueueagelimit: i32) -> ::windows::core::Result<()>;
    fn Branding(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn IncomingQueueBlocked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingQueueBlocked(&self, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn AutoCreateAccountOnConnect(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoCreateAccountOnConnect(&self, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn IncomingFaxesArePublic(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncomingFaxesArePublic(&self, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxConfiguration {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>() -> IFaxConfiguration_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveLocation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivelocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArchiveLocation(::core::mem::transmute(&bstrarchivelocation)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn ArchiveAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveAgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarchiveagelimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArchiveAgeLimit(::core::mem::transmute_copy(&larchiveagelimit)).into()
        }
        unsafe extern "system" fn ArchiveSizeLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveSizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveSizeHigh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveSizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingQueueBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutgoingblocked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutgoingQueueBlocked(::core::mem::transmute_copy(&boutgoingblocked)).into()
        }
        unsafe extern "system" fn OutgoingQueuePaused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingQueuePaused() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboutgoingpaused, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutgoingQueuePaused(::core::mem::transmute_copy(&boutgoingpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowPersonalCoverPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pballowpersonalcoverpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowPersonalCoverPages(::core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseDeviceTSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevicetsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseDeviceTSID(::core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetries(::core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretrydelay, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetryDelay(::core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscountRateStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountratestart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscountRateStart(::core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscountRateEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountrateend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscountRateEnd(::core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingQueueAgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploutgoingqueueagelimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutgoingQueueAgeLimit(::core::mem::transmute_copy(&loutgoingqueueagelimit)).into()
        }
        unsafe extern "system" fn Branding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Branding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbbranding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBranding(::core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn IncomingQueueBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingQueueBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbincomingblocked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIncomingQueueBlocked(::core::mem::transmute_copy(&bincomingblocked)).into()
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoCreateAccountOnConnect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbautocreateaccountonconnect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoCreateAccountOnConnect(::core::mem::transmute_copy(&bautocreateaccountonconnect)).into()
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingFaxesArePublic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbincomingfaxesarepublic, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIncomingFaxesArePublic(::core::mem::transmute_copy(&bincomingfaxesarepublic)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveLocation: ArchiveLocation::<Identity, Impl, OFFSET>,
            SetArchiveLocation: SetArchiveLocation::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            ArchiveAgeLimit: ArchiveAgeLimit::<Identity, Impl, OFFSET>,
            SetArchiveAgeLimit: SetArchiveAgeLimit::<Identity, Impl, OFFSET>,
            ArchiveSizeLow: ArchiveSizeLow::<Identity, Impl, OFFSET>,
            ArchiveSizeHigh: ArchiveSizeHigh::<Identity, Impl, OFFSET>,
            OutgoingQueueBlocked: OutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            SetOutgoingQueueBlocked: SetOutgoingQueueBlocked::<Identity, Impl, OFFSET>,
            OutgoingQueuePaused: OutgoingQueuePaused::<Identity, Impl, OFFSET>,
            SetOutgoingQueuePaused: SetOutgoingQueuePaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            OutgoingQueueAgeLimit: OutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            SetOutgoingQueueAgeLimit: SetOutgoingQueueAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            IncomingQueueBlocked: IncomingQueueBlocked::<Identity, Impl, OFFSET>,
            SetIncomingQueueBlocked: SetIncomingQueueBlocked::<Identity, Impl, OFFSET>,
            AutoCreateAccountOnConnect: AutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            SetAutoCreateAccountOnConnect: SetAutoCreateAccountOnConnect::<Identity, Impl, OFFSET>,
            IncomingFaxesArePublic: IncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            SetIncomingFaxesArePublic: SetIncomingFaxesArePublic::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxConfiguration as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn DeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ProviderUniqueName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn PoweredOff(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceivingNow(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SendingNow(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UsedRoutingMethods(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SendEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSendEnabled(&self, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ReceiveMode(&self) -> ::windows::core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM>;
    fn SetReceiveMode(&self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::Result<()>;
    fn RingsBeforeAnswer(&self) -> ::windows::core::Result<i32>;
    fn SetRingsBeforeAnswer(&self, lringsbeforeanswer: i32) -> ::windows::core::Result<()>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCSID(&self, bstrcsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTSID(&self, bstrtsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &::windows::core::BSTR, vproperty: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UseRoutingMethod(&self, bstrmethodguid: &::windows::core::BSTR, buse: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn RingingNow(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerCall(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>() -> IFaxDevice_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderUniqueName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProviderUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprovideruniquename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PoweredOff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PoweredOff() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpoweredoff, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivingNow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivingNow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbreceivingnow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendingNow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendingNow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsendingnow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedRoutingMethods<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UsedRoutingMethods() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvusedroutingmethods, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn SendEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsendenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsendenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSendEnabled(::core::mem::transmute_copy(&bsendenabled)).into()
        }
        unsafe extern "system" fn ReceiveMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceivemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceiveMode(::core::mem::transmute_copy(&receivemode)).into()
        }
        unsafe extern "system" fn RingsBeforeAnswer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RingsBeforeAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plringsbeforeanswer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRingsBeforeAnswer(::core::mem::transmute_copy(&lringsbeforeanswer)).into()
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCSID(::core::mem::transmute(&bstrcsid)).into()
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTSID(::core::mem::transmute(&bstrtsid)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtensionProperty(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtensionProperty(::core::mem::transmute(&bstrguid), ::core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn UseRoutingMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, buse: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseRoutingMethod(::core::mem::transmute(&bstrmethodguid), ::core::mem::transmute_copy(&buse)).into()
        }
        unsafe extern "system" fn RingingNow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbringingnow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RingingNow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbringingnow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AnswerCall().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            ProviderUniqueName: ProviderUniqueName::<Identity, Impl, OFFSET>,
            PoweredOff: PoweredOff::<Identity, Impl, OFFSET>,
            ReceivingNow: ReceivingNow::<Identity, Impl, OFFSET>,
            SendingNow: SendingNow::<Identity, Impl, OFFSET>,
            UsedRoutingMethods: UsedRoutingMethods::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SendEnabled: SendEnabled::<Identity, Impl, OFFSET>,
            SetSendEnabled: SetSendEnabled::<Identity, Impl, OFFSET>,
            ReceiveMode: ReceiveMode::<Identity, Impl, OFFSET>,
            SetReceiveMode: SetReceiveMode::<Identity, Impl, OFFSET>,
            RingsBeforeAnswer: RingsBeforeAnswer::<Identity, Impl, OFFSET>,
            SetRingsBeforeAnswer: SetRingsBeforeAnswer::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            SetCSID: SetCSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            UseRoutingMethod: UseRoutingMethod::<Identity, Impl, OFFSET>,
            RingingNow: RingingNow::<Identity, Impl, OFFSET>,
            AnswerCall: AnswerCall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDevice as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceIds_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<i32>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, ldeviceid: i32) -> ::windows::core::Result<()>;
    fn Remove(&self, lindex: i32) -> ::windows::core::Result<()>;
    fn SetOrder(&self, ldeviceid: i32, lneworder: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDeviceIds {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceIds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>() -> IFaxDeviceIds_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&ldeviceid)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn SetOrder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrder(::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&lneworder)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SetOrder: SetOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceIds as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceProvider_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ImageName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TapiProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MajorVersion(&self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&self) -> ::windows::core::Result<i32>;
    fn Debug(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> ::windows::core::Result<i32>;
    fn DeviceIds(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDeviceProvider {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>() -> IFaxDeviceProvider_Vtbl {
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruniquename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapiProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TapiProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtapiprovidername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Debug() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pliniterrorcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceIds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdeviceids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            TapiProviderName: TapiProviderName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceProvider as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceProviders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxDeviceProvider>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDeviceProviders {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceProviders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>() -> IFaxDeviceProviders_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxdeviceprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdeviceprovider, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceProviders as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxDevice>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_ItemById(&self, lid: i32) -> ::windows::core::Result<IFaxDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDevices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>() -> IFaxDevices_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_ItemById(::core::mem::transmute_copy(&lid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdevice, ::core::mem::transmute(ok__));
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
            get_ItemById: get_ItemById::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDevices as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDocument_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Body(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetBody(&self, bstrbody: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Sender(&self) -> ::windows::core::Result<IFaxSender>;
    fn Recipients(&self) -> ::windows::core::Result<IFaxRecipients>;
    fn CoverPage(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCoverPage(&self, bstrcoverpage: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSubject(&self, bstrsubject: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Note(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetNote(&self, bstrnote: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ScheduleTime(&self) -> ::windows::core::Result<f64>;
    fn SetScheduleTime(&self, datescheduletime: f64) -> ::windows::core::Result<()>;
    fn ReceiptAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetReceiptAddress(&self, bstrreceiptaddress: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn DocumentName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDocumentName(&self, bstrdocumentname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CallHandle(&self) -> ::windows::core::Result<i32>;
    fn SetCallHandle(&self, lcallhandle: i32) -> ::windows::core::Result<()>;
    fn CoverPageType(&self) -> ::windows::core::Result<FAX_COVERPAGE_TYPE_ENUM>;
    fn SetCoverPageType(&self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn ScheduleType(&self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM>;
    fn SetScheduleType(&self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetReceiptType(&self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGroupBroadcastReceipts(&self, busegrouping: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn SetPriority(&self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn TapiConnection(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn putref_TapiConnection(&self, ptapiconnection: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Submit(&self, bstrfaxservername: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ConnectedSubmit(&self, pfaxserver: ::core::option::Option<&IFaxServer>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AttachFaxToReceipt(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttachFaxToReceipt(&self, battachfax: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDocument {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>() -> IFaxDocument_Vtbl {
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbody: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&bstrbody)).into()
        }
        unsafe extern "system" fn Sender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcoverpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcoverpage: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCoverPage(::core::mem::transmute(&bstrcoverpage)).into()
        }
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubject(::core::mem::transmute(&bstrsubject)).into()
        }
        unsafe extern "system" fn Note<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnote: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Note() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnote, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnote: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNote(::core::mem::transmute(&bstrnote)).into()
        }
        unsafe extern "system" fn ScheduleTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduleTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduletime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheduleTime(::core::mem::transmute_copy(&datescheduletime)).into()
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceiptAddress(::core::mem::transmute(&bstrreceiptaddress)).into()
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdocumentname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentName(::core::mem::transmute(&bstrdocumentname)).into()
        }
        unsafe extern "system" fn CallHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCallHandle(::core::mem::transmute_copy(&lcallhandle)).into()
        }
        unsafe extern "system" fn CoverPageType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CoverPageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoverpagetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPageType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCoverPageType(::core::mem::transmute_copy(&coverpagetype)).into()
        }
        unsafe extern "system" fn ScheduleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduleType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pscheduletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheduleType(::core::mem::transmute_copy(&scheduletype)).into()
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceiptType(::core::mem::transmute_copy(&receipttype)).into()
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusegrouping: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupBroadcastReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusegrouping, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busegrouping: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupBroadcastReceipts(::core::mem::transmute_copy(&busegrouping)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn TapiConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiconnection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TapiConnection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptapiconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_TapiConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptapiconnection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_TapiConnection(::windows::core::from_raw_borrowed(&ptapiconnection)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Submit(::core::mem::transmute(&bstrfaxservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfaxoutgoingjobids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedSubmit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectedSubmit(::windows::core::from_raw_borrowed(&pfaxserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfaxoutgoingjobids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachFaxToReceipt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattachfax: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttachFaxToReceipt() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbattachfax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, battachfax: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachFaxToReceipt(::core::mem::transmute_copy(&battachfax)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            CoverPage: CoverPage::<Identity, Impl, OFFSET>,
            SetCoverPage: SetCoverPage::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            Note: Note::<Identity, Impl, OFFSET>,
            SetNote: SetNote::<Identity, Impl, OFFSET>,
            ScheduleTime: ScheduleTime::<Identity, Impl, OFFSET>,
            SetScheduleTime: SetScheduleTime::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            SetReceiptAddress: SetReceiptAddress::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            SetDocumentName: SetDocumentName::<Identity, Impl, OFFSET>,
            CallHandle: CallHandle::<Identity, Impl, OFFSET>,
            SetCallHandle: SetCallHandle::<Identity, Impl, OFFSET>,
            CoverPageType: CoverPageType::<Identity, Impl, OFFSET>,
            SetCoverPageType: SetCoverPageType::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
            SetScheduleType: SetScheduleType::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            SetReceiptType: SetReceiptType::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            SetGroupBroadcastReceipts: SetGroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            TapiConnection: TapiConnection::<Identity, Impl, OFFSET>,
            putref_TapiConnection: putref_TapiConnection::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            ConnectedSubmit: ConnectedSubmit::<Identity, Impl, OFFSET>,
            AttachFaxToReceipt: AttachFaxToReceipt::<Identity, Impl, OFFSET>,
            SetAttachFaxToReceipt: SetAttachFaxToReceipt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDocument as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDocument2_Impl: Sized + IFaxDocument_Impl {
    fn SubmissionId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Bodies(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetBodies(&self, vbodies: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit2(&self, bstrfaxservername: &::windows::core::BSTR, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()>;
    fn ConnectedSubmit2(&self, pfaxserver: ::core::option::Option<&IFaxServer>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxDocument2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>() -> IFaxDocument2_Vtbl {
        unsafe extern "system" fn SubmissionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bodies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bodies() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbodies, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodies<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbodies: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBodies(::core::mem::transmute(&vbodies)).into()
        }
        unsafe extern "system" fn Submit2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit2(::core::mem::transmute(&bstrfaxservername), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        unsafe extern "system" fn ConnectedSubmit2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectedSubmit2(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        Self {
            base__: IFaxDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Bodies: Bodies::<Identity, Impl, OFFSET>,
            SetBodies: SetBodies::<Identity, Impl, OFFSET>,
            Submit2: Submit2::<Identity, Impl, OFFSET>,
            ConnectedSubmit2: ConnectedSubmit2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDocument2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFaxDocument as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxEventLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn InitEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInitEventsLevel(&self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn InboundEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInboundEventsLevel(&self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn OutboundEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetOutboundEventsLevel(&self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn GeneralEventsLevel(&self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetGeneralEventsLevel(&self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxEventLogging {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxEventLogging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>() -> IFaxEventLogging_Vtbl {
        unsafe extern "system" fn InitEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piniteventlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitEventsLevel(::core::mem::transmute_copy(&initeventlevel)).into()
        }
        unsafe extern "system" fn InboundEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InboundEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinboundeventlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInboundEventsLevel(::core::mem::transmute_copy(&inboundeventlevel)).into()
        }
        unsafe extern "system" fn OutboundEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutboundEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutboundeventlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutboundEventsLevel(::core::mem::transmute_copy(&outboundeventlevel)).into()
        }
        unsafe extern "system" fn GeneralEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GeneralEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgeneraleventlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGeneralEventsLevel(::core::mem::transmute_copy(&generaleventlevel)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitEventsLevel: InitEventsLevel::<Identity, Impl, OFFSET>,
            SetInitEventsLevel: SetInitEventsLevel::<Identity, Impl, OFFSET>,
            InboundEventsLevel: InboundEventsLevel::<Identity, Impl, OFFSET>,
            SetInboundEventsLevel: SetInboundEventsLevel::<Identity, Impl, OFFSET>,
            OutboundEventsLevel: OutboundEventsLevel::<Identity, Impl, OFFSET>,
            SetOutboundEventsLevel: SetOutboundEventsLevel::<Identity, Impl, OFFSET>,
            GeneralEventsLevel: GeneralEventsLevel::<Identity, Impl, OFFSET>,
            SetGeneralEventsLevel: SetGeneralEventsLevel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxEventLogging as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&self) -> ::windows::core::Result<IFaxOutgoingQueue>;
    fn IncomingQueue(&self) -> ::windows::core::Result<IFaxIncomingQueue>;
    fn IncomingArchive(&self) -> ::windows::core::Result<IFaxIncomingArchive>;
    fn OutgoingArchive(&self) -> ::windows::core::Result<IFaxOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxFolders {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxFolders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>() -> IFaxFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncomingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingarchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutgoingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingarchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Identity, Impl, OFFSET>,
            IncomingQueue: IncomingQueue::<Identity, Impl, OFFSET>,
            IncomingArchive: IncomingArchive::<Identity, Impl, OFFSET>,
            OutgoingArchive: OutgoingArchive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxFolders as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetExtensions(&self) -> ::windows::core::Result<IFaxInboundRoutingExtensions>;
    fn GetMethods(&self) -> ::windows::core::Result<IFaxInboundRoutingMethods>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxInboundRouting {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRouting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>() -> IFaxInboundRouting_Vtbl {
        unsafe extern "system" fn GetExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingextensions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethods<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMethods() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingmethods, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetExtensions: GetExtensions::<Identity, Impl, OFFSET>,
            GetMethods: GetMethods::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRouting as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingExtension_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ImageName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MajorVersion(&self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&self) -> ::windows::core::Result<i32>;
    fn Debug(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Status(&self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&self) -> ::windows::core::Result<i32>;
    fn Methods(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxInboundRoutingExtension {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>() -> IFaxInboundRoutingExtension_Vtbl {
        unsafe extern "system" fn FriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfriendlyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrimagename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruniquename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Debug() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pliniterrorcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Methods() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvmethods, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FriendlyName: FriendlyName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            UniqueName: UniqueName::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            InitErrorCode: InitErrorCode::<Identity, Impl, OFFSET>,
            Methods: Methods::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtension as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingExtensions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxInboundRoutingExtension>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxInboundRoutingExtensions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingExtensions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>() -> IFaxInboundRoutingExtensions_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxinboundroutingextension: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtensions as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingMethod_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GUID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn FunctionName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ExtensionFriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ExtensionImageName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Priority(&self) -> ::windows::core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxInboundRoutingMethod {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingMethod_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>() -> IFaxInboundRoutingMethod_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FunctionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FunctionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfunctionname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionFriendlyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtensionFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextensionfriendlyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionImageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtensionImageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextensionimagename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            GUID: GUID::<Identity, Impl, OFFSET>,
            FunctionName: FunctionName::<Identity, Impl, OFFSET>,
            ExtensionFriendlyName: ExtensionFriendlyName::<Identity, Impl, OFFSET>,
            ExtensionImageName: ExtensionImageName::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethod as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingMethods_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxInboundRoutingMethod>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxInboundRoutingMethods {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>() -> IFaxInboundRoutingMethods_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxinboundroutingmethod: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxinboundroutingmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethods as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ArchiveFolder(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn AgeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn SizeLow(&self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&self) -> ::windows::core::Result<i32>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingArchive {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>() -> IFaxIncomingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivefolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArchiveFolder(::core::mem::transmute(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessage(::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingArchive as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Size(&self) -> ::windows::core::Result<i32>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CurrentPage(&self) -> ::windows::core::Result<i32>;
    fn DeviceId(&self) -> ::windows::core::Result<i32>;
    fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn TransmissionStart(&self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&self) -> ::windows::core::Result<f64>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CallerId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RoutingInformation(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn JobType(&self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>() -> IFaxIncomingJob_Vtbl {
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JobType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjobtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTiff(::core::mem::transmute(&bstrtiffpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingJob as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxIncomingJob>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingJobs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingJobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>() -> IFaxIncomingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingJobs as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Pages(&self) -> ::windows::core::Result<i32>;
    fn Size(&self) -> ::windows::core::Result<i32>;
    fn DeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn TransmissionStart(&self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&self) -> ::windows::core::Result<f64>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CallerId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RoutingInformation(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingMessage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>() -> IFaxIncomingMessage_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTiff(::core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessage as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessage2_Impl: Sized + IFaxIncomingMessage_Impl {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSubject(&self, bstrsubject: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SenderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSenderName(&self, bstrsendername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SenderFaxNumber(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSenderFaxNumber(&self, bstrsenderfaxnumber: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn HasCoverPage(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetHasCoverPage(&self, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Recipients(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetRecipients(&self, bstrrecipients: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn WasReAssigned(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Read(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ReAssign(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingMessage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>() -> IFaxIncomingMessage2_Vtbl {
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubject(::core::mem::transmute(&bstrsubject)).into()
        }
        unsafe extern "system" fn SenderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsendername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsendername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsendername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderName(::core::mem::transmute(&bstrsendername)).into()
        }
        unsafe extern "system" fn SenderFaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderFaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsenderfaxnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderFaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderFaxNumber(::core::mem::transmute(&bstrsenderfaxnumber)).into()
        }
        unsafe extern "system" fn HasCoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHasCoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhascoverpage: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHasCoverPage(::core::mem::transmute_copy(&bhascoverpage)).into()
        }
        unsafe extern "system" fn Recipients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrecipients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrecipients: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecipients(::core::mem::transmute(&bstrrecipients)).into()
        }
        unsafe extern "system" fn WasReAssigned<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WasReAssigned() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbwasreassigned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Read() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbread, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRead(::core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn ReAssign<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReAssign().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: IFaxIncomingMessage_Vtbl::new::<Identity, Impl, OFFSET>(),
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            SenderName: SenderName::<Identity, Impl, OFFSET>,
            SetSenderName: SetSenderName::<Identity, Impl, OFFSET>,
            SenderFaxNumber: SenderFaxNumber::<Identity, Impl, OFFSET>,
            SetSenderFaxNumber: SetSenderFaxNumber::<Identity, Impl, OFFSET>,
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            SetHasCoverPage: SetHasCoverPage::<Identity, Impl, OFFSET>,
            Recipients: Recipients::<Identity, Impl, OFFSET>,
            SetRecipients: SetRecipients::<Identity, Impl, OFFSET>,
            WasReAssigned: WasReAssigned::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            ReAssign: ReAssign::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessage2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFaxIncomingMessage as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> ::windows::core::Result<IFaxIncomingMessage>;
    fn PrefetchSize(&self) -> ::windows::core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::core::Result<()>;
    fn AtEOF(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MoveFirst(&self) -> ::windows::core::Result<()>;
    fn MoveNext(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingMessageIterator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessageIterator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>() -> IFaxIncomingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrefetchSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprefetchsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrefetchSize(::core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn AtEOF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AtEOF() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeof, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveFirst().into()
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessageIterator as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetJobs(&self) -> ::windows::core::Result<IFaxIncomingJobs>;
    fn GetJob(&self, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxIncomingQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>() -> IFaxIncomingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Blocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbblocked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBlocked(::core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxincomingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJob(::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxincomingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingQueue as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxJobStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn Pages(&self) -> ::windows::core::Result<i32>;
    fn Size(&self) -> ::windows::core::Result<i32>;
    fn CurrentPage(&self) -> ::windows::core::Result<i32>;
    fn DeviceId(&self) -> ::windows::core::Result<i32>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn JobType(&self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM>;
    fn ScheduledTime(&self) -> ::windows::core::Result<f64>;
    fn TransmissionStart(&self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&self) -> ::windows::core::Result<f64>;
    fn CallerId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn RoutingInformation(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxJobStatus {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxJobStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>() -> IFaxJobStatus_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JobType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjobtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduledtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcallerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrroutinginformation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            JobType: JobType::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CallerId: CallerId::<Identity, Impl, OFFSET>,
            RoutingInformation: RoutingInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxJobStatus as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxLoggingOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EventLogging(&self) -> ::windows::core::Result<IFaxEventLogging>;
    fn ActivityLogging(&self) -> ::windows::core::Result<IFaxActivityLogging>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxLoggingOptions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxLoggingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>() -> IFaxLoggingOptions_Vtbl {
        unsafe extern "system" fn EventLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventLogging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxeventlogging, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityLogging<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivityLogging() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxactivitylogging, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventLogging: EventLogging::<Identity, Impl, OFFSET>,
            ActivityLogging: ActivityLogging::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxLoggingOptions as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetGroups(&self) -> ::windows::core::Result<IFaxOutboundRoutingGroups>;
    fn GetRules(&self) -> ::windows::core::Result<IFaxOutboundRoutingRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutboundRouting {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRouting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>() -> IFaxOutboundRouting_Vtbl {
        unsafe extern "system" fn GetGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroups, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRules<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrules, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            GetRules: GetRules::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRouting as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Status(&self) -> ::windows::core::Result<FAX_GROUP_STATUS_ENUM>;
    fn DeviceIds(&self) -> ::windows::core::Result<IFaxDeviceIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceIds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxdeviceids, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            DeviceIds: DeviceIds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroup as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxOutboundRoutingGroup>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<IFaxOutboundRoutingGroup>;
    fn Remove(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingGroups {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingGroups_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxoutboundroutinggroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutinggroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&vindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroups as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CountryCode(&self) -> ::windows::core::Result<i32>;
    fn AreaCode(&self) -> ::windows::core::Result<i32>;
    fn Status(&self) -> ::windows::core::Result<FAX_RULE_STATUS_ENUM>;
    fn UseDevice(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDevice(&self, busedevice: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn DeviceId(&self) -> ::windows::core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> ::windows::core::Result<()>;
    fn GroupName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetGroupName(&self, bstrgroupname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingRule {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingRule_Vtbl {
        unsafe extern "system" fn CountryCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcountrycode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreaCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plareacode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevice: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevice: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseDevice(::core::mem::transmute_copy(&busedevice)).into()
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceId(::core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn GroupName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgroupname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupName(::core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            AreaCode: AreaCode::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            UseDevice: UseDevice::<Identity, Impl, OFFSET>,
            SetUseDevice: SetUseDevice::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GroupName: GroupName::<Identity, Impl, OFFSET>,
            SetGroupName: SetGroupName::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRule as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn ItemByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
    fn RemoveByCountryAndArea(&self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<()>;
    fn Remove(&self, lindex: i32) -> ::windows::core::Result<()>;
    fn Add(&self, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: &::windows::core::BSTR, ldeviceid: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingRules {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>() -> IFaxOutboundRoutingRules_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByCountryAndArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemByCountryAndArea(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveByCountryAndArea(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: super::super::Foundation::VARIANT_BOOL, bstrgroupname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode), ::core::mem::transmute_copy(&busedevice), ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute_copy(&ldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutboundroutingrule, ::core::mem::transmute(ok__));
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
            ItemByCountryAndArea: ItemByCountryAndArea::<Identity, Impl, OFFSET>,
            RemoveByCountryAndArea: RemoveByCountryAndArea::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRules as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseArchive(&self, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn ArchiveFolder(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetArchiveFolder(&self, bstrarchivefolder: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSizeQuotaWarning(&self, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn AgeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn SizeLow(&self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&self) -> ::windows::core::Result<i32>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetMessages(&self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&self, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingArchive {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>() -> IFaxOutgoingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusearchive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArchiveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrarchivefolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArchiveFolder(::core::mem::transmute(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsizequotawarning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhighquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllowquotawatermark, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizelow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsizehigh, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessageiterator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessage(::core::mem::transmute(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseArchive: UseArchive::<Identity, Impl, OFFSET>,
            SetUseArchive: SetUseArchive::<Identity, Impl, OFFSET>,
            ArchiveFolder: ArchiveFolder::<Identity, Impl, OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Identity, Impl, OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Identity, Impl, OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Identity, Impl, OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Identity, Impl, OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Identity, Impl, OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            SizeLow: SizeLow::<Identity, Impl, OFFSET>,
            SizeHigh: SizeHigh::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetMessages: GetMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingArchive as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn DocumentName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Pages(&self) -> ::windows::core::Result<i32>;
    fn Size(&self) -> ::windows::core::Result<i32>;
    fn SubmissionId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64>;
    fn SubmissionTime(&self) -> ::windows::core::Result<f64>;
    fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> ::windows::core::Result<IFaxSender>;
    fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient>;
    fn CurrentPage(&self) -> ::windows::core::Result<i32>;
    fn DeviceId(&self) -> ::windows::core::Result<i32>;
    fn Status(&self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AvailableOperations(&self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn ScheduledTime(&self) -> ::windows::core::Result<f64>;
    fn TransmissionStart(&self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&self) -> ::windows::core::Result<f64>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GroupBroadcastReceipts(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Restart(&self) -> ::windows::core::Result<()>;
    fn CopyTiff(&self, bstrtiffpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>() -> IFaxOutgoingJob_Vtbl {
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OriginalScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdateoriginalscheduledtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatesubmissiontime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcurrentpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextendedstatuscode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextendedstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pavailableoperations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatescheduledtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupBroadcastReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbgroupbroadcastreceipts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Restart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restart().into()
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTiff(::core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            CurrentPage: CurrentPage::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Identity, Impl, OFFSET>,
            ExtendedStatus: ExtendedStatus::<Identity, Impl, OFFSET>,
            AvailableOperations: AvailableOperations::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            ScheduledTime: ScheduledTime::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Restart: Restart::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJob as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJob2_Impl: Sized + IFaxOutgoingJob_Impl {
    fn HasCoverPage(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ScheduleType(&self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingJob2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJob2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>() -> IFaxOutgoingJob2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduleType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pscheduletype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFaxOutgoingJob_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            ScheduleType: ScheduleType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJob2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFaxOutgoingJob as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, vindex: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxOutgoingJob>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingJobs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>() -> IFaxOutgoingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: super::super::System::Com::VARIANT, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJobs as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SubmissionId(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn DocumentName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn Pages(&self) -> ::windows::core::Result<i32>;
    fn Size(&self) -> ::windows::core::Result<i32>;
    fn OriginalScheduledTime(&self) -> ::windows::core::Result<f64>;
    fn SubmissionTime(&self) -> ::windows::core::Result<f64>;
    fn Priority(&self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&self) -> ::windows::core::Result<IFaxSender>;
    fn Recipient(&self) -> ::windows::core::Result<IFaxRecipient>;
    fn DeviceName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TransmissionStart(&self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&self) -> ::windows::core::Result<f64>;
    fn CSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CopyTiff(&self, bstrtiffpath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingMessage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>() -> IFaxOutgoingMessage_Vtbl {
        unsafe extern "system" fn SubmissionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubmissionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdocumentname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OriginalScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdateoriginalscheduledtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatesubmissiontime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionstart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatetransmissionend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTiff(::core::mem::transmute(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmissionId: SubmissionId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            DocumentName: DocumentName::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            Pages: Pages::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Identity, Impl, OFFSET>,
            SubmissionTime: SubmissionTime::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            Sender: Sender::<Identity, Impl, OFFSET>,
            Recipient: Recipient::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            TransmissionStart: TransmissionStart::<Identity, Impl, OFFSET>,
            TransmissionEnd: TransmissionEnd::<Identity, Impl, OFFSET>,
            CSID: CSID::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            CopyTiff: CopyTiff::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessage2_Impl: Sized + IFaxOutgoingMessage_Impl {
    fn HasCoverPage(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiptType(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn ReceiptAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Read(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetRead(&self, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingMessage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>() -> IFaxOutgoingMessage2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbhascoverpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preceipttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreceiptaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Read() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbread, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRead(::core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: IFaxOutgoingMessage_Vtbl::new::<Identity, Impl, OFFSET>(),
            HasCoverPage: HasCoverPage::<Identity, Impl, OFFSET>,
            ReceiptType: ReceiptType::<Identity, Impl, OFFSET>,
            ReceiptAddress: ReceiptAddress::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            SetRead: SetRead::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFaxOutgoingMessage as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&self) -> ::windows::core::Result<IFaxOutgoingMessage>;
    fn AtEOF(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn PrefetchSize(&self) -> ::windows::core::Result<i32>;
    fn SetPrefetchSize(&self, lprefetchsize: i32) -> ::windows::core::Result<()>;
    fn MoveFirst(&self) -> ::windows::core::Result<()>;
    fn MoveNext(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingMessageIterator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessageIterator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>() -> IFaxOutgoingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEOF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AtEOF() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbeof, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrefetchSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprefetchsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrefetchSize(::core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn MoveFirst<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveFirst().into()
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveNext().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            AtEOF: AtEOF::<Identity, Impl, OFFSET>,
            PrefetchSize: PrefetchSize::<Identity, Impl, OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Identity, Impl, OFFSET>,
            MoveFirst: MoveFirst::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessageIterator as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBlocked(&self, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Paused(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPaused(&self, bpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn AllowPersonalCoverPages(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowPersonalCoverPages(&self, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn UseDeviceTSID(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseDeviceTSID(&self, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Retries(&self) -> ::windows::core::Result<i32>;
    fn SetRetries(&self, lretries: i32) -> ::windows::core::Result<()>;
    fn RetryDelay(&self) -> ::windows::core::Result<i32>;
    fn SetRetryDelay(&self, lretrydelay: i32) -> ::windows::core::Result<()>;
    fn DiscountRateStart(&self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateStart(&self, datediscountratestart: f64) -> ::windows::core::Result<()>;
    fn DiscountRateEnd(&self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateEnd(&self, datediscountrateend: f64) -> ::windows::core::Result<()>;
    fn AgeLimit(&self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn Branding(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBranding(&self, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn GetJobs(&self) -> ::windows::core::Result<IFaxOutgoingJobs>;
    fn GetJob(&self, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxOutgoingQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>() -> IFaxOutgoingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Blocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbblocked, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBlocked(::core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Paused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Paused() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbpaused, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpaused: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPaused(::core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowPersonalCoverPages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pballowpersonalcoverpages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowPersonalCoverPages(::core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseDeviceTSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbusedevicetsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseDeviceTSID(::core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Retries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetries(::core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plretrydelay, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetryDelay(::core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscountRateStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountratestart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscountRateStart(::core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscountRateEnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatediscountrateend, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDiscountRateEnd(::core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn AgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plagelimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn Branding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Branding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbbranding, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBranding(::core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn GetJobs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjobs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfaxoutgoingjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJob(::core::mem::transmute(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxoutgoingjob, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Blocked: Blocked::<Identity, Impl, OFFSET>,
            SetBlocked: SetBlocked::<Identity, Impl, OFFSET>,
            Paused: Paused::<Identity, Impl, OFFSET>,
            SetPaused: SetPaused::<Identity, Impl, OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Identity, Impl, OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Identity, Impl, OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Identity, Impl, OFFSET>,
            Retries: Retries::<Identity, Impl, OFFSET>,
            SetRetries: SetRetries::<Identity, Impl, OFFSET>,
            RetryDelay: RetryDelay::<Identity, Impl, OFFSET>,
            SetRetryDelay: SetRetryDelay::<Identity, Impl, OFFSET>,
            DiscountRateStart: DiscountRateStart::<Identity, Impl, OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Identity, Impl, OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Identity, Impl, OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Identity, Impl, OFFSET>,
            AgeLimit: AgeLimit::<Identity, Impl, OFFSET>,
            SetAgeLimit: SetAgeLimit::<Identity, Impl, OFFSET>,
            Branding: Branding::<Identity, Impl, OFFSET>,
            SetBranding: SetBranding::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetJobs: GetJobs::<Identity, Impl, OFFSET>,
            GetJob: GetJob::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingQueue as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxReceiptOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AuthenticationType(&self) -> ::windows::core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>;
    fn SetAuthenticationType(&self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn SMTPServer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSMTPServer(&self, bstrsmtpserver: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SMTPPort(&self) -> ::windows::core::Result<i32>;
    fn SetSMTPPort(&self, lsmtpport: i32) -> ::windows::core::Result<()>;
    fn SMTPSender(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSMTPSender(&self, bstrsmtpsender: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SMTPUser(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSMTPUser(&self, bstrsmtpuser: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AllowedReceipts(&self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetAllowedReceipts(&self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn SMTPPassword(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSMTPPassword(&self, bstrsmtppassword: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn UseForInboundRouting(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseForInboundRouting(&self, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxReceiptOptions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxReceiptOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>() -> IFaxReceiptOptions_Vtbl {
        unsafe extern "system" fn AuthenticationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SMTPServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SMTPServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMTPServer(::core::mem::transmute(&bstrsmtpserver)).into()
        }
        unsafe extern "system" fn SMTPPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SMTPPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsmtpport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMTPPort(::core::mem::transmute_copy(&lsmtpport)).into()
        }
        unsafe extern "system" fn SMTPSender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SMTPSender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpsender, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPSender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMTPSender(::core::mem::transmute(&bstrsmtpsender)).into()
        }
        unsafe extern "system" fn SMTPUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SMTPUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtpuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMTPUser(::core::mem::transmute(&bstrsmtpuser)).into()
        }
        unsafe extern "system" fn AllowedReceipts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowedReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallowedreceipts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedReceipts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowedReceipts(::core::mem::transmute_copy(&allowedreceipts)).into()
        }
        unsafe extern "system" fn SMTPPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SMTPPassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsmtppassword, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMTPPassword(::core::mem::transmute(&bstrsmtppassword)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn UseForInboundRouting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseForInboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuseforinboundrouting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseForInboundRouting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buseforinboundrouting: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseForInboundRouting(::core::mem::transmute_copy(&buseforinboundrouting)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AuthenticationType: AuthenticationType::<Identity, Impl, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, Impl, OFFSET>,
            SMTPServer: SMTPServer::<Identity, Impl, OFFSET>,
            SetSMTPServer: SetSMTPServer::<Identity, Impl, OFFSET>,
            SMTPPort: SMTPPort::<Identity, Impl, OFFSET>,
            SetSMTPPort: SetSMTPPort::<Identity, Impl, OFFSET>,
            SMTPSender: SMTPSender::<Identity, Impl, OFFSET>,
            SetSMTPSender: SetSMTPSender::<Identity, Impl, OFFSET>,
            SMTPUser: SMTPUser::<Identity, Impl, OFFSET>,
            SetSMTPUser: SetSMTPUser::<Identity, Impl, OFFSET>,
            AllowedReceipts: AllowedReceipts::<Identity, Impl, OFFSET>,
            SetAllowedReceipts: SetAllowedReceipts::<Identity, Impl, OFFSET>,
            SMTPPassword: SMTPPassword::<Identity, Impl, OFFSET>,
            SetSMTPPassword: SetSMTPPassword::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            UseForInboundRouting: UseForInboundRouting::<Identity, Impl, OFFSET>,
            SetUseForInboundRouting: SetUseForInboundRouting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxReceiptOptions as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxRecipient_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FaxNumber(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxRecipient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxRecipient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>() -> IFaxRecipient_Vtbl {
        unsafe extern "system" fn FaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfaxnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFaxNumber(::core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxRecipient as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxRecipients_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<IFaxRecipient>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, bstrfaxnumber: &::windows::core::BSTR, bstrrecipientname: &::windows::core::BSTR) -> ::windows::core::Result<IFaxRecipient>;
    fn Remove(&self, lindex: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxRecipients {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxRecipients_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>() -> IFaxRecipients_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrrecipientname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppfaxrecipient: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&bstrfaxnumber), ::core::mem::transmute(&bstrrecipientname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxrecipient, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxRecipients as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSecurity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GrantedRights(&self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn InformationType(&self) -> ::windows::core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxSecurity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>() -> IFaxSecurity_Vtbl {
        unsafe extern "system" fn Descriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Descriptor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdescriptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescriptor(::core::mem::transmute(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GrantedRights() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrantedrights, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn InformationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InformationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plinformationtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInformationType(::core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSecurity as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSecurity2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDescriptor(&self, vdescriptor: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GrantedRights(&self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM_2>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
    fn InformationType(&self) -> ::windows::core::Result<i32>;
    fn SetInformationType(&self, linformationtype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxSecurity2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSecurity2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>() -> IFaxSecurity2_Vtbl {
        unsafe extern "system" fn Descriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Descriptor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvdescriptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescriptor(::core::mem::transmute(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GrantedRights() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrantedrights, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        unsafe extern "system" fn InformationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InformationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plinformationtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInformationType(::core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Descriptor: Descriptor::<Identity, Impl, OFFSET>,
            SetDescriptor: SetDescriptor::<Identity, Impl, OFFSET>,
            GrantedRights: GrantedRights::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            InformationType: InformationType::<Identity, Impl, OFFSET>,
            SetInformationType: SetInformationType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSecurity2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSender_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BillingCode(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetBillingCode(&self, bstrbillingcode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn City(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCity(&self, bstrcity: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCompany(&self, bstrcompany: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCountry(&self, bstrcountry: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Department(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDepartment(&self, bstrdepartment: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Email(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetEmail(&self, bstremail: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFaxNumber(&self, bstrfaxnumber: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn HomePhone(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetHomePhone(&self, bstrhomephone: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTSID(&self, bstrtsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OfficePhone(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOfficePhone(&self, bstrofficephone: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OfficeLocation(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOfficeLocation(&self, bstrofficelocation: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetState(&self, bstrstate: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn StreetAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetStreetAddress(&self, bstrstreetaddress: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTitle(&self, bstrtitle: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ZipCode(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetZipCode(&self, bstrzipcode: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn LoadDefaultSender(&self) -> ::windows::core::Result<()>;
    fn SaveDefaultSender(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxSender {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSender_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>() -> IFaxSender_Vtbl {
        unsafe extern "system" fn BillingCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BillingCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbillingcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBillingCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbillingcode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBillingCode(::core::mem::transmute(&bstrbillingcode)).into()
        }
        unsafe extern "system" fn City<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.City() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcity: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCity(::core::mem::transmute(&bstrcity)).into()
        }
        unsafe extern "system" fn Company<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcompany: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Company() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcompany, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcompany: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompany(::core::mem::transmute(&bstrcompany)).into()
        }
        unsafe extern "system" fn Country<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountry: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Country() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcountry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountry: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCountry(::core::mem::transmute(&bstrcountry)).into()
        }
        unsafe extern "system" fn Department<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Department() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdepartment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDepartment(::core::mem::transmute(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Email<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstremail: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Email() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstremail, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremail: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEmail(::core::mem::transmute(&bstremail)).into()
        }
        unsafe extern "system" fn FaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfaxnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFaxNumber(::core::mem::transmute(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn HomePhone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HomePhone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhomephone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePhone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomephone: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHomePhone(::core::mem::transmute(&bstrhomephone)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn TSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTSID(::core::mem::transmute(&bstrtsid)).into()
        }
        unsafe extern "system" fn OfficePhone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OfficePhone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrofficephone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficePhone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficephone: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOfficePhone(::core::mem::transmute(&bstrofficephone)).into()
        }
        unsafe extern "system" fn OfficeLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OfficeLocation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrofficelocation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOfficeLocation(::core::mem::transmute(&bstrofficelocation)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstate: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstate: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute(&bstrstate)).into()
        }
        unsafe extern "system" fn StreetAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StreetAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstreetaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreetAddress(::core::mem::transmute(&bstrstreetaddress)).into()
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtitle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn ZipCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZipCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrzipcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZipCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrzipcode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZipCode(::core::mem::transmute(&bstrzipcode)).into()
        }
        unsafe extern "system" fn LoadDefaultSender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadDefaultSender().into()
        }
        unsafe extern "system" fn SaveDefaultSender<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveDefaultSender().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BillingCode: BillingCode::<Identity, Impl, OFFSET>,
            SetBillingCode: SetBillingCode::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            SetCity: SetCity::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            SetCompany: SetCompany::<Identity, Impl, OFFSET>,
            Country: Country::<Identity, Impl, OFFSET>,
            SetCountry: SetCountry::<Identity, Impl, OFFSET>,
            Department: Department::<Identity, Impl, OFFSET>,
            SetDepartment: SetDepartment::<Identity, Impl, OFFSET>,
            Email: Email::<Identity, Impl, OFFSET>,
            SetEmail: SetEmail::<Identity, Impl, OFFSET>,
            FaxNumber: FaxNumber::<Identity, Impl, OFFSET>,
            SetFaxNumber: SetFaxNumber::<Identity, Impl, OFFSET>,
            HomePhone: HomePhone::<Identity, Impl, OFFSET>,
            SetHomePhone: SetHomePhone::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            TSID: TSID::<Identity, Impl, OFFSET>,
            SetTSID: SetTSID::<Identity, Impl, OFFSET>,
            OfficePhone: OfficePhone::<Identity, Impl, OFFSET>,
            SetOfficePhone: SetOfficePhone::<Identity, Impl, OFFSET>,
            OfficeLocation: OfficeLocation::<Identity, Impl, OFFSET>,
            SetOfficeLocation: SetOfficeLocation::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            StreetAddress: StreetAddress::<Identity, Impl, OFFSET>,
            SetStreetAddress: SetStreetAddress::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            ZipCode: ZipCode::<Identity, Impl, OFFSET>,
            SetZipCode: SetZipCode::<Identity, Impl, OFFSET>,
            LoadDefaultSender: LoadDefaultSender::<Identity, Impl, OFFSET>,
            SaveDefaultSender: SaveDefaultSender::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSender as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&self, bstrservername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ServerName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn GetDeviceProviders(&self) -> ::windows::core::Result<IFaxDeviceProviders>;
    fn GetDevices(&self) -> ::windows::core::Result<IFaxDevices>;
    fn InboundRouting(&self) -> ::windows::core::Result<IFaxInboundRouting>;
    fn Folders(&self) -> ::windows::core::Result<IFaxFolders>;
    fn LoggingOptions(&self) -> ::windows::core::Result<IFaxLoggingOptions>;
    fn MajorVersion(&self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&self) -> ::windows::core::Result<i32>;
    fn Debug(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Activity(&self) -> ::windows::core::Result<IFaxActivity>;
    fn OutboundRouting(&self) -> ::windows::core::Result<IFaxOutboundRouting>;
    fn ReceiptOptions(&self) -> ::windows::core::Result<IFaxReceiptOptions>;
    fn Security(&self) -> ::windows::core::Result<IFaxSecurity>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn GetExtensionProperty(&self, bstrguid: &::windows::core::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtensionProperty(&self, bstrguid: &::windows::core::BSTR, vproperty: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ListenToServerEvents(&self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn RegisterDeviceProvider(&self, bstrguid: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR, bstrimagename: &::windows::core::BSTR, tspname: &::windows::core::BSTR, lfspiversion: i32) -> ::windows::core::Result<()>;
    fn UnregisterDeviceProvider(&self, bstruniquename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn RegisterInboundRoutingExtension(&self, bstrextensionname: &::windows::core::BSTR, bstrfriendlyname: &::windows::core::BSTR, bstrimagename: &::windows::core::BSTR, vmethods: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UnregisterInboundRoutingExtension(&self, bstrextensionuniquename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn RegisteredEvents(&self) -> ::windows::core::Result<FAX_SERVER_EVENTS_TYPE_ENUM>;
    fn APIVersion(&self) -> ::windows::core::Result<FAX_SERVER_APIVERSION_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxServer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>() -> IFaxServer_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&bstrservername)).into()
        }
        unsafe extern "system" fn ServerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrservername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrservername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceProviders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceProviders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdeviceproviders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxdevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundRouting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxinboundrouting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxfolders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Folders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaxfolders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxloggingoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Debug() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdebug, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxactivity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundRouting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxoutboundrouting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiptOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxreceiptoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn GetExtensionProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtensionProperty(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtensionProperty(::core::mem::transmute(&bstrguid), ::core::mem::transmute(&vproperty)).into()
        }
        unsafe extern "system" fn ListenToServerEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ListenToServerEvents(::core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisterDeviceProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows::core::BSTR>, tspname: ::std::mem::MaybeUninit<::windows::core::BSTR>, lfspiversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterDeviceProvider(::core::mem::transmute(&bstrguid), ::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute(&bstrimagename), ::core::mem::transmute(&tspname), ::core::mem::transmute_copy(&lfspiversion)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruniquename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDeviceProvider(::core::mem::transmute(&bstruniquename)).into()
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrimagename: ::std::mem::MaybeUninit<::windows::core::BSTR>, vmethods: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterInboundRoutingExtension(::core::mem::transmute(&bstrextensionname), ::core::mem::transmute(&bstrfriendlyname), ::core::mem::transmute(&bstrimagename), ::core::mem::transmute(&vmethods)).into()
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterInboundRoutingExtension(::core::mem::transmute(&bstrextensionuniquename)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisteredEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtypes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn APIVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.APIVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papiversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            ServerName: ServerName::<Identity, Impl, OFFSET>,
            GetDeviceProviders: GetDeviceProviders::<Identity, Impl, OFFSET>,
            GetDevices: GetDevices::<Identity, Impl, OFFSET>,
            InboundRouting: InboundRouting::<Identity, Impl, OFFSET>,
            Folders: Folders::<Identity, Impl, OFFSET>,
            LoggingOptions: LoggingOptions::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            MajorBuild: MajorBuild::<Identity, Impl, OFFSET>,
            MinorBuild: MinorBuild::<Identity, Impl, OFFSET>,
            Debug: Debug::<Identity, Impl, OFFSET>,
            Activity: Activity::<Identity, Impl, OFFSET>,
            OutboundRouting: OutboundRouting::<Identity, Impl, OFFSET>,
            ReceiptOptions: ReceiptOptions::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Identity, Impl, OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Identity, Impl, OFFSET>,
            ListenToServerEvents: ListenToServerEvents::<Identity, Impl, OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Identity, Impl, OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Identity, Impl, OFFSET>,
            RegisterInboundRoutingExtension: RegisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            UnregisterInboundRoutingExtension: UnregisterInboundRoutingExtension::<Identity, Impl, OFFSET>,
            RegisteredEvents: RegisteredEvents::<Identity, Impl, OFFSET>,
            APIVersion: APIVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServer as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServer2_Impl: Sized + IFaxServer_Impl {
    fn Configuration(&self) -> ::windows::core::Result<IFaxConfiguration>;
    fn CurrentAccount(&self) -> ::windows::core::Result<IFaxAccount>;
    fn FaxAccountSet(&self) -> ::windows::core::Result<IFaxAccountSet>;
    fn Security2(&self) -> ::windows::core::Result<IFaxSecurity2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxServer2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>() -> IFaxServer2_Vtbl {
        unsafe extern "system" fn Configuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxconfiguration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAccount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcurrentaccount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaxAccountSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FaxAccountSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxaccountset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfaxsecurity2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFaxServer_Vtbl::new::<Identity, Impl, OFFSET>(),
            Configuration: Configuration::<Identity, Impl, OFFSET>,
            CurrentAccount: CurrentAccount::<Identity, Impl, OFFSET>,
            FaxAccountSet: FaxAccountSet::<Identity, Impl, OFFSET>,
            Security2: Security2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServer2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IFaxServer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServerNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxServerNotify {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServerNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify_Impl, const OFFSET: isize>() -> IFaxServerNotify_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServerNotify as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServerNotify2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobRemoved(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobChanged(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnOutgoingJobAdded(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobRemoved(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobChanged(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrjobid: &::windows::core::BSTR, pjobstatus: ::core::option::Option<&IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnIncomingMessageAdded(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingMessageRemoved(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageAdded(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageRemoved(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, bstrmessageid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnReceiptOptionsChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnActivityLoggingConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnSecurityConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnEventLoggingConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutgoingQueueConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutgoingArchiveConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnIncomingArchiveConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnDevicesConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutboundRoutingGroupsConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutboundRoutingRulesConfigChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnServerActivityChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::Result<()>;
    fn OnQueuesStatusChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OnNewCall(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, lcallid: i32, ldeviceid: i32, bstrcallerid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OnServerShutDown(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnDeviceStatusChange(&self, pfaxserver: ::core::option::Option<&IFaxServer2>, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>;
    fn OnGeneralServerConfigChanged(&self, pfaxserver: ::core::option::Option<&IFaxServer2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFaxServerNotify2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServerNotify2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>() -> IFaxServerNotify2_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobAdded(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobRemoved(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingJobChanged(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid), ::windows::core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobAdded(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobRemoved(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrjobid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pjobstatus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingJobChanged(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrjobid), ::windows::core::from_raw_borrowed(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingMessageAdded(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingMessageRemoved(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingMessageAdded(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, bstrmessageid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingMessageRemoved(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnReceiptOptionsChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityLoggingConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnSecurityConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSecurityConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEventLoggingConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingQueueConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutgoingArchiveConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIncomingArchiveConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDevicesConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDevicesConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutboundRoutingGroupsConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutboundRoutingRulesConfigChange(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnServerActivityChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnServerActivityChange(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&lincomingmessages), ::core::mem::transmute_copy(&lroutingmessages), ::core::mem::transmute_copy(&loutgoingmessages), ::core::mem::transmute_copy(&lqueuedmessages)).into()
        }
        unsafe extern "system" fn OnQueuesStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, boutgoingqueueblocked: super::super::Foundation::VARIANT_BOOL, boutgoingqueuepaused: super::super::Foundation::VARIANT_BOOL, bincomingqueueblocked: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQueuesStatusChange(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&boutgoingqueueblocked), ::core::mem::transmute_copy(&boutgoingqueuepaused), ::core::mem::transmute_copy(&bincomingqueueblocked)).into()
        }
        unsafe extern "system" fn OnNewCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, lcallid: i32, ldeviceid: i32, bstrcallerid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNewCall(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&lcallid), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute(&bstrcallerid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnServerShutDown(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDeviceStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void, ldeviceid: i32, bpoweredoff: super::super::Foundation::VARIANT_BOOL, bsending: super::super::Foundation::VARIANT_BOOL, breceiving: super::super::Foundation::VARIANT_BOOL, bringing: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDeviceStatusChange(::windows::core::from_raw_borrowed(&pfaxserver), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&bpoweredoff), ::core::mem::transmute_copy(&bsending), ::core::mem::transmute_copy(&breceiving), ::core::mem::transmute_copy(&bringing)).into()
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnGeneralServerConfigChanged(::windows::core::from_raw_borrowed(&pfaxserver)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Identity, Impl, OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Identity, Impl, OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Identity, Impl, OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Identity, Impl, OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Identity, Impl, OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Identity, Impl, OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Identity, Impl, OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Identity, Impl, OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Identity, Impl, OFFSET>,
            OnReceiptOptionsChange: OnReceiptOptionsChange::<Identity, Impl, OFFSET>,
            OnActivityLoggingConfigChange: OnActivityLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnSecurityConfigChange: OnSecurityConfigChange::<Identity, Impl, OFFSET>,
            OnEventLoggingConfigChange: OnEventLoggingConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingQueueConfigChange: OnOutgoingQueueConfigChange::<Identity, Impl, OFFSET>,
            OnOutgoingArchiveConfigChange: OnOutgoingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnIncomingArchiveConfigChange: OnIncomingArchiveConfigChange::<Identity, Impl, OFFSET>,
            OnDevicesConfigChange: OnDevicesConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingGroupsConfigChange: OnOutboundRoutingGroupsConfigChange::<Identity, Impl, OFFSET>,
            OnOutboundRoutingRulesConfigChange: OnOutboundRoutingRulesConfigChange::<Identity, Impl, OFFSET>,
            OnServerActivityChange: OnServerActivityChange::<Identity, Impl, OFFSET>,
            OnQueuesStatusChange: OnQueuesStatusChange::<Identity, Impl, OFFSET>,
            OnNewCall: OnNewCall::<Identity, Impl, OFFSET>,
            OnServerShutDown: OnServerShutDown::<Identity, Impl, OFFSET>,
            OnDeviceStatusChange: OnDeviceStatusChange::<Identity, Impl, OFFSET>,
            OnGeneralServerConfigChanged: OnGeneralServerConfigChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServerNotify2 as ::windows::core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDevice_Impl: Sized {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: &::windows::core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::Result<()>;
    fn GetCapabilities(&self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::Result<()>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()>;
    fn DeviceReset(&self) -> ::windows::core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&self) -> ::windows::core::Result<u32>;
    fn LockDevice(&self, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn UnLockDevice(&self) -> ::windows::core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn Subscribe(&self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::Result<()>;
    fn GetLastNotificationData(&self, lpnotify: *mut STINOTIFY) -> ::windows::core::Result<()>;
    fn UnSubscribe(&self) -> ::windows::core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::RuntimeName for IStiDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>() -> IStiDevice_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: ::windows::core::PCWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hinst), ::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&dwversion), ::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCapabilities(::core::mem::transmute_copy(&pdevcaps)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceReset().into()
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Diagnostic(::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastdeviceerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockDevice(::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn UnLockDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnLockDevice().into()
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn Subscribe<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Subscribe(::core::mem::transmute_copy(&lpsubsribe)).into()
        }
        unsafe extern "system" fn GetLastNotificationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastNotificationData(::core::mem::transmute_copy(&lpnotify)).into()
        }
        unsafe extern "system" fn UnSubscribe<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnSubscribe().into()
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastErrorInfo(::core::mem::transmute_copy(&plasterrorinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            Subscribe: Subscribe::<Identity, Impl, OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Identity, Impl, OFFSET>,
            UnSubscribe: UnSubscribe::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDeviceControl_Impl: Sized {
    fn Initialize(&self, dwdevicetype: u32, dwmode: u32, pwszportname: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawDeviceControl(&self, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&self, lpdwlasterror: *mut u32) -> ::windows::core::Result<()>;
    fn GetMyDevicePortName(&self, lpszdevicepath: ::windows::core::PWSTR, cwdevicepathsize: u32) -> ::windows::core::Result<()>;
    fn GetMyDeviceHandle(&self, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetMyDeviceOpenMode(&self, pdwopenmode: *mut u32) -> ::windows::core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &::windows::core::PCWSTR, dwerrorcode: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::RuntimeName for IStiDeviceControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl IStiDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>() -> IStiDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&dwdevicetype), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute(&pwszportname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawDeviceControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawDeviceControl(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastError(::core::mem::transmute_copy(&lpdwlasterror)).into()
        }
        unsafe extern "system" fn GetMyDevicePortName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdevicepath: ::windows::core::PWSTR, cwdevicepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMyDevicePortName(::core::mem::transmute_copy(&lpszdevicepath), ::core::mem::transmute_copy(&cwdevicepathsize)).into()
        }
        unsafe extern "system" fn GetMyDeviceHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMyDeviceHandle(::core::mem::transmute_copy(&lph)).into()
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMyDeviceOpenMode(::core::mem::transmute_copy(&pdwopenmode)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows::core::PCWSTR, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToErrorLog(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pszmessage), ::core::mem::transmute_copy(&dwerrorcode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            RawDeviceControl: RawDeviceControl::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GetMyDevicePortName: GetMyDevicePortName::<Identity, Impl, OFFSET>,
            GetMyDeviceHandle: GetMyDeviceHandle::<Identity, Impl, OFFSET>,
            GetMyDeviceOpenMode: GetMyDeviceOpenMode::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiDeviceControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
pub trait IStiUSD_Impl: Sized {
    fn Initialize(&self, pheldcb: ::core::option::Option<&IStiDeviceControl>, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetCapabilities(&self) -> ::windows::core::Result<STI_USD_CAPS>;
    fn GetStatus(&self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()>;
    fn DeviceReset(&self) -> ::windows::core::Result<()>;
    fn Diagnostic(&self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()>;
    fn Escape(&self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&self) -> ::windows::core::Result<u32>;
    fn LockDevice(&self) -> ::windows::core::Result<()>;
    fn UnLockDevice(&self) -> ::windows::core::Result<()>;
    fn RawReadData(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn SetNotificationHandle(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetNotificationData(&self, lpnotify: *mut STINOTIFY) -> ::windows::core::Result<()>;
    fn GetLastErrorInfo(&self, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IStiUSD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl IStiUSD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>() -> IStiUSD_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheldcb: *mut ::core::ffi::c_void, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&pheldcb), ::core::mem::transmute_copy(&dwstiversion), ::core::mem::transmute_copy(&hparameterskey)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevcaps, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceReset().into()
        }
        unsafe extern "system" fn Diagnostic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Diagnostic(::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Escape(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&cboutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlastdeviceerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockDevice().into()
        }
        unsafe extern "system" fn UnLockDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnLockDevice().into()
        }
        unsafe extern "system" fn RawReadData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNotificationHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotificationHandle(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetNotificationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNotificationData(::core::mem::transmute_copy(&lpnotify)).into()
        }
        unsafe extern "system" fn GetLastErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLastErrorInfo(::core::mem::transmute_copy(&plasterrorinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            DeviceReset: DeviceReset::<Identity, Impl, OFFSET>,
            Diagnostic: Diagnostic::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            LockDevice: LockDevice::<Identity, Impl, OFFSET>,
            UnLockDevice: UnLockDevice::<Identity, Impl, OFFSET>,
            RawReadData: RawReadData::<Identity, Impl, OFFSET>,
            RawWriteData: RawWriteData::<Identity, Impl, OFFSET>,
            RawReadCommand: RawReadCommand::<Identity, Impl, OFFSET>,
            RawWriteCommand: RawWriteCommand::<Identity, Impl, OFFSET>,
            SetNotificationHandle: SetNotificationHandle::<Identity, Impl, OFFSET>,
            GetNotificationData: GetNotificationData::<Identity, Impl, OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiUSD as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Fax\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStillImageW_Impl: Sized {
    fn Initialize(&self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::Result<()>;
    fn GetDeviceList(&self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&self, pwszdevicename: &::windows::core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateDevice(&self, pwszdevicename: &::windows::core::PCWSTR, dwmode: u32, pdevice: *mut ::core::option::Option<IStiDevice>, punkouter: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDeviceValue(&self, pwszdevicename: &::windows::core::PCWSTR, pvaluename: &::windows::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::Result<()>;
    fn SetDeviceValue(&self, pwszdevicename: &::windows::core::PCWSTR, pvaluename: &::windows::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn GetSTILaunchInformation(&self, pwszdevicename: ::windows::core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn RegisterLaunchApplication(&self, pwszappname: &::windows::core::PCWSTR, pwszcommandline: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UnregisterLaunchApplication(&self, pwszappname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn EnableHwNotifications(&self, pwszdevicename: &::windows::core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetHwNotificationState(&self, pwszdevicename: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn RefreshDeviceBus(&self, pwszdevicename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn LaunchApplicationForDevice(&self, pwszdevicename: &::windows::core::PCWSTR, pwszappname: &::windows::core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::Result<()>;
    fn SetupDeviceParameters(&self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::Result<()>;
    fn WriteToErrorLog(&self, dwmessagetype: u32, pszmessage: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IStillImageW {}
#[cfg(feature = "Win32_Foundation")]
impl IStillImageW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>() -> IStillImageW_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hinst), ::core::mem::transmute_copy(&dwversion)).into()
        }
        unsafe extern "system" fn GetDeviceList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceList(::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwitemsreturned), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceInfo(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn CreateDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, dwmode: u32, pdevice: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateDevice(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdevice), ::windows::core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetDeviceValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceValue(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn SetDeviceValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pvaluename: ::windows::core::PCWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceValue(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pvaluename), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetSTILaunchInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PWSTR, pdweventcode: *mut u32, pwszeventname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSTILaunchInformation(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pdweventcode), ::core::mem::transmute_copy(&pwszeventname)).into()
        }
        unsafe extern "system" fn RegisterLaunchApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: ::windows::core::PCWSTR, pwszcommandline: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterLaunchApplication(::core::mem::transmute(&pwszappname), ::core::mem::transmute(&pwszcommandline)).into()
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterLaunchApplication(::core::mem::transmute(&pwszappname)).into()
        }
        unsafe extern "system" fn EnableHwNotifications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableHwNotifications(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute_copy(&bnewstate)).into()
        }
        unsafe extern "system" fn GetHwNotificationState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHwNotificationState(::core::mem::transmute(&pwszdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcurrentstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshDeviceBus(::core::mem::transmute(&pwszdevicename)).into()
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: ::windows::core::PCWSTR, pwszappname: ::windows::core::PCWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LaunchApplicationForDevice(::core::mem::transmute(&pwszdevicename), ::core::mem::transmute(&pwszappname), ::core::mem::transmute_copy(&pstinotify)).into()
        }
        unsafe extern "system" fn SetupDeviceParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetupDeviceParameters(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteToErrorLog(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute(&pszmessage)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetDeviceList: GetDeviceList::<Identity, Impl, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
            GetDeviceValue: GetDeviceValue::<Identity, Impl, OFFSET>,
            SetDeviceValue: SetDeviceValue::<Identity, Impl, OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Identity, Impl, OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Identity, Impl, OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Identity, Impl, OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Identity, Impl, OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Identity, Impl, OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Identity, Impl, OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Identity, Impl, OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Identity, Impl, OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStillImageW as ::windows::core::ComInterface>::IID
    }
}
