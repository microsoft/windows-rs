#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountImpl: Sized + IDispatchImpl {
    fn AccountName();
    fn Folders();
    fn ListenToAccountEvents();
    fn RegisteredEvents();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccount {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccount";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountImpl, const OFFSET: isize>() -> IFaxAccountVtbl {
        unsafe extern "system" fn AccountName<Impl: IFaxAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraccountname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName(::core::mem::transmute_copy(&pbstraccountname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Impl: IFaxAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folders(::core::mem::transmute_copy(&ppfolders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenToAccountEvents<Impl: IFaxAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenToAccountEvents(eventtypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisteredEvents<Impl: IFaxAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregisteredevents: *mut FAX_ACCOUNT_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredEvents(::core::mem::transmute_copy(&pregisteredevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccount>, ::windows::core::GetTrustLevel, AccountName::<Impl, OFFSET>, Folders::<Impl, OFFSET>, ListenToAccountEvents::<Impl, OFFSET>, RegisteredEvents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountFoldersImpl: Sized + IDispatchImpl {
    fn OutgoingQueue();
    fn IncomingQueue();
    fn IncomingArchive();
    fn OutgoingArchive();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountFolders {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountFolders";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountFoldersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountFoldersImpl, const OFFSET: isize>() -> IFaxAccountFoldersVtbl {
        unsafe extern "system" fn OutgoingQueue<Impl: IFaxAccountFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueue(::core::mem::transmute_copy(&pfaxoutgoingqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Impl: IFaxAccountFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueue(::core::mem::transmute_copy(&pfaxincomingqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Impl: IFaxAccountFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingArchive(::core::mem::transmute_copy(&pfaxincomingarchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Impl: IFaxAccountFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingArchive(::core::mem::transmute_copy(&pfaxoutgoingarchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountFolders>, ::windows::core::GetTrustLevel, OutgoingQueue::<Impl, OFFSET>, IncomingQueue::<Impl, OFFSET>, IncomingArchive::<Impl, OFFSET>, OutgoingArchive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingArchiveImpl: Sized + IDispatchImpl {
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountIncomingArchive {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountIncomingArchive";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingArchiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>() -> IFaxAccountIncomingArchiveVtbl {
        unsafe extern "system" fn SizeLow<Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow(::core::mem::transmute_copy(&plsizelow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh(::core::mem::transmute_copy(&plsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(lprefetchsize, ::core::mem::transmute_copy(&pfaxincomingmessageiterator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxAccountIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(&*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxincomingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountIncomingArchive>, ::windows::core::GetTrustLevel, SizeLow::<Impl, OFFSET>, SizeHigh::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, GetMessages::<Impl, OFFSET>, GetMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountIncomingQueueImpl: Sized + IDispatchImpl {
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountIncomingQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountIncomingQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountIncomingQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountIncomingQueueImpl, const OFFSET: isize>() -> IFaxAccountIncomingQueueVtbl {
        unsafe extern "system" fn GetJobs<Impl: IFaxAccountIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs(::core::mem::transmute_copy(&pfaxincomingjobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxAccountIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxincomingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountIncomingQueue>, ::windows::core::GetTrustLevel, GetJobs::<Impl, OFFSET>, GetJob::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountNotifyImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountNotify {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountNotify";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountNotifyImpl, const OFFSET: isize>() -> IFaxAccountNotifyVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountNotify>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingArchiveImpl: Sized + IDispatchImpl {
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountOutgoingArchive {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountOutgoingArchive";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingArchiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>() -> IFaxAccountOutgoingArchiveVtbl {
        unsafe extern "system" fn SizeLow<Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow(::core::mem::transmute_copy(&plsizelow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh(::core::mem::transmute_copy(&plsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(lprefetchsize, ::core::mem::transmute_copy(&pfaxoutgoingmessageiterator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxAccountOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(&*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutgoingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountOutgoingArchive>, ::windows::core::GetTrustLevel, SizeLow::<Impl, OFFSET>, SizeHigh::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, GetMessages::<Impl, OFFSET>, GetMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountOutgoingQueueImpl: Sized + IDispatchImpl {
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountOutgoingQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountOutgoingQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountOutgoingQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountOutgoingQueueImpl, const OFFSET: isize>() -> IFaxAccountOutgoingQueueVtbl {
        unsafe extern "system" fn GetJobs<Impl: IFaxAccountOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs(::core::mem::transmute_copy(&pfaxoutgoingjobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxAccountOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutgoingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountOutgoingQueue>, ::windows::core::GetTrustLevel, GetJobs::<Impl, OFFSET>, GetJob::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountSetImpl: Sized + IDispatchImpl {
    fn GetAccounts();
    fn GetAccount();
    fn AddAccount();
    fn RemoveAccount();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccountSet {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccountSet";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountSetImpl, const OFFSET: isize>() -> IFaxAccountSetVtbl {
        unsafe extern "system" fn GetAccounts<Impl: IFaxAccountSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccounts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccounts(::core::mem::transmute_copy(&ppfaxaccounts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccount<Impl: IFaxAccountSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccount(&*(&bstraccountname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxaccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccount<Impl: IFaxAccountSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAccount(&*(&bstraccountname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxaccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccount<Impl: IFaxAccountSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstraccountname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAccount(&*(&bstraccountname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccountSet>, ::windows::core::GetTrustLevel, GetAccounts::<Impl, OFFSET>, GetAccount::<Impl, OFFSET>, AddAccount::<Impl, OFFSET>, RemoveAccount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxAccountsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxAccounts {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxAccounts";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxAccountsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxAccountsImpl, const OFFSET: isize>() -> IFaxAccountsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxAccountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxAccountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxaccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxAccountsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxAccounts>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivityImpl: Sized + IDispatchImpl {
    fn IncomingMessages();
    fn RoutingMessages();
    fn OutgoingMessages();
    fn QueuedMessages();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxActivity {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxActivity";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxActivityImpl, const OFFSET: isize>() -> IFaxActivityVtbl {
        unsafe extern "system" fn IncomingMessages<Impl: IFaxActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plincomingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingMessages(::core::mem::transmute_copy(&plincomingmessages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingMessages<Impl: IFaxActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plroutingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingMessages(::core::mem::transmute_copy(&plroutingmessages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingMessages<Impl: IFaxActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingMessages(::core::mem::transmute_copy(&ploutgoingmessages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueuedMessages<Impl: IFaxActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuedmessages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueuedMessages(::core::mem::transmute_copy(&plqueuedmessages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxActivity>, ::windows::core::GetTrustLevel, IncomingMessages::<Impl, OFFSET>, RoutingMessages::<Impl, OFFSET>, OutgoingMessages::<Impl, OFFSET>, QueuedMessages::<Impl, OFFSET>, Refresh::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxActivityLoggingImpl: Sized + IDispatchImpl {
    fn LogIncoming();
    fn SetLogIncoming();
    fn LogOutgoing();
    fn SetLogOutgoing();
    fn DatabasePath();
    fn SetDatabasePath();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxActivityLogging {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxActivityLogging";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxActivityLoggingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxActivityLoggingImpl, const OFFSET: isize>() -> IFaxActivityLoggingVtbl {
        unsafe extern "system" fn LogIncoming<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogincoming: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogIncoming(::core::mem::transmute_copy(&pblogincoming)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogIncoming<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogincoming: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLogIncoming(blogincoming) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogOutgoing<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblogoutgoing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogOutgoing(::core::mem::transmute_copy(&pblogoutgoing)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOutgoing<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blogoutgoing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLogOutgoing(blogoutgoing) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DatabasePath<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdatabasepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DatabasePath(::core::mem::transmute_copy(&pbstrdatabasepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDatabasePath<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdatabasepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDatabasePath(&*(&bstrdatabasepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxActivityLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxActivityLogging>,
            ::windows::core::GetTrustLevel,
            LogIncoming::<Impl, OFFSET>,
            SetLogIncoming::<Impl, OFFSET>,
            LogOutgoing::<Impl, OFFSET>,
            SetLogOutgoing::<Impl, OFFSET>,
            DatabasePath::<Impl, OFFSET>,
            SetDatabasePath::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxConfigurationImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveLocation();
    fn SetArchiveLocation();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn ArchiveAgeLimit();
    fn SetArchiveAgeLimit();
    fn ArchiveSizeLow();
    fn ArchiveSizeHigh();
    fn OutgoingQueueBlocked();
    fn SetOutgoingQueueBlocked();
    fn OutgoingQueuePaused();
    fn SetOutgoingQueuePaused();
    fn AllowPersonalCoverPages();
    fn SetAllowPersonalCoverPages();
    fn UseDeviceTSID();
    fn SetUseDeviceTSID();
    fn Retries();
    fn SetRetries();
    fn RetryDelay();
    fn SetRetryDelay();
    fn DiscountRateStart();
    fn SetDiscountRateStart();
    fn DiscountRateEnd();
    fn SetDiscountRateEnd();
    fn OutgoingQueueAgeLimit();
    fn SetOutgoingQueueAgeLimit();
    fn Branding();
    fn SetBranding();
    fn IncomingQueueBlocked();
    fn SetIncomingQueueBlocked();
    fn AutoCreateAccountOnConnect();
    fn SetAutoCreateAccountOnConnect();
    fn IncomingFaxesArePublic();
    fn SetIncomingFaxesArePublic();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxConfiguration {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxConfiguration";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxConfigurationImpl, const OFFSET: isize>() -> IFaxConfigurationVtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive(::core::mem::transmute_copy(&pbusearchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseArchive(busearchive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveLocation<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveLocation(::core::mem::transmute_copy(&pbstrarchivelocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveLocation<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetArchiveLocation(&*(&bstrarchivelocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning(::core::mem::transmute_copy(&pbsizequotawarning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSizeQuotaWarning(bsizequotawarning) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark(::core::mem::transmute_copy(&plhighquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHighQuotaWaterMark(lhighquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark(::core::mem::transmute_copy(&pllowquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLowQuotaWaterMark(llowquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveAgeLimit<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarchiveagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveAgeLimit(::core::mem::transmute_copy(&plarchiveagelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveAgeLimit<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, larchiveagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetArchiveAgeLimit(larchiveagelimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveSizeLow<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveSizeLow(::core::mem::transmute_copy(&plsizelow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveSizeHigh<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveSizeHigh(::core::mem::transmute_copy(&plsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueueBlocked<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueueBlocked(::core::mem::transmute_copy(&pboutgoingblocked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueBlocked<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutgoingQueueBlocked(boutgoingblocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueuePaused<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboutgoingpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueuePaused(::core::mem::transmute_copy(&pboutgoingpaused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueuePaused<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boutgoingpaused: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutgoingQueuePaused(boutgoingpaused) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPersonalCoverPages(::core::mem::transmute_copy(&pballowpersonalcoverpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowPersonalCoverPages(ballowpersonalcoverpages) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDeviceTSID<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDeviceTSID(::core::mem::transmute_copy(&pbusedevicetsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseDeviceTSID(busedevicetsid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetries(lretries) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetryDelay<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryDelay(::core::mem::transmute_copy(&plretrydelay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetryDelay(lretrydelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscountRateStart<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateStart(::core::mem::transmute_copy(&pdatediscountratestart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDiscountRateStart(datediscountratestart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscountRateEnd<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateEnd(::core::mem::transmute_copy(&pdatediscountrateend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDiscountRateEnd(datediscountrateend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingQueueAgeLimit<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploutgoingqueueagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueueAgeLimit(::core::mem::transmute_copy(&ploutgoingqueueagelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingQueueAgeLimit<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loutgoingqueueagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutgoingQueueAgeLimit(loutgoingqueueagelimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Branding<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Branding(::core::mem::transmute_copy(&pbbranding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBranding(bbranding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueueBlocked<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueueBlocked(::core::mem::transmute_copy(&pbincomingblocked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingQueueBlocked<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIncomingQueueBlocked(bincomingblocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoCreateAccountOnConnect<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbautocreateaccountonconnect: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoCreateAccountOnConnect(::core::mem::transmute_copy(&pbautocreateaccountonconnect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoCreateAccountOnConnect<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bautocreateaccountonconnect: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoCreateAccountOnConnect(bautocreateaccountonconnect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingFaxesArePublic<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbincomingfaxesarepublic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingFaxesArePublic(::core::mem::transmute_copy(&pbincomingfaxesarepublic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingFaxesArePublic<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bincomingfaxesarepublic: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIncomingFaxesArePublic(bincomingfaxesarepublic) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxConfiguration>,
            ::windows::core::GetTrustLevel,
            UseArchive::<Impl, OFFSET>,
            SetUseArchive::<Impl, OFFSET>,
            ArchiveLocation::<Impl, OFFSET>,
            SetArchiveLocation::<Impl, OFFSET>,
            SizeQuotaWarning::<Impl, OFFSET>,
            SetSizeQuotaWarning::<Impl, OFFSET>,
            HighQuotaWaterMark::<Impl, OFFSET>,
            SetHighQuotaWaterMark::<Impl, OFFSET>,
            LowQuotaWaterMark::<Impl, OFFSET>,
            SetLowQuotaWaterMark::<Impl, OFFSET>,
            ArchiveAgeLimit::<Impl, OFFSET>,
            SetArchiveAgeLimit::<Impl, OFFSET>,
            ArchiveSizeLow::<Impl, OFFSET>,
            ArchiveSizeHigh::<Impl, OFFSET>,
            OutgoingQueueBlocked::<Impl, OFFSET>,
            SetOutgoingQueueBlocked::<Impl, OFFSET>,
            OutgoingQueuePaused::<Impl, OFFSET>,
            SetOutgoingQueuePaused::<Impl, OFFSET>,
            AllowPersonalCoverPages::<Impl, OFFSET>,
            SetAllowPersonalCoverPages::<Impl, OFFSET>,
            UseDeviceTSID::<Impl, OFFSET>,
            SetUseDeviceTSID::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            SetRetries::<Impl, OFFSET>,
            RetryDelay::<Impl, OFFSET>,
            SetRetryDelay::<Impl, OFFSET>,
            DiscountRateStart::<Impl, OFFSET>,
            SetDiscountRateStart::<Impl, OFFSET>,
            DiscountRateEnd::<Impl, OFFSET>,
            SetDiscountRateEnd::<Impl, OFFSET>,
            OutgoingQueueAgeLimit::<Impl, OFFSET>,
            SetOutgoingQueueAgeLimit::<Impl, OFFSET>,
            Branding::<Impl, OFFSET>,
            SetBranding::<Impl, OFFSET>,
            IncomingQueueBlocked::<Impl, OFFSET>,
            SetIncomingQueueBlocked::<Impl, OFFSET>,
            AutoCreateAccountOnConnect::<Impl, OFFSET>,
            SetAutoCreateAccountOnConnect::<Impl, OFFSET>,
            IncomingFaxesArePublic::<Impl, OFFSET>,
            SetIncomingFaxesArePublic::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceImpl: Sized + IDispatchImpl {
    fn Id();
    fn DeviceName();
    fn ProviderUniqueName();
    fn PoweredOff();
    fn ReceivingNow();
    fn SendingNow();
    fn UsedRoutingMethods();
    fn Description();
    fn SetDescription();
    fn SendEnabled();
    fn SetSendEnabled();
    fn ReceiveMode();
    fn SetReceiveMode();
    fn RingsBeforeAnswer();
    fn SetRingsBeforeAnswer();
    fn CSID();
    fn SetCSID();
    fn TSID();
    fn SetTSID();
    fn Refresh();
    fn Save();
    fn GetExtensionProperty();
    fn SetExtensionProperty();
    fn UseRoutingMethod();
    fn RingingNow();
    fn AnswerCall();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDevice {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDevice";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceImpl, const OFFSET: isize>() -> IFaxDeviceVtbl {
        unsafe extern "system" fn Id<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&plid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderUniqueName<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprovideruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderUniqueName(::core::mem::transmute_copy(&pbstrprovideruniquename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PoweredOff<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoweredoff: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PoweredOff(::core::mem::transmute_copy(&pbpoweredoff)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivingNow<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbreceivingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivingNow(::core::mem::transmute_copy(&pbreceivingnow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendingNow<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendingNow(::core::mem::transmute_copy(&pbsendingnow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedRoutingMethods<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvusedroutingmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedRoutingMethods(::core::mem::transmute_copy(&pvusedroutingmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendEnabled<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsendenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendEnabled(::core::mem::transmute_copy(&pbsendenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendEnabled<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsendenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSendEnabled(bsendenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveMode<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceivemode: *mut FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveMode(::core::mem::transmute_copy(&preceivemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveMode<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receivemode: FAX_DEVICE_RECEIVE_MODE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReceiveMode(receivemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingsBeforeAnswer<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringsbeforeanswer: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingsBeforeAnswer(::core::mem::transmute_copy(&plringsbeforeanswer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingsBeforeAnswer<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringsbeforeanswer: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRingsBeforeAnswer(lringsbeforeanswer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCSID<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCSID(&*(&bstrcsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTSID(&*(&bstrtsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtensionProperty<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionProperty(&*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtensionProperty(&*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&vproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseRoutingMethod<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, buse: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseRoutingMethod(&*(&bstrmethodguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), buse) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingingNow<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbringingnow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingingNow(::core::mem::transmute_copy(&pbringingnow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnswerCall<Impl: IFaxDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnswerCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxDevice>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            DeviceName::<Impl, OFFSET>,
            ProviderUniqueName::<Impl, OFFSET>,
            PoweredOff::<Impl, OFFSET>,
            ReceivingNow::<Impl, OFFSET>,
            SendingNow::<Impl, OFFSET>,
            UsedRoutingMethods::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            SendEnabled::<Impl, OFFSET>,
            SetSendEnabled::<Impl, OFFSET>,
            ReceiveMode::<Impl, OFFSET>,
            SetReceiveMode::<Impl, OFFSET>,
            RingsBeforeAnswer::<Impl, OFFSET>,
            SetRingsBeforeAnswer::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            SetCSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            SetTSID::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            GetExtensionProperty::<Impl, OFFSET>,
            SetExtensionProperty::<Impl, OFFSET>,
            UseRoutingMethod::<Impl, OFFSET>,
            RingingNow::<Impl, OFFSET>,
            AnswerCall::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceIdsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn SetOrder();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDeviceIds {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDeviceIds";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceIdsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceIdsImpl, const OFFSET: isize>() -> IFaxDeviceIdsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&pldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(ldeviceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrder<Impl: IFaxDeviceIdsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldeviceid: i32, lneworder: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOrder(ldeviceid, lneworder) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxDeviceIds>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, SetOrder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProviderImpl: Sized + IDispatchImpl {
    fn FriendlyName();
    fn ImageName();
    fn UniqueName();
    fn TapiProviderName();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Status();
    fn InitErrorCode();
    fn DeviceIds();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDeviceProvider {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDeviceProvider";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceProviderImpl, const OFFSET: isize>() -> IFaxDeviceProviderVtbl {
        unsafe extern "system" fn FriendlyName<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName(::core::mem::transmute_copy(&pbstrfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName(::core::mem::transmute_copy(&pbstrimagename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName(::core::mem::transmute_copy(&pbstruniquename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapiProviderName<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtapiprovidername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapiProviderName(::core::mem::transmute_copy(&pbstrtapiprovidername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion(::core::mem::transmute_copy(&plmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion(::core::mem::transmute_copy(&plminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild(::core::mem::transmute_copy(&plmajorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild(::core::mem::transmute_copy(&plminorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug(::core::mem::transmute_copy(&pbdebug)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitErrorCode(::core::mem::transmute_copy(&pliniterrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Impl: IFaxDeviceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdeviceids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIds(::core::mem::transmute_copy(&pvdeviceids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxDeviceProvider>,
            ::windows::core::GetTrustLevel,
            FriendlyName::<Impl, OFFSET>,
            ImageName::<Impl, OFFSET>,
            UniqueName::<Impl, OFFSET>,
            TapiProviderName::<Impl, OFFSET>,
            MajorVersion::<Impl, OFFSET>,
            MinorVersion::<Impl, OFFSET>,
            MajorBuild::<Impl, OFFSET>,
            MinorBuild::<Impl, OFFSET>,
            Debug::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            InitErrorCode::<Impl, OFFSET>,
            DeviceIds::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDeviceProvidersImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDeviceProviders {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDeviceProviders";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDeviceProvidersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDeviceProvidersImpl, const OFFSET: isize>() -> IFaxDeviceProvidersVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDeviceProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDeviceProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdeviceprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxdeviceprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDeviceProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxDeviceProviders>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDevicesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemById();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDevices {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDevices";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDevicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDevicesImpl, const OFFSET: isize>() -> IFaxDevicesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemById<Impl: IFaxDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, ppfaxdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemById(lid, ::core::mem::transmute_copy(&ppfaxdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxDevices>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, ItemById::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocumentImpl: Sized + IDispatchImpl {
    fn Body();
    fn SetBody();
    fn Sender();
    fn Recipients();
    fn CoverPage();
    fn SetCoverPage();
    fn Subject();
    fn SetSubject();
    fn Note();
    fn SetNote();
    fn ScheduleTime();
    fn SetScheduleTime();
    fn ReceiptAddress();
    fn SetReceiptAddress();
    fn DocumentName();
    fn SetDocumentName();
    fn CallHandle();
    fn SetCallHandle();
    fn CoverPageType();
    fn SetCoverPageType();
    fn ScheduleType();
    fn SetScheduleType();
    fn ReceiptType();
    fn SetReceiptType();
    fn GroupBroadcastReceipts();
    fn SetGroupBroadcastReceipts();
    fn Priority();
    fn SetPriority();
    fn TapiConnection();
    fn putref_TapiConnection();
    fn Submit();
    fn ConnectedSubmit();
    fn AttachFaxToReceipt();
    fn SetAttachFaxToReceipt();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDocument {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDocument";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocumentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDocumentImpl, const OFFSET: isize>() -> IFaxDocumentVtbl {
        unsafe extern "system" fn Body<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body(::core::mem::transmute_copy(&pbstrbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBody(&*(&bstrbody as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender(::core::mem::transmute_copy(&ppfaxsender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipients: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients(::core::mem::transmute_copy(&ppfaxrecipients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoverPage<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcoverpage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoverPage(::core::mem::transmute_copy(&pbstrcoverpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPage<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcoverpage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoverPage(&*(&bstrcoverpage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject(::core::mem::transmute_copy(&pbstrsubject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSubject(&*(&bstrsubject as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Note<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrnote: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Note(::core::mem::transmute_copy(&pbstrnote)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNote<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrnote: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNote(&*(&bstrnote as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTime<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduletime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleTime(::core::mem::transmute_copy(&pdatescheduletime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleTime<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datescheduletime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScheduleTime(datescheduletime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress(::core::mem::transmute_copy(&pbstrreceiptaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptAddress<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreceiptaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReceiptAddress(&*(&bstrreceiptaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName(::core::mem::transmute_copy(&pbstrdocumentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentName<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdocumentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDocumentName(&*(&bstrdocumentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHandle<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHandle(::core::mem::transmute_copy(&plcallhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHandle<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcallhandle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCallHandle(lcallhandle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoverPageType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoverpagetype: *mut FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoverPageType(::core::mem::transmute_copy(&pcoverpagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoverPageType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coverpagetype: FAX_COVERPAGE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoverPageType(coverpagetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleType(::core::mem::transmute_copy(&pscheduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduleType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheduletype: FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScheduleType(scheduletype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType(::core::mem::transmute_copy(&preceipttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiptType<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, receipttype: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReceiptType(receipttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusegrouping: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupBroadcastReceipts(::core::mem::transmute_copy(&pbusegrouping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupBroadcastReceipts<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busegrouping: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGroupBroadcastReceipts(busegrouping) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&ppriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TapiConnection<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiconnection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TapiConnection(::core::mem::transmute_copy(&pptapiconnection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_TapiConnection<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptapiconnection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_TapiConnection(&*(&ptapiconnection as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(&*(&bstrfaxservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvfaxoutgoingjobids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedSubmit<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedSubmit(&*(&pfaxserver as *const <IFaxServer as ::windows::core::Abi>::Abi as *const <IFaxServer as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvfaxoutgoingjobids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachFaxToReceipt<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbattachfax: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachFaxToReceipt(::core::mem::transmute_copy(&pbattachfax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachFaxToReceipt<Impl: IFaxDocumentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, battachfax: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAttachFaxToReceipt(battachfax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxDocument>,
            ::windows::core::GetTrustLevel,
            Body::<Impl, OFFSET>,
            SetBody::<Impl, OFFSET>,
            Sender::<Impl, OFFSET>,
            Recipients::<Impl, OFFSET>,
            CoverPage::<Impl, OFFSET>,
            SetCoverPage::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            SetSubject::<Impl, OFFSET>,
            Note::<Impl, OFFSET>,
            SetNote::<Impl, OFFSET>,
            ScheduleTime::<Impl, OFFSET>,
            SetScheduleTime::<Impl, OFFSET>,
            ReceiptAddress::<Impl, OFFSET>,
            SetReceiptAddress::<Impl, OFFSET>,
            DocumentName::<Impl, OFFSET>,
            SetDocumentName::<Impl, OFFSET>,
            CallHandle::<Impl, OFFSET>,
            SetCallHandle::<Impl, OFFSET>,
            CoverPageType::<Impl, OFFSET>,
            SetCoverPageType::<Impl, OFFSET>,
            ScheduleType::<Impl, OFFSET>,
            SetScheduleType::<Impl, OFFSET>,
            ReceiptType::<Impl, OFFSET>,
            SetReceiptType::<Impl, OFFSET>,
            GroupBroadcastReceipts::<Impl, OFFSET>,
            SetGroupBroadcastReceipts::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            TapiConnection::<Impl, OFFSET>,
            putref_TapiConnection::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            ConnectedSubmit::<Impl, OFFSET>,
            AttachFaxToReceipt::<Impl, OFFSET>,
            SetAttachFaxToReceipt::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxDocument2Impl: Sized + IFaxDocumentImpl + IDispatchImpl {
    fn SubmissionId();
    fn Bodies();
    fn SetBodies();
    fn Submit2();
    fn ConnectedSubmit2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxDocument2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxDocument2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxDocument2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxDocument2Impl, const OFFSET: isize>() -> IFaxDocument2Vtbl {
        unsafe extern "system" fn SubmissionId<Impl: IFaxDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId(::core::mem::transmute_copy(&pbstrsubmissionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bodies<Impl: IFaxDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbodies: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bodies(::core::mem::transmute_copy(&pvbodies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBodies<Impl: IFaxDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vbodies: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBodies(&*(&vbodies as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit2<Impl: IFaxDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit2(&*(&bstrfaxservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedSubmit2<Impl: IFaxDocument2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, pvfaxoutgoingjobids: *mut super::super::System::Com::VARIANT, plerrorbodyfile: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedSubmit2(&*(&pfaxserver as *const <IFaxServer as ::windows::core::Abi>::Abi as *const <IFaxServer as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvfaxoutgoingjobids), ::core::mem::transmute_copy(&plerrorbodyfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxDocument2>, ::windows::core::GetTrustLevel, SubmissionId::<Impl, OFFSET>, Bodies::<Impl, OFFSET>, SetBodies::<Impl, OFFSET>, Submit2::<Impl, OFFSET>, ConnectedSubmit2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxEventLoggingImpl: Sized + IDispatchImpl {
    fn InitEventsLevel();
    fn SetInitEventsLevel();
    fn InboundEventsLevel();
    fn SetInboundEventsLevel();
    fn OutboundEventsLevel();
    fn SetOutboundEventsLevel();
    fn GeneralEventsLevel();
    fn SetGeneralEventsLevel();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxEventLogging {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxEventLogging";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxEventLoggingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxEventLoggingImpl, const OFFSET: isize>() -> IFaxEventLoggingVtbl {
        unsafe extern "system" fn InitEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piniteventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitEventsLevel(::core::mem::transmute_copy(&piniteventlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInitEventsLevel(initeventlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundEventsLevel(::core::mem::transmute_copy(&pinboundeventlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInboundEventsLevel(inboundeventlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poutboundeventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundEventsLevel(::core::mem::transmute_copy(&poutboundeventlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, outboundeventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutboundEventsLevel(outboundeventlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GeneralEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgeneraleventlevel: *mut FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneralEventsLevel(::core::mem::transmute_copy(&pgeneraleventlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneralEventsLevel<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generaleventlevel: FAX_LOG_LEVEL_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGeneralEventsLevel(generaleventlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxEventLoggingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxEventLogging>,
            ::windows::core::GetTrustLevel,
            InitEventsLevel::<Impl, OFFSET>,
            SetInitEventsLevel::<Impl, OFFSET>,
            InboundEventsLevel::<Impl, OFFSET>,
            SetInboundEventsLevel::<Impl, OFFSET>,
            OutboundEventsLevel::<Impl, OFFSET>,
            SetOutboundEventsLevel::<Impl, OFFSET>,
            GeneralEventsLevel::<Impl, OFFSET>,
            SetGeneralEventsLevel::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxFoldersImpl: Sized + IDispatchImpl {
    fn OutgoingQueue();
    fn IncomingQueue();
    fn IncomingArchive();
    fn OutgoingArchive();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxFolders {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxFolders";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxFoldersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxFoldersImpl, const OFFSET: isize>() -> IFaxFoldersVtbl {
        unsafe extern "system" fn OutgoingQueue<Impl: IFaxFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingQueue(::core::mem::transmute_copy(&pfaxoutgoingqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingQueue<Impl: IFaxFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingQueue(::core::mem::transmute_copy(&pfaxincomingqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncomingArchive<Impl: IFaxFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingArchive(::core::mem::transmute_copy(&pfaxincomingarchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutgoingArchive<Impl: IFaxFoldersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingarchive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingArchive(::core::mem::transmute_copy(&pfaxoutgoingarchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxFolders>, ::windows::core::GetTrustLevel, OutgoingQueue::<Impl, OFFSET>, IncomingQueue::<Impl, OFFSET>, IncomingArchive::<Impl, OFFSET>, OutgoingArchive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingImpl: Sized + IDispatchImpl {
    fn GetExtensions();
    fn GetMethods();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxInboundRouting {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxInboundRouting";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingImpl, const OFFSET: isize>() -> IFaxInboundRoutingVtbl {
        unsafe extern "system" fn GetExtensions<Impl: IFaxInboundRoutingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingextensions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensions(::core::mem::transmute_copy(&pfaxinboundroutingextensions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethods<Impl: IFaxInboundRoutingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxinboundroutingmethods: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethods(::core::mem::transmute_copy(&pfaxinboundroutingmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxInboundRouting>, ::windows::core::GetTrustLevel, GetExtensions::<Impl, OFFSET>, GetMethods::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtensionImpl: Sized + IDispatchImpl {
    fn FriendlyName();
    fn ImageName();
    fn UniqueName();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Status();
    fn InitErrorCode();
    fn Methods();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxInboundRoutingExtension {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxInboundRoutingExtension";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>() -> IFaxInboundRoutingExtensionVtbl {
        unsafe extern "system" fn FriendlyName<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName(::core::mem::transmute_copy(&pbstrfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName(::core::mem::transmute_copy(&pbstrimagename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UniqueName<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruniquename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UniqueName(::core::mem::transmute_copy(&pbstruniquename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion(::core::mem::transmute_copy(&plmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion(::core::mem::transmute_copy(&plminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild(::core::mem::transmute_copy(&plmajorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild(::core::mem::transmute_copy(&plminorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug(::core::mem::transmute_copy(&pbdebug)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_PROVIDER_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitErrorCode<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pliniterrorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitErrorCode(::core::mem::transmute_copy(&pliniterrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods<Impl: IFaxInboundRoutingExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvmethods: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Methods(::core::mem::transmute_copy(&pvmethods)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxInboundRoutingExtension>,
            ::windows::core::GetTrustLevel,
            FriendlyName::<Impl, OFFSET>,
            ImageName::<Impl, OFFSET>,
            UniqueName::<Impl, OFFSET>,
            MajorVersion::<Impl, OFFSET>,
            MinorVersion::<Impl, OFFSET>,
            MajorBuild::<Impl, OFFSET>,
            MinorBuild::<Impl, OFFSET>,
            Debug::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            InitErrorCode::<Impl, OFFSET>,
            Methods::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxInboundRoutingExtensions {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxInboundRoutingExtensions";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingExtensionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingExtensionsImpl, const OFFSET: isize>() -> IFaxInboundRoutingExtensionsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxInboundRoutingExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxInboundRoutingExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingextension: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxinboundroutingextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxInboundRoutingExtensionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxInboundRoutingExtensions>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethodImpl: Sized + IDispatchImpl {
    fn Name();
    fn GUID();
    fn FunctionName();
    fn ExtensionFriendlyName();
    fn ExtensionImageName();
    fn Priority();
    fn SetPriority();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxInboundRoutingMethod {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxInboundRoutingMethod";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethodVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>() -> IFaxInboundRoutingMethodVtbl {
        unsafe extern "system" fn Name<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GUID<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GUID(::core::mem::transmute_copy(&pbstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FunctionName<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfunctionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FunctionName(::core::mem::transmute_copy(&pbstrfunctionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionFriendlyName<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionFriendlyName(::core::mem::transmute_copy(&pbstrextensionfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtensionImageName<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextensionimagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionImageName(::core::mem::transmute_copy(&pbstrextensionimagename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&plpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(lpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxInboundRoutingMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxInboundRoutingMethod>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            GUID::<Impl, OFFSET>,
            FunctionName::<Impl, OFFSET>,
            ExtensionFriendlyName::<Impl, OFFSET>,
            ExtensionImageName::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxInboundRoutingMethodsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxInboundRoutingMethods {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxInboundRoutingMethods";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxInboundRoutingMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxInboundRoutingMethodsImpl, const OFFSET: isize>() -> IFaxInboundRoutingMethodsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxInboundRoutingMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxInboundRoutingMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxinboundroutingmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxinboundroutingmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxInboundRoutingMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxInboundRoutingMethods>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingArchiveImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveFolder();
    fn SetArchiveFolder();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn AgeLimit();
    fn SetAgeLimit();
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn Save();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingArchive {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingArchive";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingArchiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>() -> IFaxIncomingArchiveVtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive(::core::mem::transmute_copy(&pbusearchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseArchive(busearchive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveFolder<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveFolder(::core::mem::transmute_copy(&pbstrarchivefolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetArchiveFolder(&*(&bstrarchivefolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning(::core::mem::transmute_copy(&pbsizequotawarning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSizeQuotaWarning(bsizequotawarning) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark(::core::mem::transmute_copy(&plhighquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHighQuotaWaterMark(lhighquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark(::core::mem::transmute_copy(&pllowquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLowQuotaWaterMark(llowquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit(::core::mem::transmute_copy(&plagelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAgeLimit(lagelimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeLow<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow(::core::mem::transmute_copy(&plsizelow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh(::core::mem::transmute_copy(&plsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxincomingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(lprefetchsize, ::core::mem::transmute_copy(&pfaxincomingmessageiterator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxIncomingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(&*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxincomingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxIncomingArchive>,
            ::windows::core::GetTrustLevel,
            UseArchive::<Impl, OFFSET>,
            SetUseArchive::<Impl, OFFSET>,
            ArchiveFolder::<Impl, OFFSET>,
            SetArchiveFolder::<Impl, OFFSET>,
            SizeQuotaWarning::<Impl, OFFSET>,
            SetSizeQuotaWarning::<Impl, OFFSET>,
            HighQuotaWaterMark::<Impl, OFFSET>,
            SetHighQuotaWaterMark::<Impl, OFFSET>,
            LowQuotaWaterMark::<Impl, OFFSET>,
            SetLowQuotaWaterMark::<Impl, OFFSET>,
            AgeLimit::<Impl, OFFSET>,
            SetAgeLimit::<Impl, OFFSET>,
            SizeLow::<Impl, OFFSET>,
            SizeHigh::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            GetMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJobImpl: Sized + IDispatchImpl {
    fn Size();
    fn Id();
    fn CurrentPage();
    fn DeviceId();
    fn Status();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CallerId();
    fn RoutingInformation();
    fn JobType();
    fn Cancel();
    fn Refresh();
    fn CopyTiff();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingJob {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingJob";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingJobImpl, const OFFSET: isize>() -> IFaxIncomingJobVtbl {
        unsafe extern "system" fn Size<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage(::core::mem::transmute_copy(&plcurrentpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId(::core::mem::transmute_copy(&pldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode(::core::mem::transmute_copy(&pextendedstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus(::core::mem::transmute_copy(&pbstrextendedstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations(::core::mem::transmute_copy(&pavailableoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart(::core::mem::transmute_copy(&pdatetransmissionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd(::core::mem::transmute_copy(&pdatetransmissionend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId(::core::mem::transmute_copy(&pbstrcallerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation(::core::mem::transmute_copy(&pbstrroutinginformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobType(::core::mem::transmute_copy(&pjobtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxIncomingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTiff(&*(&bstrtiffpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxIncomingJob>,
            ::windows::core::GetTrustLevel,
            Size::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            CurrentPage::<Impl, OFFSET>,
            DeviceId::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            ExtendedStatusCode::<Impl, OFFSET>,
            ExtendedStatus::<Impl, OFFSET>,
            AvailableOperations::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            TransmissionStart::<Impl, OFFSET>,
            TransmissionEnd::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            CallerId::<Impl, OFFSET>,
            RoutingInformation::<Impl, OFFSET>,
            JobType::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            CopyTiff::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingJobsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingJobs {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingJobs";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingJobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingJobsImpl, const OFFSET: isize>() -> IFaxIncomingJobsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxIncomingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxIncomingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxincomingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxIncomingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxIncomingJobs>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessageImpl: Sized + IDispatchImpl {
    fn Id();
    fn Pages();
    fn Size();
    fn DeviceName();
    fn Retries();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CallerId();
    fn RoutingInformation();
    fn CopyTiff();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingMessage {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingMessage";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessageImpl, const OFFSET: isize>() -> IFaxIncomingMessageVtbl {
        unsafe extern "system" fn Id<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages(::core::mem::transmute_copy(&plpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart(::core::mem::transmute_copy(&pdatetransmissionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd(::core::mem::transmute_copy(&pdatetransmissionend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId(::core::mem::transmute_copy(&pbstrcallerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation(::core::mem::transmute_copy(&pbstrroutinginformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTiff(&*(&bstrtiffpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFaxIncomingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxIncomingMessage>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Pages::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            DeviceName::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            TransmissionStart::<Impl, OFFSET>,
            TransmissionEnd::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            CallerId::<Impl, OFFSET>,
            RoutingInformation::<Impl, OFFSET>,
            CopyTiff::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessage2Impl: Sized + IFaxIncomingMessageImpl + IDispatchImpl {
    fn Subject();
    fn SetSubject();
    fn SenderName();
    fn SetSenderName();
    fn SenderFaxNumber();
    fn SetSenderFaxNumber();
    fn HasCoverPage();
    fn SetHasCoverPage();
    fn Recipients();
    fn SetRecipients();
    fn WasReAssigned();
    fn Read();
    fn SetRead();
    fn ReAssign();
    fn Save();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingMessage2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingMessage2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>() -> IFaxIncomingMessage2Vtbl {
        unsafe extern "system" fn Subject<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject(::core::mem::transmute_copy(&pbstrsubject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSubject(&*(&bstrsubject as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderName<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsendername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderName(::core::mem::transmute_copy(&pbstrsendername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderName<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsendername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderName(&*(&bstrsendername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderFaxNumber<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsenderfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderFaxNumber(::core::mem::transmute_copy(&pbstrsenderfaxnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderFaxNumber<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsenderfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderFaxNumber(&*(&bstrsenderfaxnumber as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCoverPage<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage(::core::mem::transmute_copy(&pbhascoverpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHasCoverPage<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhascoverpage: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHasCoverPage(bhascoverpage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrecipients: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients(::core::mem::transmute_copy(&pbstrrecipients)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecipients<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrecipients: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecipients(&*(&bstrrecipients as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WasReAssigned<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbwasreassigned: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasReAssigned(::core::mem::transmute_copy(&pbwasreassigned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRead(bread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReAssign<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReAssign() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxIncomingMessage2>,
            ::windows::core::GetTrustLevel,
            Subject::<Impl, OFFSET>,
            SetSubject::<Impl, OFFSET>,
            SenderName::<Impl, OFFSET>,
            SetSenderName::<Impl, OFFSET>,
            SenderFaxNumber::<Impl, OFFSET>,
            SetSenderFaxNumber::<Impl, OFFSET>,
            HasCoverPage::<Impl, OFFSET>,
            SetHasCoverPage::<Impl, OFFSET>,
            Recipients::<Impl, OFFSET>,
            SetRecipients::<Impl, OFFSET>,
            WasReAssigned::<Impl, OFFSET>,
            Read::<Impl, OFFSET>,
            SetRead::<Impl, OFFSET>,
            ReAssign::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingMessageIteratorImpl: Sized + IDispatchImpl {
    fn Message();
    fn PrefetchSize();
    fn SetPrefetchSize();
    fn AtEOF();
    fn MoveFirst();
    fn MoveNext();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingMessageIterator {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingMessageIterator";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingMessageIteratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>() -> IFaxIncomingMessageIteratorVtbl {
        unsafe extern "system" fn Message<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&pfaxincomingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchSize(::core::mem::transmute_copy(&plprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrefetchSize(lprefetchsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEOF<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AtEOF(::core::mem::transmute_copy(&pbeof)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveFirst<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IFaxIncomingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxIncomingMessageIterator>, ::windows::core::GetTrustLevel, Message::<Impl, OFFSET>, PrefetchSize::<Impl, OFFSET>, SetPrefetchSize::<Impl, OFFSET>, AtEOF::<Impl, OFFSET>, MoveFirst::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxIncomingQueueImpl: Sized + IDispatchImpl {
    fn Blocked();
    fn SetBlocked();
    fn Refresh();
    fn Save();
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxIncomingQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxIncomingQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxIncomingQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxIncomingQueueImpl, const OFFSET: isize>() -> IFaxIncomingQueueVtbl {
        unsafe extern "system" fn Blocked<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Blocked(::core::mem::transmute_copy(&pbblocked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlocked(bblocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobs<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxincomingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs(::core::mem::transmute_copy(&pfaxincomingjobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxIncomingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxincomingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxincomingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxIncomingQueue>, ::windows::core::GetTrustLevel, Blocked::<Impl, OFFSET>, SetBlocked::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Save::<Impl, OFFSET>, GetJobs::<Impl, OFFSET>, GetJob::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxJobStatusImpl: Sized + IDispatchImpl {
    fn Status();
    fn Pages();
    fn Size();
    fn CurrentPage();
    fn DeviceId();
    fn CSID();
    fn TSID();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn JobType();
    fn ScheduledTime();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CallerId();
    fn RoutingInformation();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxJobStatus {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxJobStatus";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxJobStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxJobStatusImpl, const OFFSET: isize>() -> IFaxJobStatusVtbl {
        unsafe extern "system" fn Status<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages(::core::mem::transmute_copy(&plpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage(::core::mem::transmute_copy(&plcurrentpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId(::core::mem::transmute_copy(&pldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode(::core::mem::transmute_copy(&pextendedstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus(::core::mem::transmute_copy(&pbstrextendedstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations(::core::mem::transmute_copy(&pavailableoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JobType<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjobtype: *mut FAX_JOB_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JobType(::core::mem::transmute_copy(&pjobtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledTime(::core::mem::transmute_copy(&pdatescheduledtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart(::core::mem::transmute_copy(&pdatetransmissionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd(::core::mem::transmute_copy(&pdatetransmissionend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerId<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcallerid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerId(::core::mem::transmute_copy(&pbstrcallerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingInformation<Impl: IFaxJobStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrroutinginformation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingInformation(::core::mem::transmute_copy(&pbstrroutinginformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxJobStatus>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, OFFSET>,
            Pages::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            CurrentPage::<Impl, OFFSET>,
            DeviceId::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            ExtendedStatusCode::<Impl, OFFSET>,
            ExtendedStatus::<Impl, OFFSET>,
            AvailableOperations::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            JobType::<Impl, OFFSET>,
            ScheduledTime::<Impl, OFFSET>,
            TransmissionStart::<Impl, OFFSET>,
            TransmissionEnd::<Impl, OFFSET>,
            CallerId::<Impl, OFFSET>,
            RoutingInformation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxLoggingOptionsImpl: Sized + IDispatchImpl {
    fn EventLogging();
    fn ActivityLogging();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxLoggingOptions {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxLoggingOptions";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxLoggingOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxLoggingOptionsImpl, const OFFSET: isize>() -> IFaxLoggingOptionsVtbl {
        unsafe extern "system" fn EventLogging<Impl: IFaxLoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxeventlogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventLogging(::core::mem::transmute_copy(&pfaxeventlogging)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityLogging<Impl: IFaxLoggingOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxactivitylogging: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityLogging(::core::mem::transmute_copy(&pfaxactivitylogging)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxLoggingOptions>, ::windows::core::GetTrustLevel, EventLogging::<Impl, OFFSET>, ActivityLogging::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingImpl: Sized + IDispatchImpl {
    fn GetGroups();
    fn GetRules();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutboundRouting {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutboundRouting";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingImpl, const OFFSET: isize>() -> IFaxOutboundRoutingVtbl {
        unsafe extern "system" fn GetGroups<Impl: IFaxOutboundRoutingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutinggroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroups(::core::mem::transmute_copy(&pfaxoutboundroutinggroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRules<Impl: IFaxOutboundRoutingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutboundroutingrules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRules(::core::mem::transmute_copy(&pfaxoutboundroutingrules)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutboundRouting>, ::windows::core::GetTrustLevel, GetGroups::<Impl, OFFSET>, GetRules::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn Status();
    fn DeviceIds();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingGroup {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutboundRoutingGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingGroupImpl, const OFFSET: isize>() -> IFaxOutboundRoutingGroupVtbl {
        unsafe extern "system" fn Name<Impl: IFaxOutboundRoutingGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutboundRoutingGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_GROUP_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceIds<Impl: IFaxOutboundRoutingGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxdeviceids: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceIds(::core::mem::transmute_copy(&pfaxdeviceids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutboundRoutingGroup>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Status::<Impl, OFFSET>, DeviceIds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingGroupsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingGroups {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutboundRoutingGroups";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>() -> IFaxOutboundRoutingGroupsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutboundroutinggroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutboundroutinggroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutboundroutinggroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxOutboundRoutingGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutboundRoutingGroups>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRuleImpl: Sized + IDispatchImpl {
    fn CountryCode();
    fn AreaCode();
    fn Status();
    fn UseDevice();
    fn SetUseDevice();
    fn DeviceId();
    fn SetDeviceId();
    fn GroupName();
    fn SetGroupName();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingRule {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutboundRoutingRule";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>() -> IFaxOutboundRoutingRuleVtbl {
        unsafe extern "system" fn CountryCode<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryCode(::core::mem::transmute_copy(&plcountrycode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreaCode<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plareacode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreaCode(::core::mem::transmute_copy(&plareacode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_RULE_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDevice<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevice: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDevice(::core::mem::transmute_copy(&pbusedevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDevice<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevice: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseDevice(busedevice) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId(::core::mem::transmute_copy(&pldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceId(deviceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupName<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupName(::core::mem::transmute_copy(&pbstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupName<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGroupName(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxOutboundRoutingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxOutboundRoutingRule>,
            ::windows::core::GetTrustLevel,
            CountryCode::<Impl, OFFSET>,
            AreaCode::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            UseDevice::<Impl, OFFSET>,
            SetUseDevice::<Impl, OFFSET>,
            DeviceId::<Impl, OFFSET>,
            SetDeviceId::<Impl, OFFSET>,
            GroupName::<Impl, OFFSET>,
            SetGroupName::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutboundRoutingRulesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByCountryAndArea();
    fn RemoveByCountryAndArea();
    fn Remove();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutboundRoutingRules {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutboundRoutingRules";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutboundRoutingRulesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>() -> IFaxOutboundRoutingRulesVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&pfaxoutboundroutingrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemByCountryAndArea<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemByCountryAndArea(lcountrycode, lareacode, ::core::mem::transmute_copy(&pfaxoutboundroutingrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveByCountryAndArea<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveByCountryAndArea(lcountrycode, lareacode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxOutboundRoutingRulesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcountrycode: i32, lareacode: i32, busedevice: i16, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ldeviceid: i32, pfaxoutboundroutingrule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(lcountrycode, lareacode, busedevice, &*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ldeviceid, ::core::mem::transmute_copy(&pfaxoutboundroutingrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutboundRoutingRules>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, ItemByCountryAndArea::<Impl, OFFSET>, RemoveByCountryAndArea::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Add::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingArchiveImpl: Sized + IDispatchImpl {
    fn UseArchive();
    fn SetUseArchive();
    fn ArchiveFolder();
    fn SetArchiveFolder();
    fn SizeQuotaWarning();
    fn SetSizeQuotaWarning();
    fn HighQuotaWaterMark();
    fn SetHighQuotaWaterMark();
    fn LowQuotaWaterMark();
    fn SetLowQuotaWaterMark();
    fn AgeLimit();
    fn SetAgeLimit();
    fn SizeLow();
    fn SizeHigh();
    fn Refresh();
    fn Save();
    fn GetMessages();
    fn GetMessage();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingArchive {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingArchive";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingArchiveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>() -> IFaxOutgoingArchiveVtbl {
        unsafe extern "system" fn UseArchive<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusearchive: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseArchive(::core::mem::transmute_copy(&pbusearchive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseArchive<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busearchive: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseArchive(busearchive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchiveFolder<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrarchivefolder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArchiveFolder(::core::mem::transmute_copy(&pbstrarchivefolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchiveFolder<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrarchivefolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetArchiveFolder(&*(&bstrarchivefolder as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeQuotaWarning<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsizequotawarning: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeQuotaWarning(::core::mem::transmute_copy(&pbsizequotawarning)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSizeQuotaWarning<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsizequotawarning: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSizeQuotaWarning(bsizequotawarning) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighQuotaWaterMark<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhighquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighQuotaWaterMark(::core::mem::transmute_copy(&plhighquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighQuotaWaterMark<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhighquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHighQuotaWaterMark(lhighquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LowQuotaWaterMark<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllowquotawatermark: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LowQuotaWaterMark(::core::mem::transmute_copy(&pllowquotawatermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowQuotaWaterMark<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowquotawatermark: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLowQuotaWaterMark(llowquotawatermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit(::core::mem::transmute_copy(&plagelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAgeLimit(lagelimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeLow<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizelow: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeLow(::core::mem::transmute_copy(&plsizelow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SizeHigh<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsizehigh: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeHigh(::core::mem::transmute_copy(&plsizehigh)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessages<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32, pfaxoutgoingmessageiterator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessages(lprefetchsize, ::core::mem::transmute_copy(&pfaxoutgoingmessageiterator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessage<Impl: IFaxOutgoingArchiveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(&*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutgoingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxOutgoingArchive>,
            ::windows::core::GetTrustLevel,
            UseArchive::<Impl, OFFSET>,
            SetUseArchive::<Impl, OFFSET>,
            ArchiveFolder::<Impl, OFFSET>,
            SetArchiveFolder::<Impl, OFFSET>,
            SizeQuotaWarning::<Impl, OFFSET>,
            SetSizeQuotaWarning::<Impl, OFFSET>,
            HighQuotaWaterMark::<Impl, OFFSET>,
            SetHighQuotaWaterMark::<Impl, OFFSET>,
            LowQuotaWaterMark::<Impl, OFFSET>,
            SetLowQuotaWaterMark::<Impl, OFFSET>,
            AgeLimit::<Impl, OFFSET>,
            SetAgeLimit::<Impl, OFFSET>,
            SizeLow::<Impl, OFFSET>,
            SizeHigh::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            GetMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJobImpl: Sized + IDispatchImpl {
    fn Subject();
    fn DocumentName();
    fn Pages();
    fn Size();
    fn SubmissionId();
    fn Id();
    fn OriginalScheduledTime();
    fn SubmissionTime();
    fn ReceiptType();
    fn Priority();
    fn Sender();
    fn Recipient();
    fn CurrentPage();
    fn DeviceId();
    fn Status();
    fn ExtendedStatusCode();
    fn ExtendedStatus();
    fn AvailableOperations();
    fn Retries();
    fn ScheduledTime();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn GroupBroadcastReceipts();
    fn Pause();
    fn Resume();
    fn Restart();
    fn CopyTiff();
    fn Refresh();
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingJob {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingJob";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJobImpl, const OFFSET: isize>() -> IFaxOutgoingJobVtbl {
        unsafe extern "system" fn Subject<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject(::core::mem::transmute_copy(&pbstrsubject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName(::core::mem::transmute_copy(&pbstrdocumentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages(::core::mem::transmute_copy(&plpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionId<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId(::core::mem::transmute_copy(&pbstrsubmissionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalScheduledTime(::core::mem::transmute_copy(&pdateoriginalscheduledtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionTime(::core::mem::transmute_copy(&pdatesubmissiontime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType(::core::mem::transmute_copy(&preceipttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&ppriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender(::core::mem::transmute_copy(&ppfaxsender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient(::core::mem::transmute_copy(&ppfaxrecipient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPage<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcurrentpage: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPage(::core::mem::transmute_copy(&plcurrentpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldeviceid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId(::core::mem::transmute_copy(&pldeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut FAX_JOB_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatusCode<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pextendedstatuscode: *mut FAX_JOB_EXTENDED_STATUS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatusCode(::core::mem::transmute_copy(&pextendedstatuscode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextendedstatus: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus(::core::mem::transmute_copy(&pbstrextendedstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableOperations<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pavailableoperations: *mut FAX_JOB_OPERATIONS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableOperations(::core::mem::transmute_copy(&pavailableoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledTime<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatescheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledTime(::core::mem::transmute_copy(&pdatescheduledtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart(::core::mem::transmute_copy(&pdatetransmissionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd(::core::mem::transmute_copy(&pdatetransmissionend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupBroadcastReceipts<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbgroupbroadcastreceipts: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GroupBroadcastReceipts(::core::mem::transmute_copy(&pbgroupbroadcastreceipts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restart<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Restart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTiff(&*(&bstrtiffpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IFaxOutgoingJobImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxOutgoingJob>,
            ::windows::core::GetTrustLevel,
            Subject::<Impl, OFFSET>,
            DocumentName::<Impl, OFFSET>,
            Pages::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            SubmissionId::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            OriginalScheduledTime::<Impl, OFFSET>,
            SubmissionTime::<Impl, OFFSET>,
            ReceiptType::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            Sender::<Impl, OFFSET>,
            Recipient::<Impl, OFFSET>,
            CurrentPage::<Impl, OFFSET>,
            DeviceId::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            ExtendedStatusCode::<Impl, OFFSET>,
            ExtendedStatus::<Impl, OFFSET>,
            AvailableOperations::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            ScheduledTime::<Impl, OFFSET>,
            TransmissionStart::<Impl, OFFSET>,
            TransmissionEnd::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            GroupBroadcastReceipts::<Impl, OFFSET>,
            Pause::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Restart::<Impl, OFFSET>,
            CopyTiff::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJob2Impl: Sized + IFaxOutgoingJobImpl + IDispatchImpl {
    fn HasCoverPage();
    fn ReceiptAddress();
    fn ScheduleType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingJob2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingJob2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJob2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJob2Impl, const OFFSET: isize>() -> IFaxOutgoingJob2Vtbl {
        unsafe extern "system" fn HasCoverPage<Impl: IFaxOutgoingJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage(::core::mem::transmute_copy(&pbhascoverpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxOutgoingJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress(::core::mem::transmute_copy(&pbstrreceiptaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleType<Impl: IFaxOutgoingJob2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscheduletype: *mut FAX_SCHEDULE_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduleType(::core::mem::transmute_copy(&pscheduletype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutgoingJob2>, ::windows::core::GetTrustLevel, HasCoverPage::<Impl, OFFSET>, ReceiptAddress::<Impl, OFFSET>, ScheduleType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingJobsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingJobs {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingJobs";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingJobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingJobsImpl, const OFFSET: isize>() -> IFaxOutgoingJobsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxOutgoingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxOutgoingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vindex: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&vindex as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutgoingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxOutgoingJobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutgoingJobs>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessageImpl: Sized + IDispatchImpl {
    fn SubmissionId();
    fn Id();
    fn Subject();
    fn DocumentName();
    fn Retries();
    fn Pages();
    fn Size();
    fn OriginalScheduledTime();
    fn SubmissionTime();
    fn Priority();
    fn Sender();
    fn Recipient();
    fn DeviceName();
    fn TransmissionStart();
    fn TransmissionEnd();
    fn CSID();
    fn TSID();
    fn CopyTiff();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingMessage {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingMessage";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>() -> IFaxOutgoingMessageVtbl {
        unsafe extern "system" fn SubmissionId<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubmissionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionId(::core::mem::transmute_copy(&pbstrsubmissionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pbstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject(::core::mem::transmute_copy(&pbstrsubject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentName<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdocumentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentName(::core::mem::transmute_copy(&pbstrdocumentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pages<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpages: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pages(::core::mem::transmute_copy(&plpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size(::core::mem::transmute_copy(&plsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OriginalScheduledTime<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdateoriginalscheduledtime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalScheduledTime(::core::mem::transmute_copy(&pdateoriginalscheduledtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubmissionTime<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesubmissiontime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubmissionTime(::core::mem::transmute_copy(&pdatesubmissiontime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut FAX_PRIORITY_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&ppriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sender<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsender: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender(::core::mem::transmute_copy(&ppfaxsender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipient<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipient(::core::mem::transmute_copy(&ppfaxrecipient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceName(::core::mem::transmute_copy(&pbstrdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStart<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStart(::core::mem::transmute_copy(&pdatetransmissionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionEnd<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatetransmissionend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionEnd(::core::mem::transmute_copy(&pdatetransmissionend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSID<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CSID(::core::mem::transmute_copy(&pbstrcsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTiff<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtiffpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTiff(&*(&bstrtiffpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFaxOutgoingMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxOutgoingMessage>,
            ::windows::core::GetTrustLevel,
            SubmissionId::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            DocumentName::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            Pages::<Impl, OFFSET>,
            Size::<Impl, OFFSET>,
            OriginalScheduledTime::<Impl, OFFSET>,
            SubmissionTime::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            Sender::<Impl, OFFSET>,
            Recipient::<Impl, OFFSET>,
            DeviceName::<Impl, OFFSET>,
            TransmissionStart::<Impl, OFFSET>,
            TransmissionEnd::<Impl, OFFSET>,
            CSID::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            CopyTiff::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessage2Impl: Sized + IFaxOutgoingMessageImpl + IDispatchImpl {
    fn HasCoverPage();
    fn ReceiptType();
    fn ReceiptAddress();
    fn Read();
    fn SetRead();
    fn Save();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingMessage2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingMessage2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>() -> IFaxOutgoingMessage2Vtbl {
        unsafe extern "system" fn HasCoverPage<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbhascoverpage: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCoverPage(::core::mem::transmute_copy(&pbhascoverpage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptType<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preceipttype: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptType(::core::mem::transmute_copy(&preceipttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptAddress<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrreceiptaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptAddress(::core::mem::transmute_copy(&pbstrreceiptaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbread: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRead<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bread: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRead(bread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutgoingMessage2>, ::windows::core::GetTrustLevel, HasCoverPage::<Impl, OFFSET>, ReceiptType::<Impl, OFFSET>, ReceiptAddress::<Impl, OFFSET>, Read::<Impl, OFFSET>, SetRead::<Impl, OFFSET>, Save::<Impl, OFFSET>, Refresh::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingMessageIteratorImpl: Sized + IDispatchImpl {
    fn Message();
    fn AtEOF();
    fn PrefetchSize();
    fn SetPrefetchSize();
    fn MoveFirst();
    fn MoveNext();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingMessageIterator {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingMessageIterator";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingMessageIteratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>() -> IFaxOutgoingMessageIteratorVtbl {
        unsafe extern "system" fn Message<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&pfaxoutgoingmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEOF<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbeof: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AtEOF(::core::mem::transmute_copy(&pbeof)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchSize<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprefetchsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchSize(::core::mem::transmute_copy(&plprefetchsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefetchSize<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprefetchsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrefetchSize(lprefetchsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveFirst<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveFirst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<Impl: IFaxOutgoingMessageIteratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxOutgoingMessageIterator>, ::windows::core::GetTrustLevel, Message::<Impl, OFFSET>, AtEOF::<Impl, OFFSET>, PrefetchSize::<Impl, OFFSET>, SetPrefetchSize::<Impl, OFFSET>, MoveFirst::<Impl, OFFSET>, MoveNext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxOutgoingQueueImpl: Sized + IDispatchImpl {
    fn Blocked();
    fn SetBlocked();
    fn Paused();
    fn SetPaused();
    fn AllowPersonalCoverPages();
    fn SetAllowPersonalCoverPages();
    fn UseDeviceTSID();
    fn SetUseDeviceTSID();
    fn Retries();
    fn SetRetries();
    fn RetryDelay();
    fn SetRetryDelay();
    fn DiscountRateStart();
    fn SetDiscountRateStart();
    fn DiscountRateEnd();
    fn SetDiscountRateEnd();
    fn AgeLimit();
    fn SetAgeLimit();
    fn Branding();
    fn SetBranding();
    fn Refresh();
    fn Save();
    fn GetJobs();
    fn GetJob();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxOutgoingQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxOutgoingQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxOutgoingQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>() -> IFaxOutgoingQueueVtbl {
        unsafe extern "system" fn Blocked<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbblocked: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Blocked(::core::mem::transmute_copy(&pbblocked)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlocked<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBlocked(bblocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paused<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paused(::core::mem::transmute_copy(&pbpaused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPaused<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpaused: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPaused(bpaused) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowPersonalCoverPages<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballowpersonalcoverpages: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowPersonalCoverPages(::core::mem::transmute_copy(&pballowpersonalcoverpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowPersonalCoverPages<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballowpersonalcoverpages: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowPersonalCoverPages(ballowpersonalcoverpages) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDeviceTSID<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbusedevicetsid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseDeviceTSID(::core::mem::transmute_copy(&pbusedevicetsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseDeviceTSID<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, busedevicetsid: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseDeviceTSID(busedevicetsid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Retries<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretries: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Retries(::core::mem::transmute_copy(&plretries)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetries<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretries: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetries(lretries) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetryDelay<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plretrydelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryDelay(::core::mem::transmute_copy(&plretrydelay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetryDelay<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lretrydelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetryDelay(lretrydelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscountRateStart<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountratestart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateStart(::core::mem::transmute_copy(&pdatediscountratestart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateStart<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountratestart: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDiscountRateStart(datediscountratestart) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscountRateEnd<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatediscountrateend: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscountRateEnd(::core::mem::transmute_copy(&pdatediscountrateend)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscountRateEnd<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datediscountrateend: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDiscountRateEnd(datediscountrateend) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgeLimit<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plagelimit: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeLimit(::core::mem::transmute_copy(&plagelimit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAgeLimit<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lagelimit: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAgeLimit(lagelimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Branding<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbbranding: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Branding(::core::mem::transmute_copy(&pbbranding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBranding<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bbranding: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBranding(bbranding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJobs<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxoutgoingjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJobs(::core::mem::transmute_copy(&pfaxoutgoingjobs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJob<Impl: IFaxOutgoingQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfaxoutgoingjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJob(&*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfaxoutgoingjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxOutgoingQueue>,
            ::windows::core::GetTrustLevel,
            Blocked::<Impl, OFFSET>,
            SetBlocked::<Impl, OFFSET>,
            Paused::<Impl, OFFSET>,
            SetPaused::<Impl, OFFSET>,
            AllowPersonalCoverPages::<Impl, OFFSET>,
            SetAllowPersonalCoverPages::<Impl, OFFSET>,
            UseDeviceTSID::<Impl, OFFSET>,
            SetUseDeviceTSID::<Impl, OFFSET>,
            Retries::<Impl, OFFSET>,
            SetRetries::<Impl, OFFSET>,
            RetryDelay::<Impl, OFFSET>,
            SetRetryDelay::<Impl, OFFSET>,
            DiscountRateStart::<Impl, OFFSET>,
            SetDiscountRateStart::<Impl, OFFSET>,
            DiscountRateEnd::<Impl, OFFSET>,
            SetDiscountRateEnd::<Impl, OFFSET>,
            AgeLimit::<Impl, OFFSET>,
            SetAgeLimit::<Impl, OFFSET>,
            Branding::<Impl, OFFSET>,
            SetBranding::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            GetJobs::<Impl, OFFSET>,
            GetJob::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxReceiptOptionsImpl: Sized + IDispatchImpl {
    fn AuthenticationType();
    fn SetAuthenticationType();
    fn SMTPServer();
    fn SetSMTPServer();
    fn SMTPPort();
    fn SetSMTPPort();
    fn SMTPSender();
    fn SetSMTPSender();
    fn SMTPUser();
    fn SetSMTPUser();
    fn AllowedReceipts();
    fn SetAllowedReceipts();
    fn SMTPPassword();
    fn SetSMTPPassword();
    fn Refresh();
    fn Save();
    fn UseForInboundRouting();
    fn SetUseForInboundRouting();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxReceiptOptions {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxReceiptOptions";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxReceiptOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>() -> IFaxReceiptOptionsVtbl {
        unsafe extern "system" fn AuthenticationType<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: FAX_SMTP_AUTHENTICATION_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SMTPServer<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPServer(::core::mem::transmute_copy(&pbstrsmtpserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPServer<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMTPServer(&*(&bstrsmtpserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SMTPPort<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsmtpport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPPort(::core::mem::transmute_copy(&plsmtpport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPort<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsmtpport: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMTPPort(lsmtpport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SMTPSender<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpsender: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPSender(::core::mem::transmute_copy(&pbstrsmtpsender)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPSender<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpsender: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMTPSender(&*(&bstrsmtpsender as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SMTPUser<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtpuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPUser(::core::mem::transmute_copy(&pbstrsmtpuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPUser<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtpuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMTPUser(&*(&bstrsmtpuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowedReceipts<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowedreceipts: *mut FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedReceipts(::core::mem::transmute_copy(&pallowedreceipts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedReceipts<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowedreceipts: FAX_RECEIPT_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowedReceipts(allowedreceipts) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SMTPPassword<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsmtppassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SMTPPassword(::core::mem::transmute_copy(&pbstrsmtppassword)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSMTPPassword<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsmtppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSMTPPassword(&*(&bstrsmtppassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseForInboundRouting<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuseforinboundrouting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseForInboundRouting(::core::mem::transmute_copy(&pbuseforinboundrouting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseForInboundRouting<Impl: IFaxReceiptOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buseforinboundrouting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseForInboundRouting(buseforinboundrouting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxReceiptOptions>,
            ::windows::core::GetTrustLevel,
            AuthenticationType::<Impl, OFFSET>,
            SetAuthenticationType::<Impl, OFFSET>,
            SMTPServer::<Impl, OFFSET>,
            SetSMTPServer::<Impl, OFFSET>,
            SMTPPort::<Impl, OFFSET>,
            SetSMTPPort::<Impl, OFFSET>,
            SMTPSender::<Impl, OFFSET>,
            SetSMTPSender::<Impl, OFFSET>,
            SMTPUser::<Impl, OFFSET>,
            SetSMTPUser::<Impl, OFFSET>,
            AllowedReceipts::<Impl, OFFSET>,
            SetAllowedReceipts::<Impl, OFFSET>,
            SMTPPassword::<Impl, OFFSET>,
            SetSMTPPassword::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Save::<Impl, OFFSET>,
            UseForInboundRouting::<Impl, OFFSET>,
            SetUseForInboundRouting::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipientImpl: Sized + IDispatchImpl {
    fn FaxNumber();
    fn SetFaxNumber();
    fn Name();
    fn SetName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxRecipient {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxRecipient";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxRecipientImpl, const OFFSET: isize>() -> IFaxRecipientVtbl {
        unsafe extern "system" fn FaxNumber<Impl: IFaxRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber(::core::mem::transmute_copy(&pbstrfaxnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IFaxRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFaxNumber(&*(&bstrfaxnumber as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFaxRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFaxRecipientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxRecipient>, ::windows::core::GetTrustLevel, FaxNumber::<Impl, OFFSET>, SetFaxNumber::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxRecipientsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxRecipients {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxRecipients";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxRecipientsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxRecipientsImpl, const OFFSET: isize>() -> IFaxRecipientsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFaxRecipientsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFaxRecipientsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&ppfaxrecipient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFaxRecipientsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFaxRecipientsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrecipientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfaxrecipient: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&bstrfaxnumber as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrrecipientname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfaxrecipient)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFaxRecipientsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxRecipients>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurityImpl: Sized + IDispatchImpl {
    fn Descriptor();
    fn SetDescriptor();
    fn GrantedRights();
    fn Refresh();
    fn Save();
    fn InformationType();
    fn SetInformationType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxSecurity {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxSecurity";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSecurityImpl, const OFFSET: isize>() -> IFaxSecurityVtbl {
        unsafe extern "system" fn Descriptor<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptor(::core::mem::transmute_copy(&pvdescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescriptor(&*(&vdescriptor as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrantedRights<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrantedRights(::core::mem::transmute_copy(&pgrantedrights)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InformationType<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationType(::core::mem::transmute_copy(&plinformationtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Impl: IFaxSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInformationType(linformationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxSecurity>, ::windows::core::GetTrustLevel, Descriptor::<Impl, OFFSET>, SetDescriptor::<Impl, OFFSET>, GrantedRights::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Save::<Impl, OFFSET>, InformationType::<Impl, OFFSET>, SetInformationType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSecurity2Impl: Sized + IDispatchImpl {
    fn Descriptor();
    fn SetDescriptor();
    fn GrantedRights();
    fn Refresh();
    fn Save();
    fn InformationType();
    fn SetInformationType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxSecurity2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxSecurity2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSecurity2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSecurity2Impl, const OFFSET: isize>() -> IFaxSecurity2Vtbl {
        unsafe extern "system" fn Descriptor<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvdescriptor: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptor(::core::mem::transmute_copy(&pvdescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescriptor<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vdescriptor: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescriptor(&*(&vdescriptor as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrantedRights<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrantedrights: *mut FAX_ACCESS_RIGHTS_ENUM_2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GrantedRights(::core::mem::transmute_copy(&pgrantedrights)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InformationType<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plinformationtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InformationType(::core::mem::transmute_copy(&plinformationtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInformationType<Impl: IFaxSecurity2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linformationtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInformationType(linformationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxSecurity2>, ::windows::core::GetTrustLevel, Descriptor::<Impl, OFFSET>, SetDescriptor::<Impl, OFFSET>, GrantedRights::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Save::<Impl, OFFSET>, InformationType::<Impl, OFFSET>, SetInformationType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxSenderImpl: Sized + IDispatchImpl {
    fn BillingCode();
    fn SetBillingCode();
    fn City();
    fn SetCity();
    fn Company();
    fn SetCompany();
    fn Country();
    fn SetCountry();
    fn Department();
    fn SetDepartment();
    fn Email();
    fn SetEmail();
    fn FaxNumber();
    fn SetFaxNumber();
    fn HomePhone();
    fn SetHomePhone();
    fn Name();
    fn SetName();
    fn TSID();
    fn SetTSID();
    fn OfficePhone();
    fn SetOfficePhone();
    fn OfficeLocation();
    fn SetOfficeLocation();
    fn State();
    fn SetState();
    fn StreetAddress();
    fn SetStreetAddress();
    fn Title();
    fn SetTitle();
    fn ZipCode();
    fn SetZipCode();
    fn LoadDefaultSender();
    fn SaveDefaultSender();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxSender {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxSender";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxSenderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxSenderImpl, const OFFSET: isize>() -> IFaxSenderVtbl {
        unsafe extern "system" fn BillingCode<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbillingcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BillingCode(::core::mem::transmute_copy(&pbstrbillingcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBillingCode<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbillingcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBillingCode(&*(&bstrbillingcode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City(::core::mem::transmute_copy(&pbstrcity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCity<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCity(&*(&bstrcity as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Company(::core::mem::transmute_copy(&pbstrcompany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompany<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcompany: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompany(&*(&bstrcompany as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Country<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountry: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country(::core::mem::transmute_copy(&pbstrcountry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCountry<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcountry: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCountry(&*(&bstrcountry as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Department<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdepartment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Department(::core::mem::transmute_copy(&pbstrdepartment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartment<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdepartment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDepartment(&*(&bstrdepartment as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Email<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstremail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Email(::core::mem::transmute_copy(&pbstremail)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmail<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstremail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEmail(&*(&bstremail as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaxNumber<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfaxnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxNumber(::core::mem::transmute_copy(&pbstrfaxnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFaxNumber<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFaxNumber(&*(&bstrfaxnumber as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomePhone<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhomephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomePhone(::core::mem::transmute_copy(&pbstrhomephone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHomePhone<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrhomephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHomePhone(&*(&bstrhomephone as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TSID<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TSID(::core::mem::transmute_copy(&pbstrtsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTSID<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTSID(&*(&bstrtsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfficePhone<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficephone: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficePhone(::core::mem::transmute_copy(&pbstrofficephone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficePhone<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficephone: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOfficePhone(&*(&bstrofficephone as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfficeLocation<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrofficelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfficeLocation(::core::mem::transmute_copy(&pbstrofficelocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOfficeLocation<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrofficelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOfficeLocation(&*(&bstrofficelocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pbstrstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(&*(&bstrstate as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreetAddress<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstreetaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreetAddress(::core::mem::transmute_copy(&pbstrstreetaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreetAddress<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrstreetaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStreetAddress(&*(&bstrstreetaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&pbstrtitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitle(&*(&bstrtitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZipCode<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrzipcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZipCode(::core::mem::transmute_copy(&pbstrzipcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZipCode<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrzipcode: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZipCode(&*(&bstrzipcode as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadDefaultSender<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadDefaultSender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveDefaultSender<Impl: IFaxSenderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveDefaultSender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxSender>,
            ::windows::core::GetTrustLevel,
            BillingCode::<Impl, OFFSET>,
            SetBillingCode::<Impl, OFFSET>,
            City::<Impl, OFFSET>,
            SetCity::<Impl, OFFSET>,
            Company::<Impl, OFFSET>,
            SetCompany::<Impl, OFFSET>,
            Country::<Impl, OFFSET>,
            SetCountry::<Impl, OFFSET>,
            Department::<Impl, OFFSET>,
            SetDepartment::<Impl, OFFSET>,
            Email::<Impl, OFFSET>,
            SetEmail::<Impl, OFFSET>,
            FaxNumber::<Impl, OFFSET>,
            SetFaxNumber::<Impl, OFFSET>,
            HomePhone::<Impl, OFFSET>,
            SetHomePhone::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            TSID::<Impl, OFFSET>,
            SetTSID::<Impl, OFFSET>,
            OfficePhone::<Impl, OFFSET>,
            SetOfficePhone::<Impl, OFFSET>,
            OfficeLocation::<Impl, OFFSET>,
            SetOfficeLocation::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            StreetAddress::<Impl, OFFSET>,
            SetStreetAddress::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            ZipCode::<Impl, OFFSET>,
            SetZipCode::<Impl, OFFSET>,
            LoadDefaultSender::<Impl, OFFSET>,
            SaveDefaultSender::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerImpl: Sized + IDispatchImpl {
    fn Connect();
    fn ServerName();
    fn GetDeviceProviders();
    fn GetDevices();
    fn InboundRouting();
    fn Folders();
    fn LoggingOptions();
    fn MajorVersion();
    fn MinorVersion();
    fn MajorBuild();
    fn MinorBuild();
    fn Debug();
    fn Activity();
    fn OutboundRouting();
    fn ReceiptOptions();
    fn Security();
    fn Disconnect();
    fn GetExtensionProperty();
    fn SetExtensionProperty();
    fn ListenToServerEvents();
    fn RegisterDeviceProvider();
    fn UnregisterDeviceProvider();
    fn RegisterInboundRoutingExtension();
    fn UnregisterInboundRoutingExtension();
    fn RegisteredEvents();
    fn APIVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxServer {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxServer";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServerImpl, const OFFSET: isize>() -> IFaxServerVtbl {
        unsafe extern "system" fn Connect<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&bstrservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerName<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrservername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerName(::core::mem::transmute_copy(&pbstrservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceProviders<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdeviceproviders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceProviders(::core::mem::transmute_copy(&ppfaxdeviceproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevices<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevices(::core::mem::transmute_copy(&ppfaxdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundRouting<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxinboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundRouting(::core::mem::transmute_copy(&ppfaxinboundrouting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folders(::core::mem::transmute_copy(&pfaxfolders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingOptions<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxloggingoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoggingOptions(::core::mem::transmute_copy(&ppfaxloggingoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion(::core::mem::transmute_copy(&plmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion(::core::mem::transmute_copy(&plminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorBuild<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorBuild(::core::mem::transmute_copy(&plmajorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorBuild<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorbuild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorBuild(::core::mem::transmute_copy(&plminorbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Debug<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdebug: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Debug(::core::mem::transmute_copy(&pbdebug)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activity<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxactivity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activity(::core::mem::transmute_copy(&ppfaxactivity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutboundRouting<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxoutboundrouting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundRouting(::core::mem::transmute_copy(&ppfaxoutboundrouting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiptOptions<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxreceiptoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiptOptions(::core::mem::transmute_copy(&ppfaxreceiptoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security(::core::mem::transmute_copy(&ppfaxsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtensionProperty<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvproperty: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionProperty(&*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionProperty<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtensionProperty(&*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&vproperty as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListenToServerEvents<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventtypes: FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenToServerEvents(eventtypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDeviceProvider<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, tspname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfspiversion: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDeviceProvider(
                &*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrimagename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&tspname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lfspiversion,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceProvider<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstruniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDeviceProvider(&*(&bstruniquename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterInboundRoutingExtension<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfriendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrimagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vmethods: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterInboundRoutingExtension(
                &*(&bstrextensionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrfriendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrimagename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vmethods as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterInboundRoutingExtension<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrextensionuniquename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterInboundRoutingExtension(&*(&bstrextensionuniquename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisteredEvents<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventtypes: *mut FAX_SERVER_EVENTS_TYPE_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredEvents(::core::mem::transmute_copy(&peventtypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn APIVersion<Impl: IFaxServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papiversion: *mut FAX_SERVER_APIVERSION_ENUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).APIVersion(::core::mem::transmute_copy(&papiversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFaxServer>,
            ::windows::core::GetTrustLevel,
            Connect::<Impl, OFFSET>,
            ServerName::<Impl, OFFSET>,
            GetDeviceProviders::<Impl, OFFSET>,
            GetDevices::<Impl, OFFSET>,
            InboundRouting::<Impl, OFFSET>,
            Folders::<Impl, OFFSET>,
            LoggingOptions::<Impl, OFFSET>,
            MajorVersion::<Impl, OFFSET>,
            MinorVersion::<Impl, OFFSET>,
            MajorBuild::<Impl, OFFSET>,
            MinorBuild::<Impl, OFFSET>,
            Debug::<Impl, OFFSET>,
            Activity::<Impl, OFFSET>,
            OutboundRouting::<Impl, OFFSET>,
            ReceiptOptions::<Impl, OFFSET>,
            Security::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            GetExtensionProperty::<Impl, OFFSET>,
            SetExtensionProperty::<Impl, OFFSET>,
            ListenToServerEvents::<Impl, OFFSET>,
            RegisterDeviceProvider::<Impl, OFFSET>,
            UnregisterDeviceProvider::<Impl, OFFSET>,
            RegisterInboundRoutingExtension::<Impl, OFFSET>,
            UnregisterInboundRoutingExtension::<Impl, OFFSET>,
            RegisteredEvents::<Impl, OFFSET>,
            APIVersion::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServer2Impl: Sized + IFaxServerImpl + IDispatchImpl {
    fn Configuration();
    fn CurrentAccount();
    fn FaxAccountSet();
    fn Security2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxServer2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxServer2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServer2Impl, const OFFSET: isize>() -> IFaxServer2Vtbl {
        unsafe extern "system" fn Configuration<Impl: IFaxServer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxconfiguration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration(::core::mem::transmute_copy(&ppfaxconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccount<Impl: IFaxServer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcurrentaccount: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAccount(::core::mem::transmute_copy(&ppcurrentaccount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaxAccountSet<Impl: IFaxServer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxaccountset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaxAccountSet(::core::mem::transmute_copy(&ppfaxaccountset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security2<Impl: IFaxServer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfaxsecurity2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security2(::core::mem::transmute_copy(&ppfaxsecurity2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxServer2>, ::windows::core::GetTrustLevel, Configuration::<Impl, OFFSET>, CurrentAccount::<Impl, OFFSET>, FaxAccountSet::<Impl, OFFSET>, Security2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotifyImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxServerNotify {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxServerNotify";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServerNotifyImpl, const OFFSET: isize>() -> IFaxServerNotifyVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxServerNotify>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFaxServerNotify2Impl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFaxServerNotify2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IFaxServerNotify2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFaxServerNotify2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaxServerNotify2Impl, const OFFSET: isize>() -> IFaxServerNotify2Vtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFaxServerNotify2>, ::windows::core::GetTrustLevel)
    }
}
pub trait IStiDeviceImpl: Sized {
    fn Initialize();
    fn GetCapabilities();
    fn GetStatus();
    fn DeviceReset();
    fn Diagnostic();
    fn Escape();
    fn GetLastError();
    fn LockDevice();
    fn UnLockDevice();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn Subscribe();
    fn GetLastNotificationData();
    fn UnSubscribe();
    fn GetLastErrorInfo();
}
impl ::windows::core::RuntimeName for IStiDevice {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IStiDevice";
}
impl IStiDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiDeviceImpl, const OFFSET: isize>() -> IStiDeviceVtbl {
        unsafe extern "system" fn Initialize<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, pwszdevicename: super::super::Foundation::PWSTR, dwversion: u32, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&hinst as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), &*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwversion, dwmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_DEV_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&pdevcaps as *const <STI_DEV_CAPS as ::windows::core::Abi>::Abi as *const <STI_DEV_CAPS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pdevstatus as *const <STI_DEVICE_STATUS as ::windows::core::Abi>::Abi as *const <STI_DEVICE_STATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceReset<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Diagnostic(&*(&pbuffer as *const <STI_DIAG as ::windows::core::Abi>::Abi as *const <STI_DIAG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(escapefunction, &*(&lpindata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbindatasize, ::core::mem::transmute_copy(&poutdata), dwoutdatasize, ::core::mem::transmute_copy(&pdwactualdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastError<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(::core::mem::transmute_copy(&pdwlastdeviceerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockDevice(dwtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnLockDevice<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnLockDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadData<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadData(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteData(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadCommand(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteCommand(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subscribe<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpsubsribe: *mut STISUBSCRIBE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subscribe(&*(&lpsubsribe as *const <STISUBSCRIBE as ::windows::core::Abi>::Abi as *const <STISUBSCRIBE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastNotificationData<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastNotificationData(::core::mem::transmute_copy(&lpnotify)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnSubscribe<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnSubscribe() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Impl: IStiDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastErrorInfo(::core::mem::transmute_copy(&plasterrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStiDevice>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetCapabilities::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            DeviceReset::<Impl, OFFSET>,
            Diagnostic::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            GetLastError::<Impl, OFFSET>,
            LockDevice::<Impl, OFFSET>,
            UnLockDevice::<Impl, OFFSET>,
            RawReadData::<Impl, OFFSET>,
            RawWriteData::<Impl, OFFSET>,
            RawReadCommand::<Impl, OFFSET>,
            RawWriteCommand::<Impl, OFFSET>,
            Subscribe::<Impl, OFFSET>,
            GetLastNotificationData::<Impl, OFFSET>,
            UnSubscribe::<Impl, OFFSET>,
            GetLastErrorInfo::<Impl, OFFSET>,
        )
    }
}
pub trait IStiDeviceControlImpl: Sized {
    fn Initialize();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn RawDeviceControl();
    fn GetLastError();
    fn GetMyDevicePortName();
    fn GetMyDeviceHandle();
    fn GetMyDeviceOpenMode();
    fn WriteToErrorLog();
}
impl ::windows::core::RuntimeName for IStiDeviceControl {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IStiDeviceControl";
}
impl IStiDeviceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiDeviceControlImpl, const OFFSET: isize>() -> IStiDeviceControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdevicetype: u32, dwmode: u32, pwszportname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(dwdevicetype, dwmode, &*(&pwszportname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadData<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadData(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteData(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadCommand(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteCommand(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDeviceControl<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *mut ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, dwoutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDeviceControl(escapefunction, &*(&lpindata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbindatasize, &*(&poutdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwoutdatasize, pdwactualdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastError<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwlasterror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(lpdwlasterror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMyDevicePortName<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpszdevicepath: super::super::Foundation::PWSTR, cwdevicepathsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyDevicePortName(::core::mem::transmute_copy(&lpszdevicepath), cwdevicepathsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMyDeviceHandle<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyDeviceHandle(&*(&lph as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMyDeviceOpenMode<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwopenmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyDeviceOpenMode(pdwopenmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Impl: IStiDeviceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR, dwerrorcode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteToErrorLog(dwmessagetype, &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStiDeviceControl>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            RawReadData::<Impl, OFFSET>,
            RawWriteData::<Impl, OFFSET>,
            RawReadCommand::<Impl, OFFSET>,
            RawWriteCommand::<Impl, OFFSET>,
            RawDeviceControl::<Impl, OFFSET>,
            GetLastError::<Impl, OFFSET>,
            GetMyDevicePortName::<Impl, OFFSET>,
            GetMyDeviceHandle::<Impl, OFFSET>,
            GetMyDeviceOpenMode::<Impl, OFFSET>,
            WriteToErrorLog::<Impl, OFFSET>,
        )
    }
}
pub trait IStiUSDImpl: Sized {
    fn Initialize();
    fn GetCapabilities();
    fn GetStatus();
    fn DeviceReset();
    fn Diagnostic();
    fn Escape();
    fn GetLastError();
    fn LockDevice();
    fn UnLockDevice();
    fn RawReadData();
    fn RawWriteData();
    fn RawReadCommand();
    fn RawWriteCommand();
    fn SetNotificationHandle();
    fn GetNotificationData();
    fn GetLastErrorInfo();
}
impl ::windows::core::RuntimeName for IStiUSD {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IStiUSD";
}
impl IStiUSDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStiUSDImpl, const OFFSET: isize>() -> IStiUSDVtbl {
        unsafe extern "system" fn Initialize<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheldcb: ::windows::core::RawPtr, dwstiversion: u32, hparameterskey: super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pheldcb as *const <IStiDeviceControl as ::windows::core::Abi>::Abi as *const <IStiDeviceControl as ::windows::core::DefaultType>::DefaultType), dwstiversion, &*(&hparameterskey as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilities<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevcaps: *mut STI_USD_CAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(::core::mem::transmute_copy(&pdevcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevstatus: *mut STI_DEVICE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(&*(&pdevstatus as *const <STI_DEVICE_STATUS as ::windows::core::Abi>::Abi as *const <STI_DEVICE_STATUS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceReset<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Diagnostic<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut STI_DIAG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Diagnostic(&*(&pbuffer as *const <STI_DIAG as ::windows::core::Abi>::Abi as *const <STI_DIAG as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Escape<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, escapefunction: u32, lpindata: *const ::core::ffi::c_void, cbindatasize: u32, poutdata: *mut ::core::ffi::c_void, cboutdatasize: u32, pdwactualdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Escape(escapefunction, &*(&lpindata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbindatasize, ::core::mem::transmute_copy(&poutdata), cboutdatasize, ::core::mem::transmute_copy(&pdwactualdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastError<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlastdeviceerror: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastError(::core::mem::transmute_copy(&pdwlastdeviceerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockDevice<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnLockDevice<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnLockDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadData<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadData(::core::mem::transmute_copy(&lpbuffer), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteData<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteData(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawReadCommand<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, lpdwnumberofbytes: *mut u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawReadCommand(::core::mem::transmute_copy(&lpbuffer), lpdwnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawWriteCommand<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nnumberofbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawWriteCommand(&*(&lpbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), nnumberofbytes, &*(&lpoverlapped as *const <super::super::System::IO::OVERLAPPED as ::windows::core::Abi>::Abi as *const <super::super::System::IO::OVERLAPPED as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationHandle<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationHandle(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotificationData<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpnotify: *mut STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNotificationData(::core::mem::transmute_copy(&lpnotify)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastErrorInfo<Impl: IStiUSDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasterrorinfo: *mut _ERROR_INFOW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastErrorInfo(::core::mem::transmute_copy(&plasterrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStiUSD>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetCapabilities::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            DeviceReset::<Impl, OFFSET>,
            Diagnostic::<Impl, OFFSET>,
            Escape::<Impl, OFFSET>,
            GetLastError::<Impl, OFFSET>,
            LockDevice::<Impl, OFFSET>,
            UnLockDevice::<Impl, OFFSET>,
            RawReadData::<Impl, OFFSET>,
            RawWriteData::<Impl, OFFSET>,
            RawReadCommand::<Impl, OFFSET>,
            RawWriteCommand::<Impl, OFFSET>,
            SetNotificationHandle::<Impl, OFFSET>,
            GetNotificationData::<Impl, OFFSET>,
            GetLastErrorInfo::<Impl, OFFSET>,
        )
    }
}
pub trait IStillImageWImpl: Sized {
    fn Initialize();
    fn GetDeviceList();
    fn GetDeviceInfo();
    fn CreateDevice();
    fn GetDeviceValue();
    fn SetDeviceValue();
    fn GetSTILaunchInformation();
    fn RegisterLaunchApplication();
    fn UnregisterLaunchApplication();
    fn EnableHwNotifications();
    fn GetHwNotificationState();
    fn RefreshDeviceBus();
    fn LaunchApplicationForDevice();
    fn SetupDeviceParameters();
    fn WriteToErrorLog();
}
impl ::windows::core::RuntimeName for IStillImageW {
    const NAME: &'static str = "Windows.Win32.Devices.Fax.IStillImageW";
}
impl IStillImageWVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStillImageWImpl, const OFFSET: isize>() -> IStillImageWVtbl {
        unsafe extern "system" fn Initialize<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hinst: super::super::Foundation::HINSTANCE, dwversion: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&hinst as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), dwversion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceList<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtype: u32, dwflags: u32, pdwitemsreturned: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceList(dwtype, dwflags, ::core::mem::transmute_copy(&pdwitemsreturned), ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceInfo(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDevice<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, dwmode: u32, pdevice: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDevice(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmode, ::core::mem::transmute_copy(&pdevice), &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceValue<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, ptype: *mut u32, pdata: *mut u8, cbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceValue(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pdata), cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceValue<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pvaluename: super::super::Foundation::PWSTR, r#type: u32, pdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeviceValue(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvaluename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), r#type, pdata, cbdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSTILaunchInformation<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pdweventcode: *mut u32, pwszeventname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSTILaunchInformation(::core::mem::transmute_copy(&pwszdevicename), ::core::mem::transmute_copy(&pdweventcode), ::core::mem::transmute_copy(&pwszeventname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterLaunchApplication<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: super::super::Foundation::PWSTR, pwszcommandline: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterLaunchApplication(&*(&pwszappname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszcommandline as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterLaunchApplication<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszappname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterLaunchApplication(&*(&pwszappname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableHwNotifications<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, bnewstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableHwNotifications(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&bnewstate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwNotificationState<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pbcurrentstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHwNotificationState(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbcurrentstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDeviceBus<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshDeviceBus(&*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchApplicationForDevice<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicename: super::super::Foundation::PWSTR, pwszappname: super::super::Foundation::PWSTR, pstinotify: *const STINOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchApplicationForDevice(
                &*(&pwszdevicename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszappname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pstinotify as *const <STINOTIFY as ::windows::core::Abi>::Abi as *const <STINOTIFY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupDeviceParameters<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, param0: *mut STI_DEVICE_INFORMATIONW) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupDeviceParameters(&*(&param0 as *const <STI_DEVICE_INFORMATIONW as ::windows::core::Abi>::Abi as *const <STI_DEVICE_INFORMATIONW as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteToErrorLog<Impl: IStillImageWImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmessagetype: u32, pszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteToErrorLog(dwmessagetype, &*(&pszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStillImageW>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetDeviceList::<Impl, OFFSET>,
            GetDeviceInfo::<Impl, OFFSET>,
            CreateDevice::<Impl, OFFSET>,
            GetDeviceValue::<Impl, OFFSET>,
            SetDeviceValue::<Impl, OFFSET>,
            GetSTILaunchInformation::<Impl, OFFSET>,
            RegisterLaunchApplication::<Impl, OFFSET>,
            UnregisterLaunchApplication::<Impl, OFFSET>,
            EnableHwNotifications::<Impl, OFFSET>,
            GetHwNotificationState::<Impl, OFFSET>,
            RefreshDeviceBus::<Impl, OFFSET>,
            LaunchApplicationForDevice::<Impl, OFFSET>,
            SetupDeviceParameters::<Impl, OFFSET>,
            WriteToErrorLog::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IFaxAccountNotifyImpl: Sized + IDispatchImpl {
    fn OnIncomingJobAdded();
    fn OnIncomingJobRemoved();
    fn OnIncomingJobChanged();
    fn OnOutgoingJobAdded();
    fn OnOutgoingJobRemoved();
    fn OnOutgoingJobChanged();
    fn OnIncomingMessageAdded();
    fn OnIncomingMessageRemoved();
    fn OnOutgoingMessageAdded();
    fn OnOutgoingMessageRemoved();
    fn OnServerShutDown();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IFaxAccountNotify {
    const NAME: &'static str = "Windows.Win32.Devices.Fax._IFaxAccountNotify";
}
#[cfg(feature = "Win32_System_Com")]
impl _IFaxAccountNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>() -> _IFaxAccountNotifyVtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobAdded(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobRemoved(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingJobChanged<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobChanged(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pjobstatus as *const <IFaxJobStatus as ::windows::core::Abi>::Abi as *const <IFaxJobStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobAdded(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobRemoved(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobChanged(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pjobstatus as *const <IFaxJobStatus as ::windows::core::Abi>::Abi as *const <IFaxJobStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, faddedtoreceivefolder: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingMessageAdded(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), faddedtoreceivefolder) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fremovedfromreceivefolder: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingMessageRemoved(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fremovedfromreceivefolder) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingMessageAdded(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxaccount: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingMessageRemoved(&*(&pfaxaccount as *const <IFaxAccount as ::windows::core::Abi>::Abi as *const <IFaxAccount as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnServerShutDown<Impl: _IFaxAccountNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnServerShutDown(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<_IFaxAccountNotify>,
            ::windows::core::GetTrustLevel,
            OnIncomingJobAdded::<Impl, OFFSET>,
            OnIncomingJobRemoved::<Impl, OFFSET>,
            OnIncomingJobChanged::<Impl, OFFSET>,
            OnOutgoingJobAdded::<Impl, OFFSET>,
            OnOutgoingJobRemoved::<Impl, OFFSET>,
            OnOutgoingJobChanged::<Impl, OFFSET>,
            OnIncomingMessageAdded::<Impl, OFFSET>,
            OnIncomingMessageRemoved::<Impl, OFFSET>,
            OnOutgoingMessageAdded::<Impl, OFFSET>,
            OnOutgoingMessageRemoved::<Impl, OFFSET>,
            OnServerShutDown::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IFaxServerNotify2Impl: Sized + IDispatchImpl {
    fn OnIncomingJobAdded();
    fn OnIncomingJobRemoved();
    fn OnIncomingJobChanged();
    fn OnOutgoingJobAdded();
    fn OnOutgoingJobRemoved();
    fn OnOutgoingJobChanged();
    fn OnIncomingMessageAdded();
    fn OnIncomingMessageRemoved();
    fn OnOutgoingMessageAdded();
    fn OnOutgoingMessageRemoved();
    fn OnReceiptOptionsChange();
    fn OnActivityLoggingConfigChange();
    fn OnSecurityConfigChange();
    fn OnEventLoggingConfigChange();
    fn OnOutgoingQueueConfigChange();
    fn OnOutgoingArchiveConfigChange();
    fn OnIncomingArchiveConfigChange();
    fn OnDevicesConfigChange();
    fn OnOutboundRoutingGroupsConfigChange();
    fn OnOutboundRoutingRulesConfigChange();
    fn OnServerActivityChange();
    fn OnQueuesStatusChange();
    fn OnNewCall();
    fn OnServerShutDown();
    fn OnDeviceStatusChange();
    fn OnGeneralServerConfigChanged();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _IFaxServerNotify2 {
    const NAME: &'static str = "Windows.Win32.Devices.Fax._IFaxServerNotify2";
}
#[cfg(feature = "Win32_System_Com")]
impl _IFaxServerNotify2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _IFaxServerNotify2Impl, const OFFSET: isize>() -> _IFaxServerNotify2Vtbl {
        unsafe extern "system" fn OnIncomingJobAdded<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobAdded(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingJobRemoved<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobRemoved(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingJobChanged<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingJobChanged(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pjobstatus as *const <IFaxJobStatus as ::windows::core::Abi>::Abi as *const <IFaxJobStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobAdded<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobAdded(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobRemoved<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobRemoved(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingJobChanged<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrjobid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pjobstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingJobChanged(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrjobid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pjobstatus as *const <IFaxJobStatus as ::windows::core::Abi>::Abi as *const <IFaxJobStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingMessageAdded<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingMessageAdded(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingMessageRemoved<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingMessageRemoved(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingMessageAdded<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingMessageAdded(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingMessageRemoved<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, bstrmessageid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingMessageRemoved(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), &*(&bstrmessageid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnReceiptOptionsChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnReceiptOptionsChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityLoggingConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityLoggingConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSecurityConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSecurityConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEventLoggingConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEventLoggingConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingQueueConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingQueueConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutgoingArchiveConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutgoingArchiveConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIncomingArchiveConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIncomingArchiveConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDevicesConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDevicesConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutboundRoutingGroupsConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutboundRoutingGroupsConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnOutboundRoutingRulesConfigChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnOutboundRoutingRulesConfigChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnServerActivityChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lincomingmessages: i32, lroutingmessages: i32, loutgoingmessages: i32, lqueuedmessages: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnServerActivityChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), lincomingmessages, lroutingmessages, loutgoingmessages, lqueuedmessages) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQueuesStatusChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, boutgoingqueueblocked: i16, boutgoingqueuepaused: i16, bincomingqueueblocked: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQueuesStatusChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), boutgoingqueueblocked, boutgoingqueuepaused, bincomingqueueblocked) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnNewCall<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, lcallid: i32, ldeviceid: i32, bstrcallerid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNewCall(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), lcallid, ldeviceid, &*(&bstrcallerid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnServerShutDown<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnServerShutDown(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDeviceStatusChange<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr, ldeviceid: i32, bpoweredoff: i16, bsending: i16, breceiving: i16, bringing: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDeviceStatusChange(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType), ldeviceid, bpoweredoff, bsending, breceiving, bringing) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGeneralServerConfigChanged<Impl: _IFaxServerNotify2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfaxserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnGeneralServerConfigChanged(&*(&pfaxserver as *const <IFaxServer2 as ::windows::core::Abi>::Abi as *const <IFaxServer2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<_IFaxServerNotify2>,
            ::windows::core::GetTrustLevel,
            OnIncomingJobAdded::<Impl, OFFSET>,
            OnIncomingJobRemoved::<Impl, OFFSET>,
            OnIncomingJobChanged::<Impl, OFFSET>,
            OnOutgoingJobAdded::<Impl, OFFSET>,
            OnOutgoingJobRemoved::<Impl, OFFSET>,
            OnOutgoingJobChanged::<Impl, OFFSET>,
            OnIncomingMessageAdded::<Impl, OFFSET>,
            OnIncomingMessageRemoved::<Impl, OFFSET>,
            OnOutgoingMessageAdded::<Impl, OFFSET>,
            OnOutgoingMessageRemoved::<Impl, OFFSET>,
            OnReceiptOptionsChange::<Impl, OFFSET>,
            OnActivityLoggingConfigChange::<Impl, OFFSET>,
            OnSecurityConfigChange::<Impl, OFFSET>,
            OnEventLoggingConfigChange::<Impl, OFFSET>,
            OnOutgoingQueueConfigChange::<Impl, OFFSET>,
            OnOutgoingArchiveConfigChange::<Impl, OFFSET>,
            OnIncomingArchiveConfigChange::<Impl, OFFSET>,
            OnDevicesConfigChange::<Impl, OFFSET>,
            OnOutboundRoutingGroupsConfigChange::<Impl, OFFSET>,
            OnOutboundRoutingRulesConfigChange::<Impl, OFFSET>,
            OnServerActivityChange::<Impl, OFFSET>,
            OnQueuesStatusChange::<Impl, OFFSET>,
            OnNewCall::<Impl, OFFSET>,
            OnServerShutDown::<Impl, OFFSET>,
            OnDeviceStatusChange::<Impl, OFFSET>,
            OnGeneralServerConfigChanged::<Impl, OFFSET>,
        )
    }
}
