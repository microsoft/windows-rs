#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&mut self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAction_Vtbl {
        unsafe extern "system" fn Id<Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Type<Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IActionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IAction>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn XmlText(&mut self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&mut self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Create(&mut self, r#type: TASK_ACTION_TYPE) -> ::windows::core::Result<IAction>;
    fn Remove(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Context(&mut self, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetContext(&mut self, context: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IActionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActionCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlText<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn Create<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Context<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Context(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetContext<Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            XmlText: XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText: SetXmlText::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBootTrigger_Vtbl {
        unsafe extern "system" fn Delay<Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComHandlerAction_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl {
    fn ClassId(&mut self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClassId(&mut self, clsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Data(&mut self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetData(&mut self, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComHandlerAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComHandlerAction_Vtbl {
        unsafe extern "system" fn ClassId<Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClassId(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetClassId<Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassId(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Data<Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClassId: ClassId::<Impl, IMPL_OFFSET>,
            SetClassId: SetClassId::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComHandlerAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDailyTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn DaysInterval(&mut self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysInterval(&mut self, days: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&mut self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&mut self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDailyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDailyTrigger_Vtbl {
        unsafe extern "system" fn DaysInterval<Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DaysInterval(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysInterval<Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysInterval(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn RandomDelay<Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DaysInterval: DaysInterval::<Impl, IMPL_OFFSET>,
            SetDaysInterval: SetDaysInterval::<Impl, IMPL_OFFSET>,
            RandomDelay: RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay: SetRandomDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDailyTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEmailAction_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl {
    fn Server(&mut self, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetServer(&mut self, server: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Subject(&mut self, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSubject(&mut self, subject: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn To(&mut self, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTo(&mut self, to: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Cc(&mut self, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetCc(&mut self, cc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Bcc(&mut self, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetBcc(&mut self, bcc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReplyTo(&mut self, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetReplyTo(&mut self, replyto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn From(&mut self, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetFrom(&mut self, from: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HeaderFields(&mut self) -> ::windows::core::Result<ITaskNamedValueCollection>;
    fn SetHeaderFields(&mut self, pheaderfields: &::core::option::Option<ITaskNamedValueCollection>) -> ::windows::core::Result<()>;
    fn Body(&mut self, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetBody(&mut self, body: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Attachments(&mut self, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn SetAttachments(&mut self, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEmailAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmailAction_Vtbl {
        unsafe extern "system" fn Server<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Server(::core::mem::transmute_copy(&pserver)).into()
        }
        unsafe extern "system" fn SetServer<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn Subject<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Subject(::core::mem::transmute_copy(&psubject)).into()
        }
        unsafe extern "system" fn SetSubject<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&subject)).into()
        }
        unsafe extern "system" fn To<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).To(::core::mem::transmute_copy(&pto)).into()
        }
        unsafe extern "system" fn SetTo<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(::core::mem::transmute_copy(&to)).into()
        }
        unsafe extern "system" fn Cc<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cc(::core::mem::transmute_copy(&pcc)).into()
        }
        unsafe extern "system" fn SetCc<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCc(::core::mem::transmute_copy(&cc)).into()
        }
        unsafe extern "system" fn Bcc<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bcc(::core::mem::transmute_copy(&pbcc)).into()
        }
        unsafe extern "system" fn SetBcc<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBcc(::core::mem::transmute_copy(&bcc)).into()
        }
        unsafe extern "system" fn ReplyTo<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplyTo(::core::mem::transmute_copy(&preplyto)).into()
        }
        unsafe extern "system" fn SetReplyTo<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReplyTo(::core::mem::transmute_copy(&replyto)).into()
        }
        unsafe extern "system" fn From<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).From(::core::mem::transmute_copy(&pfrom)).into()
        }
        unsafe extern "system" fn SetFrom<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(::core::mem::transmute_copy(&from)).into()
        }
        unsafe extern "system" fn HeaderFields<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderfields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderFields() {
                ::core::result::Result::Ok(ok__) => {
                    *ppheaderfields = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderFields<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaderfields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderFields(::core::mem::transmute(&pheaderfields)).into()
        }
        unsafe extern "system" fn Body<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Body(::core::mem::transmute_copy(&pbody)).into()
        }
        unsafe extern "system" fn SetBody<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&body)).into()
        }
        unsafe extern "system" fn Attachments<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Attachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        unsafe extern "system" fn SetAttachments<Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Server: Server::<Impl, IMPL_OFFSET>,
            SetServer: SetServer::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            To: To::<Impl, IMPL_OFFSET>,
            SetTo: SetTo::<Impl, IMPL_OFFSET>,
            Cc: Cc::<Impl, IMPL_OFFSET>,
            SetCc: SetCc::<Impl, IMPL_OFFSET>,
            Bcc: Bcc::<Impl, IMPL_OFFSET>,
            SetBcc: SetBcc::<Impl, IMPL_OFFSET>,
            ReplyTo: ReplyTo::<Impl, IMPL_OFFSET>,
            SetReplyTo: SetReplyTo::<Impl, IMPL_OFFSET>,
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            HeaderFields: HeaderFields::<Impl, IMPL_OFFSET>,
            SetHeaderFields: SetHeaderFields::<Impl, IMPL_OFFSET>,
            Body: Body::<Impl, IMPL_OFFSET>,
            SetBody: SetBody::<Impl, IMPL_OFFSET>,
            Attachments: Attachments::<Impl, IMPL_OFFSET>,
            SetAttachments: SetAttachments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumWorkItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgpwsznames: *mut *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&mut self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumWorkItems>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumWorkItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWorkItems_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgpwsznames), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumworkitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWorkItems as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEventTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Subscription(&mut self, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSubscription(&mut self, query: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ValueQueries(&mut self) -> ::windows::core::Result<ITaskNamedValueCollection>;
    fn SetValueQueries(&mut self, pnamedxpaths: &::core::option::Option<ITaskNamedValueCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventTrigger_Vtbl {
        unsafe extern "system" fn Subscription<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Subscription(::core::mem::transmute_copy(&pquery)).into()
        }
        unsafe extern "system" fn SetSubscription<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscription(::core::mem::transmute_copy(&query)).into()
        }
        unsafe extern "system" fn Delay<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn ValueQueries<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueQueries() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamedxpaths = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueQueries<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamedxpaths: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueQueries(::core::mem::transmute(&pnamedxpaths)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Subscription: Subscription::<Impl, IMPL_OFFSET>,
            SetSubscription: SetSubscription::<Impl, IMPL_OFFSET>,
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            ValueQueries: ValueQueries::<Impl, IMPL_OFFSET>,
            SetValueQueries: SetValueQueries::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExecAction_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl {
    fn Path(&mut self, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPath(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Arguments(&mut self, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetArguments(&mut self, argument: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&mut self, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWorkingDirectory(&mut self, workingdirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecAction_Vtbl {
        unsafe extern "system" fn Path<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Path(::core::mem::transmute_copy(&ppath)).into()
        }
        unsafe extern "system" fn SetPath<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn Arguments<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Arguments(::core::mem::transmute_copy(&pargument)).into()
        }
        unsafe extern "system" fn SetArguments<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetArguments(::core::mem::transmute_copy(&argument)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WorkingDirectory(::core::mem::transmute_copy(&pworkingdirectory)).into()
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&workingdirectory)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            Arguments: Arguments::<Impl, IMPL_OFFSET>,
            SetArguments: SetArguments::<Impl, IMPL_OFFSET>,
            WorkingDirectory: WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExecAction2_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl + IExecAction_Impl {
    fn HideAppWindow(&mut self, phideappwindow: *mut i16) -> ::windows::core::Result<()>;
    fn SetHideAppWindow(&mut self, hideappwindow: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecAction2_Vtbl {
        unsafe extern "system" fn HideAppWindow<Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HideAppWindow(::core::mem::transmute_copy(&phideappwindow)).into()
        }
        unsafe extern "system" fn SetHideAppWindow<Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHideAppWindow(::core::mem::transmute_copy(&hideappwindow)).into()
        }
        Self {
            base: IExecAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HideAppWindow: HideAppWindow::<Impl, IMPL_OFFSET>,
            SetHideAppWindow: SetHideAppWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIdleSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn IdleDuration(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetIdleDuration(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WaitTimeout(&mut self, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWaitTimeout(&mut self, timeout: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopOnIdleEnd(&mut self, pstop: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopOnIdleEnd(&mut self, stop: i16) -> ::windows::core::Result<()>;
    fn RestartOnIdle(&mut self, prestart: *mut i16) -> ::windows::core::Result<()>;
    fn SetRestartOnIdle(&mut self, restart: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdleSettings_Vtbl {
        unsafe extern "system" fn IdleDuration<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IdleDuration(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetIdleDuration<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdleDuration(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn WaitTimeout<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitTimeout(::core::mem::transmute_copy(&ptimeout)).into()
        }
        unsafe extern "system" fn SetWaitTimeout<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWaitTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn StopOnIdleEnd<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopOnIdleEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopOnIdleEnd(::core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn RestartOnIdle<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartOnIdle(::core::mem::transmute_copy(&prestart)).into()
        }
        unsafe extern "system" fn SetRestartOnIdle<Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestartOnIdle(::core::mem::transmute_copy(&restart)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IdleDuration: IdleDuration::<Impl, IMPL_OFFSET>,
            SetIdleDuration: SetIdleDuration::<Impl, IMPL_OFFSET>,
            WaitTimeout: WaitTimeout::<Impl, IMPL_OFFSET>,
            SetWaitTimeout: SetWaitTimeout::<Impl, IMPL_OFFSET>,
            StopOnIdleEnd: StopOnIdleEnd::<Impl, IMPL_OFFSET>,
            SetStopOnIdleEnd: SetStopOnIdleEnd::<Impl, IMPL_OFFSET>,
            RestartOnIdle: RestartOnIdle::<Impl, IMPL_OFFSET>,
            SetRestartOnIdle: SetRestartOnIdle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIdleTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdleTrigger_Vtbl {
        Self { base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILogonTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&mut self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&mut self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILogonTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogonTrigger_Vtbl {
        unsafe extern "system" fn Delay<Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn UserId<Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            UserId: UserId::<Impl, IMPL_OFFSET>,
            SetUserId: SetUserId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogonTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMaintenanceSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetPeriod(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Period(&mut self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDeadline(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Deadline(&mut self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExclusive(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Exclusive(&mut self, target: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMaintenanceSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMaintenanceSettings_Vtbl {
        unsafe extern "system" fn SetPeriod<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeriod(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Period<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Period(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetDeadline<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeadline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Deadline<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deadline(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetExclusive<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusive(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Exclusive<Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Exclusive(::core::mem::transmute_copy(&target)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPeriod: SetPeriod::<Impl, IMPL_OFFSET>,
            Period: Period::<Impl, IMPL_OFFSET>,
            SetDeadline: SetDeadline::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            SetExclusive: SetExclusive::<Impl, IMPL_OFFSET>,
            Exclusive: Exclusive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMaintenanceSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMonthlyDOWTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn DaysOfWeek(&mut self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysOfWeek(&mut self, days: i16) -> ::windows::core::Result<()>;
    fn WeeksOfMonth(&mut self, pweeks: *mut i16) -> ::windows::core::Result<()>;
    fn SetWeeksOfMonth(&mut self, weeks: i16) -> ::windows::core::Result<()>;
    fn MonthsOfYear(&mut self, pmonths: *mut i16) -> ::windows::core::Result<()>;
    fn SetMonthsOfYear(&mut self, months: i16) -> ::windows::core::Result<()>;
    fn RunOnLastWeekOfMonth(&mut self, plastweek: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnLastWeekOfMonth(&mut self, lastweek: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&mut self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&mut self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyDOWTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonthlyDOWTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksOfMonth<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WeeksOfMonth(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksOfMonth<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeeksOfMonth(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunOnLastWeekOfMonth(::core::mem::transmute_copy(&plastweek)).into()
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunOnLastWeekOfMonth(::core::mem::transmute_copy(&lastweek)).into()
        }
        unsafe extern "system" fn RandomDelay<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeeksOfMonth: WeeksOfMonth::<Impl, IMPL_OFFSET>,
            SetWeeksOfMonth: SetWeeksOfMonth::<Impl, IMPL_OFFSET>,
            MonthsOfYear: MonthsOfYear::<Impl, IMPL_OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Impl, IMPL_OFFSET>,
            RunOnLastWeekOfMonth: RunOnLastWeekOfMonth::<Impl, IMPL_OFFSET>,
            SetRunOnLastWeekOfMonth: SetRunOnLastWeekOfMonth::<Impl, IMPL_OFFSET>,
            RandomDelay: RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay: SetRandomDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMonthlyTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn DaysOfMonth(&mut self, pdays: *mut i32) -> ::windows::core::Result<()>;
    fn SetDaysOfMonth(&mut self, days: i32) -> ::windows::core::Result<()>;
    fn MonthsOfYear(&mut self, pmonths: *mut i16) -> ::windows::core::Result<()>;
    fn SetMonthsOfYear(&mut self, months: i16) -> ::windows::core::Result<()>;
    fn RunOnLastDayOfMonth(&mut self, plastday: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnLastDayOfMonth(&mut self, lastday: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&mut self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&mut self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonthlyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfMonth<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DaysOfMonth(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfMonth<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysOfMonth(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunOnLastDayOfMonth(::core::mem::transmute_copy(&plastday)).into()
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunOnLastDayOfMonth(::core::mem::transmute_copy(&lastday)).into()
        }
        unsafe extern "system" fn RandomDelay<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DaysOfMonth: DaysOfMonth::<Impl, IMPL_OFFSET>,
            SetDaysOfMonth: SetDaysOfMonth::<Impl, IMPL_OFFSET>,
            MonthsOfYear: MonthsOfYear::<Impl, IMPL_OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Impl, IMPL_OFFSET>,
            RunOnLastDayOfMonth: RunOnLastDayOfMonth::<Impl, IMPL_OFFSET>,
            SetRunOnLastDayOfMonth: SetRunOnLastDayOfMonth::<Impl, IMPL_OFFSET>,
            RandomDelay: RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay: SetRandomDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INetworkSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Id(&mut self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkSettings_Vtbl {
        unsafe extern "system" fn Name<Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Id<Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrincipal_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&mut self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDisplayName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&mut self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&mut self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogonType(&mut self, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::Result<()>;
    fn SetLogonType(&mut self, logon: TASK_LOGON_TYPE) -> ::windows::core::Result<()>;
    fn GroupId(&mut self, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetGroupId(&mut self, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RunLevel(&mut self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()>;
    fn SetRunLevel(&mut self, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrincipal_Vtbl {
        unsafe extern "system" fn Id<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayName(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn UserId<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        unsafe extern "system" fn LogonType<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogonType(::core::mem::transmute_copy(&plogon)).into()
        }
        unsafe extern "system" fn SetLogonType<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogonType(::core::mem::transmute_copy(&logon)).into()
        }
        unsafe extern "system" fn GroupId<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GroupId(::core::mem::transmute_copy(&pgroup)).into()
        }
        unsafe extern "system" fn SetGroupId<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupId(::core::mem::transmute_copy(&group)).into()
        }
        unsafe extern "system" fn RunLevel<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunLevel(::core::mem::transmute_copy(&prunlevel)).into()
        }
        unsafe extern "system" fn SetRunLevel<Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunLevel(::core::mem::transmute_copy(&runlevel)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            UserId: UserId::<Impl, IMPL_OFFSET>,
            SetUserId: SetUserId::<Impl, IMPL_OFFSET>,
            LogonType: LogonType::<Impl, IMPL_OFFSET>,
            SetLogonType: SetLogonType::<Impl, IMPL_OFFSET>,
            GroupId: GroupId::<Impl, IMPL_OFFSET>,
            SetGroupId: SetGroupId::<Impl, IMPL_OFFSET>,
            RunLevel: RunLevel::<Impl, IMPL_OFFSET>,
            SetRunLevel: SetRunLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPrincipal2_Impl: Sized + super::Com::IDispatch_Impl {
    fn ProcessTokenSidType(&mut self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()>;
    fn SetProcessTokenSidType(&mut self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()>;
    fn RequiredPrivilegeCount(&mut self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn RequiredPrivilege(&mut self, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddRequiredPrivilege(&mut self, privilege: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipal2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPrincipal2_Vtbl {
        unsafe extern "system" fn ProcessTokenSidType<Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessTokenSidType(::core::mem::transmute_copy(&pprocesstokensidtype)).into()
        }
        unsafe extern "system" fn SetProcessTokenSidType<Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessTokenSidType(::core::mem::transmute_copy(&processtokensidtype)).into()
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequiredPrivilegeCount(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn RequiredPrivilege<Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequiredPrivilege(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pprivilege)).into()
        }
        unsafe extern "system" fn AddRequiredPrivilege<Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRequiredPrivilege(::core::mem::transmute_copy(&privilege)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProcessTokenSidType: ProcessTokenSidType::<Impl, IMPL_OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Impl, IMPL_OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Impl, IMPL_OFFSET>,
            RequiredPrivilege: RequiredPrivilege::<Impl, IMPL_OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait IProvideTaskPage_Impl: Sized {
    fn GetPage(&mut self, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IProvideTaskPage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideTaskPage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvideTaskPage_Vtbl {
        unsafe extern "system" fn GetPage<Impl: IProvideTaskPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPage(::core::mem::transmute_copy(&tptype), ::core::mem::transmute_copy(&fpersistchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *phpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPage: GetPage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideTaskPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegisteredTask_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<TASK_STATE>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn Run(&mut self, params: &super::Com::VARIANT) -> ::windows::core::Result<IRunningTask>;
    fn RunEx(&mut self, params: &super::Com::VARIANT, flags: i32, sessionid: i32, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRunningTask>;
    fn GetInstances(&mut self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection>;
    fn LastRunTime(&mut self) -> ::windows::core::Result<f64>;
    fn LastTaskResult(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfMissedRuns(&mut self) -> ::windows::core::Result<i32>;
    fn NextRunTime(&mut self) -> ::windows::core::Result<f64>;
    fn Definition(&mut self) -> ::windows::core::Result<ITaskDefinition>;
    fn Xml(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSecurityDescriptor(&mut self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSecurityDescriptor(&mut self, sddl: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn Stop(&mut self, flags: i32) -> ::windows::core::Result<()>;
    fn GetRunTimes(&mut self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredTask_Vtbl {
        unsafe extern "system" fn Name<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *penabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Run<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Run(::core::mem::transmute_copy(&params)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunEx<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunEx(::core::mem::transmute_copy(&params), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&user)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstances<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstances(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRunTime<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plastruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTaskResult<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastTaskResult() {
                ::core::result::Result::Ok(ok__) => {
                    *plasttaskresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfMissedRuns<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfMissedRuns() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumberofmissedruns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRunTime<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pnextruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Definition<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Definition() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *pxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *psddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Stop<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetRunTimes<Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRunTimes(::core::mem::transmute_copy(&pststart), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&pruntimes)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            RunEx: RunEx::<Impl, IMPL_OFFSET>,
            GetInstances: GetInstances::<Impl, IMPL_OFFSET>,
            LastRunTime: LastRunTime::<Impl, IMPL_OFFSET>,
            LastTaskResult: LastTaskResult::<Impl, IMPL_OFFSET>,
            NumberOfMissedRuns: NumberOfMissedRuns::<Impl, IMPL_OFFSET>,
            NextRunTime: NextRunTime::<Impl, IMPL_OFFSET>,
            Definition: Definition::<Impl, IMPL_OFFSET>,
            Xml: Xml::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            GetRunTimes: GetRunTimes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegisteredTaskCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTaskCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisteredTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppregisteredtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Description(&mut self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&mut self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetAuthor(&mut self, author: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetVersion(&mut self, version: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Date(&mut self, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDate(&mut self, date: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Documentation(&mut self, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDocumentation(&mut self, documentation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn XmlText(&mut self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&mut self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn URI(&mut self, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetURI(&mut self, uri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SecurityDescriptor(&mut self, psddl: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetSecurityDescriptor(&mut self, sddl: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Source(&mut self, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSource(&mut self, source: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegistrationInfo_Vtbl {
        unsafe extern "system" fn Description<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Author<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn SetAuthor<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthor(::core::mem::transmute_copy(&author)).into()
        }
        unsafe extern "system" fn Version<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn SetVersion<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&version)).into()
        }
        unsafe extern "system" fn Date<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Date(::core::mem::transmute_copy(&pdate)).into()
        }
        unsafe extern "system" fn SetDate<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDate(::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn Documentation<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Documentation(::core::mem::transmute_copy(&pdocumentation)).into()
        }
        unsafe extern "system" fn SetDocumentation<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDocumentation(::core::mem::transmute_copy(&documentation)).into()
        }
        unsafe extern "system" fn XmlText<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn URI<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).URI(::core::mem::transmute_copy(&puri)).into()
        }
        unsafe extern "system" fn SetURI<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetURI(::core::mem::transmute_copy(&uri)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SecurityDescriptor(::core::mem::transmute_copy(&psddl)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl)).into()
        }
        unsafe extern "system" fn Source<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Source(::core::mem::transmute_copy(&psource)).into()
        }
        unsafe extern "system" fn SetSource<Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(::core::mem::transmute_copy(&source)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
            SetAuthor: SetAuthor::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            SetDate: SetDate::<Impl, IMPL_OFFSET>,
            Documentation: Documentation::<Impl, IMPL_OFFSET>,
            SetDocumentation: SetDocumentation::<Impl, IMPL_OFFSET>,
            XmlText: XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText: SetXmlText::<Impl, IMPL_OFFSET>,
            URI: URI::<Impl, IMPL_OFFSET>,
            SetURI: SetURI::<Impl, IMPL_OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegistrationTrigger_Vtbl {
        unsafe extern "system" fn Delay<Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRepetitionPattern_Impl: Sized + super::Com::IDispatch_Impl {
    fn Interval(&mut self, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInterval(&mut self, interval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Duration(&mut self, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDuration(&mut self, duration: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopAtDurationEnd(&mut self, pstop: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopAtDurationEnd(&mut self, stop: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRepetitionPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepetitionPattern_Vtbl {
        unsafe extern "system" fn Interval<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Interval(::core::mem::transmute_copy(&pinterval)).into()
        }
        unsafe extern "system" fn SetInterval<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn Duration<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn SetDuration<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn StopAtDurationEnd<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAtDurationEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopAtDurationEnd(::core::mem::transmute_copy(&stop)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            StopAtDurationEnd: StopAtDurationEnd::<Impl, IMPL_OFFSET>,
            SetStopAtDurationEnd: SetStopAtDurationEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepetitionPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRunningTask_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InstanceGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<TASK_STATE>;
    fn CurrentAction(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn EnginePID(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunningTask_Vtbl {
        unsafe extern "system" fn Name<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceGuid<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Refresh<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn EnginePID<Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnginePID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            InstanceGuid: InstanceGuid::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            CurrentAction: CurrentAction::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            EnginePID: EnginePID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRunningTaskCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IRunningTask>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTaskCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunningTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTaskCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IScheduledWorkItem_Impl: Sized {
    fn CreateTrigger(&mut self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows::core::Result<()>;
    fn DeleteTrigger(&mut self, itrigger: u16) -> ::windows::core::Result<()>;
    fn GetTriggerCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetTrigger(&mut self, itrigger: u16) -> ::windows::core::Result<ITaskTrigger>;
    fn GetTriggerString(&mut self, itrigger: u16) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetRunTimes(&mut self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn GetNextRunTime(&mut self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn SetIdleWait(&mut self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::Result<()>;
    fn GetIdleWait(&mut self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::Result<()>;
    fn Run(&mut self) -> ::windows::core::Result<()>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
    fn EditWorkItem(&mut self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetMostRecentRunTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetStatus(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetExitCode(&mut self) -> ::windows::core::Result<u32>;
    fn SetComment(&mut self, pwszcomment: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetComment(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetCreator(&mut self, pwszcreator: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetCreator(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetWorkItemData(&mut self, cbdata: u16, rgbdata: *const u8) -> ::windows::core::Result<()>;
    fn GetWorkItemData(&mut self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetErrorRetryCount(&mut self, wretrycount: u16) -> ::windows::core::Result<()>;
    fn GetErrorRetryCount(&mut self) -> ::windows::core::Result<u16>;
    fn SetErrorRetryInterval(&mut self, wretryinterval: u16) -> ::windows::core::Result<()>;
    fn GetErrorRetryInterval(&mut self) -> ::windows::core::Result<u16>;
    fn SetFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetAccountInformation(&mut self, pwszaccountname: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetAccountInformation(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IScheduledWorkItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduledWorkItem_Vtbl {
        unsafe extern "system" fn CreateTrigger<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateTrigger(::core::mem::transmute_copy(&pinewtrigger), ::core::mem::transmute_copy(&pptrigger)).into()
        }
        unsafe extern "system" fn DeleteTrigger<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTrigger(::core::mem::transmute_copy(&itrigger)).into()
        }
        unsafe extern "system" fn GetTriggerCount<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTriggerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrigger<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrigger(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTriggerString(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwsztrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunTimes<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRunTimes(::core::mem::transmute_copy(&pstbegin), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&rgsttasktimes)).into()
        }
        unsafe extern "system" fn GetNextRunTime<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextRunTime(::core::mem::transmute_copy(&pstnextrun)).into()
        }
        unsafe extern "system" fn SetIdleWait<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdleWait(::core::mem::transmute_copy(&widleminutes), ::core::mem::transmute_copy(&wdeadlineminutes)).into()
        }
        unsafe extern "system" fn GetIdleWait<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdleWait(::core::mem::transmute_copy(&pwidleminutes), ::core::mem::transmute_copy(&pwdeadlineminutes)).into()
        }
        unsafe extern "system" fn Run<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run().into()
        }
        unsafe extern "system" fn Terminate<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn EditWorkItem<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EditWorkItem(::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetMostRecentRunTime<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMostRecentRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastrun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *phrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitCode<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwexitcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn GetComment<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcreator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreator(::core::mem::transmute_copy(&pwszcreator)).into()
        }
        unsafe extern "system" fn GetCreator<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcreator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkItemData<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkItemData(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&rgbdata)).into()
        }
        unsafe extern "system" fn GetWorkItemData<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWorkItemData(::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&prgbdata)).into()
        }
        unsafe extern "system" fn SetErrorRetryCount<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorRetryCount(::core::mem::transmute_copy(&wretrycount)).into()
        }
        unsafe extern "system" fn GetErrorRetryCount<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorRetryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwretrycount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorRetryInterval<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorRetryInterval(::core::mem::transmute_copy(&wretryinterval)).into()
        }
        unsafe extern "system" fn GetErrorRetryInterval<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorRetryInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pwretryinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFlags<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountInformation<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszaccountname: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountInformation(::core::mem::transmute_copy(&pwszaccountname), ::core::mem::transmute_copy(&pwszpassword)).into()
        }
        unsafe extern "system" fn GetAccountInformation<Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccountInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszaccountname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTrigger: CreateTrigger::<Impl, IMPL_OFFSET>,
            DeleteTrigger: DeleteTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerCount: GetTriggerCount::<Impl, IMPL_OFFSET>,
            GetTrigger: GetTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerString: GetTriggerString::<Impl, IMPL_OFFSET>,
            GetRunTimes: GetRunTimes::<Impl, IMPL_OFFSET>,
            GetNextRunTime: GetNextRunTime::<Impl, IMPL_OFFSET>,
            SetIdleWait: SetIdleWait::<Impl, IMPL_OFFSET>,
            GetIdleWait: GetIdleWait::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            EditWorkItem: EditWorkItem::<Impl, IMPL_OFFSET>,
            GetMostRecentRunTime: GetMostRecentRunTime::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetExitCode: GetExitCode::<Impl, IMPL_OFFSET>,
            SetComment: SetComment::<Impl, IMPL_OFFSET>,
            GetComment: GetComment::<Impl, IMPL_OFFSET>,
            SetCreator: SetCreator::<Impl, IMPL_OFFSET>,
            GetCreator: GetCreator::<Impl, IMPL_OFFSET>,
            SetWorkItemData: SetWorkItemData::<Impl, IMPL_OFFSET>,
            GetWorkItemData: GetWorkItemData::<Impl, IMPL_OFFSET>,
            SetErrorRetryCount: SetErrorRetryCount::<Impl, IMPL_OFFSET>,
            GetErrorRetryCount: GetErrorRetryCount::<Impl, IMPL_OFFSET>,
            SetErrorRetryInterval: SetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            GetErrorRetryInterval: GetErrorRetryInterval::<Impl, IMPL_OFFSET>,
            SetFlags: SetFlags::<Impl, IMPL_OFFSET>,
            GetFlags: GetFlags::<Impl, IMPL_OFFSET>,
            SetAccountInformation: SetAccountInformation::<Impl, IMPL_OFFSET>,
            GetAccountInformation: GetAccountInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduledWorkItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISessionStateChangeTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&mut self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&mut self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StateChange(&mut self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()>;
    fn SetStateChange(&mut self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISessionStateChangeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISessionStateChangeTrigger_Vtbl {
        unsafe extern "system" fn Delay<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn UserId<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        unsafe extern "system" fn StateChange<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StateChange(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn SetStateChange<Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStateChange(::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            UserId: UserId::<Impl, IMPL_OFFSET>,
            SetUserId: SetUserId::<Impl, IMPL_OFFSET>,
            StateChange: StateChange::<Impl, IMPL_OFFSET>,
            SetStateChange: SetStateChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IShowMessageAction_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl {
    fn Title(&mut self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTitle(&mut self, title: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MessageBody(&mut self, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetMessageBody(&mut self, messagebody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IShowMessageAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShowMessageAction_Vtbl {
        unsafe extern "system" fn Title<Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn SetTitle<Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn MessageBody<Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MessageBody(::core::mem::transmute_copy(&pmessagebody)).into()
        }
        unsafe extern "system" fn SetMessageBody<Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageBody(::core::mem::transmute_copy(&messagebody)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            MessageBody: MessageBody::<Impl, IMPL_OFFSET>,
            SetMessageBody: SetMessageBody::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShowMessageAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITask_Impl: Sized + IScheduledWorkItem_Impl {
    fn SetApplicationName(&mut self, pwszapplicationname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetApplicationName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetParameters(&mut self, pwszparameters: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetParameters(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetWorkingDirectory(&mut self, pwszworkingdirectory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetWorkingDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetPriority(&mut self, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self) -> ::windows::core::Result<u32>;
    fn SetTaskFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetTaskFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxRunTime(&mut self, dwmaxruntimems: u32) -> ::windows::core::Result<()>;
    fn GetMaxRunTime(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITask_Vtbl {
        unsafe extern "system" fn SetApplicationName<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszapplicationname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationName(::core::mem::transmute_copy(&pwszapplicationname)).into()
        }
        unsafe extern "system" fn GetApplicationName<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszapplicationname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszparameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&pwszparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszworkingdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&pwszworkingdirectory)).into()
        }
        unsafe extern "system" fn GetWorkingDirectory<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszworkingdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskFlags<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTaskFlags<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTaskFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRunTime<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxRunTime(::core::mem::transmute_copy(&dwmaxruntimems)).into()
        }
        unsafe extern "system" fn GetMaxRunTime<Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxruntimems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IScheduledWorkItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetApplicationName: SetApplicationName::<Impl, IMPL_OFFSET>,
            GetApplicationName: GetApplicationName::<Impl, IMPL_OFFSET>,
            SetParameters: SetParameters::<Impl, IMPL_OFFSET>,
            GetParameters: GetParameters::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            GetWorkingDirectory: GetWorkingDirectory::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
            SetTaskFlags: SetTaskFlags::<Impl, IMPL_OFFSET>,
            GetTaskFlags: GetTaskFlags::<Impl, IMPL_OFFSET>,
            SetMaxRunTime: SetMaxRunTime::<Impl, IMPL_OFFSET>,
            GetMaxRunTime: GetMaxRunTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskDefinition_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationInfo(&mut self) -> ::windows::core::Result<IRegistrationInfo>;
    fn SetRegistrationInfo(&mut self, pregistrationinfo: &::core::option::Option<IRegistrationInfo>) -> ::windows::core::Result<()>;
    fn Triggers(&mut self) -> ::windows::core::Result<ITriggerCollection>;
    fn SetTriggers(&mut self, ptriggers: &::core::option::Option<ITriggerCollection>) -> ::windows::core::Result<()>;
    fn Settings(&mut self) -> ::windows::core::Result<ITaskSettings>;
    fn SetSettings(&mut self, psettings: &::core::option::Option<ITaskSettings>) -> ::windows::core::Result<()>;
    fn Data(&mut self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetData(&mut self, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Principal(&mut self) -> ::windows::core::Result<IPrincipal>;
    fn SetPrincipal(&mut self, pprincipal: &::core::option::Option<IPrincipal>) -> ::windows::core::Result<()>;
    fn Actions(&mut self) -> ::windows::core::Result<IActionCollection>;
    fn SetActions(&mut self, pactions: &::core::option::Option<IActionCollection>) -> ::windows::core::Result<()>;
    fn XmlText(&mut self, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&mut self, xml: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskDefinition_Vtbl {
        unsafe extern "system" fn RegistrationInfo<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregistrationinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistrationInfo<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregistrationinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegistrationInfo(::core::mem::transmute(&pregistrationinfo)).into()
        }
        unsafe extern "system" fn Triggers<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptriggers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Triggers() {
                ::core::result::Result::Ok(ok__) => {
                    *pptriggers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggers<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptriggers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTriggers(::core::mem::transmute(&ptriggers)).into()
        }
        unsafe extern "system" fn Settings<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSettings(::core::mem::transmute(&psettings)).into()
        }
        unsafe extern "system" fn Data<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Principal<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprincipal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Principal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprincipal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrincipal<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprincipal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrincipal(::core::mem::transmute(&pprincipal)).into()
        }
        unsafe extern "system" fn Actions<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppactions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppactions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActions(::core::mem::transmute(&pactions)).into()
        }
        unsafe extern "system" fn XmlText<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&pxml)).into()
        }
        unsafe extern "system" fn SetXmlText<Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&xml)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegistrationInfo: RegistrationInfo::<Impl, IMPL_OFFSET>,
            SetRegistrationInfo: SetRegistrationInfo::<Impl, IMPL_OFFSET>,
            Triggers: Triggers::<Impl, IMPL_OFFSET>,
            SetTriggers: SetTriggers::<Impl, IMPL_OFFSET>,
            Settings: Settings::<Impl, IMPL_OFFSET>,
            SetSettings: SetSettings::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Principal: Principal::<Impl, IMPL_OFFSET>,
            SetPrincipal: SetPrincipal::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
            SetActions: SetActions::<Impl, IMPL_OFFSET>,
            XmlText: XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText: SetXmlText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskFolder_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFolder(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskFolder>;
    fn GetFolders(&mut self, flags: i32) -> ::windows::core::Result<ITaskFolderCollection>;
    fn CreateFolder(&mut self, subfoldername: &super::super::Foundation::BSTR, sddl: &super::Com::VARIANT) -> ::windows::core::Result<ITaskFolder>;
    fn DeleteFolder(&mut self, subfoldername: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn GetTask(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRegisteredTask>;
    fn GetTasks(&mut self, flags: i32) -> ::windows::core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(&mut self, name: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn RegisterTask(&mut self, path: &super::super::Foundation::BSTR, xmltext: &super::super::Foundation::BSTR, flags: i32, userid: &super::Com::VARIANT, password: &super::Com::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(&mut self, path: &super::super::Foundation::BSTR, pdefinition: &::core::option::Option<ITaskDefinition>, flags: i32, userid: &super::Com::VARIANT, password: &super::Com::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(&mut self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSecurityDescriptor(&mut self, sddl: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskFolder_Vtbl {
        unsafe extern "system" fn Name<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolders<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolders(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolder<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolder(::core::mem::transmute_copy(&subfoldername), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&subfoldername), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetTask<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTask(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn RegisterTask<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterTask(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&xmltext), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: ::windows::core::RawPtr, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterTaskDefinition(::core::mem::transmute_copy(&path), ::core::mem::transmute(&pdefinition), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *psddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            GetFolder: GetFolder::<Impl, IMPL_OFFSET>,
            GetFolders: GetFolders::<Impl, IMPL_OFFSET>,
            CreateFolder: CreateFolder::<Impl, IMPL_OFFSET>,
            DeleteFolder: DeleteFolder::<Impl, IMPL_OFFSET>,
            GetTask: GetTask::<Impl, IMPL_OFFSET>,
            GetTasks: GetTasks::<Impl, IMPL_OFFSET>,
            DeleteTask: DeleteTask::<Impl, IMPL_OFFSET>,
            RegisterTask: RegisterTask::<Impl, IMPL_OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Impl, IMPL_OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskFolderCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<ITaskFolder>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskFolderCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskHandler_Impl: Sized {
    fn Start(&mut self, phandlerservices: &::core::option::Option<::windows::core::IUnknown>, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskHandler_Vtbl {
        unsafe extern "system" fn Start<Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute(&phandlerservices), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Stop<Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *pretcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskHandlerStatus_Impl: Sized {
    fn UpdateStatus(&mut self, percentcomplete: i16, statusmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskCompleted(&mut self, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandlerStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskHandlerStatus_Vtbl {
        unsafe extern "system" fn UpdateStatus<Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateStatus(::core::mem::transmute_copy(&percentcomplete), ::core::mem::transmute_copy(&statusmessage)).into()
        }
        unsafe extern "system" fn TaskCompleted<Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TaskCompleted(::core::mem::transmute_copy(&taskerrcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            UpdateStatus: UpdateStatus::<Impl, IMPL_OFFSET>,
            TaskCompleted: TaskCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskHandlerStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskNamedValueCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ITaskNamedValuePair>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Create(&mut self, name: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskNamedValuePair>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskNamedValueCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppair = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppair = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskNamedValuePair_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValuePair_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskNamedValuePair_Vtbl {
        unsafe extern "system" fn Name<Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Value<Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Value(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskScheduler_Impl: Sized {
    fn SetTargetComputer(&mut self, pwszcomputer: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetTargetComputer(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Enum(&mut self) -> ::windows::core::Result<IEnumWorkItems>;
    fn Activate(&mut self, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Delete(&mut self, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NewWorkItem(&mut self, pwsztaskname: super::super::Foundation::PWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn AddWorkItem(&mut self, pwsztaskname: super::super::Foundation::PWSTR, pworkitem: &::core::option::Option<IScheduledWorkItem>) -> ::windows::core::Result<()>;
    fn IsOfType(&mut self, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskScheduler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskScheduler_Vtbl {
        unsafe extern "system" fn SetTargetComputer<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomputer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetComputer(::core::mem::transmute_copy(&pwszcomputer)).into()
        }
        unsafe extern "system" fn GetTargetComputer<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcomputer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumworkitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn NewWorkItem<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewWorkItem(::core::mem::transmute_copy(&pwsztaskname), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWorkItem<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, pworkitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddWorkItem(::core::mem::transmute_copy(&pwsztaskname), ::core::mem::transmute(&pworkitem)).into()
        }
        unsafe extern "system" fn IsOfType<Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsOfType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTargetComputer: SetTargetComputer::<Impl, IMPL_OFFSET>,
            GetTargetComputer: GetTargetComputer::<Impl, IMPL_OFFSET>,
            Enum: Enum::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            NewWorkItem: NewWorkItem::<Impl, IMPL_OFFSET>,
            AddWorkItem: AddWorkItem::<Impl, IMPL_OFFSET>,
            IsOfType: IsOfType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskScheduler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskService_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetFolder(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskFolder>;
    fn GetRunningTasks(&mut self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection>;
    fn NewTask(&mut self, flags: u32) -> ::windows::core::Result<ITaskDefinition>;
    fn Connect(&mut self, servername: &super::Com::VARIANT, user: &super::Com::VARIANT, domain: &super::Com::VARIANT, password: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Connected(&mut self) -> ::windows::core::Result<i16>;
    fn TargetServer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectedUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectedDomain(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HighestVersion(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskService_Vtbl {
        unsafe extern "system" fn GetFolder<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningTasks<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRunningTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewTask<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewTask(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&servername), ::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn Connected<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connected() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetServer<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedUser<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    *puser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedDomain<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectedDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pdomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighestVersion<Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetFolder: GetFolder::<Impl, IMPL_OFFSET>,
            GetRunningTasks: GetRunningTasks::<Impl, IMPL_OFFSET>,
            NewTask: NewTask::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Connected: Connected::<Impl, IMPL_OFFSET>,
            TargetServer: TargetServer::<Impl, IMPL_OFFSET>,
            ConnectedUser: ConnectedUser::<Impl, IMPL_OFFSET>,
            ConnectedDomain: ConnectedDomain::<Impl, IMPL_OFFSET>,
            HighestVersion: HighestVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn AllowDemandStart(&mut self, pallowdemandstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetAllowDemandStart(&mut self, allowdemandstart: i16) -> ::windows::core::Result<()>;
    fn RestartInterval(&mut self, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRestartInterval(&mut self, restartinterval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RestartCount(&mut self, prestartcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetRestartCount(&mut self, restartcount: i32) -> ::windows::core::Result<()>;
    fn MultipleInstances(&mut self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::Result<()>;
    fn SetMultipleInstances(&mut self, policy: TASK_INSTANCES_POLICY) -> ::windows::core::Result<()>;
    fn StopIfGoingOnBatteries(&mut self, pstopifonbatteries: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopIfGoingOnBatteries(&mut self, stopifonbatteries: i16) -> ::windows::core::Result<()>;
    fn DisallowStartIfOnBatteries(&mut self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartIfOnBatteries(&mut self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn AllowHardTerminate(&mut self, pallowhardterminate: *mut i16) -> ::windows::core::Result<()>;
    fn SetAllowHardTerminate(&mut self, allowhardterminate: i16) -> ::windows::core::Result<()>;
    fn StartWhenAvailable(&mut self, pstartwhenavailable: *mut i16) -> ::windows::core::Result<()>;
    fn SetStartWhenAvailable(&mut self, startwhenavailable: i16) -> ::windows::core::Result<()>;
    fn XmlText(&mut self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&mut self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RunOnlyIfNetworkAvailable(&mut self, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnlyIfNetworkAvailable(&mut self, runonlyifnetworkavailable: i16) -> ::windows::core::Result<()>;
    fn ExecutionTimeLimit(&mut self, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExecutionTimeLimit(&mut self, executiontimelimit: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self, penabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn DeleteExpiredTaskAfter(&mut self, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDeleteExpiredTaskAfter(&mut self, expirationdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Priority(&mut self, ppriority: *mut i32) -> ::windows::core::Result<()>;
    fn SetPriority(&mut self, priority: i32) -> ::windows::core::Result<()>;
    fn Compatibility(&mut self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::Result<()>;
    fn SetCompatibility(&mut self, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::Result<()>;
    fn Hidden(&mut self, phidden: *mut i16) -> ::windows::core::Result<()>;
    fn SetHidden(&mut self, hidden: i16) -> ::windows::core::Result<()>;
    fn IdleSettings(&mut self) -> ::windows::core::Result<IIdleSettings>;
    fn SetIdleSettings(&mut self, pidlesettings: &::core::option::Option<IIdleSettings>) -> ::windows::core::Result<()>;
    fn RunOnlyIfIdle(&mut self, prunonlyifidle: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnlyIfIdle(&mut self, runonlyifidle: i16) -> ::windows::core::Result<()>;
    fn WakeToRun(&mut self, pwake: *mut i16) -> ::windows::core::Result<()>;
    fn SetWakeToRun(&mut self, wake: i16) -> ::windows::core::Result<()>;
    fn NetworkSettings(&mut self) -> ::windows::core::Result<INetworkSettings>;
    fn SetNetworkSettings(&mut self, pnetworksettings: &::core::option::Option<INetworkSettings>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettings_Vtbl {
        unsafe extern "system" fn AllowDemandStart<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllowDemandStart(::core::mem::transmute_copy(&pallowdemandstart)).into()
        }
        unsafe extern "system" fn SetAllowDemandStart<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowDemandStart(::core::mem::transmute_copy(&allowdemandstart)).into()
        }
        unsafe extern "system" fn RestartInterval<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartInterval(::core::mem::transmute_copy(&prestartinterval)).into()
        }
        unsafe extern "system" fn SetRestartInterval<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestartInterval(::core::mem::transmute_copy(&restartinterval)).into()
        }
        unsafe extern "system" fn RestartCount<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartCount(::core::mem::transmute_copy(&prestartcount)).into()
        }
        unsafe extern "system" fn SetRestartCount<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRestartCount(::core::mem::transmute_copy(&restartcount)).into()
        }
        unsafe extern "system" fn MultipleInstances<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MultipleInstances(::core::mem::transmute_copy(&ppolicy)).into()
        }
        unsafe extern "system" fn SetMultipleInstances<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultipleInstances(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopIfGoingOnBatteries(::core::mem::transmute_copy(&pstopifonbatteries)).into()
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopIfGoingOnBatteries(::core::mem::transmute_copy(&stopifonbatteries)).into()
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisallowStartIfOnBatteries(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisallowStartIfOnBatteries(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn AllowHardTerminate<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllowHardTerminate(::core::mem::transmute_copy(&pallowhardterminate)).into()
        }
        unsafe extern "system" fn SetAllowHardTerminate<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowHardTerminate(::core::mem::transmute_copy(&allowhardterminate)).into()
        }
        unsafe extern "system" fn StartWhenAvailable<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartWhenAvailable(::core::mem::transmute_copy(&pstartwhenavailable)).into()
        }
        unsafe extern "system" fn SetStartWhenAvailable<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartWhenAvailable(::core::mem::transmute_copy(&startwhenavailable)).into()
        }
        unsafe extern "system" fn XmlText<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&prunonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecutionTimeLimit(::core::mem::transmute_copy(&pexecutiontimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExecutionTimeLimit(::core::mem::transmute_copy(&executiontimelimit)).into()
        }
        unsafe extern "system" fn Enabled<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteExpiredTaskAfter(::core::mem::transmute_copy(&pexpirationdelay)).into()
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteExpiredTaskAfter(::core::mem::transmute_copy(&expirationdelay)).into()
        }
        unsafe extern "system" fn Priority<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Priority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetPriority<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Compatibility<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compatibility(::core::mem::transmute_copy(&pcompatlevel)).into()
        }
        unsafe extern "system" fn SetCompatibility<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompatibility(::core::mem::transmute_copy(&compatlevel)).into()
        }
        unsafe extern "system" fn Hidden<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hidden(::core::mem::transmute_copy(&phidden)).into()
        }
        unsafe extern "system" fn SetHidden<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHidden(::core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn IdleSettings<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidlesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdleSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidlesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdleSettings<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidlesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIdleSettings(::core::mem::transmute(&pidlesettings)).into()
        }
        unsafe extern "system" fn RunOnlyIfIdle<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunOnlyIfIdle(::core::mem::transmute_copy(&prunonlyifidle)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRunOnlyIfIdle(::core::mem::transmute_copy(&runonlyifidle)).into()
        }
        unsafe extern "system" fn WakeToRun<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WakeToRun(::core::mem::transmute_copy(&pwake)).into()
        }
        unsafe extern "system" fn SetWakeToRun<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWakeToRun(::core::mem::transmute_copy(&wake)).into()
        }
        unsafe extern "system" fn NetworkSettings<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworksettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSettings<Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkSettings(::core::mem::transmute(&pnetworksettings)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AllowDemandStart: AllowDemandStart::<Impl, IMPL_OFFSET>,
            SetAllowDemandStart: SetAllowDemandStart::<Impl, IMPL_OFFSET>,
            RestartInterval: RestartInterval::<Impl, IMPL_OFFSET>,
            SetRestartInterval: SetRestartInterval::<Impl, IMPL_OFFSET>,
            RestartCount: RestartCount::<Impl, IMPL_OFFSET>,
            SetRestartCount: SetRestartCount::<Impl, IMPL_OFFSET>,
            MultipleInstances: MultipleInstances::<Impl, IMPL_OFFSET>,
            SetMultipleInstances: SetMultipleInstances::<Impl, IMPL_OFFSET>,
            StopIfGoingOnBatteries: StopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            SetStopIfGoingOnBatteries: SetStopIfGoingOnBatteries::<Impl, IMPL_OFFSET>,
            DisallowStartIfOnBatteries: DisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            SetDisallowStartIfOnBatteries: SetDisallowStartIfOnBatteries::<Impl, IMPL_OFFSET>,
            AllowHardTerminate: AllowHardTerminate::<Impl, IMPL_OFFSET>,
            SetAllowHardTerminate: SetAllowHardTerminate::<Impl, IMPL_OFFSET>,
            StartWhenAvailable: StartWhenAvailable::<Impl, IMPL_OFFSET>,
            SetStartWhenAvailable: SetStartWhenAvailable::<Impl, IMPL_OFFSET>,
            XmlText: XmlText::<Impl, IMPL_OFFSET>,
            SetXmlText: SetXmlText::<Impl, IMPL_OFFSET>,
            RunOnlyIfNetworkAvailable: RunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfNetworkAvailable: SetRunOnlyIfNetworkAvailable::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            DeleteExpiredTaskAfter: DeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            SetDeleteExpiredTaskAfter: SetDeleteExpiredTaskAfter::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Compatibility: Compatibility::<Impl, IMPL_OFFSET>,
            SetCompatibility: SetCompatibility::<Impl, IMPL_OFFSET>,
            Hidden: Hidden::<Impl, IMPL_OFFSET>,
            SetHidden: SetHidden::<Impl, IMPL_OFFSET>,
            IdleSettings: IdleSettings::<Impl, IMPL_OFFSET>,
            SetIdleSettings: SetIdleSettings::<Impl, IMPL_OFFSET>,
            RunOnlyIfIdle: RunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            SetRunOnlyIfIdle: SetRunOnlyIfIdle::<Impl, IMPL_OFFSET>,
            WakeToRun: WakeToRun::<Impl, IMPL_OFFSET>,
            SetWakeToRun: SetWakeToRun::<Impl, IMPL_OFFSET>,
            NetworkSettings: NetworkSettings::<Impl, IMPL_OFFSET>,
            SetNetworkSettings: SetNetworkSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettings2_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisallowStartOnRemoteAppSession(&mut self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&mut self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn UseUnifiedSchedulingEngine(&mut self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&mut self, useunifiedengine: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettings2_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITaskSettings3_Impl: Sized + super::Com::IDispatch_Impl + ITaskSettings_Impl {
    fn DisallowStartOnRemoteAppSession(&mut self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&mut self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn UseUnifiedSchedulingEngine(&mut self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&mut self, useunifiedengine: i16) -> ::windows::core::Result<()>;
    fn MaintenanceSettings(&mut self) -> ::windows::core::Result<IMaintenanceSettings>;
    fn SetMaintenanceSettings(&mut self, pmaintenancesettings: &::core::option::Option<IMaintenanceSettings>) -> ::windows::core::Result<()>;
    fn CreateMaintenanceSettings(&mut self) -> ::windows::core::Result<IMaintenanceSettings>;
    fn Volatile(&mut self, pvolatile: *mut i16) -> ::windows::core::Result<()>;
    fn SetVolatile(&mut self, volatile: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskSettings3_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        unsafe extern "system" fn MaintenanceSettings<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmaintenancesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceSettings<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaintenancesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaintenanceSettings(::core::mem::transmute(&pmaintenancesettings)).into()
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmaintenancesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volatile<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Volatile(::core::mem::transmute_copy(&pvolatile)).into()
        }
        unsafe extern "system" fn SetVolatile<Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolatile(::core::mem::transmute_copy(&volatile)).into()
        }
        Self {
            base: ITaskSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Impl, IMPL_OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Impl, IMPL_OFFSET>,
            MaintenanceSettings: MaintenanceSettings::<Impl, IMPL_OFFSET>,
            SetMaintenanceSettings: SetMaintenanceSettings::<Impl, IMPL_OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Impl, IMPL_OFFSET>,
            Volatile: Volatile::<Impl, IMPL_OFFSET>,
            SetVolatile: SetVolatile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskTrigger_Impl: Sized {
    fn SetTrigger(&mut self, ptrigger: *const TASK_TRIGGER) -> ::windows::core::Result<()>;
    fn GetTrigger(&mut self) -> ::windows::core::Result<TASK_TRIGGER>;
    fn GetTriggerString(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskTrigger_Vtbl {
        unsafe extern "system" fn SetTrigger<Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrigger(::core::mem::transmute_copy(&ptrigger)).into()
        }
        unsafe extern "system" fn GetTrigger<Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *ptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTriggerString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwsztrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTrigger: SetTrigger::<Impl, IMPL_OFFSET>,
            GetTrigger: GetTrigger::<Impl, IMPL_OFFSET>,
            GetTriggerString: GetTriggerString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskVariables_Impl: Sized {
    fn GetInput(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOutput(&mut self, input: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetContext(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITaskVariables_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariables_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITaskVariables_Vtbl {
        unsafe extern "system" fn GetInput<Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInput() {
                ::core::result::Result::Ok(ok__) => {
                    *pinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutput<Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutput(::core::mem::transmute_copy(&input)).into()
        }
        unsafe extern "system" fn GetContext<Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInput: GetInput::<Impl, IMPL_OFFSET>,
            SetOutput: SetOutput::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskVariables as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITimeTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn RandomDelay(&mut self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&mut self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeTrigger_Vtbl {
        unsafe extern "system" fn RandomDelay<Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RandomDelay: RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay: SetRandomDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITrigger_Impl: Sized + super::Com::IDispatch_Impl {
    fn Type(&mut self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()>;
    fn Id(&mut self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Repetition(&mut self) -> ::windows::core::Result<IRepetitionPattern>;
    fn SetRepetition(&mut self, prepeat: &::core::option::Option<IRepetitionPattern>) -> ::windows::core::Result<()>;
    fn ExecutionTimeLimit(&mut self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExecutionTimeLimit(&mut self, timelimit: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartBoundary(&mut self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetStartBoundary(&mut self, start: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EndBoundary(&mut self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetEndBoundary(&mut self, end: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self, penabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITrigger_Vtbl {
        unsafe extern "system" fn Type<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Id<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Repetition<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepeat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Repetition() {
                ::core::result::Result::Ok(ok__) => {
                    *pprepeat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetition<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prepeat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepetition(::core::mem::transmute(&prepeat)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecutionTimeLimit(::core::mem::transmute_copy(&ptimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExecutionTimeLimit(::core::mem::transmute_copy(&timelimit)).into()
        }
        unsafe extern "system" fn StartBoundary<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartBoundary(::core::mem::transmute_copy(&pstart)).into()
        }
        unsafe extern "system" fn SetStartBoundary<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartBoundary(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn EndBoundary<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndBoundary(::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetEndBoundary<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndBoundary(::core::mem::transmute_copy(&end)).into()
        }
        unsafe extern "system" fn Enabled<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Repetition: Repetition::<Impl, IMPL_OFFSET>,
            SetRepetition: SetRepetition::<Impl, IMPL_OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Impl, IMPL_OFFSET>,
            StartBoundary: StartBoundary::<Impl, IMPL_OFFSET>,
            SetStartBoundary: SetStartBoundary::<Impl, IMPL_OFFSET>,
            EndBoundary: EndBoundary::<Impl, IMPL_OFFSET>,
            SetEndBoundary: SetEndBoundary::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITriggerCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ITrigger>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Create(&mut self, r#type: TASK_TRIGGER_TYPE2) -> ::windows::core::Result<ITrigger>;
    fn Remove(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITriggerCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWeeklyTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn DaysOfWeek(&mut self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysOfWeek(&mut self, days: i16) -> ::windows::core::Result<()>;
    fn WeeksInterval(&mut self, pweeks: *mut i16) -> ::windows::core::Result<()>;
    fn SetWeeksInterval(&mut self, weeks: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&mut self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&mut self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWeeklyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWeeklyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksInterval<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WeeksInterval(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksInterval<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeeksInterval(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn RandomDelay<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeeksInterval: WeeksInterval::<Impl, IMPL_OFFSET>,
            SetWeeksInterval: SetWeeksInterval::<Impl, IMPL_OFFSET>,
            RandomDelay: RandomDelay::<Impl, IMPL_OFFSET>,
            SetRandomDelay: SetRandomDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeeklyTrigger as ::windows::core::Interface>::IID
    }
}
