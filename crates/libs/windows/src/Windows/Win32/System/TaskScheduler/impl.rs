#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&mut self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&mut self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAction_Impl, const OFFSET: isize>() -> IAction_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>() -> IActionCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Context<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Context(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContext(::core::mem::transmute_copy(&context)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActionCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootTrigger_Impl, const OFFSET: isize>() -> IBootTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl, Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl, Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        Self { base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(), Delay: Delay::<Identity, Impl, OFFSET>, SetDelay: SetDelay::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const OFFSET: isize>() -> IComHandlerAction_Vtbl {
        unsafe extern "system" fn ClassId<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClassId(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetClassId<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClassId(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClassId: ClassId::<Identity, Impl, OFFSET>,
            SetClassId: SetClassId::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComHandlerAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAction as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const OFFSET: isize>() -> IDailyTrigger_Vtbl {
        unsafe extern "system" fn DaysInterval<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DaysInterval(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysInterval<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDaysInterval(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            DaysInterval: DaysInterval::<Identity, Impl, OFFSET>,
            SetDaysInterval: SetDaysInterval::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDailyTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>() -> IEmailAction_Vtbl {
        unsafe extern "system" fn Server<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Server(::core::mem::transmute_copy(&pserver)).into()
        }
        unsafe extern "system" fn SetServer<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Subject(::core::mem::transmute_copy(&psubject)).into()
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubject(::core::mem::transmute_copy(&subject)).into()
        }
        unsafe extern "system" fn To<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).To(::core::mem::transmute_copy(&pto)).into()
        }
        unsafe extern "system" fn SetTo<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTo(::core::mem::transmute_copy(&to)).into()
        }
        unsafe extern "system" fn Cc<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cc(::core::mem::transmute_copy(&pcc)).into()
        }
        unsafe extern "system" fn SetCc<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCc(::core::mem::transmute_copy(&cc)).into()
        }
        unsafe extern "system" fn Bcc<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Bcc(::core::mem::transmute_copy(&pbcc)).into()
        }
        unsafe extern "system" fn SetBcc<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBcc(::core::mem::transmute_copy(&bcc)).into()
        }
        unsafe extern "system" fn ReplyTo<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReplyTo(::core::mem::transmute_copy(&preplyto)).into()
        }
        unsafe extern "system" fn SetReplyTo<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReplyTo(::core::mem::transmute_copy(&replyto)).into()
        }
        unsafe extern "system" fn From<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).From(::core::mem::transmute_copy(&pfrom)).into()
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrom(::core::mem::transmute_copy(&from)).into()
        }
        unsafe extern "system" fn HeaderFields<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderfields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HeaderFields() {
                ::core::result::Result::Ok(ok__) => {
                    *ppheaderfields = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderFields<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaderfields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHeaderFields(::core::mem::transmute(&pheaderfields)).into()
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Body(::core::mem::transmute_copy(&pbody)).into()
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(::core::mem::transmute_copy(&body)).into()
        }
        unsafe extern "system" fn Attachments<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Attachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        unsafe extern "system" fn SetAttachments<Identity: ::windows::core::IUnknownImpl, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            Server: Server::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            Subject: Subject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            To: To::<Identity, Impl, OFFSET>,
            SetTo: SetTo::<Identity, Impl, OFFSET>,
            Cc: Cc::<Identity, Impl, OFFSET>,
            SetCc: SetCc::<Identity, Impl, OFFSET>,
            Bcc: Bcc::<Identity, Impl, OFFSET>,
            SetBcc: SetBcc::<Identity, Impl, OFFSET>,
            ReplyTo: ReplyTo::<Identity, Impl, OFFSET>,
            SetReplyTo: SetReplyTo::<Identity, Impl, OFFSET>,
            From: From::<Identity, Impl, OFFSET>,
            SetFrom: SetFrom::<Identity, Impl, OFFSET>,
            HeaderFields: HeaderFields::<Identity, Impl, OFFSET>,
            SetHeaderFields: SetHeaderFields::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            Attachments: Attachments::<Identity, Impl, OFFSET>,
            SetAttachments: SetAttachments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmailAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAction as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const OFFSET: isize>() -> IEnumWorkItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgpwsznames), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumworkitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>() -> IEventTrigger_Vtbl {
        unsafe extern "system" fn Subscription<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Subscription(::core::mem::transmute_copy(&pquery)).into()
        }
        unsafe extern "system" fn SetSubscription<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscription(::core::mem::transmute_copy(&query)).into()
        }
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn ValueQueries<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ValueQueries() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamedxpaths = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueQueries<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamedxpaths: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValueQueries(::core::mem::transmute(&pnamedxpaths)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            Subscription: Subscription::<Identity, Impl, OFFSET>,
            SetSubscription: SetSubscription::<Identity, Impl, OFFSET>,
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            ValueQueries: ValueQueries::<Identity, Impl, OFFSET>,
            SetValueQueries: SetValueQueries::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>() -> IExecAction_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Path(::core::mem::transmute_copy(&ppath)).into()
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn Arguments<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Arguments(::core::mem::transmute_copy(&pargument)).into()
        }
        unsafe extern "system" fn SetArguments<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetArguments(::core::mem::transmute_copy(&argument)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WorkingDirectory(::core::mem::transmute_copy(&pworkingdirectory)).into()
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&workingdirectory)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            Arguments: Arguments::<Identity, Impl, OFFSET>,
            SetArguments: SetArguments::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IExecAction2_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl + IExecAction_Impl {
    fn HideAppWindow(&mut self, phideappwindow: *mut i16) -> ::windows::core::Result<()>;
    fn SetHideAppWindow(&mut self, hideappwindow: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction2_Impl, const OFFSET: isize>() -> IExecAction2_Vtbl {
        unsafe extern "system" fn HideAppWindow<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HideAppWindow(::core::mem::transmute_copy(&phideappwindow)).into()
        }
        unsafe extern "system" fn SetHideAppWindow<Identity: ::windows::core::IUnknownImpl, Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHideAppWindow(::core::mem::transmute_copy(&hideappwindow)).into()
        }
        Self {
            base: IExecAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            HideAppWindow: HideAppWindow::<Identity, Impl, OFFSET>,
            SetHideAppWindow: SetHideAppWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecAction2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAction as ::windows::core::Interface>::IID || iid == &<IExecAction as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>() -> IIdleSettings_Vtbl {
        unsafe extern "system" fn IdleDuration<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IdleDuration(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetIdleDuration<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIdleDuration(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn WaitTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitTimeout(::core::mem::transmute_copy(&ptimeout)).into()
        }
        unsafe extern "system" fn SetWaitTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWaitTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn StopOnIdleEnd<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopOnIdleEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStopOnIdleEnd(::core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn RestartOnIdle<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestartOnIdle(::core::mem::transmute_copy(&prestart)).into()
        }
        unsafe extern "system" fn SetRestartOnIdle<Identity: ::windows::core::IUnknownImpl, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestartOnIdle(::core::mem::transmute_copy(&restart)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IdleDuration: IdleDuration::<Identity, Impl, OFFSET>,
            SetIdleDuration: SetIdleDuration::<Identity, Impl, OFFSET>,
            WaitTimeout: WaitTimeout::<Identity, Impl, OFFSET>,
            SetWaitTimeout: SetWaitTimeout::<Identity, Impl, OFFSET>,
            StopOnIdleEnd: StopOnIdleEnd::<Identity, Impl, OFFSET>,
            SetStopOnIdleEnd: SetStopOnIdleEnd::<Identity, Impl, OFFSET>,
            RestartOnIdle: RestartOnIdle::<Identity, Impl, OFFSET>,
            SetRestartOnIdle: SetRestartOnIdle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIdleTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleTrigger_Impl, const OFFSET: isize>() -> IIdleTrigger_Vtbl {
        Self { base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const OFFSET: isize>() -> ILogonTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogonTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>() -> IMaintenanceSettings_Vtbl {
        unsafe extern "system" fn SetPeriod<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPeriod(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Period<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Period(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetDeadline<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDeadline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Deadline<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deadline(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExclusive(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Exclusive<Identity: ::windows::core::IUnknownImpl, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Exclusive(::core::mem::transmute_copy(&target)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPeriod: SetPeriod::<Identity, Impl, OFFSET>,
            Period: Period::<Identity, Impl, OFFSET>,
            SetDeadline: SetDeadline::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
            SetExclusive: SetExclusive::<Identity, Impl, OFFSET>,
            Exclusive: Exclusive::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMaintenanceSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>() -> IMonthlyDOWTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WeeksOfMonth(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWeeksOfMonth(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunOnLastWeekOfMonth(::core::mem::transmute_copy(&plastweek)).into()
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRunOnLastWeekOfMonth(::core::mem::transmute_copy(&lastweek)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, Impl, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, Impl, OFFSET>,
            WeeksOfMonth: WeeksOfMonth::<Identity, Impl, OFFSET>,
            SetWeeksOfMonth: SetWeeksOfMonth::<Identity, Impl, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, Impl, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, Impl, OFFSET>,
            RunOnLastWeekOfMonth: RunOnLastWeekOfMonth::<Identity, Impl, OFFSET>,
            SetRunOnLastWeekOfMonth: SetRunOnLastWeekOfMonth::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>() -> IMonthlyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DaysOfMonth(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDaysOfMonth(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunOnLastDayOfMonth(::core::mem::transmute_copy(&plastday)).into()
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRunOnLastDayOfMonth(::core::mem::transmute_copy(&lastday)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            DaysOfMonth: DaysOfMonth::<Identity, Impl, OFFSET>,
            SetDaysOfMonth: SetDaysOfMonth::<Identity, Impl, OFFSET>,
            MonthsOfYear: MonthsOfYear::<Identity, Impl, OFFSET>,
            SetMonthsOfYear: SetMonthsOfYear::<Identity, Impl, OFFSET>,
            RunOnLastDayOfMonth: RunOnLastDayOfMonth::<Identity, Impl, OFFSET>,
            SetRunOnLastDayOfMonth: SetRunOnLastDayOfMonth::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonthlyTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const OFFSET: isize>() -> INetworkSettings_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>() -> IPrincipal_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayName(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        unsafe extern "system" fn LogonType<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogonType(::core::mem::transmute_copy(&plogon)).into()
        }
        unsafe extern "system" fn SetLogonType<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogonType(::core::mem::transmute_copy(&logon)).into()
        }
        unsafe extern "system" fn GroupId<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GroupId(::core::mem::transmute_copy(&pgroup)).into()
        }
        unsafe extern "system" fn SetGroupId<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGroupId(::core::mem::transmute_copy(&group)).into()
        }
        unsafe extern "system" fn RunLevel<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunLevel(::core::mem::transmute_copy(&prunlevel)).into()
        }
        unsafe extern "system" fn SetRunLevel<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRunLevel(::core::mem::transmute_copy(&runlevel)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
            LogonType: LogonType::<Identity, Impl, OFFSET>,
            SetLogonType: SetLogonType::<Identity, Impl, OFFSET>,
            GroupId: GroupId::<Identity, Impl, OFFSET>,
            SetGroupId: SetGroupId::<Identity, Impl, OFFSET>,
            RunLevel: RunLevel::<Identity, Impl, OFFSET>,
            SetRunLevel: SetRunLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>() -> IPrincipal2_Vtbl {
        unsafe extern "system" fn ProcessTokenSidType<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessTokenSidType(::core::mem::transmute_copy(&pprocesstokensidtype)).into()
        }
        unsafe extern "system" fn SetProcessTokenSidType<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProcessTokenSidType(::core::mem::transmute_copy(&processtokensidtype)).into()
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequiredPrivilegeCount(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn RequiredPrivilege<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequiredPrivilege(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pprivilege)).into()
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: ::windows::core::IUnknownImpl, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRequiredPrivilege(::core::mem::transmute_copy(&privilege)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProcessTokenSidType: ProcessTokenSidType::<Identity, Impl, OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Identity, Impl, OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Identity, Impl, OFFSET>,
            RequiredPrivilege: RequiredPrivilege::<Identity, Impl, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait IProvideTaskPage_Impl: Sized {
    fn GetPage(&mut self, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IProvideTaskPage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvideTaskPage_Impl, const OFFSET: isize>() -> IProvideTaskPage_Vtbl {
        unsafe extern "system" fn GetPage<Identity: ::windows::core::IUnknownImpl, Impl: IProvideTaskPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPage(::core::mem::transmute_copy(&tptype), ::core::mem::transmute_copy(&fpersistchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *phpage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPage: GetPage::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>() -> IRegisteredTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *penabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Run(::core::mem::transmute_copy(&params)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunEx<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RunEx(::core::mem::transmute_copy(&params), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&user)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstances<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstances(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRunTime<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plastruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTaskResult<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastTaskResult() {
                ::core::result::Result::Ok(ok__) => {
                    *plasttaskresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfMissedRuns<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfMissedRuns() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumberofmissedruns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRunTime<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pnextruntime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Definition<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Definition() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *pxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *psddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRunTimes(::core::mem::transmute_copy(&pststart), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&pruntimes)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            RunEx: RunEx::<Identity, Impl, OFFSET>,
            GetInstances: GetInstances::<Identity, Impl, OFFSET>,
            LastRunTime: LastRunTime::<Identity, Impl, OFFSET>,
            LastTaskResult: LastTaskResult::<Identity, Impl, OFFSET>,
            NumberOfMissedRuns: NumberOfMissedRuns::<Identity, Impl, OFFSET>,
            NextRunTime: NextRunTime::<Identity, Impl, OFFSET>,
            Definition: Definition::<Identity, Impl, OFFSET>,
            Xml: Xml::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTask as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>() -> IRegisteredTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppregisteredtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>() -> IRegistrationInfo_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn SetAuthor<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthor(::core::mem::transmute_copy(&author)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&version)).into()
        }
        unsafe extern "system" fn Date<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Date(::core::mem::transmute_copy(&pdate)).into()
        }
        unsafe extern "system" fn SetDate<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDate(::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn Documentation<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Documentation(::core::mem::transmute_copy(&pdocumentation)).into()
        }
        unsafe extern "system" fn SetDocumentation<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDocumentation(::core::mem::transmute_copy(&documentation)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn URI<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).URI(::core::mem::transmute_copy(&puri)).into()
        }
        unsafe extern "system" fn SetURI<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetURI(::core::mem::transmute_copy(&uri)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SecurityDescriptor(::core::mem::transmute_copy(&psddl)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl)).into()
        }
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Source(::core::mem::transmute_copy(&psource)).into()
        }
        unsafe extern "system" fn SetSource<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSource(::core::mem::transmute_copy(&source)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            SetAuthor: SetAuthor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            Date: Date::<Identity, Impl, OFFSET>,
            SetDate: SetDate::<Identity, Impl, OFFSET>,
            Documentation: Documentation::<Identity, Impl, OFFSET>,
            SetDocumentation: SetDocumentation::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            URI: URI::<Identity, Impl, OFFSET>,
            SetURI: SetURI::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            Source: Source::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&mut self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&mut self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>() -> IRegistrationTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        Self { base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(), Delay: Delay::<Identity, Impl, OFFSET>, SetDelay: SetDelay::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>() -> IRepetitionPattern_Vtbl {
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Interval(::core::mem::transmute_copy(&pinterval)).into()
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn StopAtDurationEnd<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopAtDurationEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Identity: ::windows::core::IUnknownImpl, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStopAtDurationEnd(::core::mem::transmute_copy(&stop)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            StopAtDurationEnd: StopAtDurationEnd::<Identity, Impl, OFFSET>,
            SetStopAtDurationEnd: SetStopAtDurationEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepetitionPattern as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>() -> IRunningTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceGuid<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn EnginePID<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnginePID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            InstanceGuid: InstanceGuid::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            EnginePID: EnginePID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTask as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>() -> IRunningTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTaskCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>() -> IScheduledWorkItem_Vtbl {
        unsafe extern "system" fn CreateTrigger<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateTrigger(::core::mem::transmute_copy(&pinewtrigger), ::core::mem::transmute_copy(&pptrigger)).into()
        }
        unsafe extern "system" fn DeleteTrigger<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTrigger(::core::mem::transmute_copy(&itrigger)).into()
        }
        unsafe extern "system" fn GetTriggerCount<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTriggerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrigger(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTriggerString(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwsztrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRunTimes(::core::mem::transmute_copy(&pstbegin), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&rgsttasktimes)).into()
        }
        unsafe extern "system" fn GetNextRunTime<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextRunTime(::core::mem::transmute_copy(&pstnextrun)).into()
        }
        unsafe extern "system" fn SetIdleWait<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIdleWait(::core::mem::transmute_copy(&widleminutes), ::core::mem::transmute_copy(&wdeadlineminutes)).into()
        }
        unsafe extern "system" fn GetIdleWait<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdleWait(::core::mem::transmute_copy(&pwidleminutes), ::core::mem::transmute_copy(&pwdeadlineminutes)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Run().into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn EditWorkItem<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EditWorkItem(::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetMostRecentRunTime<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMostRecentRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pstlastrun = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *phrstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitCode<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwexitcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetComment(::core::mem::transmute_copy(&pwszcomment)).into()
        }
        unsafe extern "system" fn GetComment<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetComment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcreator: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCreator(::core::mem::transmute_copy(&pwszcreator)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcreator: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkItemData<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkItemData(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&rgbdata)).into()
        }
        unsafe extern "system" fn GetWorkItemData<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetWorkItemData(::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&prgbdata)).into()
        }
        unsafe extern "system" fn SetErrorRetryCount<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorRetryCount(::core::mem::transmute_copy(&wretrycount)).into()
        }
        unsafe extern "system" fn GetErrorRetryCount<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorRetryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwretrycount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorRetryInterval<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorRetryInterval(::core::mem::transmute_copy(&wretryinterval)).into()
        }
        unsafe extern "system" fn GetErrorRetryInterval<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorRetryInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pwretryinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszaccountname: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAccountInformation(::core::mem::transmute_copy(&pwszaccountname), ::core::mem::transmute_copy(&pwszpassword)).into()
        }
        unsafe extern "system" fn GetAccountInformation<Identity: ::windows::core::IUnknownImpl, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAccountInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszaccountname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTrigger: CreateTrigger::<Identity, Impl, OFFSET>,
            DeleteTrigger: DeleteTrigger::<Identity, Impl, OFFSET>,
            GetTriggerCount: GetTriggerCount::<Identity, Impl, OFFSET>,
            GetTrigger: GetTrigger::<Identity, Impl, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, Impl, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, Impl, OFFSET>,
            GetNextRunTime: GetNextRunTime::<Identity, Impl, OFFSET>,
            SetIdleWait: SetIdleWait::<Identity, Impl, OFFSET>,
            GetIdleWait: GetIdleWait::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            EditWorkItem: EditWorkItem::<Identity, Impl, OFFSET>,
            GetMostRecentRunTime: GetMostRecentRunTime::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetExitCode: GetExitCode::<Identity, Impl, OFFSET>,
            SetComment: SetComment::<Identity, Impl, OFFSET>,
            GetComment: GetComment::<Identity, Impl, OFFSET>,
            SetCreator: SetCreator::<Identity, Impl, OFFSET>,
            GetCreator: GetCreator::<Identity, Impl, OFFSET>,
            SetWorkItemData: SetWorkItemData::<Identity, Impl, OFFSET>,
            GetWorkItemData: GetWorkItemData::<Identity, Impl, OFFSET>,
            SetErrorRetryCount: SetErrorRetryCount::<Identity, Impl, OFFSET>,
            GetErrorRetryCount: GetErrorRetryCount::<Identity, Impl, OFFSET>,
            SetErrorRetryInterval: SetErrorRetryInterval::<Identity, Impl, OFFSET>,
            GetErrorRetryInterval: GetErrorRetryInterval::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetAccountInformation: SetAccountInformation::<Identity, Impl, OFFSET>,
            GetAccountInformation: GetAccountInformation::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>() -> ISessionStateChangeTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelay(::core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserId(::core::mem::transmute_copy(&user)).into()
        }
        unsafe extern "system" fn StateChange<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StateChange(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn SetStateChange<Identity: ::windows::core::IUnknownImpl, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStateChange(::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            Delay: Delay::<Identity, Impl, OFFSET>,
            SetDelay: SetDelay::<Identity, Impl, OFFSET>,
            UserId: UserId::<Identity, Impl, OFFSET>,
            SetUserId: SetUserId::<Identity, Impl, OFFSET>,
            StateChange: StateChange::<Identity, Impl, OFFSET>,
            SetStateChange: SetStateChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const OFFSET: isize>() -> IShowMessageAction_Vtbl {
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn MessageBody<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MessageBody(::core::mem::transmute_copy(&pmessagebody)).into()
        }
        unsafe extern "system" fn SetMessageBody<Identity: ::windows::core::IUnknownImpl, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageBody(::core::mem::transmute_copy(&messagebody)).into()
        }
        Self {
            base: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            MessageBody: MessageBody::<Identity, Impl, OFFSET>,
            SetMessageBody: SetMessageBody::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShowMessageAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAction as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>() -> ITask_Vtbl {
        unsafe extern "system" fn SetApplicationName<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszapplicationname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationName(::core::mem::transmute_copy(&pwszapplicationname)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszapplicationname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszparameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParameters(::core::mem::transmute_copy(&pwszparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszworkingdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&pwszworkingdirectory)).into()
        }
        unsafe extern "system" fn GetWorkingDirectory<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszworkingdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTaskFlags<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTaskFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRunTime<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxRunTime(::core::mem::transmute_copy(&dwmaxruntimems)).into()
        }
        unsafe extern "system" fn GetMaxRunTime<Identity: ::windows::core::IUnknownImpl, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxruntimems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IScheduledWorkItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetApplicationName: SetApplicationName::<Identity, Impl, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, Impl, OFFSET>,
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            GetWorkingDirectory: GetWorkingDirectory::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetTaskFlags: SetTaskFlags::<Identity, Impl, OFFSET>,
            GetTaskFlags: GetTaskFlags::<Identity, Impl, OFFSET>,
            SetMaxRunTime: SetMaxRunTime::<Identity, Impl, OFFSET>,
            GetMaxRunTime: GetMaxRunTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITask as ::windows::core::Interface>::IID || iid == &<IScheduledWorkItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>() -> ITaskDefinition_Vtbl {
        unsafe extern "system" fn RegistrationInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegistrationInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregistrationinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistrationInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregistrationinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRegistrationInfo(::core::mem::transmute(&pregistrationinfo)).into()
        }
        unsafe extern "system" fn Triggers<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptriggers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Triggers() {
                ::core::result::Result::Ok(ok__) => {
                    *pptriggers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggers<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptriggers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTriggers(::core::mem::transmute(&ptriggers)).into()
        }
        unsafe extern "system" fn Settings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSettings(::core::mem::transmute(&psettings)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Principal<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprincipal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Principal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprincipal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrincipal<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprincipal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrincipal(::core::mem::transmute(&pprincipal)).into()
        }
        unsafe extern "system" fn Actions<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppactions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppactions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActions(::core::mem::transmute(&pactions)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&pxml)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&xml)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegistrationInfo: RegistrationInfo::<Identity, Impl, OFFSET>,
            SetRegistrationInfo: SetRegistrationInfo::<Identity, Impl, OFFSET>,
            Triggers: Triggers::<Identity, Impl, OFFSET>,
            SetTriggers: SetTriggers::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            SetSettings: SetSettings::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Principal: Principal::<Identity, Impl, OFFSET>,
            SetPrincipal: SetPrincipal::<Identity, Impl, OFFSET>,
            Actions: Actions::<Identity, Impl, OFFSET>,
            SetActions: SetActions::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskDefinition as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>() -> ITaskFolder_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *ppath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolders<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFolders(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolder(::core::mem::transmute_copy(&subfoldername), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteFolder(::core::mem::transmute_copy(&subfoldername), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetTask<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTask(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn RegisterTask<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterTask(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&xmltext), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: ::windows::core::RawPtr, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterTaskDefinition(::core::mem::transmute_copy(&path), ::core::mem::transmute(&pdefinition), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&userid), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute_copy(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *psddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute_copy(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            GetFolders: GetFolders::<Identity, Impl, OFFSET>,
            CreateFolder: CreateFolder::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            GetTask: GetTask::<Identity, Impl, OFFSET>,
            GetTasks: GetTasks::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            RegisterTask: RegisterTask::<Identity, Impl, OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolder as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>() -> ITaskFolderCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolderCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const OFFSET: isize>() -> ITaskHandler_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute(&phandlerservices), ::core::mem::transmute_copy(&data)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *pretcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>() -> ITaskHandlerStatus_Vtbl {
        unsafe extern "system" fn UpdateStatus<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateStatus(::core::mem::transmute_copy(&percentcomplete), ::core::mem::transmute_copy(&statusmessage)).into()
        }
        unsafe extern "system" fn TaskCompleted<Identity: ::windows::core::IUnknownImpl, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TaskCompleted(::core::mem::transmute_copy(&taskerrcode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            UpdateStatus: UpdateStatus::<Identity, Impl, OFFSET>,
            TaskCompleted: TaskCompleted::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>() -> ITaskNamedValueCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppair = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppair = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>() -> ITaskNamedValuePair_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Value(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>() -> ITaskScheduler_Vtbl {
        unsafe extern "system" fn SetTargetComputer<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomputer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTargetComputer(::core::mem::transmute_copy(&pwszcomputer)).into()
        }
        unsafe extern "system" fn GetTargetComputer<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTargetComputer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszcomputer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumworkitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Activate(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn NewWorkItem<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NewWorkItem(::core::mem::transmute_copy(&pwsztaskname), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWorkItem<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: super::super::Foundation::PWSTR, pworkitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddWorkItem(::core::mem::transmute_copy(&pwsztaskname), ::core::mem::transmute(&pworkitem)).into()
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows::core::IUnknownImpl, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsOfType(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetTargetComputer: SetTargetComputer::<Identity, Impl, OFFSET>,
            GetTargetComputer: GetTargetComputer::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            NewWorkItem: NewWorkItem::<Identity, Impl, OFFSET>,
            AddWorkItem: AddWorkItem::<Identity, Impl, OFFSET>,
            IsOfType: IsOfType::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>() -> ITaskService_Vtbl {
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFolder(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfolder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningTasks<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRunningTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprunningtasks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewTask<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NewTask(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdefinition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&servername), ::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&domain), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn Connected<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Connected() {
                ::core::result::Result::Ok(ok__) => {
                    *pconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetServer<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedUser<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    *puser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedDomain<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectedDomain() {
                ::core::result::Result::Ok(ok__) => {
                    *pdomain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighestVersion<Identity: ::windows::core::IUnknownImpl, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HighestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            GetRunningTasks: GetRunningTasks::<Identity, Impl, OFFSET>,
            NewTask: NewTask::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Connected: Connected::<Identity, Impl, OFFSET>,
            TargetServer: TargetServer::<Identity, Impl, OFFSET>,
            ConnectedUser: ConnectedUser::<Identity, Impl, OFFSET>,
            ConnectedDomain: ConnectedDomain::<Identity, Impl, OFFSET>,
            HighestVersion: HighestVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskService as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>() -> ITaskSettings_Vtbl {
        unsafe extern "system" fn AllowDemandStart<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllowDemandStart(::core::mem::transmute_copy(&pallowdemandstart)).into()
        }
        unsafe extern "system" fn SetAllowDemandStart<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowDemandStart(::core::mem::transmute_copy(&allowdemandstart)).into()
        }
        unsafe extern "system" fn RestartInterval<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestartInterval(::core::mem::transmute_copy(&prestartinterval)).into()
        }
        unsafe extern "system" fn SetRestartInterval<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestartInterval(::core::mem::transmute_copy(&restartinterval)).into()
        }
        unsafe extern "system" fn RestartCount<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestartCount(::core::mem::transmute_copy(&prestartcount)).into()
        }
        unsafe extern "system" fn SetRestartCount<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRestartCount(::core::mem::transmute_copy(&restartcount)).into()
        }
        unsafe extern "system" fn MultipleInstances<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MultipleInstances(::core::mem::transmute_copy(&ppolicy)).into()
        }
        unsafe extern "system" fn SetMultipleInstances<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultipleInstances(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopIfGoingOnBatteries(::core::mem::transmute_copy(&pstopifonbatteries)).into()
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStopIfGoingOnBatteries(::core::mem::transmute_copy(&stopifonbatteries)).into()
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisallowStartIfOnBatteries(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisallowStartIfOnBatteries(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn AllowHardTerminate<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllowHardTerminate(::core::mem::transmute_copy(&pallowhardterminate)).into()
        }
        unsafe extern "system" fn SetAllowHardTerminate<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowHardTerminate(::core::mem::transmute_copy(&allowhardterminate)).into()
        }
        unsafe extern "system" fn StartWhenAvailable<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartWhenAvailable(::core::mem::transmute_copy(&pstartwhenavailable)).into()
        }
        unsafe extern "system" fn SetStartWhenAvailable<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartWhenAvailable(::core::mem::transmute_copy(&startwhenavailable)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetXmlText(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&prunonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecutionTimeLimit(::core::mem::transmute_copy(&pexecutiontimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExecutionTimeLimit(::core::mem::transmute_copy(&executiontimelimit)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteExpiredTaskAfter(::core::mem::transmute_copy(&pexpirationdelay)).into()
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDeleteExpiredTaskAfter(::core::mem::transmute_copy(&expirationdelay)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Priority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Compatibility<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Compatibility(::core::mem::transmute_copy(&pcompatlevel)).into()
        }
        unsafe extern "system" fn SetCompatibility<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompatibility(::core::mem::transmute_copy(&compatlevel)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Hidden(::core::mem::transmute_copy(&phidden)).into()
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHidden(::core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn IdleSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidlesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IdleSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidlesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdleSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidlesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIdleSettings(::core::mem::transmute(&pidlesettings)).into()
        }
        unsafe extern "system" fn RunOnlyIfIdle<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunOnlyIfIdle(::core::mem::transmute_copy(&prunonlyifidle)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRunOnlyIfIdle(::core::mem::transmute_copy(&runonlyifidle)).into()
        }
        unsafe extern "system" fn WakeToRun<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WakeToRun(::core::mem::transmute_copy(&pwake)).into()
        }
        unsafe extern "system" fn SetWakeToRun<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWakeToRun(::core::mem::transmute_copy(&wake)).into()
        }
        unsafe extern "system" fn NetworkSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NetworkSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetworksettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetworkSettings(::core::mem::transmute(&pnetworksettings)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AllowDemandStart: AllowDemandStart::<Identity, Impl, OFFSET>,
            SetAllowDemandStart: SetAllowDemandStart::<Identity, Impl, OFFSET>,
            RestartInterval: RestartInterval::<Identity, Impl, OFFSET>,
            SetRestartInterval: SetRestartInterval::<Identity, Impl, OFFSET>,
            RestartCount: RestartCount::<Identity, Impl, OFFSET>,
            SetRestartCount: SetRestartCount::<Identity, Impl, OFFSET>,
            MultipleInstances: MultipleInstances::<Identity, Impl, OFFSET>,
            SetMultipleInstances: SetMultipleInstances::<Identity, Impl, OFFSET>,
            StopIfGoingOnBatteries: StopIfGoingOnBatteries::<Identity, Impl, OFFSET>,
            SetStopIfGoingOnBatteries: SetStopIfGoingOnBatteries::<Identity, Impl, OFFSET>,
            DisallowStartIfOnBatteries: DisallowStartIfOnBatteries::<Identity, Impl, OFFSET>,
            SetDisallowStartIfOnBatteries: SetDisallowStartIfOnBatteries::<Identity, Impl, OFFSET>,
            AllowHardTerminate: AllowHardTerminate::<Identity, Impl, OFFSET>,
            SetAllowHardTerminate: SetAllowHardTerminate::<Identity, Impl, OFFSET>,
            StartWhenAvailable: StartWhenAvailable::<Identity, Impl, OFFSET>,
            SetStartWhenAvailable: SetStartWhenAvailable::<Identity, Impl, OFFSET>,
            XmlText: XmlText::<Identity, Impl, OFFSET>,
            SetXmlText: SetXmlText::<Identity, Impl, OFFSET>,
            RunOnlyIfNetworkAvailable: RunOnlyIfNetworkAvailable::<Identity, Impl, OFFSET>,
            SetRunOnlyIfNetworkAvailable: SetRunOnlyIfNetworkAvailable::<Identity, Impl, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, Impl, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            DeleteExpiredTaskAfter: DeleteExpiredTaskAfter::<Identity, Impl, OFFSET>,
            SetDeleteExpiredTaskAfter: SetDeleteExpiredTaskAfter::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Compatibility: Compatibility::<Identity, Impl, OFFSET>,
            SetCompatibility: SetCompatibility::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            IdleSettings: IdleSettings::<Identity, Impl, OFFSET>,
            SetIdleSettings: SetIdleSettings::<Identity, Impl, OFFSET>,
            RunOnlyIfIdle: RunOnlyIfIdle::<Identity, Impl, OFFSET>,
            SetRunOnlyIfIdle: SetRunOnlyIfIdle::<Identity, Impl, OFFSET>,
            WakeToRun: WakeToRun::<Identity, Impl, OFFSET>,
            SetWakeToRun: SetWakeToRun::<Identity, Impl, OFFSET>,
            NetworkSettings: NetworkSettings::<Identity, Impl, OFFSET>,
            SetNetworkSettings: SetNetworkSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const OFFSET: isize>() -> ITaskSettings2_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>() -> ITaskSettings3_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        unsafe extern "system" fn MaintenanceSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmaintenancesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaintenancesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaintenanceSettings(::core::mem::transmute(&pmaintenancesettings)).into()
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmaintenancesettings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volatile<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Volatile(::core::mem::transmute_copy(&pvolatile)).into()
        }
        unsafe extern "system" fn SetVolatile<Identity: ::windows::core::IUnknownImpl, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolatile(::core::mem::transmute_copy(&volatile)).into()
        }
        Self {
            base: ITaskSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisallowStartOnRemoteAppSession: DisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            SetDisallowStartOnRemoteAppSession: SetDisallowStartOnRemoteAppSession::<Identity, Impl, OFFSET>,
            UseUnifiedSchedulingEngine: UseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            SetUseUnifiedSchedulingEngine: SetUseUnifiedSchedulingEngine::<Identity, Impl, OFFSET>,
            MaintenanceSettings: MaintenanceSettings::<Identity, Impl, OFFSET>,
            SetMaintenanceSettings: SetMaintenanceSettings::<Identity, Impl, OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Identity, Impl, OFFSET>,
            Volatile: Volatile::<Identity, Impl, OFFSET>,
            SetVolatile: SetVolatile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskSettings3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITaskSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTrigger_Impl, const OFFSET: isize>() -> ITaskTrigger_Vtbl {
        unsafe extern "system" fn SetTrigger<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrigger(::core::mem::transmute_copy(&ptrigger)).into()
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *ptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows::core::IUnknownImpl, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTriggerString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwsztrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetTrigger: SetTrigger::<Identity, Impl, OFFSET>,
            GetTrigger: GetTrigger::<Identity, Impl, OFFSET>,
            GetTriggerString: GetTriggerString::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariables_Impl, const OFFSET: isize>() -> ITaskVariables_Vtbl {
        unsafe extern "system" fn GetInput<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInput() {
                ::core::result::Result::Ok(ok__) => {
                    *pinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutput<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutput(::core::mem::transmute_copy(&input)).into()
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *pcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInput: GetInput::<Identity, Impl, OFFSET>,
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTrigger_Impl, const OFFSET: isize>() -> ITimeTrigger_Vtbl {
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>() -> ITrigger_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Type(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Repetition<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepeat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Repetition() {
                ::core::result::Result::Ok(ok__) => {
                    *pprepeat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetition<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prepeat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRepetition(::core::mem::transmute(&prepeat)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExecutionTimeLimit(::core::mem::transmute_copy(&ptimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExecutionTimeLimit(::core::mem::transmute_copy(&timelimit)).into()
        }
        unsafe extern "system" fn StartBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartBoundary(::core::mem::transmute_copy(&pstart)).into()
        }
        unsafe extern "system" fn SetStartBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartBoundary(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn EndBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndBoundary(::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetEndBoundary<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndBoundary(::core::mem::transmute_copy(&end)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Repetition: Repetition::<Identity, Impl, OFFSET>,
            SetRepetition: SetRepetition::<Identity, Impl, OFFSET>,
            ExecutionTimeLimit: ExecutionTimeLimit::<Identity, Impl, OFFSET>,
            SetExecutionTimeLimit: SetExecutionTimeLimit::<Identity, Impl, OFFSET>,
            StartBoundary: StartBoundary::<Identity, Impl, OFFSET>,
            SetStartBoundary: SetStartBoundary::<Identity, Impl, OFFSET>,
            EndBoundary: EndBoundary::<Identity, Impl, OFFSET>,
            SetEndBoundary: SetEndBoundary::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>() -> ITriggerCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptrigger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>() -> IWeeklyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksInterval<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WeeksInterval(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksInterval<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWeeksInterval(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRandomDelay(::core::mem::transmute_copy(&randomdelay)).into()
        }
        Self {
            base: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
            DaysOfWeek: DaysOfWeek::<Identity, Impl, OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Identity, Impl, OFFSET>,
            WeeksInterval: WeeksInterval::<Identity, Impl, OFFSET>,
            SetWeeksInterval: SetWeeksInterval::<Identity, Impl, OFFSET>,
            RandomDelay: RandomDelay::<Identity, Impl, OFFSET>,
            SetRandomDelay: SetRandomDelay::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeeklyTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
