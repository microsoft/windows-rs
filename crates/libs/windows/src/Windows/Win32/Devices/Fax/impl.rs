#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccount_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccountName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Folders(&mut self) -> ::windows::core::Result<IFaxAccountFolders>;
    fn ListenToAccountEvents(&mut self, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn RegisteredEvents(&mut self) -> ::windows::core::Result<FAX_ACCOUNT_EVENTS_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccount_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccount_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccount_Vtbl {
        unsafe extern "system" fn AccountName<Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraccountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstraccountname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenToAccountEvents<Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ListenToAccountEvents(::core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Impl: IFaxAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *pregisteredevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AccountName: AccountName::<Impl, IMPL_OFFSET>,
            Folders: Folders::<Impl, IMPL_OFFSET>,
            ListenToAccountEvents: ListenToAccountEvents::<Impl, IMPL_OFFSET>,
            RegisteredEvents: RegisteredEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&mut self) -> ::windows::core::Result<IFaxAccountOutgoingQueue>;
    fn IncomingQueue(&mut self) -> ::windows::core::Result<IFaxAccountIncomingQueue>;
    fn IncomingArchive(&mut self) -> ::windows::core::Result<IFaxAccountIncomingArchive>;
    fn OutgoingArchive(&mut self) -> ::windows::core::Result<IFaxAccountOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountFolders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountFolders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingarchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Impl: IFaxAccountFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingarchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Impl, IMPL_OFFSET>,
            IncomingQueue: IncomingQueue::<Impl, IMPL_OFFSET>,
            IncomingArchive: IncomingArchive::<Impl, IMPL_OFFSET>,
            OutgoingArchive: OutgoingArchive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountFolders as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&mut self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn GetMessages(&mut self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&mut self, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountIncomingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountIncomingArchive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountIncomingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizelow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizehigh = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingmessageiterator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxAccountIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(::core::mem::transmute_copy(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SizeLow: SizeLow::<Impl, IMPL_OFFSET>,
            SizeHigh: SizeHigh::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            GetMessages: GetMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountIncomingArchive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&mut self) -> ::windows::core::Result<IFaxIncomingJobs>;
    fn GetJob(&mut self, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountIncomingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountIncomingQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountIncomingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxAccountIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetJobs: GetJobs::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountIncomingQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountNotify_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SizeLow(&mut self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn GetMessages(&mut self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&mut self, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountOutgoingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountOutgoingArchive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountOutgoingArchive_Vtbl {
        unsafe extern "system" fn SizeLow<Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizelow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizehigh = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingmessageiterator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxAccountOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(::core::mem::transmute_copy(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SizeLow: SizeLow::<Impl, IMPL_OFFSET>,
            SizeHigh: SizeHigh::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            GetMessages: GetMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingArchive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetJobs(&mut self) -> ::windows::core::Result<IFaxOutgoingJobs>;
    fn GetJob(&mut self, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountOutgoingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountOutgoingQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountOutgoingQueue_Vtbl {
        unsafe extern "system" fn GetJobs<Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxAccountOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetJobs: GetJobs::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountOutgoingQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccountSet_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetAccounts(&mut self) -> ::windows::core::Result<IFaxAccounts>;
    fn GetAccount(&mut self, bstraccountname: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxAccount>;
    fn AddAccount(&mut self, bstraccountname: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxAccount>;
    fn RemoveAccount(&mut self, bstraccountname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccountSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccountSet_Vtbl {
        unsafe extern "system" fn GetAccounts<Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxaccounts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccount<Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccount(::core::mem::transmute_copy(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxaccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccount<Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAccount(::core::mem::transmute_copy(&bstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxaccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccount<Impl: IFaxAccountSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccount(::core::mem::transmute_copy(&bstraccountname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAccounts: GetAccounts::<Impl, IMPL_OFFSET>,
            GetAccount: GetAccount::<Impl, IMPL_OFFSET>,
            AddAccount: AddAccount::<Impl, IMPL_OFFSET>,
            RemoveAccount: RemoveAccount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccountSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxAccounts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxAccount>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxAccounts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccounts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxAccounts_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxaccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxAccounts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxAccounts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxActivity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IncomingMessages(&mut self) -> ::windows::core::Result<i32>;
    fn RoutingMessages(&mut self) -> ::windows::core::Result<i32>;
    fn OutgoingMessages(&mut self) -> ::windows::core::Result<i32>;
    fn QueuedMessages(&mut self) -> ::windows::core::Result<i32>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxActivity_Vtbl {
        unsafe extern "system" fn IncomingMessages<Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *plincomingmessages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingMessages<Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *plroutingmessages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingMessages<Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *ploutgoingmessages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueuedMessages<Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueuedMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *plqueuedmessages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IncomingMessages: IncomingMessages::<Impl, IMPL_OFFSET>,
            RoutingMessages: RoutingMessages::<Impl, IMPL_OFFSET>,
            OutgoingMessages: OutgoingMessages::<Impl, IMPL_OFFSET>,
            QueuedMessages: QueuedMessages::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxActivityLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LogIncoming(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogIncoming(&mut self, blogincoming: i16) -> ::windows::core::Result<()>;
    fn LogOutgoing(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogOutgoing(&mut self, blogoutgoing: i16) -> ::windows::core::Result<()>;
    fn DatabasePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDatabasePath(&mut self, bstrdatabasepath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxActivityLogging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxActivityLogging_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxActivityLogging_Vtbl {
        unsafe extern "system" fn LogIncoming<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogincoming: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *pblogincoming = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogIncoming<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogincoming: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogIncoming(::core::mem::transmute_copy(&blogincoming)).into()
        }
        unsafe extern "system" fn LogOutgoing<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogOutgoing() {
                ::core::result::Result::Ok(ok__) => {
                    *pblogoutgoing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOutgoing<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogoutgoing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogOutgoing(::core::mem::transmute_copy(&blogoutgoing)).into()
        }
        unsafe extern "system" fn DatabasePath<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DatabasePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdatabasepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatabasePath<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDatabasePath(::core::mem::transmute_copy(&bstrdatabasepath)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxActivityLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LogIncoming: LogIncoming::<Impl, IMPL_OFFSET>,
            SetLogIncoming: SetLogIncoming::<Impl, IMPL_OFFSET>,
            LogOutgoing: LogOutgoing::<Impl, IMPL_OFFSET>,
            SetLogOutgoing: SetLogOutgoing::<Impl, IMPL_OFFSET>,
            DatabasePath: DatabasePath::<Impl, IMPL_OFFSET>,
            SetDatabasePath: SetDatabasePath::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxActivityLogging as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseArchive(&mut self, busearchive: i16) -> ::windows::core::Result<()>;
    fn ArchiveLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetArchiveLocation(&mut self, bstrarchivelocation: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&mut self) -> ::windows::core::Result<i16>;
    fn SetSizeQuotaWarning(&mut self, bsizequotawarning: i16) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&mut self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&mut self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn ArchiveAgeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetArchiveAgeLimit(&mut self, larchiveagelimit: i32) -> ::windows::core::Result<()>;
    fn ArchiveSizeLow(&mut self) -> ::windows::core::Result<i32>;
    fn ArchiveSizeHigh(&mut self) -> ::windows::core::Result<i32>;
    fn OutgoingQueueBlocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetOutgoingQueueBlocked(&mut self, boutgoingblocked: i16) -> ::windows::core::Result<()>;
    fn OutgoingQueuePaused(&mut self) -> ::windows::core::Result<i16>;
    fn SetOutgoingQueuePaused(&mut self, boutgoingpaused: i16) -> ::windows::core::Result<()>;
    fn AllowPersonalCoverPages(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowPersonalCoverPages(&mut self, ballowpersonalcoverpages: i16) -> ::windows::core::Result<()>;
    fn UseDeviceTSID(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseDeviceTSID(&mut self, busedevicetsid: i16) -> ::windows::core::Result<()>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn SetRetries(&mut self, lretries: i32) -> ::windows::core::Result<()>;
    fn RetryDelay(&mut self) -> ::windows::core::Result<i32>;
    fn SetRetryDelay(&mut self, lretrydelay: i32) -> ::windows::core::Result<()>;
    fn DiscountRateStart(&mut self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateStart(&mut self, datediscountratestart: f64) -> ::windows::core::Result<()>;
    fn DiscountRateEnd(&mut self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateEnd(&mut self, datediscountrateend: f64) -> ::windows::core::Result<()>;
    fn OutgoingQueueAgeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetOutgoingQueueAgeLimit(&mut self, loutgoingqueueagelimit: i32) -> ::windows::core::Result<()>;
    fn Branding(&mut self) -> ::windows::core::Result<i16>;
    fn SetBranding(&mut self, bbranding: i16) -> ::windows::core::Result<()>;
    fn IncomingQueueBlocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetIncomingQueueBlocked(&mut self, bincomingblocked: i16) -> ::windows::core::Result<()>;
    fn AutoCreateAccountOnConnect(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoCreateAccountOnConnect(&mut self, bautocreateaccountonconnect: i16) -> ::windows::core::Result<()>;
    fn IncomingFaxesArePublic(&mut self) -> ::windows::core::Result<i16>;
    fn SetIncomingFaxesArePublic(&mut self, bincomingfaxesarepublic: i16) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxConfiguration_Vtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusearchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveLocation<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrarchivelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveLocation<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArchiveLocation(::core::mem::transmute_copy(&bstrarchivelocation)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsizequotawarning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *plhighquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *pllowquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn ArchiveAgeLimit<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveAgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *plarchiveagelimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArchiveAgeLimit(::core::mem::transmute_copy(&larchiveagelimit)).into()
        }
        unsafe extern "system" fn ArchiveSizeLow<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveSizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizelow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveSizeHigh<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveSizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizehigh = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueueBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pboutgoingblocked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingQueueBlocked(::core::mem::transmute_copy(&boutgoingblocked)).into()
        }
        unsafe extern "system" fn OutgoingQueuePaused<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueuePaused() {
                ::core::result::Result::Ok(ok__) => {
                    *pboutgoingpaused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingpaused: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingQueuePaused(::core::mem::transmute_copy(&boutgoingpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPersonalCoverPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pballowpersonalcoverpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowPersonalCoverPages(::core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDeviceTSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusedevicetsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseDeviceTSID(::core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetries(::core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *plretrydelay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetryDelay(::core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatediscountratestart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscountRateStart(::core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatediscountrateend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscountRateEnd(::core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueueAgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *ploutgoingqueueagelimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingQueueAgeLimit(::core::mem::transmute_copy(&loutgoingqueueagelimit)).into()
        }
        unsafe extern "system" fn Branding<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Branding() {
                ::core::result::Result::Ok(ok__) => {
                    *pbbranding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBranding(::core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn IncomingQueueBlocked<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueueBlocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pbincomingblocked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingQueueBlocked(::core::mem::transmute_copy(&bincomingblocked)).into()
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoCreateAccountOnConnect() {
                ::core::result::Result::Ok(ok__) => {
                    *pbautocreateaccountonconnect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoCreateAccountOnConnect(::core::mem::transmute_copy(&bautocreateaccountonconnect)).into()
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingFaxesArePublic() {
                ::core::result::Result::Ok(ok__) => {
                    *pbincomingfaxesarepublic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingFaxesArePublic(::core::mem::transmute_copy(&bincomingfaxesarepublic)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UseArchive: UseArchive::<Impl, IMPL_OFFSET>,
            SetUseArchive: SetUseArchive::<Impl, IMPL_OFFSET>,
            ArchiveLocation: ArchiveLocation::<Impl, IMPL_OFFSET>,
            SetArchiveLocation: SetArchiveLocation::<Impl, IMPL_OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Impl, IMPL_OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Impl, IMPL_OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            ArchiveAgeLimit: ArchiveAgeLimit::<Impl, IMPL_OFFSET>,
            SetArchiveAgeLimit: SetArchiveAgeLimit::<Impl, IMPL_OFFSET>,
            ArchiveSizeLow: ArchiveSizeLow::<Impl, IMPL_OFFSET>,
            ArchiveSizeHigh: ArchiveSizeHigh::<Impl, IMPL_OFFSET>,
            OutgoingQueueBlocked: OutgoingQueueBlocked::<Impl, IMPL_OFFSET>,
            SetOutgoingQueueBlocked: SetOutgoingQueueBlocked::<Impl, IMPL_OFFSET>,
            OutgoingQueuePaused: OutgoingQueuePaused::<Impl, IMPL_OFFSET>,
            SetOutgoingQueuePaused: SetOutgoingQueuePaused::<Impl, IMPL_OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Impl, IMPL_OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Impl, IMPL_OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Impl, IMPL_OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            SetRetries: SetRetries::<Impl, IMPL_OFFSET>,
            RetryDelay: RetryDelay::<Impl, IMPL_OFFSET>,
            SetRetryDelay: SetRetryDelay::<Impl, IMPL_OFFSET>,
            DiscountRateStart: DiscountRateStart::<Impl, IMPL_OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Impl, IMPL_OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Impl, IMPL_OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Impl, IMPL_OFFSET>,
            OutgoingQueueAgeLimit: OutgoingQueueAgeLimit::<Impl, IMPL_OFFSET>,
            SetOutgoingQueueAgeLimit: SetOutgoingQueueAgeLimit::<Impl, IMPL_OFFSET>,
            Branding: Branding::<Impl, IMPL_OFFSET>,
            SetBranding: SetBranding::<Impl, IMPL_OFFSET>,
            IncomingQueueBlocked: IncomingQueueBlocked::<Impl, IMPL_OFFSET>,
            SetIncomingQueueBlocked: SetIncomingQueueBlocked::<Impl, IMPL_OFFSET>,
            AutoCreateAccountOnConnect: AutoCreateAccountOnConnect::<Impl, IMPL_OFFSET>,
            SetAutoCreateAccountOnConnect: SetAutoCreateAccountOnConnect::<Impl, IMPL_OFFSET>,
            IncomingFaxesArePublic: IncomingFaxesArePublic::<Impl, IMPL_OFFSET>,
            SetIncomingFaxesArePublic: SetIncomingFaxesArePublic::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProviderUniqueName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PoweredOff(&mut self) -> ::windows::core::Result<i16>;
    fn ReceivingNow(&mut self) -> ::windows::core::Result<i16>;
    fn SendingNow(&mut self) -> ::windows::core::Result<i16>;
    fn UsedRoutingMethods(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SendEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetSendEnabled(&mut self, bsendenabled: i16) -> ::windows::core::Result<()>;
    fn ReceiveMode(&mut self) -> ::windows::core::Result<FAX_DEVICE_RECEIVE_MODE_ENUM>;
    fn SetReceiveMode(&mut self, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::Result<()>;
    fn RingsBeforeAnswer(&mut self) -> ::windows::core::Result<i32>;
    fn SetRingsBeforeAnswer(&mut self, lringsbeforeanswer: i32) -> ::windows::core::Result<()>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCSID(&mut self, bstrcsid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTSID(&mut self, bstrtsid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetExtensionProperty(&mut self, bstrguid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtensionProperty(&mut self, bstrguid: super::super::Foundation::BSTR, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UseRoutingMethod(&mut self, bstrmethodguid: super::super::Foundation::BSTR, buse: i16) -> ::windows::core::Result<()>;
    fn RingingNow(&mut self) -> ::windows::core::Result<i16>;
    fn AnswerCall(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDevice_Vtbl {
        unsafe extern "system" fn Id<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *plid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderUniqueName<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderUniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprovideruniquename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PoweredOff<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PoweredOff() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpoweredoff = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivingNow<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivingNow() {
                ::core::result::Result::Ok(ok__) => {
                    *pbreceivingnow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendingNow<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendingNow() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsendingnow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedRoutingMethods<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedRoutingMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *pvusedroutingmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn SendEnabled<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsendenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendEnabled<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsendenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSendEnabled(::core::mem::transmute_copy(&bsendenabled)).into()
        }
        unsafe extern "system" fn ReceiveMode<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveMode() {
                ::core::result::Result::Ok(ok__) => {
                    *preceivemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveMode<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiveMode(::core::mem::transmute_copy(&receivemode)).into()
        }
        unsafe extern "system" fn RingsBeforeAnswer<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingsBeforeAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *plringsbeforeanswer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingsBeforeAnswer(::core::mem::transmute_copy(&lringsbeforeanswer)).into()
        }
        unsafe extern "system" fn CSID<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCSID<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCSID(::core::mem::transmute_copy(&bstrcsid)).into()
        }
        unsafe extern "system" fn TSID<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTSID(::core::mem::transmute_copy(&bstrtsid)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetExtensionProperty<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionProperty(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtensionProperty(::core::mem::transmute_copy(&bstrguid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        unsafe extern "system" fn UseRoutingMethod<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, buse: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseRoutingMethod(::core::mem::transmute_copy(&bstrmethodguid), ::core::mem::transmute_copy(&buse)).into()
        }
        unsafe extern "system" fn RingingNow<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbringingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingingNow() {
                ::core::result::Result::Ok(ok__) => {
                    *pbringingnow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerCall<Impl: IFaxDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AnswerCall().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DeviceName: DeviceName::<Impl, IMPL_OFFSET>,
            ProviderUniqueName: ProviderUniqueName::<Impl, IMPL_OFFSET>,
            PoweredOff: PoweredOff::<Impl, IMPL_OFFSET>,
            ReceivingNow: ReceivingNow::<Impl, IMPL_OFFSET>,
            SendingNow: SendingNow::<Impl, IMPL_OFFSET>,
            UsedRoutingMethods: UsedRoutingMethods::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            SendEnabled: SendEnabled::<Impl, IMPL_OFFSET>,
            SetSendEnabled: SetSendEnabled::<Impl, IMPL_OFFSET>,
            ReceiveMode: ReceiveMode::<Impl, IMPL_OFFSET>,
            SetReceiveMode: SetReceiveMode::<Impl, IMPL_OFFSET>,
            RingsBeforeAnswer: RingsBeforeAnswer::<Impl, IMPL_OFFSET>,
            SetRingsBeforeAnswer: SetRingsBeforeAnswer::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            SetCSID: SetCSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            SetTSID: SetTSID::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Impl, IMPL_OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Impl, IMPL_OFFSET>,
            UseRoutingMethod: UseRoutingMethod::<Impl, IMPL_OFFSET>,
            RingingNow: RingingNow::<Impl, IMPL_OFFSET>,
            AnswerCall: AnswerCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceIds_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<i32>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, ldeviceid: i32) -> ::windows::core::Result<()>;
    fn Remove(&mut self, lindex: i32) -> ::windows::core::Result<()>;
    fn SetOrder(&mut self, ldeviceid: i32, lneworder: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceIds_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceIds_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDeviceIds_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pldeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&ldeviceid)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn SetOrder<Impl: IFaxDeviceIds_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrder(::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&lneworder)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            SetOrder: SetOrder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceIds as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceProvider_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImageName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UniqueName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TapiProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn Debug(&mut self) -> ::windows::core::Result<i16>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceIds(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDeviceProvider_Vtbl {
        unsafe extern "system" fn FriendlyName<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrimagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruniquename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapiProviderName<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapiProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtapiprovidername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdebug = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pliniterrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Impl: IFaxDeviceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIds() {
                ::core::result::Result::Ok(ok__) => {
                    *pvdeviceids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            ImageName: ImageName::<Impl, IMPL_OFFSET>,
            UniqueName: UniqueName::<Impl, IMPL_OFFSET>,
            TapiProviderName: TapiProviderName::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            MajorBuild: MajorBuild::<Impl, IMPL_OFFSET>,
            MinorBuild: MinorBuild::<Impl, IMPL_OFFSET>,
            Debug: Debug::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            InitErrorCode: InitErrorCode::<Impl, IMPL_OFFSET>,
            DeviceIds: DeviceIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDeviceProviders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxDeviceProvider>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDeviceProviders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceProviders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDeviceProviders_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdeviceprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxdeviceprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDeviceProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDeviceProviders as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxDevice>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ItemById(&mut self, lid: i32) -> ::windows::core::Result<IFaxDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDevices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDevices_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemById<Impl: IFaxDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemById(::core::mem::transmute_copy(&lid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxdevice = ::core::mem::transmute(ok__);
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
            ItemById: ItemById::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDevices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDocument_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Body(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBody(&mut self, bstrbody: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Sender(&mut self) -> ::windows::core::Result<IFaxSender>;
    fn Recipients(&mut self) -> ::windows::core::Result<IFaxRecipients>;
    fn CoverPage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCoverPage(&mut self, bstrcoverpage: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Subject(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubject(&mut self, bstrsubject: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Note(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNote(&mut self, bstrnote: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ScheduleTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetScheduleTime(&mut self, datescheduletime: f64) -> ::windows::core::Result<()>;
    fn ReceiptAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetReceiptAddress(&mut self, bstrreceiptaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DocumentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDocumentName(&mut self, bstrdocumentname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CallHandle(&mut self) -> ::windows::core::Result<i32>;
    fn SetCallHandle(&mut self, lcallhandle: i32) -> ::windows::core::Result<()>;
    fn CoverPageType(&mut self) -> ::windows::core::Result<FAX_COVERPAGE_TYPE_ENUM>;
    fn SetCoverPageType(&mut self, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn ScheduleType(&mut self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM>;
    fn SetScheduleType(&mut self, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn ReceiptType(&mut self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetReceiptType(&mut self, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn GroupBroadcastReceipts(&mut self) -> ::windows::core::Result<i16>;
    fn SetGroupBroadcastReceipts(&mut self, busegrouping: i16) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn SetPriority(&mut self, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn TapiConnection(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn putref_TapiConnection(&mut self, ptapiconnection: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Submit(&mut self, bstrfaxservername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn ConnectedSubmit(&mut self, pfaxserver: ::core::option::Option<IFaxServer>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AttachFaxToReceipt(&mut self) -> ::windows::core::Result<i16>;
    fn SetAttachFaxToReceipt(&mut self, battachfax: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDocument_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDocument_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDocument_Vtbl {
        unsafe extern "system" fn Body<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&bstrbody)).into()
        }
        unsafe extern "system" fn Sender<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxsender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxrecipients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoverPage<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcoverpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPage<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcoverpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoverPage(::core::mem::transmute_copy(&bstrcoverpage)).into()
        }
        unsafe extern "system" fn Subject<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&bstrsubject)).into()
        }
        unsafe extern "system" fn Note<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnote: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Note() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrnote = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNote<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnote: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNote(::core::mem::transmute_copy(&bstrnote)).into()
        }
        unsafe extern "system" fn ScheduleTime<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatescheduletime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleTime<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScheduleTime(::core::mem::transmute_copy(&datescheduletime)).into()
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreceiptaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptAddress<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiptAddress(::core::mem::transmute_copy(&bstrreceiptaddress)).into()
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocumentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentName<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdocumentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentName(::core::mem::transmute_copy(&bstrdocumentname)).into()
        }
        unsafe extern "system" fn CallHandle<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHandle<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallHandle(::core::mem::transmute_copy(&lcallhandle)).into()
        }
        unsafe extern "system" fn CoverPageType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoverPageType() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoverpagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPageType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCoverPageType(::core::mem::transmute_copy(&coverpagetype)).into()
        }
        unsafe extern "system" fn ScheduleType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleType() {
                ::core::result::Result::Ok(ok__) => {
                    *pscheduletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScheduleType(::core::mem::transmute_copy(&scheduletype)).into()
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    *preceipttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptType<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiptType(::core::mem::transmute_copy(&receipttype)).into()
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusegrouping: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupBroadcastReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusegrouping = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busegrouping: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupBroadcastReceipts(::core::mem::transmute_copy(&busegrouping)).into()
        }
        unsafe extern "system" fn Priority<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn TapiConnection<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapiConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiconnection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_TapiConnection<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptapiconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_TapiConnection(::core::mem::transmute(&ptapiconnection)).into()
        }
        unsafe extern "system" fn Submit<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(::core::mem::transmute_copy(&bstrfaxservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvfaxoutgoingjobids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedSubmit<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedSubmit(::core::mem::transmute(&pfaxserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvfaxoutgoingjobids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachFaxToReceipt<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattachfax: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachFaxToReceipt() {
                ::core::result::Result::Ok(ok__) => {
                    *pbattachfax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Impl: IFaxDocument_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, battachfax: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttachFaxToReceipt(::core::mem::transmute_copy(&battachfax)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            Sender: Sender::<Impl, IMPL_OFFSET>,
            Recipients: Recipients::<Impl, IMPL_OFFSET>,
            CoverPage: CoverPage::<Impl, IMPL_OFFSET>,
            SetCoverPage: SetCoverPage::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            Note: Note::<Impl, IMPL_OFFSET>,
            SetNote: SetNote::<Impl, IMPL_OFFSET>,
            ScheduleTime: ScheduleTime::<Impl, IMPL_OFFSET>,
            SetScheduleTime: SetScheduleTime::<Impl, IMPL_OFFSET>,
            ReceiptAddress: ReceiptAddress::<Impl, IMPL_OFFSET>,
            SetReceiptAddress: SetReceiptAddress::<Impl, IMPL_OFFSET>,
            DocumentName: DocumentName::<Impl, IMPL_OFFSET>,
            SetDocumentName: SetDocumentName::<Impl, IMPL_OFFSET>,
            CallHandle: CallHandle::<Impl, IMPL_OFFSET>,
            SetCallHandle: SetCallHandle::<Impl, IMPL_OFFSET>,
            CoverPageType: CoverPageType::<Impl, IMPL_OFFSET>,
            SetCoverPageType: SetCoverPageType::<Impl, IMPL_OFFSET>,
            ScheduleType: ScheduleType::<Impl, IMPL_OFFSET>,
            SetScheduleType: SetScheduleType::<Impl, IMPL_OFFSET>,
            ReceiptType: ReceiptType::<Impl, IMPL_OFFSET>,
            SetReceiptType: SetReceiptType::<Impl, IMPL_OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Impl, IMPL_OFFSET>,
            SetGroupBroadcastReceipts: SetGroupBroadcastReceipts::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            TapiConnection: TapiConnection::<Impl, IMPL_OFFSET>,
            putref_TapiConnection: putref_TapiConnection::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            ConnectedSubmit: ConnectedSubmit::<Impl, IMPL_OFFSET>,
            AttachFaxToReceipt: AttachFaxToReceipt::<Impl, IMPL_OFFSET>,
            SetAttachFaxToReceipt: SetAttachFaxToReceipt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDocument as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxDocument2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFaxDocument_Impl {
    fn SubmissionId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Bodies(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetBodies(&mut self, vbodies: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit2(&mut self, bstrfaxservername: super::super::Foundation::BSTR, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()>;
    fn ConnectedSubmit2(&mut self, pfaxserver: ::core::option::Option<IFaxServer>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxDocument2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDocument2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxDocument2_Vtbl {
        unsafe extern "system" fn SubmissionId<Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubmissionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bodies<Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bodies() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbodies = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodies<Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbodies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBodies(::core::mem::transmute_copy(&vbodies)).into()
        }
        unsafe extern "system" fn Submit2<Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit2(::core::mem::transmute_copy(&bstrfaxservername), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        unsafe extern "system" fn ConnectedSubmit2<Impl: IFaxDocument2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectedSubmit2(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)).into()
        }
        Self {
            base: IFaxDocument_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SubmissionId: SubmissionId::<Impl, IMPL_OFFSET>,
            Bodies: Bodies::<Impl, IMPL_OFFSET>,
            SetBodies: SetBodies::<Impl, IMPL_OFFSET>,
            Submit2: Submit2::<Impl, IMPL_OFFSET>,
            ConnectedSubmit2: ConnectedSubmit2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxDocument2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxEventLogging_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn InitEventsLevel(&mut self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInitEventsLevel(&mut self, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn InboundEventsLevel(&mut self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetInboundEventsLevel(&mut self, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn OutboundEventsLevel(&mut self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetOutboundEventsLevel(&mut self, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn GeneralEventsLevel(&mut self) -> ::windows::core::Result<FAX_LOG_LEVEL_ENUM>;
    fn SetGeneralEventsLevel(&mut self, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxEventLogging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxEventLogging_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxEventLogging_Vtbl {
        unsafe extern "system" fn InitEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *piniteventlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitEventsLevel(::core::mem::transmute_copy(&initeventlevel)).into()
        }
        unsafe extern "system" fn InboundEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pinboundeventlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundEventsLevel(::core::mem::transmute_copy(&inboundeventlevel)).into()
        }
        unsafe extern "system" fn OutboundEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *poutboundeventlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundEventsLevel(::core::mem::transmute_copy(&outboundeventlevel)).into()
        }
        unsafe extern "system" fn GeneralEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneralEventsLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pgeneraleventlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeneralEventsLevel(::core::mem::transmute_copy(&generaleventlevel)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxEventLogging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitEventsLevel: InitEventsLevel::<Impl, IMPL_OFFSET>,
            SetInitEventsLevel: SetInitEventsLevel::<Impl, IMPL_OFFSET>,
            InboundEventsLevel: InboundEventsLevel::<Impl, IMPL_OFFSET>,
            SetInboundEventsLevel: SetInboundEventsLevel::<Impl, IMPL_OFFSET>,
            OutboundEventsLevel: OutboundEventsLevel::<Impl, IMPL_OFFSET>,
            SetOutboundEventsLevel: SetOutboundEventsLevel::<Impl, IMPL_OFFSET>,
            GeneralEventsLevel: GeneralEventsLevel::<Impl, IMPL_OFFSET>,
            SetGeneralEventsLevel: SetGeneralEventsLevel::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxEventLogging as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxFolders_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OutgoingQueue(&mut self) -> ::windows::core::Result<IFaxOutgoingQueue>;
    fn IncomingQueue(&mut self) -> ::windows::core::Result<IFaxIncomingQueue>;
    fn IncomingArchive(&mut self) -> ::windows::core::Result<IFaxIncomingArchive>;
    fn OutgoingArchive(&mut self) -> ::windows::core::Result<IFaxOutgoingArchive>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxFolders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxFolders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxFolders_Vtbl {
        unsafe extern "system" fn OutgoingQueue<Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingarchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Impl: IFaxFolders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingarchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OutgoingQueue: OutgoingQueue::<Impl, IMPL_OFFSET>,
            IncomingQueue: IncomingQueue::<Impl, IMPL_OFFSET>,
            IncomingArchive: IncomingArchive::<Impl, IMPL_OFFSET>,
            OutgoingArchive: OutgoingArchive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxFolders as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetExtensions(&mut self) -> ::windows::core::Result<IFaxInboundRoutingExtensions>;
    fn GetMethods(&mut self) -> ::windows::core::Result<IFaxInboundRoutingMethods>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRouting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRouting_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxInboundRouting_Vtbl {
        unsafe extern "system" fn GetExtensions<Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxinboundroutingextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethods<Impl: IFaxInboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxinboundroutingmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetExtensions: GetExtensions::<Impl, IMPL_OFFSET>,
            GetMethods: GetMethods::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRouting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingExtension_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ImageName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UniqueName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn Debug(&mut self) -> ::windows::core::Result<i16>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_PROVIDER_STATUS_ENUM>;
    fn InitErrorCode(&mut self) -> ::windows::core::Result<i32>;
    fn Methods(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxInboundRoutingExtension_Vtbl {
        unsafe extern "system" fn FriendlyName<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrimagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstruniquename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdebug = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pliniterrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Impl: IFaxInboundRoutingExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Methods() {
                ::core::result::Result::Ok(ok__) => {
                    *pvmethods = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            ImageName: ImageName::<Impl, IMPL_OFFSET>,
            UniqueName: UniqueName::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            MajorBuild: MajorBuild::<Impl, IMPL_OFFSET>,
            MinorBuild: MinorBuild::<Impl, IMPL_OFFSET>,
            Debug: Debug::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            InitErrorCode: InitErrorCode::<Impl, IMPL_OFFSET>,
            Methods: Methods::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingExtensions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxInboundRoutingExtension>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingExtensions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingExtensions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxInboundRoutingExtensions_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingextension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxinboundroutingextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxInboundRoutingExtensions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingExtensions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingMethod_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GUID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FunctionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExtensionFriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExtensionImageName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Priority(&mut self) -> ::windows::core::Result<i32>;
    fn SetPriority(&mut self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingMethod_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingMethod_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxInboundRoutingMethod_Vtbl {
        unsafe extern "system" fn Name<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FunctionName<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FunctionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfunctionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionFriendlyName<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrextensionfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionImageName<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionImageName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrextensionimagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxInboundRoutingMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            GUID: GUID::<Impl, IMPL_OFFSET>,
            FunctionName: FunctionName::<Impl, IMPL_OFFSET>,
            ExtensionFriendlyName: ExtensionFriendlyName::<Impl, IMPL_OFFSET>,
            ExtensionImageName: ExtensionImageName::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethod as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxInboundRoutingMethods_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxInboundRoutingMethod>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxInboundRoutingMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxInboundRoutingMethods_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxinboundroutingmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxInboundRoutingMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxInboundRoutingMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseArchive(&mut self, busearchive: i16) -> ::windows::core::Result<()>;
    fn ArchiveFolder(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetArchiveFolder(&mut self, bstrarchivefolder: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&mut self) -> ::windows::core::Result<i16>;
    fn SetSizeQuotaWarning(&mut self, bsizequotawarning: i16) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&mut self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&mut self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn AgeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&mut self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn SizeLow(&mut self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetMessages(&mut self, lprefetchsize: i32) -> ::windows::core::Result<IFaxIncomingMessageIterator>;
    fn GetMessage(&mut self, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxIncomingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingArchive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusearchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrarchivefolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArchiveFolder(::core::mem::transmute_copy(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsizequotawarning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *plhighquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *pllowquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *plagelimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizelow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizehigh = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingmessageiterator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxIncomingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(::core::mem::transmute_copy(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UseArchive: UseArchive::<Impl, IMPL_OFFSET>,
            SetUseArchive: SetUseArchive::<Impl, IMPL_OFFSET>,
            ArchiveFolder: ArchiveFolder::<Impl, IMPL_OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Impl, IMPL_OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Impl, IMPL_OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Impl, IMPL_OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            AgeLimit: AgeLimit::<Impl, IMPL_OFFSET>,
            SetAgeLimit: SetAgeLimit::<Impl, IMPL_OFFSET>,
            SizeLow: SizeLow::<Impl, IMPL_OFFSET>,
            SizeHigh: SizeHigh::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetMessages: GetMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingArchive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentPage(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceId(&mut self) -> ::windows::core::Result<i32>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&mut self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AvailableOperations(&mut self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn TransmissionStart(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&mut self) -> ::windows::core::Result<f64>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CallerId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RoutingInformation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn JobType(&mut self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn CopyTiff(&mut self, bstrtiffpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingJob_Vtbl {
        unsafe extern "system" fn Size<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    *plcurrentpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pldeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pextendedstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrextendedstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *pavailableoperations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcallerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrroutinginformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobType() {
                ::core::result::Result::Ok(ok__) => {
                    *pjobtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxIncomingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiff(::core::mem::transmute_copy(&bstrtiffpath)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Size: Size::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            CurrentPage: CurrentPage::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
            AvailableOperations: AvailableOperations::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            TransmissionStart: TransmissionStart::<Impl, IMPL_OFFSET>,
            TransmissionEnd: TransmissionEnd::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            CallerId: CallerId::<Impl, IMPL_OFFSET>,
            RoutingInformation: RoutingInformation::<Impl, IMPL_OFFSET>,
            JobType: JobType::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            CopyTiff: CopyTiff::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxIncomingJob>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingJobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingJobs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxIncomingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingJobs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn TransmissionStart(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&mut self) -> ::windows::core::Result<f64>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CallerId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RoutingInformation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CopyTiff(&mut self, bstrtiffpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingMessage_Vtbl {
        unsafe extern "system" fn Id<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages() {
                ::core::result::Result::Ok(ok__) => {
                    *plpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcallerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrroutinginformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiff(::core::mem::transmute_copy(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFaxIncomingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Pages: Pages::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            DeviceName: DeviceName::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            TransmissionStart: TransmissionStart::<Impl, IMPL_OFFSET>,
            TransmissionEnd: TransmissionEnd::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            CallerId: CallerId::<Impl, IMPL_OFFSET>,
            RoutingInformation: RoutingInformation::<Impl, IMPL_OFFSET>,
            CopyTiff: CopyTiff::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessage2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFaxIncomingMessage_Impl {
    fn Subject(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubject(&mut self, bstrsubject: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SenderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSenderName(&mut self, bstrsendername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SenderFaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSenderFaxNumber(&mut self, bstrsenderfaxnumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HasCoverPage(&mut self) -> ::windows::core::Result<i16>;
    fn SetHasCoverPage(&mut self, bhascoverpage: i16) -> ::windows::core::Result<()>;
    fn Recipients(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRecipients(&mut self, bstrrecipients: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WasReAssigned(&mut self) -> ::windows::core::Result<i16>;
    fn Read(&mut self) -> ::windows::core::Result<i16>;
    fn SetRead(&mut self, bread: i16) -> ::windows::core::Result<()>;
    fn ReAssign(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingMessage2_Vtbl {
        unsafe extern "system" fn Subject<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&bstrsubject)).into()
        }
        unsafe extern "system" fn SenderName<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsendername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsendername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderName<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsendername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderName(::core::mem::transmute_copy(&bstrsendername)).into()
        }
        unsafe extern "system" fn SenderFaxNumber<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderFaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsenderfaxnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderFaxNumber<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSenderFaxNumber(::core::mem::transmute_copy(&bstrsenderfaxnumber)).into()
        }
        unsafe extern "system" fn HasCoverPage<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbhascoverpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHasCoverPage<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhascoverpage: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHasCoverPage(::core::mem::transmute_copy(&bhascoverpage)).into()
        }
        unsafe extern "system" fn Recipients<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrrecipients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipients<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrecipients: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecipients(::core::mem::transmute_copy(&bstrrecipients)).into()
        }
        unsafe extern "system" fn WasReAssigned<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasReAssigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbwasreassigned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *pbread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRead(::core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn ReAssign<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReAssign().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: IFaxIncomingMessage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            SenderName: SenderName::<Impl, IMPL_OFFSET>,
            SetSenderName: SetSenderName::<Impl, IMPL_OFFSET>,
            SenderFaxNumber: SenderFaxNumber::<Impl, IMPL_OFFSET>,
            SetSenderFaxNumber: SetSenderFaxNumber::<Impl, IMPL_OFFSET>,
            HasCoverPage: HasCoverPage::<Impl, IMPL_OFFSET>,
            SetHasCoverPage: SetHasCoverPage::<Impl, IMPL_OFFSET>,
            Recipients: Recipients::<Impl, IMPL_OFFSET>,
            SetRecipients: SetRecipients::<Impl, IMPL_OFFSET>,
            WasReAssigned: WasReAssigned::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            SetRead: SetRead::<Impl, IMPL_OFFSET>,
            ReAssign: ReAssign::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&mut self) -> ::windows::core::Result<IFaxIncomingMessage>;
    fn PrefetchSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrefetchSize(&mut self, lprefetchsize: i32) -> ::windows::core::Result<()>;
    fn AtEOF(&mut self) -> ::windows::core::Result<i16>;
    fn MoveFirst(&mut self) -> ::windows::core::Result<()>;
    fn MoveNext(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingMessageIterator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessageIterator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plprefetchsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrefetchSize(::core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn AtEOF<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AtEOF() {
                ::core::result::Result::Ok(ok__) => {
                    *pbeof = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveFirst<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveFirst().into()
        }
        unsafe extern "system" fn MoveNext<Impl: IFaxIncomingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveNext().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            PrefetchSize: PrefetchSize::<Impl, IMPL_OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Impl, IMPL_OFFSET>,
            AtEOF: AtEOF::<Impl, IMPL_OFFSET>,
            MoveFirst: MoveFirst::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingMessageIterator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxIncomingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetBlocked(&mut self, bblocked: i16) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetJobs(&mut self) -> ::windows::core::Result<IFaxIncomingJobs>;
    fn GetJob(&mut self, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxIncomingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxIncomingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxIncomingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Blocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pbblocked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlocked(::core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetJobs<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxIncomingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxincomingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Blocked: Blocked::<Impl, IMPL_OFFSET>,
            SetBlocked: SetBlocked::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetJobs: GetJobs::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxIncomingQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxJobStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&mut self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn Pages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentPage(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceId(&mut self) -> ::windows::core::Result<i32>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ExtendedStatusCode(&mut self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AvailableOperations(&mut self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn JobType(&mut self) -> ::windows::core::Result<FAX_JOB_TYPE_ENUM>;
    fn ScheduledTime(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionStart(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&mut self) -> ::windows::core::Result<f64>;
    fn CallerId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RoutingInformation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxJobStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxJobStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxJobStatus_Vtbl {
        unsafe extern "system" fn Status<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages() {
                ::core::result::Result::Ok(ok__) => {
                    *plpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    *plcurrentpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pldeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pextendedstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrextendedstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *pavailableoperations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobType() {
                ::core::result::Result::Ok(ok__) => {
                    *pjobtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatescheduledtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcallerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxJobStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrroutinginformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Pages: Pages::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            CurrentPage: CurrentPage::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
            AvailableOperations: AvailableOperations::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            JobType: JobType::<Impl, IMPL_OFFSET>,
            ScheduledTime: ScheduledTime::<Impl, IMPL_OFFSET>,
            TransmissionStart: TransmissionStart::<Impl, IMPL_OFFSET>,
            TransmissionEnd: TransmissionEnd::<Impl, IMPL_OFFSET>,
            CallerId: CallerId::<Impl, IMPL_OFFSET>,
            RoutingInformation: RoutingInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxJobStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxLoggingOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EventLogging(&mut self) -> ::windows::core::Result<IFaxEventLogging>;
    fn ActivityLogging(&mut self) -> ::windows::core::Result<IFaxActivityLogging>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxLoggingOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxLoggingOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxLoggingOptions_Vtbl {
        unsafe extern "system" fn EventLogging<Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxeventlogging = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityLogging<Impl: IFaxLoggingOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityLogging() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxactivitylogging = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventLogging: EventLogging::<Impl, IMPL_OFFSET>,
            ActivityLogging: ActivityLogging::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxLoggingOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRouting_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetGroups(&mut self) -> ::windows::core::Result<IFaxOutboundRoutingGroups>;
    fn GetRules(&mut self) -> ::windows::core::Result<IFaxOutboundRoutingRules>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRouting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRouting_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutboundRouting_Vtbl {
        unsafe extern "system" fn GetGroups<Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutinggroups = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRules<Impl: IFaxOutboundRouting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRules() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutingrules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetGroups: GetGroups::<Impl, IMPL_OFFSET>,
            GetRules: GetRules::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRouting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_GROUP_STATUS_ENUM>;
    fn DeviceIds(&mut self) -> ::windows::core::Result<IFaxDeviceIds>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutboundRoutingGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Impl: IFaxOutboundRoutingGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIds() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxdeviceids = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            DeviceIds: DeviceIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxOutboundRoutingGroup>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxOutboundRoutingGroup>;
    fn Remove(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingGroups_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutboundRoutingGroups_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutinggroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutinggroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxOutboundRoutingGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&vindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingGroups as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn AreaCode(&mut self) -> ::windows::core::Result<i32>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_RULE_STATUS_ENUM>;
    fn UseDevice(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseDevice(&mut self, busedevice: i16) -> ::windows::core::Result<()>;
    fn DeviceId(&mut self) -> ::windows::core::Result<i32>;
    fn SetDeviceId(&mut self, deviceid: i32) -> ::windows::core::Result<()>;
    fn GroupName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetGroupName(&mut self, bstrgroupname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutboundRoutingRule_Vtbl {
        unsafe extern "system" fn CountryCode<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreaCode<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plareacode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDevice<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevice: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusedevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDevice<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevice: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseDevice(::core::mem::transmute_copy(&busedevice)).into()
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pldeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceId(::core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn GroupName<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrgroupname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupName<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupName(::core::mem::transmute_copy(&bstrgroupname)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxOutboundRoutingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CountryCode: CountryCode::<Impl, IMPL_OFFSET>,
            AreaCode: AreaCode::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            UseDevice: UseDevice::<Impl, IMPL_OFFSET>,
            SetUseDevice: SetUseDevice::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            SetDeviceId: SetDeviceId::<Impl, IMPL_OFFSET>,
            GroupName: GroupName::<Impl, IMPL_OFFSET>,
            SetGroupName: SetGroupName::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutboundRoutingRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ItemByCountryAndArea(&mut self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
    fn RemoveByCountryAndArea(&mut self, lcountrycode: i32, lareacode: i32) -> ::windows::core::Result<()>;
    fn Remove(&mut self, lindex: i32) -> ::windows::core::Result<()>;
    fn Add(&mut self, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: super::super::Foundation::BSTR, ldeviceid: i32) -> ::windows::core::Result<IFaxOutboundRoutingRule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutboundRoutingRules_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingRules_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutboundRoutingRules_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutingrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByCountryAndArea<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByCountryAndArea(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutingrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveByCountryAndArea(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Impl: IFaxOutboundRoutingRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&lcountrycode), ::core::mem::transmute_copy(&lareacode), ::core::mem::transmute_copy(&busedevice), ::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&ldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutboundroutingrule = ::core::mem::transmute(ok__);
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
            ItemByCountryAndArea: ItemByCountryAndArea::<Impl, IMPL_OFFSET>,
            RemoveByCountryAndArea: RemoveByCountryAndArea::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutboundRoutingRules as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingArchive_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UseArchive(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseArchive(&mut self, busearchive: i16) -> ::windows::core::Result<()>;
    fn ArchiveFolder(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetArchiveFolder(&mut self, bstrarchivefolder: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SizeQuotaWarning(&mut self) -> ::windows::core::Result<i16>;
    fn SetSizeQuotaWarning(&mut self, bsizequotawarning: i16) -> ::windows::core::Result<()>;
    fn HighQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetHighQuotaWaterMark(&mut self, lhighquotawatermark: i32) -> ::windows::core::Result<()>;
    fn LowQuotaWaterMark(&mut self) -> ::windows::core::Result<i32>;
    fn SetLowQuotaWaterMark(&mut self, llowquotawatermark: i32) -> ::windows::core::Result<()>;
    fn AgeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&mut self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn SizeLow(&mut self) -> ::windows::core::Result<i32>;
    fn SizeHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetMessages(&mut self, lprefetchsize: i32) -> ::windows::core::Result<IFaxOutgoingMessageIterator>;
    fn GetMessage(&mut self, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxOutgoingMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingArchive_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingArchive_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingArchive_Vtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusearchive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseArchive(::core::mem::transmute_copy(&busearchive)).into()
        }
        unsafe extern "system" fn ArchiveFolder<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrarchivefolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArchiveFolder(::core::mem::transmute_copy(&bstrarchivefolder)).into()
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsizequotawarning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeQuotaWarning(::core::mem::transmute_copy(&bsizequotawarning)).into()
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *plhighquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighQuotaWaterMark(::core::mem::transmute_copy(&lhighquotawatermark)).into()
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark() {
                ::core::result::Result::Ok(ok__) => {
                    *pllowquotawatermark = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLowQuotaWaterMark(::core::mem::transmute_copy(&llowquotawatermark)).into()
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *plagelimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn SizeLow<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizelow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *plsizehigh = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(::core::mem::transmute_copy(&lprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingmessageiterator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxOutgoingArchive_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(::core::mem::transmute_copy(&bstrmessageid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UseArchive: UseArchive::<Impl, IMPL_OFFSET>,
            SetUseArchive: SetUseArchive::<Impl, IMPL_OFFSET>,
            ArchiveFolder: ArchiveFolder::<Impl, IMPL_OFFSET>,
            SetArchiveFolder: SetArchiveFolder::<Impl, IMPL_OFFSET>,
            SizeQuotaWarning: SizeQuotaWarning::<Impl, IMPL_OFFSET>,
            SetSizeQuotaWarning: SetSizeQuotaWarning::<Impl, IMPL_OFFSET>,
            HighQuotaWaterMark: HighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetHighQuotaWaterMark: SetHighQuotaWaterMark::<Impl, IMPL_OFFSET>,
            LowQuotaWaterMark: LowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            SetLowQuotaWaterMark: SetLowQuotaWaterMark::<Impl, IMPL_OFFSET>,
            AgeLimit: AgeLimit::<Impl, IMPL_OFFSET>,
            SetAgeLimit: SetAgeLimit::<Impl, IMPL_OFFSET>,
            SizeLow: SizeLow::<Impl, IMPL_OFFSET>,
            SizeHigh: SizeHigh::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetMessages: GetMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingArchive as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJob_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Subject(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DocumentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Pages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn SubmissionId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn OriginalScheduledTime(&mut self) -> ::windows::core::Result<f64>;
    fn SubmissionTime(&mut self) -> ::windows::core::Result<f64>;
    fn ReceiptType(&mut self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn Priority(&mut self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&mut self) -> ::windows::core::Result<IFaxSender>;
    fn Recipient(&mut self) -> ::windows::core::Result<IFaxRecipient>;
    fn CurrentPage(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceId(&mut self) -> ::windows::core::Result<i32>;
    fn Status(&mut self) -> ::windows::core::Result<FAX_JOB_STATUS_ENUM>;
    fn ExtendedStatusCode(&mut self) -> ::windows::core::Result<FAX_JOB_EXTENDED_STATUS_ENUM>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AvailableOperations(&mut self) -> ::windows::core::Result<FAX_JOB_OPERATIONS_ENUM>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn ScheduledTime(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionStart(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&mut self) -> ::windows::core::Result<f64>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GroupBroadcastReceipts(&mut self) -> ::windows::core::Result<i16>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn Restart(&mut self) -> ::windows::core::Result<()>;
    fn CopyTiff(&mut self, bstrtiffpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingJob_Vtbl {
        unsafe extern "system" fn Subject<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocumentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages() {
                ::core::result::Result::Ok(ok__) => {
                    *plpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionId<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubmissionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdateoriginalscheduledtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatesubmissiontime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    *preceipttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxsender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxrecipient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage() {
                ::core::result::Result::Ok(ok__) => {
                    *plcurrentpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pldeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pextendedstatuscode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrextendedstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *pavailableoperations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatescheduledtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupBroadcastReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    *pbgroupbroadcastreceipts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Restart<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restart().into()
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiff(::core::mem::transmute_copy(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Cancel<Impl: IFaxOutgoingJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Subject: Subject::<Impl, IMPL_OFFSET>,
            DocumentName: DocumentName::<Impl, IMPL_OFFSET>,
            Pages: Pages::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SubmissionId: SubmissionId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Impl, IMPL_OFFSET>,
            SubmissionTime: SubmissionTime::<Impl, IMPL_OFFSET>,
            ReceiptType: ReceiptType::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            Sender: Sender::<Impl, IMPL_OFFSET>,
            Recipient: Recipient::<Impl, IMPL_OFFSET>,
            CurrentPage: CurrentPage::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedStatusCode: ExtendedStatusCode::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
            AvailableOperations: AvailableOperations::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            ScheduledTime: ScheduledTime::<Impl, IMPL_OFFSET>,
            TransmissionStart: TransmissionStart::<Impl, IMPL_OFFSET>,
            TransmissionEnd: TransmissionEnd::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            GroupBroadcastReceipts: GroupBroadcastReceipts::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            Restart: Restart::<Impl, IMPL_OFFSET>,
            CopyTiff: CopyTiff::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJob2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFaxOutgoingJob_Impl {
    fn HasCoverPage(&mut self) -> ::windows::core::Result<i16>;
    fn ReceiptAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ScheduleType(&mut self) -> ::windows::core::Result<FAX_SCHEDULE_TYPE_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJob2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJob2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingJob2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbhascoverpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreceiptaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleType<Impl: IFaxOutgoingJob2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleType() {
                ::core::result::Result::Ok(ok__) => {
                    *pscheduletype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFaxOutgoingJob_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HasCoverPage: HasCoverPage::<Impl, IMPL_OFFSET>,
            ReceiptAddress: ReceiptAddress::<Impl, IMPL_OFFSET>,
            ScheduleType: ScheduleType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJob2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingJobs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, vindex: super::super::System::Com::VARIANT) -> ::windows::core::Result<IFaxOutgoingJob>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingJobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJobs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingJobs_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&vindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutgoingJobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
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
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingJobs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SubmissionId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Id(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Subject(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DocumentName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn Pages(&mut self) -> ::windows::core::Result<i32>;
    fn Size(&mut self) -> ::windows::core::Result<i32>;
    fn OriginalScheduledTime(&mut self) -> ::windows::core::Result<f64>;
    fn SubmissionTime(&mut self) -> ::windows::core::Result<f64>;
    fn Priority(&mut self) -> ::windows::core::Result<FAX_PRIORITY_TYPE_ENUM>;
    fn Sender(&mut self) -> ::windows::core::Result<IFaxSender>;
    fn Recipient(&mut self) -> ::windows::core::Result<IFaxRecipient>;
    fn DeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TransmissionStart(&mut self) -> ::windows::core::Result<f64>;
    fn TransmissionEnd(&mut self) -> ::windows::core::Result<f64>;
    fn CSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CopyTiff(&mut self, bstrtiffpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingMessage_Vtbl {
        unsafe extern "system" fn SubmissionId<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubmissionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdocumentname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages() {
                ::core::result::Result::Ok(ok__) => {
                    *plpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *plsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalScheduledTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdateoriginalscheduledtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatesubmissiontime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxsender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxrecipient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatetransmissionend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTiff(::core::mem::transmute_copy(&bstrtiffpath)).into()
        }
        unsafe extern "system" fn Delete<Impl: IFaxOutgoingMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SubmissionId: SubmissionId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            DocumentName: DocumentName::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            Pages: Pages::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            OriginalScheduledTime: OriginalScheduledTime::<Impl, IMPL_OFFSET>,
            SubmissionTime: SubmissionTime::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            Sender: Sender::<Impl, IMPL_OFFSET>,
            Recipient: Recipient::<Impl, IMPL_OFFSET>,
            DeviceName: DeviceName::<Impl, IMPL_OFFSET>,
            TransmissionStart: TransmissionStart::<Impl, IMPL_OFFSET>,
            TransmissionEnd: TransmissionEnd::<Impl, IMPL_OFFSET>,
            CSID: CSID::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            CopyTiff: CopyTiff::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessage2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFaxOutgoingMessage_Impl {
    fn HasCoverPage(&mut self) -> ::windows::core::Result<i16>;
    fn ReceiptType(&mut self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn ReceiptAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Read(&mut self) -> ::windows::core::Result<i16>;
    fn SetRead(&mut self, bread: i16) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingMessage2_Vtbl {
        unsafe extern "system" fn HasCoverPage<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbhascoverpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType() {
                ::core::result::Result::Ok(ok__) => {
                    *preceipttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrreceiptaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read() {
                ::core::result::Result::Ok(ok__) => {
                    *pbread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRead(::core::mem::transmute_copy(&bread)).into()
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: IFaxOutgoingMessage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HasCoverPage: HasCoverPage::<Impl, IMPL_OFFSET>,
            ReceiptType: ReceiptType::<Impl, IMPL_OFFSET>,
            ReceiptAddress: ReceiptAddress::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            SetRead: SetRead::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingMessageIterator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Message(&mut self) -> ::windows::core::Result<IFaxOutgoingMessage>;
    fn AtEOF(&mut self) -> ::windows::core::Result<i16>;
    fn PrefetchSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetPrefetchSize(&mut self, lprefetchsize: i32) -> ::windows::core::Result<()>;
    fn MoveFirst(&mut self) -> ::windows::core::Result<()>;
    fn MoveNext(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingMessageIterator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessageIterator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingMessageIterator_Vtbl {
        unsafe extern "system" fn Message<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEOF<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AtEOF() {
                ::core::result::Result::Ok(ok__) => {
                    *pbeof = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *plprefetchsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrefetchSize(::core::mem::transmute_copy(&lprefetchsize)).into()
        }
        unsafe extern "system" fn MoveFirst<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveFirst().into()
        }
        unsafe extern "system" fn MoveNext<Impl: IFaxOutgoingMessageIterator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveNext().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            AtEOF: AtEOF::<Impl, IMPL_OFFSET>,
            PrefetchSize: PrefetchSize::<Impl, IMPL_OFFSET>,
            SetPrefetchSize: SetPrefetchSize::<Impl, IMPL_OFFSET>,
            MoveFirst: MoveFirst::<Impl, IMPL_OFFSET>,
            MoveNext: MoveNext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingMessageIterator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxOutgoingQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Blocked(&mut self) -> ::windows::core::Result<i16>;
    fn SetBlocked(&mut self, bblocked: i16) -> ::windows::core::Result<()>;
    fn Paused(&mut self) -> ::windows::core::Result<i16>;
    fn SetPaused(&mut self, bpaused: i16) -> ::windows::core::Result<()>;
    fn AllowPersonalCoverPages(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowPersonalCoverPages(&mut self, ballowpersonalcoverpages: i16) -> ::windows::core::Result<()>;
    fn UseDeviceTSID(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseDeviceTSID(&mut self, busedevicetsid: i16) -> ::windows::core::Result<()>;
    fn Retries(&mut self) -> ::windows::core::Result<i32>;
    fn SetRetries(&mut self, lretries: i32) -> ::windows::core::Result<()>;
    fn RetryDelay(&mut self) -> ::windows::core::Result<i32>;
    fn SetRetryDelay(&mut self, lretrydelay: i32) -> ::windows::core::Result<()>;
    fn DiscountRateStart(&mut self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateStart(&mut self, datediscountratestart: f64) -> ::windows::core::Result<()>;
    fn DiscountRateEnd(&mut self) -> ::windows::core::Result<f64>;
    fn SetDiscountRateEnd(&mut self, datediscountrateend: f64) -> ::windows::core::Result<()>;
    fn AgeLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetAgeLimit(&mut self, lagelimit: i32) -> ::windows::core::Result<()>;
    fn Branding(&mut self) -> ::windows::core::Result<i16>;
    fn SetBranding(&mut self, bbranding: i16) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn GetJobs(&mut self) -> ::windows::core::Result<IFaxOutgoingJobs>;
    fn GetJob(&mut self, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxOutgoingJob>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxOutgoingQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxOutgoingQueue_Vtbl {
        unsafe extern "system" fn Blocked<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Blocked() {
                ::core::result::Result::Ok(ok__) => {
                    *pbblocked = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlocked(::core::mem::transmute_copy(&bblocked)).into()
        }
        unsafe extern "system" fn Paused<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paused() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpaused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaused<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpaused: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPaused(::core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPersonalCoverPages() {
                ::core::result::Result::Ok(ok__) => {
                    *pballowpersonalcoverpages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowPersonalCoverPages(::core::mem::transmute_copy(&ballowpersonalcoverpages)).into()
        }
        unsafe extern "system" fn UseDeviceTSID<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDeviceTSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbusedevicetsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseDeviceTSID(::core::mem::transmute_copy(&busedevicetsid)).into()
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries() {
                ::core::result::Result::Ok(ok__) => {
                    *plretries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetries(::core::mem::transmute_copy(&lretries)).into()
        }
        unsafe extern "system" fn RetryDelay<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *plretrydelay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetryDelay(::core::mem::transmute_copy(&lretrydelay)).into()
        }
        unsafe extern "system" fn DiscountRateStart<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateStart() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatediscountratestart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscountRateStart(::core::mem::transmute_copy(&datediscountratestart)).into()
        }
        unsafe extern "system" fn DiscountRateEnd<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatediscountrateend = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscountRateEnd(::core::mem::transmute_copy(&datediscountrateend)).into()
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *plagelimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAgeLimit(::core::mem::transmute_copy(&lagelimit)).into()
        }
        unsafe extern "system" fn Branding<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Branding() {
                ::core::result::Result::Ok(ok__) => {
                    *pbbranding = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBranding(::core::mem::transmute_copy(&bbranding)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn GetJobs<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingjobs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxOutgoingQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(::core::mem::transmute_copy(&bstrjobid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxoutgoingjob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Blocked: Blocked::<Impl, IMPL_OFFSET>,
            SetBlocked: SetBlocked::<Impl, IMPL_OFFSET>,
            Paused: Paused::<Impl, IMPL_OFFSET>,
            SetPaused: SetPaused::<Impl, IMPL_OFFSET>,
            AllowPersonalCoverPages: AllowPersonalCoverPages::<Impl, IMPL_OFFSET>,
            SetAllowPersonalCoverPages: SetAllowPersonalCoverPages::<Impl, IMPL_OFFSET>,
            UseDeviceTSID: UseDeviceTSID::<Impl, IMPL_OFFSET>,
            SetUseDeviceTSID: SetUseDeviceTSID::<Impl, IMPL_OFFSET>,
            Retries: Retries::<Impl, IMPL_OFFSET>,
            SetRetries: SetRetries::<Impl, IMPL_OFFSET>,
            RetryDelay: RetryDelay::<Impl, IMPL_OFFSET>,
            SetRetryDelay: SetRetryDelay::<Impl, IMPL_OFFSET>,
            DiscountRateStart: DiscountRateStart::<Impl, IMPL_OFFSET>,
            SetDiscountRateStart: SetDiscountRateStart::<Impl, IMPL_OFFSET>,
            DiscountRateEnd: DiscountRateEnd::<Impl, IMPL_OFFSET>,
            SetDiscountRateEnd: SetDiscountRateEnd::<Impl, IMPL_OFFSET>,
            AgeLimit: AgeLimit::<Impl, IMPL_OFFSET>,
            SetAgeLimit: SetAgeLimit::<Impl, IMPL_OFFSET>,
            Branding: Branding::<Impl, IMPL_OFFSET>,
            SetBranding: SetBranding::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            GetJobs: GetJobs::<Impl, IMPL_OFFSET>,
            GetJob: GetJob::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxOutgoingQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxReceiptOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AuthenticationType(&mut self) -> ::windows::core::Result<FAX_SMTP_AUTHENTICATION_TYPE_ENUM>;
    fn SetAuthenticationType(&mut self, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn SMTPServer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSMTPServer(&mut self, bstrsmtpserver: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SMTPPort(&mut self) -> ::windows::core::Result<i32>;
    fn SetSMTPPort(&mut self, lsmtpport: i32) -> ::windows::core::Result<()>;
    fn SMTPSender(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSMTPSender(&mut self, bstrsmtpsender: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SMTPUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSMTPUser(&mut self, bstrsmtpuser: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowedReceipts(&mut self) -> ::windows::core::Result<FAX_RECEIPT_TYPE_ENUM>;
    fn SetAllowedReceipts(&mut self, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn SMTPPassword(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSMTPPassword(&mut self, bstrsmtppassword: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn UseForInboundRouting(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseForInboundRouting(&mut self, buseforinboundrouting: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxReceiptOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxReceiptOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxReceiptOptions_Vtbl {
        unsafe extern "system" fn AuthenticationType<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SMTPServer<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsmtpserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPServer<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMTPServer(::core::mem::transmute_copy(&bstrsmtpserver)).into()
        }
        unsafe extern "system" fn SMTPPort<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPPort() {
                ::core::result::Result::Ok(ok__) => {
                    *plsmtpport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPort<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMTPPort(::core::mem::transmute_copy(&lsmtpport)).into()
        }
        unsafe extern "system" fn SMTPSender<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPSender() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsmtpsender = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPSender<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMTPSender(::core::mem::transmute_copy(&bstrsmtpsender)).into()
        }
        unsafe extern "system" fn SMTPUser<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsmtpuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPUser<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMTPUser(::core::mem::transmute_copy(&bstrsmtpuser)).into()
        }
        unsafe extern "system" fn AllowedReceipts<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedReceipts() {
                ::core::result::Result::Ok(ok__) => {
                    *pallowedreceipts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedReceipts<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedReceipts(::core::mem::transmute_copy(&allowedreceipts)).into()
        }
        unsafe extern "system" fn SMTPPassword<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsmtppassword = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPassword<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSMTPPassword(::core::mem::transmute_copy(&bstrsmtppassword)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn UseForInboundRouting<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseForInboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    *pbuseforinboundrouting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseForInboundRouting<Impl: IFaxReceiptOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buseforinboundrouting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseForInboundRouting(::core::mem::transmute_copy(&buseforinboundrouting)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AuthenticationType: AuthenticationType::<Impl, IMPL_OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Impl, IMPL_OFFSET>,
            SMTPServer: SMTPServer::<Impl, IMPL_OFFSET>,
            SetSMTPServer: SetSMTPServer::<Impl, IMPL_OFFSET>,
            SMTPPort: SMTPPort::<Impl, IMPL_OFFSET>,
            SetSMTPPort: SetSMTPPort::<Impl, IMPL_OFFSET>,
            SMTPSender: SMTPSender::<Impl, IMPL_OFFSET>,
            SetSMTPSender: SetSMTPSender::<Impl, IMPL_OFFSET>,
            SMTPUser: SMTPUser::<Impl, IMPL_OFFSET>,
            SetSMTPUser: SetSMTPUser::<Impl, IMPL_OFFSET>,
            AllowedReceipts: AllowedReceipts::<Impl, IMPL_OFFSET>,
            SetAllowedReceipts: SetAllowedReceipts::<Impl, IMPL_OFFSET>,
            SMTPPassword: SMTPPassword::<Impl, IMPL_OFFSET>,
            SetSMTPPassword: SetSMTPPassword::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            UseForInboundRouting: UseForInboundRouting::<Impl, IMPL_OFFSET>,
            SetUseForInboundRouting: SetUseForInboundRouting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxReceiptOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxRecipient_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxRecipient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxRecipient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxRecipient_Vtbl {
        unsafe extern "system" fn FaxNumber<Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfaxnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn Name<Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFaxRecipient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FaxNumber: FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber: SetFaxNumber::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxRecipient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxRecipients_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<IFaxRecipient>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, bstrfaxnumber: super::super::Foundation::BSTR, bstrrecipientname: super::super::Foundation::BSTR) -> ::windows::core::Result<IFaxRecipient>;
    fn Remove(&mut self, lindex: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxRecipients_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxRecipients_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxRecipients_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxrecipient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrecipientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&bstrfaxnumber), ::core::mem::transmute_copy(&bstrrecipientname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxrecipient = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxRecipients_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxRecipients as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSecurity_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDescriptor(&mut self, vdescriptor: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GrantedRights(&mut self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn InformationType(&mut self) -> ::windows::core::Result<i32>;
    fn SetInformationType(&mut self, linformationtype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSecurity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxSecurity_Vtbl {
        unsafe extern "system" fn Descriptor<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescriptor(::core::mem::transmute_copy(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrantedRights() {
                ::core::result::Result::Ok(ok__) => {
                    *pgrantedrights = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn InformationType<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationType() {
                ::core::result::Result::Ok(ok__) => {
                    *plinformationtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Impl: IFaxSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInformationType(::core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Descriptor: Descriptor::<Impl, IMPL_OFFSET>,
            SetDescriptor: SetDescriptor::<Impl, IMPL_OFFSET>,
            GrantedRights: GrantedRights::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            InformationType: InformationType::<Impl, IMPL_OFFSET>,
            SetInformationType: SetInformationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSecurity2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Descriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetDescriptor(&mut self, vdescriptor: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GrantedRights(&mut self) -> ::windows::core::Result<FAX_ACCESS_RIGHTS_ENUM_2>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
    fn InformationType(&mut self) -> ::windows::core::Result<i32>;
    fn SetInformationType(&mut self, linformationtype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSecurity2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSecurity2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxSecurity2_Vtbl {
        unsafe extern "system" fn Descriptor<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *pvdescriptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescriptor(::core::mem::transmute_copy(&vdescriptor)).into()
        }
        unsafe extern "system" fn GrantedRights<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrantedRights() {
                ::core::result::Result::Ok(ok__) => {
                    *pgrantedrights = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        unsafe extern "system" fn InformationType<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationType() {
                ::core::result::Result::Ok(ok__) => {
                    *plinformationtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Impl: IFaxSecurity2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInformationType(::core::mem::transmute_copy(&linformationtype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Descriptor: Descriptor::<Impl, IMPL_OFFSET>,
            SetDescriptor: SetDescriptor::<Impl, IMPL_OFFSET>,
            GrantedRights: GrantedRights::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            InformationType: InformationType::<Impl, IMPL_OFFSET>,
            SetInformationType: SetInformationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSecurity2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxSender_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BillingCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBillingCode(&mut self, bstrbillingcode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn City(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCity(&mut self, bstrcity: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Company(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCompany(&mut self, bstrcompany: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Country(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCountry(&mut self, bstrcountry: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Department(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDepartment(&mut self, bstrdepartment: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Email(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEmail(&mut self, bstremail: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FaxNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFaxNumber(&mut self, bstrfaxnumber: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HomePhone(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetHomePhone(&mut self, bstrhomephone: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTSID(&mut self, bstrtsid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OfficePhone(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOfficePhone(&mut self, bstrofficephone: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OfficeLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOfficeLocation(&mut self, bstrofficelocation: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetState(&mut self, bstrstate: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StreetAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStreetAddress(&mut self, bstrstreetaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTitle(&mut self, bstrtitle: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ZipCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetZipCode(&mut self, bstrzipcode: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LoadDefaultSender(&mut self) -> ::windows::core::Result<()>;
    fn SaveDefaultSender(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxSender_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSender_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxSender_Vtbl {
        unsafe extern "system" fn BillingCode<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BillingCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbillingcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBillingCode<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbillingcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBillingCode(::core::mem::transmute_copy(&bstrbillingcode)).into()
        }
        unsafe extern "system" fn City<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCity<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCity(::core::mem::transmute_copy(&bstrcity)).into()
        }
        unsafe extern "system" fn Company<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Company() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcompany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcompany: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompany(::core::mem::transmute_copy(&bstrcompany)).into()
        }
        unsafe extern "system" fn Country<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcountry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountry: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCountry(::core::mem::transmute_copy(&bstrcountry)).into()
        }
        unsafe extern "system" fn Department<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Department() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdepartment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepartment(::core::mem::transmute_copy(&bstrdepartment)).into()
        }
        unsafe extern "system" fn Email<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstremail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Email() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstremail = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmail<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmail(::core::mem::transmute_copy(&bstremail)).into()
        }
        unsafe extern "system" fn FaxNumber<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfaxnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFaxNumber(::core::mem::transmute_copy(&bstrfaxnumber)).into()
        }
        unsafe extern "system" fn HomePhone<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomePhone() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrhomephone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePhone<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHomePhone(::core::mem::transmute_copy(&bstrhomephone)).into()
        }
        unsafe extern "system" fn Name<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn TSID<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTSID(::core::mem::transmute_copy(&bstrtsid)).into()
        }
        unsafe extern "system" fn OfficePhone<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficePhone() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrofficephone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficePhone<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOfficePhone(::core::mem::transmute_copy(&bstrofficephone)).into()
        }
        unsafe extern "system" fn OfficeLocation<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficeLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrofficelocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocation<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOfficeLocation(::core::mem::transmute_copy(&bstrofficelocation)).into()
        }
        unsafe extern "system" fn State<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&bstrstate)).into()
        }
        unsafe extern "system" fn StreetAddress<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreetAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstreetaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetAddress<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreetAddress(::core::mem::transmute_copy(&bstrstreetaddress)).into()
        }
        unsafe extern "system" fn Title<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn ZipCode<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZipCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrzipcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZipCode<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrzipcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZipCode(::core::mem::transmute_copy(&bstrzipcode)).into()
        }
        unsafe extern "system" fn LoadDefaultSender<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadDefaultSender().into()
        }
        unsafe extern "system" fn SaveDefaultSender<Impl: IFaxSender_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveDefaultSender().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BillingCode: BillingCode::<Impl, IMPL_OFFSET>,
            SetBillingCode: SetBillingCode::<Impl, IMPL_OFFSET>,
            City: City::<Impl, IMPL_OFFSET>,
            SetCity: SetCity::<Impl, IMPL_OFFSET>,
            Company: Company::<Impl, IMPL_OFFSET>,
            SetCompany: SetCompany::<Impl, IMPL_OFFSET>,
            Country: Country::<Impl, IMPL_OFFSET>,
            SetCountry: SetCountry::<Impl, IMPL_OFFSET>,
            Department: Department::<Impl, IMPL_OFFSET>,
            SetDepartment: SetDepartment::<Impl, IMPL_OFFSET>,
            Email: Email::<Impl, IMPL_OFFSET>,
            SetEmail: SetEmail::<Impl, IMPL_OFFSET>,
            FaxNumber: FaxNumber::<Impl, IMPL_OFFSET>,
            SetFaxNumber: SetFaxNumber::<Impl, IMPL_OFFSET>,
            HomePhone: HomePhone::<Impl, IMPL_OFFSET>,
            SetHomePhone: SetHomePhone::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            TSID: TSID::<Impl, IMPL_OFFSET>,
            SetTSID: SetTSID::<Impl, IMPL_OFFSET>,
            OfficePhone: OfficePhone::<Impl, IMPL_OFFSET>,
            SetOfficePhone: SetOfficePhone::<Impl, IMPL_OFFSET>,
            OfficeLocation: OfficeLocation::<Impl, IMPL_OFFSET>,
            SetOfficeLocation: SetOfficeLocation::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            StreetAddress: StreetAddress::<Impl, IMPL_OFFSET>,
            SetStreetAddress: SetStreetAddress::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            ZipCode: ZipCode::<Impl, IMPL_OFFSET>,
            SetZipCode: SetZipCode::<Impl, IMPL_OFFSET>,
            LoadDefaultSender: LoadDefaultSender::<Impl, IMPL_OFFSET>,
            SaveDefaultSender: SaveDefaultSender::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxSender as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&mut self, bstrservername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDeviceProviders(&mut self) -> ::windows::core::Result<IFaxDeviceProviders>;
    fn GetDevices(&mut self) -> ::windows::core::Result<IFaxDevices>;
    fn InboundRouting(&mut self) -> ::windows::core::Result<IFaxInboundRouting>;
    fn Folders(&mut self) -> ::windows::core::Result<IFaxFolders>;
    fn LoggingOptions(&mut self) -> ::windows::core::Result<IFaxLoggingOptions>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MajorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn MinorBuild(&mut self) -> ::windows::core::Result<i32>;
    fn Debug(&mut self) -> ::windows::core::Result<i16>;
    fn Activity(&mut self) -> ::windows::core::Result<IFaxActivity>;
    fn OutboundRouting(&mut self) -> ::windows::core::Result<IFaxOutboundRouting>;
    fn ReceiptOptions(&mut self) -> ::windows::core::Result<IFaxReceiptOptions>;
    fn Security(&mut self) -> ::windows::core::Result<IFaxSecurity>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn GetExtensionProperty(&mut self, bstrguid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetExtensionProperty(&mut self, bstrguid: super::super::Foundation::BSTR, vproperty: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ListenToServerEvents(&mut self, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::Result<()>;
    fn RegisterDeviceProvider(&mut self, bstrguid: super::super::Foundation::BSTR, bstrfriendlyname: super::super::Foundation::BSTR, bstrimagename: super::super::Foundation::BSTR, tspname: super::super::Foundation::BSTR, lfspiversion: i32) -> ::windows::core::Result<()>;
    fn UnregisterDeviceProvider(&mut self, bstruniquename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RegisterInboundRoutingExtension(&mut self, bstrextensionname: super::super::Foundation::BSTR, bstrfriendlyname: super::super::Foundation::BSTR, bstrimagename: super::super::Foundation::BSTR, vmethods: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UnregisterInboundRoutingExtension(&mut self, bstrextensionuniquename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RegisteredEvents(&mut self) -> ::windows::core::Result<FAX_SERVER_EVENTS_TYPE_ENUM>;
    fn APIVersion(&mut self) -> ::windows::core::Result<FAX_SERVER_APIVERSION_ENUM>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxServer_Vtbl {
        unsafe extern "system" fn Connect<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&bstrservername)).into()
        }
        unsafe extern "system" fn ServerName<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrservername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrservername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceProviders<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxdeviceproviders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevices<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundRouting<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxinboundrouting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folders() {
                ::core::result::Result::Ok(ok__) => {
                    *pfaxfolders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingOptions<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxloggingoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdebug = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activity<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activity() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxactivity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundRouting<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundRouting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxoutboundrouting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptOptions<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxreceiptoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn GetExtensionProperty<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionProperty(::core::mem::transmute_copy(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtensionProperty(::core::mem::transmute_copy(&bstrguid), ::core::mem::transmute_copy(&vproperty)).into()
        }
        unsafe extern "system" fn ListenToServerEvents<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ListenToServerEvents(::core::mem::transmute_copy(&eventtypes)).into()
        }
        unsafe extern "system" fn RegisterDeviceProvider<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterDeviceProvider(::core::mem::transmute_copy(&bstrguid), ::core::mem::transmute_copy(&bstrfriendlyname), ::core::mem::transmute_copy(&bstrimagename), ::core::mem::transmute_copy(&tspname), ::core::mem::transmute_copy(&lfspiversion)).into()
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDeviceProvider(::core::mem::transmute_copy(&bstruniquename)).into()
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterInboundRoutingExtension(::core::mem::transmute_copy(&bstrextensionname), ::core::mem::transmute_copy(&bstrfriendlyname), ::core::mem::transmute_copy(&bstrimagename), ::core::mem::transmute_copy(&vmethods)).into()
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterInboundRoutingExtension(::core::mem::transmute_copy(&bstrextensionuniquename)).into()
        }
        unsafe extern "system" fn RegisteredEvents<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *peventtypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn APIVersion<Impl: IFaxServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).APIVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *papiversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            ServerName: ServerName::<Impl, IMPL_OFFSET>,
            GetDeviceProviders: GetDeviceProviders::<Impl, IMPL_OFFSET>,
            GetDevices: GetDevices::<Impl, IMPL_OFFSET>,
            InboundRouting: InboundRouting::<Impl, IMPL_OFFSET>,
            Folders: Folders::<Impl, IMPL_OFFSET>,
            LoggingOptions: LoggingOptions::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            MajorBuild: MajorBuild::<Impl, IMPL_OFFSET>,
            MinorBuild: MinorBuild::<Impl, IMPL_OFFSET>,
            Debug: Debug::<Impl, IMPL_OFFSET>,
            Activity: Activity::<Impl, IMPL_OFFSET>,
            OutboundRouting: OutboundRouting::<Impl, IMPL_OFFSET>,
            ReceiptOptions: ReceiptOptions::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            GetExtensionProperty: GetExtensionProperty::<Impl, IMPL_OFFSET>,
            SetExtensionProperty: SetExtensionProperty::<Impl, IMPL_OFFSET>,
            ListenToServerEvents: ListenToServerEvents::<Impl, IMPL_OFFSET>,
            RegisterDeviceProvider: RegisterDeviceProvider::<Impl, IMPL_OFFSET>,
            UnregisterDeviceProvider: UnregisterDeviceProvider::<Impl, IMPL_OFFSET>,
            RegisterInboundRoutingExtension: RegisterInboundRoutingExtension::<Impl, IMPL_OFFSET>,
            UnregisterInboundRoutingExtension: UnregisterInboundRoutingExtension::<Impl, IMPL_OFFSET>,
            RegisteredEvents: RegisteredEvents::<Impl, IMPL_OFFSET>,
            APIVersion: APIVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServer2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFaxServer_Impl {
    fn Configuration(&mut self) -> ::windows::core::Result<IFaxConfiguration>;
    fn CurrentAccount(&mut self) -> ::windows::core::Result<IFaxAccount>;
    fn FaxAccountSet(&mut self) -> ::windows::core::Result<IFaxAccountSet>;
    fn Security2(&mut self) -> ::windows::core::Result<IFaxSecurity2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxServer2_Vtbl {
        unsafe extern "system" fn Configuration<Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxconfiguration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccount<Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcurrentaccount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaxAccountSet<Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxAccountSet() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxaccountset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security2<Impl: IFaxServer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfaxsecurity2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFaxServer_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            CurrentAccount: CurrentAccount::<Impl, IMPL_OFFSET>,
            FaxAccountSet: FaxAccountSet::<Impl, IMPL_OFFSET>,
            Security2: Security2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServerNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServerNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServerNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxServerNotify_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServerNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFaxServerNotify2_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFaxServerNotify2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServerNotify2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaxServerNotify2_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaxServerNotify2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDevice_Impl: Sized {
    fn Initialize(&mut self, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: super::super::Foundation::PWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::Result<()>;
    fn GetCapabilities(&mut self, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()>;
    fn DeviceReset(&mut self) -> ::windows::core::Result<()>;
    fn Diagnostic(&mut self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()>;
    fn Escape(&mut self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&mut self) -> ::windows::core::Result<u32>;
    fn LockDevice(&mut self, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn UnLockDevice(&mut self) -> ::windows::core::Result<()>;
    fn RawReadData(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&mut self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&mut self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn Subscribe(&mut self, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::Result<()>;
    fn GetLastNotificationData(&mut self) -> ::windows::core::Result<STINOTIFY>;
    fn UnSubscribe(&mut self) -> ::windows::core::Result<()>;
    fn GetLastErrorInfo(&mut self) -> ::windows::core::Result<_ERROR_INFOW>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl IStiDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStiDevice_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: super::super::Foundation::PWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hinst), ::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&dwversion), ::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute_copy(&pdevcaps)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceReset().into()
        }
        unsafe extern "system" fn Diagnostic<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Diagnostic(::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlastdeviceerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockDevice(::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn UnLockDevice<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnLockDevice().into()
        }
        unsafe extern "system" fn RawReadData<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn Subscribe<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Subscribe(::core::mem::transmute_copy(&lpsubsribe)).into()
        }
        unsafe extern "system" fn GetLastNotificationData<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastNotificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *lpnotify = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnSubscribe<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnSubscribe().into()
        }
        unsafe extern "system" fn GetLastErrorInfo<Impl: IStiDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *plasterrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            DeviceReset: DeviceReset::<Impl, IMPL_OFFSET>,
            Diagnostic: Diagnostic::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            GetLastError: GetLastError::<Impl, IMPL_OFFSET>,
            LockDevice: LockDevice::<Impl, IMPL_OFFSET>,
            UnLockDevice: UnLockDevice::<Impl, IMPL_OFFSET>,
            RawReadData: RawReadData::<Impl, IMPL_OFFSET>,
            RawWriteData: RawWriteData::<Impl, IMPL_OFFSET>,
            RawReadCommand: RawReadCommand::<Impl, IMPL_OFFSET>,
            RawWriteCommand: RawWriteCommand::<Impl, IMPL_OFFSET>,
            Subscribe: Subscribe::<Impl, IMPL_OFFSET>,
            GetLastNotificationData: GetLastNotificationData::<Impl, IMPL_OFFSET>,
            UnSubscribe: UnSubscribe::<Impl, IMPL_OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub trait IStiDeviceControl_Impl: Sized {
    fn Initialize(&mut self, dwdevicetype: u32, dwmode: u32, pwszportname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn RawReadData(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&mut self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&mut self, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawDeviceControl(&mut self, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&mut self, lpdwlasterror: *mut u32) -> ::windows::core::Result<()>;
    fn GetMyDevicePortName(&mut self, lpszdevicepath: super::super::Foundation::PWSTR, cwdevicepathsize: u32) -> ::windows::core::Result<()>;
    fn GetMyDeviceHandle(&mut self, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetMyDeviceOpenMode(&mut self, pdwopenmode: *mut u32) -> ::windows::core::Result<()>;
    fn WriteToErrorLog(&mut self, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR, dwerrorcode: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl IStiDeviceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiDeviceControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStiDeviceControl_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&dwdevicetype), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pwszportname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RawReadData<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawDeviceControl<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawDeviceControl(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&dwoutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLastError(::core::mem::transmute_copy(&lpdwlasterror)).into()
        }
        unsafe extern "system" fn GetMyDevicePortName<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdevicepath: super::super::Foundation::PWSTR, cwdevicepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMyDevicePortName(::core::mem::transmute_copy(&lpszdevicepath), ::core::mem::transmute_copy(&cwdevicepathsize)).into()
        }
        unsafe extern "system" fn GetMyDeviceHandle<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMyDeviceHandle(::core::mem::transmute_copy(&lph)).into()
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMyDeviceOpenMode(::core::mem::transmute_copy(&pdwopenmode)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Impl: IStiDeviceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToErrorLog(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute_copy(&pszmessage), ::core::mem::transmute_copy(&dwerrorcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            RawReadData: RawReadData::<Impl, IMPL_OFFSET>,
            RawWriteData: RawWriteData::<Impl, IMPL_OFFSET>,
            RawReadCommand: RawReadCommand::<Impl, IMPL_OFFSET>,
            RawWriteCommand: RawWriteCommand::<Impl, IMPL_OFFSET>,
            RawDeviceControl: RawDeviceControl::<Impl, IMPL_OFFSET>,
            GetLastError: GetLastError::<Impl, IMPL_OFFSET>,
            GetMyDevicePortName: GetMyDevicePortName::<Impl, IMPL_OFFSET>,
            GetMyDeviceHandle: GetMyDeviceHandle::<Impl, IMPL_OFFSET>,
            GetMyDeviceOpenMode: GetMyDeviceOpenMode::<Impl, IMPL_OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiDeviceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
pub trait IStiUSD_Impl: Sized {
    fn Initialize(&mut self, pheldcb: ::core::option::Option<IStiDeviceControl>, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::Result<()>;
    fn GetCapabilities(&mut self) -> ::windows::core::Result<STI_USD_CAPS>;
    fn GetStatus(&mut self, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::Result<()>;
    fn DeviceReset(&mut self) -> ::windows::core::Result<()>;
    fn Diagnostic(&mut self, pbuffer: *mut STI_DIAG) -> ::windows::core::Result<()>;
    fn Escape(&mut self, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::Result<()>;
    fn GetLastError(&mut self) -> ::windows::core::Result<u32>;
    fn LockDevice(&mut self) -> ::windows::core::Result<()>;
    fn UnLockDevice(&mut self) -> ::windows::core::Result<()>;
    fn RawReadData(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteData(&mut self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawReadCommand(&mut self, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn RawWriteCommand(&mut self, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()>;
    fn SetNotificationHandle(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetNotificationData(&mut self) -> ::windows::core::Result<STINOTIFY>;
    fn GetLastErrorInfo(&mut self) -> ::windows::core::Result<_ERROR_INFOW>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO", feature = "Win32_System_Registry"))]
impl IStiUSD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiUSD_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStiUSD_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheldcb: ::windows::core::RawPtr, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pheldcb), ::core::mem::transmute_copy(&dwstiversion), ::core::mem::transmute_copy(&hparameterskey)).into()
        }
        unsafe extern "system" fn GetCapabilities<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pdevcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatus(::core::mem::transmute_copy(&pdevstatus)).into()
        }
        unsafe extern "system" fn DeviceReset<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceReset().into()
        }
        unsafe extern "system" fn Diagnostic<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Diagnostic(::core::mem::transmute_copy(&pbuffer)).into()
        }
        unsafe extern "system" fn Escape<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Escape(::core::mem::transmute_copy(&escapefunction), ::core::mem::transmute_copy(&lpindata), ::core::mem::transmute_copy(&cbindatasize), ::core::mem::transmute_copy(&poutdata), ::core::mem::transmute_copy(&cboutdatasize), ::core::mem::transmute_copy(&pdwactualdata)).into()
        }
        unsafe extern "system" fn GetLastError<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlastdeviceerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockDevice().into()
        }
        unsafe extern "system" fn UnLockDevice<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnLockDevice().into()
        }
        unsafe extern "system" fn RawReadData<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteData(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawReadCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&lpdwnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RawWriteCommand(::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&nnumberofbytes), ::core::mem::transmute_copy(&lpoverlapped)).into()
        }
        unsafe extern "system" fn SetNotificationHandle<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationHandle(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetNotificationData<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationData() {
                ::core::result::Result::Ok(ok__) => {
                    *lpnotify = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Impl: IStiUSD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *plasterrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            DeviceReset: DeviceReset::<Impl, IMPL_OFFSET>,
            Diagnostic: Diagnostic::<Impl, IMPL_OFFSET>,
            Escape: Escape::<Impl, IMPL_OFFSET>,
            GetLastError: GetLastError::<Impl, IMPL_OFFSET>,
            LockDevice: LockDevice::<Impl, IMPL_OFFSET>,
            UnLockDevice: UnLockDevice::<Impl, IMPL_OFFSET>,
            RawReadData: RawReadData::<Impl, IMPL_OFFSET>,
            RawWriteData: RawWriteData::<Impl, IMPL_OFFSET>,
            RawReadCommand: RawReadCommand::<Impl, IMPL_OFFSET>,
            RawWriteCommand: RawWriteCommand::<Impl, IMPL_OFFSET>,
            SetNotificationHandle: SetNotificationHandle::<Impl, IMPL_OFFSET>,
            GetNotificationData: GetNotificationData::<Impl, IMPL_OFFSET>,
            GetLastErrorInfo: GetLastErrorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStiUSD as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStillImageW_Impl: Sized {
    fn Initialize(&mut self, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::Result<()>;
    fn GetDeviceList(&mut self, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDeviceInfo(&mut self, pwszdevicename: super::super::Foundation::PWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateDevice(&mut self, pwszdevicename: super::super::Foundation::PWSTR, dwmode: u32, pdevice: *mut ::core::option::Option<IStiDevice>, punkouter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDeviceValue(&mut self, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::Result<()>;
    fn SetDeviceValue(&mut self, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn GetSTILaunchInformation(&mut self, pwszdevicename: super::super::Foundation::PWSTR, pdweventcode: *mut u32, pwszeventname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RegisterLaunchApplication(&mut self, pwszappname: super::super::Foundation::PWSTR, pwszcommandline: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UnregisterLaunchApplication(&mut self, pwszappname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn EnableHwNotifications(&mut self, pwszdevicename: super::super::Foundation::PWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetHwNotificationState(&mut self, pwszdevicename: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn RefreshDeviceBus(&mut self, pwszdevicename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn LaunchApplicationForDevice(&mut self, pwszdevicename: super::super::Foundation::PWSTR, pwszappname: super::super::Foundation::PWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::Result<()>;
    fn SetupDeviceParameters(&mut self, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::Result<()>;
    fn WriteToErrorLog(&mut self, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IStillImageW_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStillImageW_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStillImageW_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hinst), ::core::mem::transmute_copy(&dwversion)).into()
        }
        unsafe extern "system" fn GetDeviceList<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceList(::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwitemsreturned), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceInfo(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn CreateDevice<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, dwmode: u32, pdevice: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDevice(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdevice), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn GetDeviceValue<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceValue(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn SetDeviceValue<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceValue(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pvaluename), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn GetSTILaunchInformation<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pdweventcode: *mut u32, pwszeventname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSTILaunchInformation(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pdweventcode), ::core::mem::transmute_copy(&pwszeventname)).into()
        }
        unsafe extern "system" fn RegisterLaunchApplication<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: super::super::Foundation::PWSTR, pwszcommandline: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterLaunchApplication(::core::mem::transmute_copy(&pwszappname), ::core::mem::transmute_copy(&pwszcommandline)).into()
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterLaunchApplication(::core::mem::transmute_copy(&pwszappname)).into()
        }
        unsafe extern "system" fn EnableHwNotifications<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableHwNotifications(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&bnewstate)).into()
        }
        unsafe extern "system" fn GetHwNotificationState<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHwNotificationState(::core::mem::transmute_copy(&pwszdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbcurrentstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshDeviceBus(::core::mem::transmute_copy(&pwszdevicename)).into()
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pwszappname: super::super::Foundation::PWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LaunchApplicationForDevice(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pwszappname), ::core::mem::transmute_copy(&pstinotify)).into()
        }
        unsafe extern "system" fn SetupDeviceParameters<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetupDeviceParameters(::core::mem::transmute_copy(&param0)).into()
        }
        unsafe extern "system" fn WriteToErrorLog<Impl: IStillImageW_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteToErrorLog(::core::mem::transmute_copy(&dwmessagetype), ::core::mem::transmute_copy(&pszmessage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetDeviceList: GetDeviceList::<Impl, IMPL_OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Impl, IMPL_OFFSET>,
            CreateDevice: CreateDevice::<Impl, IMPL_OFFSET>,
            GetDeviceValue: GetDeviceValue::<Impl, IMPL_OFFSET>,
            SetDeviceValue: SetDeviceValue::<Impl, IMPL_OFFSET>,
            GetSTILaunchInformation: GetSTILaunchInformation::<Impl, IMPL_OFFSET>,
            RegisterLaunchApplication: RegisterLaunchApplication::<Impl, IMPL_OFFSET>,
            UnregisterLaunchApplication: UnregisterLaunchApplication::<Impl, IMPL_OFFSET>,
            EnableHwNotifications: EnableHwNotifications::<Impl, IMPL_OFFSET>,
            GetHwNotificationState: GetHwNotificationState::<Impl, IMPL_OFFSET>,
            RefreshDeviceBus: RefreshDeviceBus::<Impl, IMPL_OFFSET>,
            LaunchApplicationForDevice: LaunchApplicationForDevice::<Impl, IMPL_OFFSET>,
            SetupDeviceParameters: SetupDeviceParameters::<Impl, IMPL_OFFSET>,
            WriteToErrorLog: WriteToErrorLog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStillImageW as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IFaxAccountNotify_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobRemoved(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobChanged(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR, pjobstatus: ::core::option::Option<IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnOutgoingJobAdded(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobRemoved(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobChanged(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrjobid: super::super::Foundation::BSTR, pjobstatus: ::core::option::Option<IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnIncomingMessageAdded(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrmessageid: super::super::Foundation::BSTR, faddedtoreceivefolder: i16) -> ::windows::core::Result<()>;
    fn OnIncomingMessageRemoved(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrmessageid: super::super::Foundation::BSTR, fremovedfromreceivefolder: i16) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageAdded(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageRemoved(&mut self, pfaxaccount: ::core::option::Option<IFaxAccount>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnServerShutDown(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IFaxAccountNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IFaxAccountNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IFaxAccountNotify_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobAdded(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobRemoved(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobChanged(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid), ::core::mem::transmute(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobAdded(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobRemoved(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobChanged(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrjobid), ::core::mem::transmute(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, faddedtoreceivefolder: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingMessageAdded(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrmessageid), ::core::mem::transmute_copy(&faddedtoreceivefolder)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fremovedfromreceivefolder: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingMessageRemoved(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrmessageid), ::core::mem::transmute_copy(&fremovedfromreceivefolder)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingMessageAdded(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingMessageRemoved(::core::mem::transmute(&pfaxaccount), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Impl: _IFaxAccountNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnServerShutDown(::core::mem::transmute(&pfaxserver)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Impl, IMPL_OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Impl, IMPL_OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Impl, IMPL_OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Impl, IMPL_OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Impl, IMPL_OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Impl, IMPL_OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Impl, IMPL_OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Impl, IMPL_OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Impl, IMPL_OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Impl, IMPL_OFFSET>,
            OnServerShutDown: OnServerShutDown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IFaxAccountNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _IFaxServerNotify2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn OnIncomingJobAdded(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobRemoved(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingJobChanged(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR, pjobstatus: ::core::option::Option<IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnOutgoingJobAdded(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobRemoved(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingJobChanged(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrjobid: super::super::Foundation::BSTR, pjobstatus: ::core::option::Option<IFaxJobStatus>) -> ::windows::core::Result<()>;
    fn OnIncomingMessageAdded(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnIncomingMessageRemoved(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageAdded(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnOutgoingMessageRemoved(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, bstrmessageid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnReceiptOptionsChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnActivityLoggingConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnSecurityConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnEventLoggingConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutgoingQueueConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutgoingArchiveConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnIncomingArchiveConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnDevicesConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutboundRoutingGroupsConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnOutboundRoutingRulesConfigChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnServerActivityChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::Result<()>;
    fn OnQueuesStatusChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::core::Result<()>;
    fn OnNewCall(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, lcallid: i32, ldeviceid: i32, bstrcallerid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OnServerShutDown(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
    fn OnDeviceStatusChange(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::core::Result<()>;
    fn OnGeneralServerConfigChanged(&mut self, pfaxserver: ::core::option::Option<IFaxServer2>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _IFaxServerNotify2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IFaxServerNotify2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _IFaxServerNotify2_Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobAdded(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobRemoved(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnIncomingJobChanged<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingJobChanged(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid), ::core::mem::transmute(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobAdded(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobRemoved(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid)).into()
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingJobChanged(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrjobid), ::core::mem::transmute(&pjobstatus)).into()
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingMessageAdded(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingMessageRemoved(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingMessageAdded(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingMessageRemoved(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&bstrmessageid)).into()
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnReceiptOptionsChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityLoggingConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnSecurityConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSecurityConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEventLoggingConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingQueueConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutgoingArchiveConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIncomingArchiveConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDevicesConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDevicesConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutboundRoutingGroupsConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutboundRoutingRulesConfigChange(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnServerActivityChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnServerActivityChange(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&lincomingmessages), ::core::mem::transmute_copy(&lroutingmessages), ::core::mem::transmute_copy(&loutgoingmessages), ::core::mem::transmute_copy(&lqueuedmessages)).into()
        }
        unsafe extern "system" fn OnQueuesStatusChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQueuesStatusChange(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&boutgoingqueueblocked), ::core::mem::transmute_copy(&boutgoingqueuepaused), ::core::mem::transmute_copy(&bincomingqueueblocked)).into()
        }
        unsafe extern "system" fn OnNewCall<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lcallid: i32, ldeviceid: i32, bstrcallerid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNewCall(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&lcallid), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&bstrcallerid)).into()
        }
        unsafe extern "system" fn OnServerShutDown<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnServerShutDown(::core::mem::transmute(&pfaxserver)).into()
        }
        unsafe extern "system" fn OnDeviceStatusChange<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDeviceStatusChange(::core::mem::transmute(&pfaxserver), ::core::mem::transmute_copy(&ldeviceid), ::core::mem::transmute_copy(&bpoweredoff), ::core::mem::transmute_copy(&bsending), ::core::mem::transmute_copy(&breceiving), ::core::mem::transmute_copy(&bringing)).into()
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Impl: _IFaxServerNotify2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGeneralServerConfigChanged(::core::mem::transmute(&pfaxserver)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnIncomingJobAdded: OnIncomingJobAdded::<Impl, IMPL_OFFSET>,
            OnIncomingJobRemoved: OnIncomingJobRemoved::<Impl, IMPL_OFFSET>,
            OnIncomingJobChanged: OnIncomingJobChanged::<Impl, IMPL_OFFSET>,
            OnOutgoingJobAdded: OnOutgoingJobAdded::<Impl, IMPL_OFFSET>,
            OnOutgoingJobRemoved: OnOutgoingJobRemoved::<Impl, IMPL_OFFSET>,
            OnOutgoingJobChanged: OnOutgoingJobChanged::<Impl, IMPL_OFFSET>,
            OnIncomingMessageAdded: OnIncomingMessageAdded::<Impl, IMPL_OFFSET>,
            OnIncomingMessageRemoved: OnIncomingMessageRemoved::<Impl, IMPL_OFFSET>,
            OnOutgoingMessageAdded: OnOutgoingMessageAdded::<Impl, IMPL_OFFSET>,
            OnOutgoingMessageRemoved: OnOutgoingMessageRemoved::<Impl, IMPL_OFFSET>,
            OnReceiptOptionsChange: OnReceiptOptionsChange::<Impl, IMPL_OFFSET>,
            OnActivityLoggingConfigChange: OnActivityLoggingConfigChange::<Impl, IMPL_OFFSET>,
            OnSecurityConfigChange: OnSecurityConfigChange::<Impl, IMPL_OFFSET>,
            OnEventLoggingConfigChange: OnEventLoggingConfigChange::<Impl, IMPL_OFFSET>,
            OnOutgoingQueueConfigChange: OnOutgoingQueueConfigChange::<Impl, IMPL_OFFSET>,
            OnOutgoingArchiveConfigChange: OnOutgoingArchiveConfigChange::<Impl, IMPL_OFFSET>,
            OnIncomingArchiveConfigChange: OnIncomingArchiveConfigChange::<Impl, IMPL_OFFSET>,
            OnDevicesConfigChange: OnDevicesConfigChange::<Impl, IMPL_OFFSET>,
            OnOutboundRoutingGroupsConfigChange: OnOutboundRoutingGroupsConfigChange::<Impl, IMPL_OFFSET>,
            OnOutboundRoutingRulesConfigChange: OnOutboundRoutingRulesConfigChange::<Impl, IMPL_OFFSET>,
            OnServerActivityChange: OnServerActivityChange::<Impl, IMPL_OFFSET>,
            OnQueuesStatusChange: OnQueuesStatusChange::<Impl, IMPL_OFFSET>,
            OnNewCall: OnNewCall::<Impl, IMPL_OFFSET>,
            OnServerShutDown: OnServerShutDown::<Impl, IMPL_OFFSET>,
            OnDeviceStatusChange: OnDeviceStatusChange::<Impl, IMPL_OFFSET>,
            OnGeneralServerConfigChanged: OnGeneralServerConfigChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_IFaxServerNotify2 as ::windows::core::Interface>::IID
    }
}
