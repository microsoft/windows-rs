#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IActionImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetId();
    fn Type();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActionVtbl {
        unsafe extern "system" fn Id<Impl: IActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetId<Impl: IActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Type<Impl: IActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, SetId::<Impl, IMPL_OFFSET>, Type::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IActionCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn XmlText();
    fn SetXmlText();
    fn Create();
    fn Remove();
    fn Clear();
    fn Context();
    fn SetContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IActionCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActionCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn XmlText<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXmlText<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Context<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContext<Impl: IActionCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Count::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText::<Impl, IMPL_OFFSET>,
            Create::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Clear::<Impl, IMPL_OFFSET>,
            Context::<Impl, IMPL_OFFSET>,
            SetContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBootTriggerVtbl {
        unsafe extern "system" fn Delay<Impl: IBootTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelay<Impl: IBootTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Delay::<Impl, IMPL_OFFSET>,
            SetDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComHandlerActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn ClassId();
    fn SetClassId();
    fn Data();
    fn SetData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComHandlerActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComHandlerActionVtbl {
        unsafe extern "system" fn ClassId<Impl: IComHandlerActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassId<Impl: IComHandlerActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IComHandlerActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IComHandlerActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            ClassId::<Impl, IMPL_OFFSET>,
            SetClassId::<Impl, IMPL_OFFSET>,
            Data::<Impl, IMPL_OFFSET>,
            SetData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComHandlerAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDailyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysInterval();
    fn SetDaysInterval();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDailyTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDailyTriggerVtbl {
        unsafe extern "system" fn DaysInterval<Impl: IDailyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysInterval<Impl: IDailyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RandomDelay<Impl: IDailyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IDailyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DaysInterval::<Impl, IMPL_OFFSET>,
            SetDaysInterval::<Impl, IMPL_OFFSET>,
            RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDailyTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEmailActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Server();
    fn SetServer();
    fn Subject();
    fn SetSubject();
    fn To();
    fn SetTo();
    fn Cc();
    fn SetCc();
    fn Bcc();
    fn SetBcc();
    fn ReplyTo();
    fn SetReplyTo();
    fn From();
    fn SetFrom();
    fn HeaderFields();
    fn SetHeaderFields();
    fn Body();
    fn SetBody();
    fn Attachments();
    fn SetAttachments();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEmailActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailActionVtbl {
        unsafe extern "system" fn Server<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServer<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subject<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubject<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn To<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTo<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cc<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCc<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Bcc<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBcc<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplyTo<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReplyTo<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn From<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrom<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HeaderFields<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderfields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHeaderFields<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaderfields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Body<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBody<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Attachments<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttachments<Impl: IEmailActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Server::<Impl, IMPL_OFFSET>,
            SetServer::<Impl, IMPL_OFFSET>,
            Subject::<Impl, IMPL_OFFSET>,
            SetSubject::<Impl, IMPL_OFFSET>,
            To::<Impl, IMPL_OFFSET>,
            SetTo::<Impl, IMPL_OFFSET>,
            Cc::<Impl, IMPL_OFFSET>,
            SetCc::<Impl, IMPL_OFFSET>,
            Bcc::<Impl, IMPL_OFFSET>,
            SetBcc::<Impl, IMPL_OFFSET>,
            ReplyTo::<Impl, IMPL_OFFSET>,
            SetReplyTo::<Impl, IMPL_OFFSET>,
            From::<Impl, IMPL_OFFSET>,
            SetFrom::<Impl, IMPL_OFFSET>,
            HeaderFields::<Impl, IMPL_OFFSET>,
            SetHeaderFields::<Impl, IMPL_OFFSET>,
            Body::<Impl, IMPL_OFFSET>,
            SetBody::<Impl, IMPL_OFFSET>,
            Attachments::<Impl, IMPL_OFFSET>,
            SetAttachments::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumWorkItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumWorkItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWorkItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumWorkItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWorkItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumWorkItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWorkItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWorkItems as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEventTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Subscription();
    fn SetSubscription();
    fn Delay();
    fn SetDelay();
    fn ValueQueries();
    fn SetValueQueries();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventTriggerVtbl {
        unsafe extern "system" fn Subscription<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscription<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delay<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelay<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValueQueries<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValueQueries<Impl: IEventTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamedxpaths: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Subscription::<Impl, IMPL_OFFSET>,
            SetSubscription::<Impl, IMPL_OFFSET>,
            Delay::<Impl, IMPL_OFFSET>,
            SetDelay::<Impl, IMPL_OFFSET>,
            ValueQueries::<Impl, IMPL_OFFSET>,
            SetValueQueries::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExecActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Path();
    fn SetPath();
    fn Arguments();
    fn SetArguments();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecActionVtbl {
        unsafe extern "system" fn Path<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Arguments<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetArguments<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IExecActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            Arguments::<Impl, IMPL_OFFSET>,
            SetArguments::<Impl, IMPL_OFFSET>,
            WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExecAction2Impl: Sized + IExecActionImpl + IActionImpl + IDispatchImpl {
    fn HideAppWindow();
    fn SetHideAppWindow();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecAction2Vtbl {
        unsafe extern "system" fn HideAppWindow<Impl: IExecAction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHideAppWindow<Impl: IExecAction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            Arguments::<Impl, IMPL_OFFSET>,
            SetArguments::<Impl, IMPL_OFFSET>,
            WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            HideAppWindow::<Impl, IMPL_OFFSET>,
            SetHideAppWindow::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIdleSettingsImpl: Sized + IDispatchImpl {
    fn IdleDuration();
    fn SetIdleDuration();
    fn WaitTimeout();
    fn SetWaitTimeout();
    fn StopOnIdleEnd();
    fn SetStopOnIdleEnd();
    fn RestartOnIdle();
    fn SetRestartOnIdle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdleSettingsVtbl {
        unsafe extern "system" fn IdleDuration<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIdleDuration<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitTimeout<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWaitTimeout<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopOnIdleEnd<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestartOnIdle<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestartOnIdle<Impl: IIdleSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows::core::HRESULT {
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
            IdleDuration::<Impl, IMPL_OFFSET>,
            SetIdleDuration::<Impl, IMPL_OFFSET>,
            WaitTimeout::<Impl, IMPL_OFFSET>,
            SetWaitTimeout::<Impl, IMPL_OFFSET>,
            StopOnIdleEnd::<Impl, IMPL_OFFSET>,
            SetStopOnIdleEnd::<Impl, IMPL_OFFSET>,
            RestartOnIdle::<Impl, IMPL_OFFSET>,
            SetRestartOnIdle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIdleTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdleTriggerVtbl {
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILogonTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
    fn UserId();
    fn SetUserId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILogonTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogonTriggerVtbl {
        unsafe extern "system" fn Delay<Impl: ILogonTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelay<Impl: ILogonTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserId<Impl: ILogonTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserId<Impl: ILogonTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Delay::<Impl, IMPL_OFFSET>,
            SetDelay::<Impl, IMPL_OFFSET>,
            UserId::<Impl, IMPL_OFFSET>,
            SetUserId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogonTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMaintenanceSettingsImpl: Sized + IDispatchImpl {
    fn SetPeriod();
    fn Period();
    fn SetDeadline();
    fn Deadline();
    fn SetExclusive();
    fn Exclusive();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMaintenanceSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMaintenanceSettingsVtbl {
        unsafe extern "system" fn SetPeriod<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Period<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeadline<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deadline<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExclusive<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Exclusive<Impl: IMaintenanceSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows::core::HRESULT {
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
            SetPeriod::<Impl, IMPL_OFFSET>,
            Period::<Impl, IMPL_OFFSET>,
            SetDeadline::<Impl, IMPL_OFFSET>,
            Deadline::<Impl, IMPL_OFFSET>,
            SetExclusive::<Impl, IMPL_OFFSET>,
            Exclusive::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMaintenanceSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMonthlyDOWTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfWeek();
    fn SetDaysOfWeek();
    fn WeeksOfMonth();
    fn SetWeeksOfMonth();
    fn MonthsOfYear();
    fn SetMonthsOfYear();
    fn RunOnLastWeekOfMonth();
    fn SetRunOnLastWeekOfMonth();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyDOWTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonthlyDOWTriggerVtbl {
        unsafe extern "system" fn DaysOfWeek<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WeeksOfMonth<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWeeksOfMonth<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonthsOfYear<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMonthsOfYear<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RandomDelay<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IMonthlyDOWTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeeksOfMonth::<Impl, IMPL_OFFSET>,
            SetWeeksOfMonth::<Impl, IMPL_OFFSET>,
            MonthsOfYear::<Impl, IMPL_OFFSET>,
            SetMonthsOfYear::<Impl, IMPL_OFFSET>,
            RunOnLastWeekOfMonth::<Impl, IMPL_OFFSET>,
            SetRunOnLastWeekOfMonth::<Impl, IMPL_OFFSET>,
            RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMonthlyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfMonth();
    fn SetDaysOfMonth();
    fn MonthsOfYear();
    fn SetMonthsOfYear();
    fn RunOnLastDayOfMonth();
    fn SetRunOnLastDayOfMonth();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonthlyTriggerVtbl {
        unsafe extern "system" fn DaysOfMonth<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysOfMonth<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonthsOfYear<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMonthsOfYear<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RandomDelay<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IMonthlyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DaysOfMonth::<Impl, IMPL_OFFSET>,
            SetDaysOfMonth::<Impl, IMPL_OFFSET>,
            MonthsOfYear::<Impl, IMPL_OFFSET>,
            SetMonthsOfYear::<Impl, IMPL_OFFSET>,
            RunOnLastDayOfMonth::<Impl, IMPL_OFFSET>,
            SetRunOnLastDayOfMonth::<Impl, IMPL_OFFSET>,
            RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkSettingsImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Id();
    fn SetId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkSettingsVtbl {
        unsafe extern "system" fn Name<Impl: INetworkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: INetworkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: INetworkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetId<Impl: INetworkSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, Id::<Impl, IMPL_OFFSET>, SetId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrincipalImpl: Sized + IDispatchImpl {
    fn Id();
    fn SetId();
    fn DisplayName();
    fn SetDisplayName();
    fn UserId();
    fn SetUserId();
    fn LogonType();
    fn SetLogonType();
    fn GroupId();
    fn SetGroupId();
    fn RunLevel();
    fn SetRunLevel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrincipalVtbl {
        unsafe extern "system" fn Id<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetId<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserId<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserId<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LogonType<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLogonType<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GroupId<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGroupId<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunLevel<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunLevel<Impl: IPrincipalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            UserId::<Impl, IMPL_OFFSET>,
            SetUserId::<Impl, IMPL_OFFSET>,
            LogonType::<Impl, IMPL_OFFSET>,
            SetLogonType::<Impl, IMPL_OFFSET>,
            GroupId::<Impl, IMPL_OFFSET>,
            SetGroupId::<Impl, IMPL_OFFSET>,
            RunLevel::<Impl, IMPL_OFFSET>,
            SetRunLevel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrincipal2Impl: Sized + IDispatchImpl {
    fn ProcessTokenSidType();
    fn SetProcessTokenSidType();
    fn RequiredPrivilegeCount();
    fn RequiredPrivilege();
    fn AddRequiredPrivilege();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipal2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrincipal2Vtbl {
        unsafe extern "system" fn ProcessTokenSidType<Impl: IPrincipal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessTokenSidType<Impl: IPrincipal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Impl: IPrincipal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequiredPrivilege<Impl: IPrincipal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRequiredPrivilege<Impl: IPrincipal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            ProcessTokenSidType::<Impl, IMPL_OFFSET>,
            SetProcessTokenSidType::<Impl, IMPL_OFFSET>,
            RequiredPrivilegeCount::<Impl, IMPL_OFFSET>,
            RequiredPrivilege::<Impl, IMPL_OFFSET>,
            AddRequiredPrivilege::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait IProvideTaskPageImpl: Sized {
    fn GetPage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IProvideTaskPageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideTaskPageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideTaskPageVtbl {
        unsafe extern "system" fn GetPage<Impl: IProvideTaskPageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideTaskPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegisteredTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn Path();
    fn State();
    fn Enabled();
    fn SetEnabled();
    fn Run();
    fn RunEx();
    fn GetInstances();
    fn LastRunTime();
    fn LastTaskResult();
    fn NumberOfMissedRuns();
    fn NextRunTime();
    fn Definition();
    fn Xml();
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Stop();
    fn GetRunTimes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredTaskVtbl {
        unsafe extern "system" fn Name<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Run<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunEx<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstances<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastRunTime<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastTaskResult<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfMissedRuns<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextRunTime<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Definition<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Xml<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRunTimes<Impl: IRegisteredTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
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
            Path::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            RunEx::<Impl, IMPL_OFFSET>,
            GetInstances::<Impl, IMPL_OFFSET>,
            LastRunTime::<Impl, IMPL_OFFSET>,
            LastTaskResult::<Impl, IMPL_OFFSET>,
            NumberOfMissedRuns::<Impl, IMPL_OFFSET>,
            NextRunTime::<Impl, IMPL_OFFSET>,
            Definition::<Impl, IMPL_OFFSET>,
            Xml::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            GetRunTimes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegisteredTaskCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTaskCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredTaskCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IRegisteredTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IRegisteredTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IRegisteredTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationInfoImpl: Sized + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn Author();
    fn SetAuthor();
    fn Version();
    fn SetVersion();
    fn Date();
    fn SetDate();
    fn Documentation();
    fn SetDocumentation();
    fn XmlText();
    fn SetXmlText();
    fn URI();
    fn SetURI();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Source();
    fn SetSource();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegistrationInfoVtbl {
        unsafe extern "system" fn Description<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Author<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthor<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Version<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVersion<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Date<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDate<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Documentation<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDocumentation<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn XmlText<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXmlText<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn URI<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetURI<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Source<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSource<Impl: IRegistrationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            Author::<Impl, IMPL_OFFSET>,
            SetAuthor::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            SetVersion::<Impl, IMPL_OFFSET>,
            Date::<Impl, IMPL_OFFSET>,
            SetDate::<Impl, IMPL_OFFSET>,
            Documentation::<Impl, IMPL_OFFSET>,
            SetDocumentation::<Impl, IMPL_OFFSET>,
            XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText::<Impl, IMPL_OFFSET>,
            URI::<Impl, IMPL_OFFSET>,
            SetURI::<Impl, IMPL_OFFSET>,
            SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Source::<Impl, IMPL_OFFSET>,
            SetSource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegistrationTriggerVtbl {
        unsafe extern "system" fn Delay<Impl: IRegistrationTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelay<Impl: IRegistrationTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Delay::<Impl, IMPL_OFFSET>,
            SetDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRepetitionPatternImpl: Sized + IDispatchImpl {
    fn Interval();
    fn SetInterval();
    fn Duration();
    fn SetDuration();
    fn StopAtDurationEnd();
    fn SetStopAtDurationEnd();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRepetitionPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepetitionPatternVtbl {
        unsafe extern "system" fn Interval<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterval<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Duration<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDuration<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopAtDurationEnd<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Impl: IRepetitionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
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
            Interval::<Impl, IMPL_OFFSET>,
            SetInterval::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            SetDuration::<Impl, IMPL_OFFSET>,
            StopAtDurationEnd::<Impl, IMPL_OFFSET>,
            SetStopAtDurationEnd::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepetitionPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRunningTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn InstanceGuid();
    fn Path();
    fn State();
    fn CurrentAction();
    fn Stop();
    fn Refresh();
    fn EnginePID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunningTaskVtbl {
        unsafe extern "system" fn Name<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstanceGuid<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAction<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnginePID<Impl: IRunningTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT {
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
            InstanceGuid::<Impl, IMPL_OFFSET>,
            Path::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            CurrentAction::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Refresh::<Impl, IMPL_OFFSET>,
            EnginePID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRunningTaskCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTaskCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunningTaskCollectionVtbl {
        unsafe extern "system" fn Count<Impl: IRunningTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IRunningTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: IRunningTaskCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTaskCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IScheduledWorkItemImpl: Sized {
    fn CreateTrigger();
    fn DeleteTrigger();
    fn GetTriggerCount();
    fn GetTrigger();
    fn GetTriggerString();
    fn GetRunTimes();
    fn GetNextRunTime();
    fn SetIdleWait();
    fn GetIdleWait();
    fn Run();
    fn Terminate();
    fn EditWorkItem();
    fn GetMostRecentRunTime();
    fn GetStatus();
    fn GetExitCode();
    fn SetComment();
    fn GetComment();
    fn SetCreator();
    fn GetCreator();
    fn SetWorkItemData();
    fn GetWorkItemData();
    fn SetErrorRetryCount();
    fn GetErrorRetryCount();
    fn SetErrorRetryInterval();
    fn GetErrorRetryInterval();
    fn SetFlags();
    fn GetFlags();
    fn SetAccountInformation();
    fn GetAccountInformation();
}
#[cfg(feature = "Win32_Foundation")]
impl IScheduledWorkItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledWorkItemVtbl {
        unsafe extern "system" fn CreateTrigger<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTrigger<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTriggerCount<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrigger<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTriggerString<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRunTimes<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextRunTime<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIdleWait<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdleWait<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Run<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Terminate<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EditWorkItem<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMostRecentRunTime<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExitCode<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComment<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComment<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreator<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcreator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreator<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcreator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkItemData<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWorkItemData<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorRetryCount<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorRetryCount<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetErrorRetryInterval<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorRetryInterval<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAccountInformation<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszaccountname: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccountInformation<Impl: IScheduledWorkItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateTrigger::<Impl, IMPL_OFFSET>,
            DeleteTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerCount::<Impl, IMPL_OFFSET>,
            GetTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerString::<Impl, IMPL_OFFSET>,
            GetRunTimes::<Impl, IMPL_OFFSET>,
            GetNextRunTime::<Impl, IMPL_OFFSET>,
            SetIdleWait::<Impl, IMPL_OFFSET>,
            GetIdleWait::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            Terminate::<Impl, IMPL_OFFSET>,
            EditWorkItem::<Impl, IMPL_OFFSET>,
            GetMostRecentRunTime::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetExitCode::<Impl, IMPL_OFFSET>,
            SetComment::<Impl, IMPL_OFFSET>,
            GetComment::<Impl, IMPL_OFFSET>,
            SetCreator::<Impl, IMPL_OFFSET>,
            GetCreator::<Impl, IMPL_OFFSET>,
            SetWorkItemData::<Impl, IMPL_OFFSET>,
            GetWorkItemData::<Impl, IMPL_OFFSET>,
            SetErrorRetryCount::<Impl, IMPL_OFFSET>,
            GetErrorRetryCount::<Impl, IMPL_OFFSET>,
            SetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            GetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetAccountInformation::<Impl, IMPL_OFFSET>,
            GetAccountInformation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledWorkItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISessionStateChangeTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn Delay();
    fn SetDelay();
    fn UserId();
    fn SetUserId();
    fn StateChange();
    fn SetStateChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISessionStateChangeTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISessionStateChangeTriggerVtbl {
        unsafe extern "system" fn Delay<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDelay<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UserId<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserId<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StateChange<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStateChange<Impl: ISessionStateChangeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Delay::<Impl, IMPL_OFFSET>,
            SetDelay::<Impl, IMPL_OFFSET>,
            UserId::<Impl, IMPL_OFFSET>,
            SetUserId::<Impl, IMPL_OFFSET>,
            StateChange::<Impl, IMPL_OFFSET>,
            SetStateChange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IShowMessageActionImpl: Sized + IActionImpl + IDispatchImpl {
    fn Title();
    fn SetTitle();
    fn MessageBody();
    fn SetMessageBody();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IShowMessageActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShowMessageActionVtbl {
        unsafe extern "system" fn Title<Impl: IShowMessageActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTitle<Impl: IShowMessageActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MessageBody<Impl: IShowMessageActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMessageBody<Impl: IShowMessageActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            SetId::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            MessageBody::<Impl, IMPL_OFFSET>,
            SetMessageBody::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShowMessageAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskImpl: Sized + IScheduledWorkItemImpl {
    fn SetApplicationName();
    fn GetApplicationName();
    fn SetParameters();
    fn GetParameters();
    fn SetWorkingDirectory();
    fn GetWorkingDirectory();
    fn SetPriority();
    fn GetPriority();
    fn SetTaskFlags();
    fn GetTaskFlags();
    fn SetMaxRunTime();
    fn GetMaxRunTime();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskVtbl {
        unsafe extern "system" fn SetApplicationName<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszapplicationname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationName<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetParameters<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszparameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParameters<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszworkingdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWorkingDirectory<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTaskFlags<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTaskFlags<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxRunTime<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxRunTime<Impl: ITaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateTrigger::<Impl, IMPL_OFFSET>,
            DeleteTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerCount::<Impl, IMPL_OFFSET>,
            GetTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerString::<Impl, IMPL_OFFSET>,
            GetRunTimes::<Impl, IMPL_OFFSET>,
            GetNextRunTime::<Impl, IMPL_OFFSET>,
            SetIdleWait::<Impl, IMPL_OFFSET>,
            GetIdleWait::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            Terminate::<Impl, IMPL_OFFSET>,
            EditWorkItem::<Impl, IMPL_OFFSET>,
            GetMostRecentRunTime::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetExitCode::<Impl, IMPL_OFFSET>,
            SetComment::<Impl, IMPL_OFFSET>,
            GetComment::<Impl, IMPL_OFFSET>,
            SetCreator::<Impl, IMPL_OFFSET>,
            GetCreator::<Impl, IMPL_OFFSET>,
            SetWorkItemData::<Impl, IMPL_OFFSET>,
            GetWorkItemData::<Impl, IMPL_OFFSET>,
            SetErrorRetryCount::<Impl, IMPL_OFFSET>,
            GetErrorRetryCount::<Impl, IMPL_OFFSET>,
            SetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            GetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags::<Impl, IMPL_OFFSET>,
            SetAccountInformation::<Impl, IMPL_OFFSET>,
            GetAccountInformation::<Impl, IMPL_OFFSET>,
            SetApplicationName::<Impl, IMPL_OFFSET>,
            GetApplicationName::<Impl, IMPL_OFFSET>,
            SetParameters::<Impl, IMPL_OFFSET>,
            GetParameters::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            GetWorkingDirectory::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            SetTaskFlags::<Impl, IMPL_OFFSET>,
            GetTaskFlags::<Impl, IMPL_OFFSET>,
            SetMaxRunTime::<Impl, IMPL_OFFSET>,
            GetMaxRunTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskDefinitionImpl: Sized + IDispatchImpl {
    fn RegistrationInfo();
    fn SetRegistrationInfo();
    fn Triggers();
    fn SetTriggers();
    fn Settings();
    fn SetSettings();
    fn Data();
    fn SetData();
    fn Principal();
    fn SetPrincipal();
    fn Actions();
    fn SetActions();
    fn XmlText();
    fn SetXmlText();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskDefinitionVtbl {
        unsafe extern "system" fn RegistrationInfo<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRegistrationInfo<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregistrationinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Triggers<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptriggers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTriggers<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptriggers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Settings<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSettings<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Principal<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprincipal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrincipal<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprincipal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Actions<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppactions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActions<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn XmlText<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXmlText<Impl: ITaskDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            RegistrationInfo::<Impl, IMPL_OFFSET>,
            SetRegistrationInfo::<Impl, IMPL_OFFSET>,
            Triggers::<Impl, IMPL_OFFSET>,
            SetTriggers::<Impl, IMPL_OFFSET>,
            Settings::<Impl, IMPL_OFFSET>,
            SetSettings::<Impl, IMPL_OFFSET>,
            Data::<Impl, IMPL_OFFSET>,
            SetData::<Impl, IMPL_OFFSET>,
            Principal::<Impl, IMPL_OFFSET>,
            SetPrincipal::<Impl, IMPL_OFFSET>,
            Actions::<Impl, IMPL_OFFSET>,
            SetActions::<Impl, IMPL_OFFSET>,
            XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskFolderImpl: Sized + IDispatchImpl {
    fn Name();
    fn Path();
    fn GetFolder();
    fn GetFolders();
    fn CreateFolder();
    fn DeleteFolder();
    fn GetTask();
    fn GetTasks();
    fn DeleteTask();
    fn RegisterTask();
    fn RegisterTaskDefinition();
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskFolderVtbl {
        unsafe extern "system" fn Name<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFolder<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFolders<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFolder<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteFolder<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTask<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTasks<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTask<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterTask<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterTaskDefinition<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: ::windows::core::RawPtr, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITaskFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
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
            Path::<Impl, IMPL_OFFSET>,
            GetFolder::<Impl, IMPL_OFFSET>,
            GetFolders::<Impl, IMPL_OFFSET>,
            CreateFolder::<Impl, IMPL_OFFSET>,
            DeleteFolder::<Impl, IMPL_OFFSET>,
            GetTask::<Impl, IMPL_OFFSET>,
            GetTasks::<Impl, IMPL_OFFSET>,
            DeleteTask::<Impl, IMPL_OFFSET>,
            RegisterTask::<Impl, IMPL_OFFSET>,
            RegisterTaskDefinition::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskFolderCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolderCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskFolderCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITaskFolderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ITaskFolderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ITaskFolderCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskHandlerImpl: Sized {
    fn Start();
    fn Stop();
    fn Pause();
    fn Resume();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskHandlerVtbl {
        unsafe extern "system" fn Start<Impl: ITaskHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: ITaskHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: ITaskHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: ITaskHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, Pause::<Impl, IMPL_OFFSET>, Resume::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskHandlerStatusImpl: Sized {
    fn UpdateStatus();
    fn TaskCompleted();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandlerStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskHandlerStatusVtbl {
        unsafe extern "system" fn UpdateStatus<Impl: ITaskHandlerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TaskCompleted<Impl: ITaskHandlerStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, UpdateStatus::<Impl, IMPL_OFFSET>, TaskCompleted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskHandlerStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskNamedValueCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Create();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValueCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskNamedValueCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ITaskNamedValueCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Create::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskNamedValuePairImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValuePairVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePairImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskNamedValuePairVtbl {
        unsafe extern "system" fn Name<Impl: ITaskNamedValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: ITaskNamedValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: ITaskNamedValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ITaskNamedValuePairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskSchedulerImpl: Sized {
    fn SetTargetComputer();
    fn GetTargetComputer();
    fn Enum();
    fn Activate();
    fn Delete();
    fn NewWorkItem();
    fn AddWorkItem();
    fn IsOfType();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskSchedulerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSchedulerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSchedulerVtbl {
        unsafe extern "system" fn SetTargetComputer<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomputer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetComputer<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enum<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Activate<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewWorkItem<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddWorkItem<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, pworkitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOfType<Impl: ITaskSchedulerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTargetComputer::<Impl, IMPL_OFFSET>, GetTargetComputer::<Impl, IMPL_OFFSET>, Enum::<Impl, IMPL_OFFSET>, Activate::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, NewWorkItem::<Impl, IMPL_OFFSET>, AddWorkItem::<Impl, IMPL_OFFSET>, IsOfType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskScheduler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskServiceImpl: Sized + IDispatchImpl {
    fn GetFolder();
    fn GetRunningTasks();
    fn NewTask();
    fn Connect();
    fn Connected();
    fn TargetServer();
    fn ConnectedUser();
    fn ConnectedDomain();
    fn HighestVersion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskServiceVtbl {
        unsafe extern "system" fn GetFolder<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRunningTasks<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewTask<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connected<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetServer<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectedUser<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectedDomain<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HighestVersion<Impl: ITaskServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows::core::HRESULT {
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
            GetFolder::<Impl, IMPL_OFFSET>,
            GetRunningTasks::<Impl, IMPL_OFFSET>,
            NewTask::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            Connected::<Impl, IMPL_OFFSET>,
            TargetServer::<Impl, IMPL_OFFSET>,
            ConnectedUser::<Impl, IMPL_OFFSET>,
            ConnectedDomain::<Impl, IMPL_OFFSET>,
            HighestVersion::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettingsImpl: Sized + IDispatchImpl {
    fn AllowDemandStart();
    fn SetAllowDemandStart();
    fn RestartInterval();
    fn SetRestartInterval();
    fn RestartCount();
    fn SetRestartCount();
    fn MultipleInstances();
    fn SetMultipleInstances();
    fn StopIfGoingOnBatteries();
    fn SetStopIfGoingOnBatteries();
    fn DisallowStartIfOnBatteries();
    fn SetDisallowStartIfOnBatteries();
    fn AllowHardTerminate();
    fn SetAllowHardTerminate();
    fn StartWhenAvailable();
    fn SetStartWhenAvailable();
    fn XmlText();
    fn SetXmlText();
    fn RunOnlyIfNetworkAvailable();
    fn SetRunOnlyIfNetworkAvailable();
    fn ExecutionTimeLimit();
    fn SetExecutionTimeLimit();
    fn Enabled();
    fn SetEnabled();
    fn DeleteExpiredTaskAfter();
    fn SetDeleteExpiredTaskAfter();
    fn Priority();
    fn SetPriority();
    fn Compatibility();
    fn SetCompatibility();
    fn Hidden();
    fn SetHidden();
    fn IdleSettings();
    fn SetIdleSettings();
    fn RunOnlyIfIdle();
    fn SetRunOnlyIfIdle();
    fn WakeToRun();
    fn SetWakeToRun();
    fn NetworkSettings();
    fn SetNetworkSettings();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettingsVtbl {
        unsafe extern "system" fn AllowDemandStart<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowDemandStart<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestartInterval<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestartInterval<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestartCount<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRestartCount<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultipleInstances<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultipleInstances<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowHardTerminate<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowHardTerminate<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartWhenAvailable<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartWhenAvailable<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn XmlText<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetXmlText<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Priority<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Compatibility<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompatibility<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hidden<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHidden<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IdleSettings<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidlesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIdleSettings<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidlesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RunOnlyIfIdle<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WakeToRun<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWakeToRun<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetworkSettings<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkSettings<Impl: ITaskSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            AllowDemandStart::<Impl, IMPL_OFFSET>,
            SetAllowDemandStart::<Impl, IMPL_OFFSET>,
            RestartInterval::<Impl, IMPL_OFFSET>,
            SetRestartInterval::<Impl, IMPL_OFFSET>,
            RestartCount::<Impl, IMPL_OFFSET>,
            SetRestartCount::<Impl, IMPL_OFFSET>,
            MultipleInstances::<Impl, IMPL_OFFSET>,
            SetMultipleInstances::<Impl, IMPL_OFFSET>,
            StopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            SetStopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            DisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            SetDisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            AllowHardTerminate::<Impl, IMPL_OFFSET>,
            SetAllowHardTerminate::<Impl, IMPL_OFFSET>,
            StartWhenAvailable::<Impl, IMPL_OFFSET>,
            SetStartWhenAvailable::<Impl, IMPL_OFFSET>,
            XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText::<Impl, IMPL_OFFSET>,
            RunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            SetDeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Compatibility::<Impl, IMPL_OFFSET>,
            SetCompatibility::<Impl, IMPL_OFFSET>,
            Hidden::<Impl, IMPL_OFFSET>,
            SetHidden::<Impl, IMPL_OFFSET>,
            IdleSettings::<Impl, IMPL_OFFSET>,
            SetIdleSettings::<Impl, IMPL_OFFSET>,
            RunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            WakeToRun::<Impl, IMPL_OFFSET>,
            SetWakeToRun::<Impl, IMPL_OFFSET>,
            NetworkSettings::<Impl, IMPL_OFFSET>,
            SetNetworkSettings::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettings2Impl: Sized + IDispatchImpl {
    fn DisallowStartOnRemoteAppSession();
    fn SetDisallowStartOnRemoteAppSession();
    fn UseUnifiedSchedulingEngine();
    fn SetUseUnifiedSchedulingEngine();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettings2Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Impl: ITaskSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Impl: ITaskSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Impl: ITaskSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Impl: ITaskSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
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
            DisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            SetDisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            UseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            SetUseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettings3Impl: Sized + ITaskSettingsImpl + IDispatchImpl {
    fn DisallowStartOnRemoteAppSession();
    fn SetDisallowStartOnRemoteAppSession();
    fn UseUnifiedSchedulingEngine();
    fn SetUseUnifiedSchedulingEngine();
    fn MaintenanceSettings();
    fn SetMaintenanceSettings();
    fn CreateMaintenanceSettings();
    fn Volatile();
    fn SetVolatile();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettings3Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaintenanceSettings<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaintenanceSettings<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaintenancesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Volatile<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolatile<Impl: ITaskSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows::core::HRESULT {
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
            AllowDemandStart::<Impl, IMPL_OFFSET>,
            SetAllowDemandStart::<Impl, IMPL_OFFSET>,
            RestartInterval::<Impl, IMPL_OFFSET>,
            SetRestartInterval::<Impl, IMPL_OFFSET>,
            RestartCount::<Impl, IMPL_OFFSET>,
            SetRestartCount::<Impl, IMPL_OFFSET>,
            MultipleInstances::<Impl, IMPL_OFFSET>,
            SetMultipleInstances::<Impl, IMPL_OFFSET>,
            StopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            SetStopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            DisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            SetDisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            AllowHardTerminate::<Impl, IMPL_OFFSET>,
            SetAllowHardTerminate::<Impl, IMPL_OFFSET>,
            StartWhenAvailable::<Impl, IMPL_OFFSET>,
            SetStartWhenAvailable::<Impl, IMPL_OFFSET>,
            XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText::<Impl, IMPL_OFFSET>,
            RunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            SetDeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Compatibility::<Impl, IMPL_OFFSET>,
            SetCompatibility::<Impl, IMPL_OFFSET>,
            Hidden::<Impl, IMPL_OFFSET>,
            SetHidden::<Impl, IMPL_OFFSET>,
            IdleSettings::<Impl, IMPL_OFFSET>,
            SetIdleSettings::<Impl, IMPL_OFFSET>,
            RunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            WakeToRun::<Impl, IMPL_OFFSET>,
            SetWakeToRun::<Impl, IMPL_OFFSET>,
            NetworkSettings::<Impl, IMPL_OFFSET>,
            SetNetworkSettings::<Impl, IMPL_OFFSET>,
            DisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            SetDisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            UseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            SetUseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            MaintenanceSettings::<Impl, IMPL_OFFSET>,
            SetMaintenanceSettings::<Impl, IMPL_OFFSET>,
            CreateMaintenanceSettings::<Impl, IMPL_OFFSET>,
            Volatile::<Impl, IMPL_OFFSET>,
            SetVolatile::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskTriggerImpl: Sized {
    fn SetTrigger();
    fn GetTrigger();
    fn GetTriggerString();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskTriggerVtbl {
        unsafe extern "system" fn SetTrigger<Impl: ITaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrigger<Impl: ITaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTriggerString<Impl: ITaskTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTrigger::<Impl, IMPL_OFFSET>, GetTrigger::<Impl, IMPL_OFFSET>, GetTriggerString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskVariablesImpl: Sized {
    fn GetInput();
    fn SetOutput();
    fn GetContext();
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskVariablesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariablesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskVariablesVtbl {
        unsafe extern "system" fn GetInput<Impl: ITaskVariablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutput<Impl: ITaskVariablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: ITaskVariablesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInput::<Impl, IMPL_OFFSET>, SetOutput::<Impl, IMPL_OFFSET>, GetContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskVariables as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITimeTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITimeTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeTriggerVtbl {
        unsafe extern "system" fn RandomDelay<Impl: ITimeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: ITimeTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITriggerImpl: Sized + IDispatchImpl {
    fn Type();
    fn Id();
    fn SetId();
    fn Repetition();
    fn SetRepetition();
    fn ExecutionTimeLimit();
    fn SetExecutionTimeLimit();
    fn StartBoundary();
    fn SetStartBoundary();
    fn EndBoundary();
    fn SetEndBoundary();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerVtbl {
        unsafe extern "system" fn Type<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Id<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetId<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Repetition<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepeat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRepetition<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prepeat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartBoundary<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartBoundary<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndBoundary<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEndBoundary<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: ITriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITriggerCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Create();
    fn Remove();
    fn Clear();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITriggerCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ITriggerCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Create::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWeeklyTriggerImpl: Sized + ITriggerImpl + IDispatchImpl {
    fn DaysOfWeek();
    fn SetDaysOfWeek();
    fn WeeksInterval();
    fn SetWeeksInterval();
    fn RandomDelay();
    fn SetRandomDelay();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWeeklyTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWeeklyTriggerVtbl {
        unsafe extern "system" fn DaysOfWeek<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WeeksInterval<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWeeksInterval<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RandomDelay<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IWeeklyTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Id::<Impl, IMPL_OFFSET>,
            SetId::<Impl, IMPL_OFFSET>,
            Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeeksInterval::<Impl, IMPL_OFFSET>,
            SetWeeksInterval::<Impl, IMPL_OFFSET>,
            RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeeklyTrigger as ::windows::core::Interface>::IID
    }
}
