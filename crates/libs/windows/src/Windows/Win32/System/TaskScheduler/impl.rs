#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAction_Impl, const OFFSET: isize>() -> IAction_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Type(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn get_Item(&self, index: i32) -> ::windows::core::Result<IAction>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Create(&self, r#type: TASK_ACTION_TYPE) -> ::windows::core::Result<IAction>;
    fn Remove(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn Context(&self, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetContext(&self, context: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IActionCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IActionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>() -> IActionCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXmlText(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn Context<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Context(::core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContext(::core::mem::transmute(&context)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IBootTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: isize>() -> IBootTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBootTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelay(::core::mem::transmute(&delay)).into()
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(), Delay: Delay::<Identity, Impl, OFFSET>, SetDelay: SetDelay::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComHandlerAction_Impl: Sized + super::Com::IDispatch_Impl + IAction_Impl {
    fn ClassId(&self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetClassId(&self, clsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Data(&self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetData(&self, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IComHandlerAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComHandlerAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: isize>() -> IComHandlerAction_Vtbl {
        unsafe extern "system" fn ClassId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClassId(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetClassId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassId(::core::mem::transmute(&clsid)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&data)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DaysInterval(&self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysInterval(&self, days: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDailyTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDailyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: isize>() -> IDailyTrigger_Vtbl {
        unsafe extern "system" fn DaysInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DaysInterval(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysInterval(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRandomDelay(::core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Server(&self, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetServer(&self, server: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Subject(&self, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSubject(&self, subject: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn To(&self, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTo(&self, to: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Cc(&self, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetCc(&self, cc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Bcc(&self, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetBcc(&self, bcc: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReplyTo(&self, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetReplyTo(&self, replyto: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn From(&self, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetFrom(&self, from: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HeaderFields(&self) -> ::windows::core::Result<ITaskNamedValueCollection>;
    fn SetHeaderFields(&self, pheaderfields: &::core::option::Option<ITaskNamedValueCollection>) -> ::windows::core::Result<()>;
    fn Body(&self, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetBody(&self, body: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEmailAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEmailAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>() -> IEmailAction_Vtbl {
        unsafe extern "system" fn Server<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Server(::core::mem::transmute_copy(&pserver)).into()
        }
        unsafe extern "system" fn SetServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServer(::core::mem::transmute(&server)).into()
        }
        unsafe extern "system" fn Subject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Subject(::core::mem::transmute_copy(&psubject)).into()
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubject(::core::mem::transmute(&subject)).into()
        }
        unsafe extern "system" fn To<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.To(::core::mem::transmute_copy(&pto)).into()
        }
        unsafe extern "system" fn SetTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTo(::core::mem::transmute(&to)).into()
        }
        unsafe extern "system" fn Cc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cc(::core::mem::transmute_copy(&pcc)).into()
        }
        unsafe extern "system" fn SetCc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCc(::core::mem::transmute(&cc)).into()
        }
        unsafe extern "system" fn Bcc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bcc(::core::mem::transmute_copy(&pbcc)).into()
        }
        unsafe extern "system" fn SetBcc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBcc(::core::mem::transmute(&bcc)).into()
        }
        unsafe extern "system" fn ReplyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplyTo(::core::mem::transmute_copy(&preplyto)).into()
        }
        unsafe extern "system" fn SetReplyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReplyTo(::core::mem::transmute(&replyto)).into()
        }
        unsafe extern "system" fn From<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.From(::core::mem::transmute_copy(&pfrom)).into()
        }
        unsafe extern "system" fn SetFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrom(::core::mem::transmute(&from)).into()
        }
        unsafe extern "system" fn HeaderFields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderfields: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HeaderFields() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppheaderfields, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeaderFields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaderfields: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeaderFields(::core::mem::transmute(&pheaderfields)).into()
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Body(::core::mem::transmute_copy(&pbody)).into()
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&body)).into()
        }
        unsafe extern "system" fn Attachments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        unsafe extern "system" fn SetAttachments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmailAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttachments(::core::mem::transmute_copy(&pattachements)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
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
pub trait IEnumWorkItems_Impl: Sized {
    fn Next(&self, celt: u32, rgpwsznames: *mut *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumWorkItems>;
}
impl ::windows::core::RuntimeName for IEnumWorkItems {}
impl IEnumWorkItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: isize>() -> IEnumWorkItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgpwsznames), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWorkItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumworkitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Subscription(&self, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSubscription(&self, query: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ValueQueries(&self) -> ::windows::core::Result<ITaskNamedValueCollection>;
    fn SetValueQueries(&self, pnamedxpaths: &::core::option::Option<ITaskNamedValueCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>() -> IEventTrigger_Vtbl {
        unsafe extern "system" fn Subscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Subscription(::core::mem::transmute_copy(&pquery)).into()
        }
        unsafe extern "system" fn SetSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscription(::core::mem::transmute(&query)).into()
        }
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelay(::core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn ValueQueries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ValueQueries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamedxpaths, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueQueries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamedxpaths: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValueQueries(::core::mem::transmute(&pnamedxpaths)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Path(&self, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPath(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Arguments(&self, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetArguments(&self, argument: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&self, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWorkingDirectory(&self, workingdirectory: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IExecAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>() -> IExecAction_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Path(::core::mem::transmute_copy(&ppath)).into()
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn Arguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Arguments(::core::mem::transmute_copy(&pargument)).into()
        }
        unsafe extern "system" fn SetArguments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetArguments(::core::mem::transmute(&argument)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WorkingDirectory(::core::mem::transmute_copy(&pworkingdirectory)).into()
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkingDirectory(::core::mem::transmute(&workingdirectory)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn HideAppWindow(&self, phideappwindow: *mut i16) -> ::windows::core::Result<()>;
    fn SetHideAppWindow(&self, hideappwindow: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IExecAction2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: isize>() -> IExecAction2_Vtbl {
        unsafe extern "system" fn HideAppWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HideAppWindow(::core::mem::transmute_copy(&phideappwindow)).into()
        }
        unsafe extern "system" fn SetHideAppWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExecAction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHideAppWindow(::core::mem::transmute_copy(&hideappwindow)).into()
        }
        Self {
            base__: IExecAction_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn IdleDuration(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetIdleDuration(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WaitTimeout(&self, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetWaitTimeout(&self, timeout: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopOnIdleEnd(&self, pstop: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopOnIdleEnd(&self, stop: i16) -> ::windows::core::Result<()>;
    fn RestartOnIdle(&self, prestart: *mut i16) -> ::windows::core::Result<()>;
    fn SetRestartOnIdle(&self, restart: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IIdleSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>() -> IIdleSettings_Vtbl {
        unsafe extern "system" fn IdleDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IdleDuration(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetIdleDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIdleDuration(::core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn WaitTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitTimeout(::core::mem::transmute_copy(&ptimeout)).into()
        }
        unsafe extern "system" fn SetWaitTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWaitTimeout(::core::mem::transmute(&timeout)).into()
        }
        unsafe extern "system" fn StopOnIdleEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopOnIdleEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopOnIdleEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStopOnIdleEnd(::core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn RestartOnIdle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartOnIdle(::core::mem::transmute_copy(&prestart)).into()
        }
        unsafe extern "system" fn SetRestartOnIdle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRestartOnIdle(::core::mem::transmute_copy(&restart)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
impl ::windows::core::RuntimeName for IIdleTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIdleTrigger_Impl, const OFFSET: isize>() -> IIdleTrigger_Vtbl {
        Self { base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILogonTrigger_Impl: Sized + super::Com::IDispatch_Impl + ITrigger_Impl {
    fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ILogonTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILogonTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: isize>() -> ILogonTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelay(::core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserId(::core::mem::transmute(&user)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn SetPeriod(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Period(&self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDeadline(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Deadline(&self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExclusive(&self, value: i16) -> ::windows::core::Result<()>;
    fn Exclusive(&self, target: *mut i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMaintenanceSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMaintenanceSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>() -> IMaintenanceSettings_Vtbl {
        unsafe extern "system" fn SetPeriod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPeriod(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Period<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Period(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetDeadline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeadline(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Deadline<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deadline(::core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExclusive(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Exclusive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Exclusive(::core::mem::transmute_copy(&target)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> ::windows::core::Result<()>;
    fn WeeksOfMonth(&self, pweeks: *mut i16) -> ::windows::core::Result<()>;
    fn SetWeeksOfMonth(&self, weeks: i16) -> ::windows::core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows::core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> ::windows::core::Result<()>;
    fn RunOnLastWeekOfMonth(&self, plastweek: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnLastWeekOfMonth(&self, lastweek: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMonthlyDOWTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyDOWTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>() -> IMonthlyDOWTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WeeksOfMonth(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWeeksOfMonth(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastWeekOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunOnLastWeekOfMonth(::core::mem::transmute_copy(&plastweek)).into()
        }
        unsafe extern "system" fn SetRunOnLastWeekOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunOnLastWeekOfMonth(::core::mem::transmute_copy(&lastweek)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRandomDelay(::core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DaysOfMonth(&self, pdays: *mut i32) -> ::windows::core::Result<()>;
    fn SetDaysOfMonth(&self, days: i32) -> ::windows::core::Result<()>;
    fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows::core::Result<()>;
    fn SetMonthsOfYear(&self, months: i16) -> ::windows::core::Result<()>;
    fn RunOnLastDayOfMonth(&self, plastday: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnLastDayOfMonth(&self, lastday: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMonthlyTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMonthlyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>() -> IMonthlyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DaysOfMonth(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysOfMonth(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn MonthsOfYear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MonthsOfYear(::core::mem::transmute_copy(&pmonths)).into()
        }
        unsafe extern "system" fn SetMonthsOfYear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMonthsOfYear(::core::mem::transmute_copy(&months)).into()
        }
        unsafe extern "system" fn RunOnLastDayOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunOnLastDayOfMonth(::core::mem::transmute_copy(&plastday)).into()
        }
        unsafe extern "system" fn SetRunOnLastDayOfMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunOnLastDayOfMonth(::core::mem::transmute_copy(&lastday)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRandomDelay(::core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for INetworkSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INetworkSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: isize>() -> INetworkSettings_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetworkSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayName(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDisplayName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::Result<()>;
    fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> ::windows::core::Result<()>;
    fn GroupId(&self, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetGroupId(&self, group: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()>;
    fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPrincipal {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>() -> IPrincipal_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayName(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserId(::core::mem::transmute(&user)).into()
        }
        unsafe extern "system" fn LogonType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogonType(::core::mem::transmute_copy(&plogon)).into()
        }
        unsafe extern "system" fn SetLogonType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogonType(::core::mem::transmute_copy(&logon)).into()
        }
        unsafe extern "system" fn GroupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GroupId(::core::mem::transmute_copy(&pgroup)).into()
        }
        unsafe extern "system" fn SetGroupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupId(::core::mem::transmute(&group)).into()
        }
        unsafe extern "system" fn RunLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunLevel(::core::mem::transmute_copy(&prunlevel)).into()
        }
        unsafe extern "system" fn SetRunLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunLevel(::core::mem::transmute_copy(&runlevel)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()>;
    fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()>;
    fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddRequiredPrivilege(&self, privilege: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPrincipal2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPrincipal2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>() -> IPrincipal2_Vtbl {
        unsafe extern "system" fn ProcessTokenSidType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessTokenSidType(::core::mem::transmute_copy(&pprocesstokensidtype)).into()
        }
        unsafe extern "system" fn SetProcessTokenSidType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessTokenSidType(::core::mem::transmute_copy(&processtokensidtype)).into()
        }
        unsafe extern "system" fn RequiredPrivilegeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequiredPrivilegeCount(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_RequiredPrivilege<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_RequiredPrivilege(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pprivilege)).into()
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPrincipal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRequiredPrivilege(::core::mem::transmute(&privilege)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProcessTokenSidType: ProcessTokenSidType::<Identity, Impl, OFFSET>,
            SetProcessTokenSidType: SetProcessTokenSidType::<Identity, Impl, OFFSET>,
            RequiredPrivilegeCount: RequiredPrivilegeCount::<Identity, Impl, OFFSET>,
            get_RequiredPrivilege: get_RequiredPrivilege::<Identity, Impl, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPrincipal2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub trait IProvideTaskPage_Impl: Sized {
    fn GetPage(&self, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::UI::Controls::HPROPSHEETPAGE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::windows::core::RuntimeName for IProvideTaskPage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IProvideTaskPage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideTaskPage_Impl, const OFFSET: isize>() -> IProvideTaskPage_Vtbl {
        unsafe extern "system" fn GetPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvideTaskPage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPage(::core::mem::transmute_copy(&tptype), ::core::mem::transmute_copy(&fpersistchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPage: GetPage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideTaskPage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegisteredTask_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<TASK_STATE>;
    fn Enabled(&self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn Run(&self, params: &super::Com::VARIANT) -> ::windows::core::Result<IRunningTask>;
    fn RunEx(&self, params: &super::Com::VARIANT, flags: i32, sessionid: i32, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRunningTask>;
    fn GetInstances(&self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection>;
    fn LastRunTime(&self) -> ::windows::core::Result<f64>;
    fn LastTaskResult(&self) -> ::windows::core::Result<i32>;
    fn NumberOfMissedRuns(&self) -> ::windows::core::Result<i32>;
    fn NextRunTime(&self) -> ::windows::core::Result<f64>;
    fn Definition(&self) -> ::windows::core::Result<ITaskDefinition>;
    fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn Stop(&self, flags: i32) -> ::windows::core::Result<()>;
    fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRegisteredTask {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>() -> IRegisteredTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Run(::core::mem::transmute(&params)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunEx(::core::mem::transmute(&params), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&sessionid), ::core::mem::transmute(&user)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstances(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtasks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastruntime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastTaskResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastTaskResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plasttaskresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfMissedRuns<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfMissedRuns() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumberofmissedruns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnextruntime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Definition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Definition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Xml() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pxml, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psddl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::core::mem::transmute(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop(::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRunTimes(::core::mem::transmute_copy(&pststart), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&pruntimes)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRegisteredTaskCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegisteredTaskCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>() -> IRegisteredTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregisteredtask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRegistrationInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDescription(&self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Author(&self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetAuthor(&self, author: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetVersion(&self, version: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Date(&self, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDate(&self, date: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Documentation(&self, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDocumentation(&self, documentation: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn URI(&self, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetURI(&self, uri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SecurityDescriptor(&self, psddl: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetSecurityDescriptor(&self, sddl: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Source(&self, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSource(&self, source: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRegistrationInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>() -> IRegistrationInfo_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Description(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&description)).into()
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Author(::core::mem::transmute_copy(&pauthor)).into()
        }
        unsafe extern "system" fn SetAuthor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthor(::core::mem::transmute(&author)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Version(::core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&version)).into()
        }
        unsafe extern "system" fn Date<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Date(::core::mem::transmute_copy(&pdate)).into()
        }
        unsafe extern "system" fn SetDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDate(::core::mem::transmute(&date)).into()
        }
        unsafe extern "system" fn Documentation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Documentation(::core::mem::transmute_copy(&pdocumentation)).into()
        }
        unsafe extern "system" fn SetDocumentation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentation(::core::mem::transmute(&documentation)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXmlText(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn URI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.URI(::core::mem::transmute_copy(&puri)).into()
        }
        unsafe extern "system" fn SetURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetURI(::core::mem::transmute(&uri)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SecurityDescriptor(::core::mem::transmute_copy(&psddl)).into()
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::core::mem::transmute(&sddl)).into()
        }
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Source(::core::mem::transmute_copy(&psource)).into()
        }
        unsafe extern "system" fn SetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSource(::core::mem::transmute(&source)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRegistrationTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>() -> IRegistrationTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelay(::core::mem::transmute(&delay)).into()
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(), Delay: Delay::<Identity, Impl, OFFSET>, SetDelay: SetDelay::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegistrationTrigger as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRepetitionPattern_Impl: Sized + super::Com::IDispatch_Impl {
    fn Interval(&self, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetInterval(&self, interval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Duration(&self, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDuration(&self, duration: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopAtDurationEnd(&self, pstop: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopAtDurationEnd(&self, stop: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRepetitionPattern {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRepetitionPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>() -> IRepetitionPattern_Vtbl {
        unsafe extern "system" fn Interval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Interval(::core::mem::transmute_copy(&pinterval)).into()
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterval(::core::mem::transmute(&interval)).into()
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Duration(::core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuration(::core::mem::transmute(&duration)).into()
        }
        unsafe extern "system" fn StopAtDurationEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopAtDurationEnd(::core::mem::transmute_copy(&pstop)).into()
        }
        unsafe extern "system" fn SetStopAtDurationEnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStopAtDurationEnd(::core::mem::transmute_copy(&stop)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InstanceGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<TASK_STATE>;
    fn CurrentAction(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn EnginePID(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRunningTask {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>() -> IRunningTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn EnginePID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnginePID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<IRunningTask>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IRunningTaskCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRunningTaskCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>() -> IRunningTaskCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningTaskCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IScheduledWorkItem_Impl: Sized {
    fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows::core::Result<()>;
    fn DeleteTrigger(&self, itrigger: u16) -> ::windows::core::Result<()>;
    fn GetTriggerCount(&self) -> ::windows::core::Result<u16>;
    fn GetTrigger(&self, itrigger: u16) -> ::windows::core::Result<ITaskTrigger>;
    fn GetTriggerString(&self, itrigger: u16) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
    fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::Result<()>;
    fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::Result<()>;
    fn Run(&self) -> ::windows::core::Result<()>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
    fn EditWorkItem(&self, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetMostRecentRunTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetExitCode(&self) -> ::windows::core::Result<u32>;
    fn SetComment(&self, pwszcomment: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetComment(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetCreator(&self, pwszcreator: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetCreator(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> ::windows::core::Result<()>;
    fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetErrorRetryCount(&self, wretrycount: u16) -> ::windows::core::Result<()>;
    fn GetErrorRetryCount(&self) -> ::windows::core::Result<u16>;
    fn SetErrorRetryInterval(&self, wretryinterval: u16) -> ::windows::core::Result<()>;
    fn GetErrorRetryInterval(&self) -> ::windows::core::Result<u16>;
    fn SetFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetFlags(&self) -> ::windows::core::Result<u32>;
    fn SetAccountInformation(&self, pwszaccountname: &::windows::core::PCWSTR, pwszpassword: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetAccountInformation(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IScheduledWorkItem {}
#[cfg(feature = "Win32_Foundation")]
impl IScheduledWorkItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>() -> IScheduledWorkItem_Vtbl {
        unsafe extern "system" fn CreateTrigger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTrigger(::core::mem::transmute_copy(&pinewtrigger), ::core::mem::transmute_copy(&pptrigger)).into()
        }
        unsafe extern "system" fn DeleteTrigger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTrigger(::core::mem::transmute_copy(&itrigger)).into()
        }
        unsafe extern "system" fn GetTriggerCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTriggerCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrigger(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTriggerString(::core::mem::transmute_copy(&itrigger)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsztrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRunTimes(::core::mem::transmute_copy(&pstbegin), ::core::mem::transmute_copy(&pstend), ::core::mem::transmute_copy(&pcount), ::core::mem::transmute_copy(&rgsttasktimes)).into()
        }
        unsafe extern "system" fn GetNextRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextRunTime(::core::mem::transmute_copy(&pstnextrun)).into()
        }
        unsafe extern "system" fn SetIdleWait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIdleWait(::core::mem::transmute_copy(&widleminutes), ::core::mem::transmute_copy(&wdeadlineminutes)).into()
        }
        unsafe extern "system" fn GetIdleWait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdleWait(::core::mem::transmute_copy(&pwidleminutes), ::core::mem::transmute_copy(&pwdeadlineminutes)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run().into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate().into()
        }
        unsafe extern "system" fn EditWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EditWorkItem(::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetMostRecentRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMostRecentRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastrun, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExitCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwexitcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComment(::core::mem::transmute(&pwszcomment)).into()
        }
        unsafe extern "system" fn GetComment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcomment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcreator: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreator(::core::mem::transmute(&pwszcreator)).into()
        }
        unsafe extern "system" fn GetCreator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcreator: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCreator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcreator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkItemData(::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&rgbdata)).into()
        }
        unsafe extern "system" fn GetWorkItemData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWorkItemData(::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&prgbdata)).into()
        }
        unsafe extern "system" fn SetErrorRetryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetErrorRetryCount(::core::mem::transmute_copy(&wretrycount)).into()
        }
        unsafe extern "system" fn GetErrorRetryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorRetryCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwretrycount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorRetryInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetErrorRetryInterval(::core::mem::transmute_copy(&wretryinterval)).into()
        }
        unsafe extern "system" fn GetErrorRetryInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorRetryInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwretryinterval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszaccountname: ::windows::core::PCWSTR, pwszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAccountInformation(::core::mem::transmute(&pwszaccountname), ::core::mem::transmute(&pwszpassword)).into()
        }
        unsafe extern "system" fn GetAccountInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IScheduledWorkItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccountInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszaccountname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDelay(&self, delay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetUserId(&self, user: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()>;
    fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISessionStateChangeTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISessionStateChangeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>() -> ISessionStateChangeTrigger_Vtbl {
        unsafe extern "system" fn Delay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delay(::core::mem::transmute_copy(&pdelay)).into()
        }
        unsafe extern "system" fn SetDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelay(::core::mem::transmute(&delay)).into()
        }
        unsafe extern "system" fn UserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UserId(::core::mem::transmute_copy(&puser)).into()
        }
        unsafe extern "system" fn SetUserId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserId(::core::mem::transmute(&user)).into()
        }
        unsafe extern "system" fn StateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StateChange(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn SetStateChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStateChange(::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Title(&self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetTitle(&self, title: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MessageBody(&self, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetMessageBody(&self, messagebody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IShowMessageAction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IShowMessageAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: isize>() -> IShowMessageAction_Vtbl {
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Title(::core::mem::transmute_copy(&ptitle)).into()
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn MessageBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MessageBody(::core::mem::transmute_copy(&pmessagebody)).into()
        }
        unsafe extern "system" fn SetMessageBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageBody(::core::mem::transmute(&messagebody)).into()
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn SetApplicationName(&self, pwszapplicationname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetApplicationName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetParameters(&self, pwszparameters: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetParameters(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetWorkingDirectory(&self, pwszworkingdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetWorkingDirectory(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetPriority(&self, dwpriority: u32) -> ::windows::core::Result<()>;
    fn GetPriority(&self) -> ::windows::core::Result<u32>;
    fn SetTaskFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetTaskFlags(&self) -> ::windows::core::Result<u32>;
    fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> ::windows::core::Result<()>;
    fn GetMaxRunTime(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITask {}
#[cfg(feature = "Win32_Foundation")]
impl ITask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>() -> ITask_Vtbl {
        unsafe extern "system" fn SetApplicationName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszapplicationname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationName(::core::mem::transmute(&pwszapplicationname)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszapplicationname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszparameters: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParameters(::core::mem::transmute(&pwszparameters)).into()
        }
        unsafe extern "system" fn GetParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszparameters: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszworkingdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkingDirectory(::core::mem::transmute(&pwszworkingdirectory)).into()
        }
        unsafe extern "system" fn GetWorkingDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszworkingdirectory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&dwpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTaskFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetTaskFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTaskFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxRunTime(::core::mem::transmute_copy(&dwmaxruntimems)).into()
        }
        unsafe extern "system" fn GetMaxRunTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxRunTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxruntimems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IScheduledWorkItem_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn RegistrationInfo(&self) -> ::windows::core::Result<IRegistrationInfo>;
    fn SetRegistrationInfo(&self, pregistrationinfo: &::core::option::Option<IRegistrationInfo>) -> ::windows::core::Result<()>;
    fn Triggers(&self) -> ::windows::core::Result<ITriggerCollection>;
    fn SetTriggers(&self, ptriggers: &::core::option::Option<ITriggerCollection>) -> ::windows::core::Result<()>;
    fn Settings(&self) -> ::windows::core::Result<ITaskSettings>;
    fn SetSettings(&self, psettings: &::core::option::Option<ITaskSettings>) -> ::windows::core::Result<()>;
    fn Data(&self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetData(&self, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Principal(&self) -> ::windows::core::Result<IPrincipal>;
    fn SetPrincipal(&self, pprincipal: &::core::option::Option<IPrincipal>) -> ::windows::core::Result<()>;
    fn Actions(&self) -> ::windows::core::Result<IActionCollection>;
    fn SetActions(&self, pactions: &::core::option::Option<IActionCollection>) -> ::windows::core::Result<()>;
    fn XmlText(&self, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&self, xml: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>() -> ITaskDefinition_Vtbl {
        unsafe extern "system" fn RegistrationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegistrationInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppregistrationinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistrationInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pregistrationinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRegistrationInfo(::core::mem::transmute(&pregistrationinfo)).into()
        }
        unsafe extern "system" fn Triggers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptriggers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Triggers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptriggers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptriggers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTriggers(::core::mem::transmute(&ptriggers)).into()
        }
        unsafe extern "system" fn Settings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Settings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSettings(::core::mem::transmute(&psettings)).into()
        }
        unsafe extern "system" fn Data<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Data(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Principal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprincipal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Principal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprincipal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrincipal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprincipal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrincipal(::core::mem::transmute(&pprincipal)).into()
        }
        unsafe extern "system" fn Actions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppactions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Actions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActions(::core::mem::transmute(&pactions)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.XmlText(::core::mem::transmute_copy(&pxml)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXmlText(::core::mem::transmute(&xml)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFolder(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskFolder>;
    fn GetFolders(&self, flags: i32) -> ::windows::core::Result<ITaskFolderCollection>;
    fn CreateFolder(&self, subfoldername: &super::super::Foundation::BSTR, sddl: &super::Com::VARIANT) -> ::windows::core::Result<ITaskFolder>;
    fn DeleteFolder(&self, subfoldername: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn GetTask(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<IRegisteredTask>;
    fn GetTasks(&self, flags: i32) -> ::windows::core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(&self, name: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
    fn RegisterTask(&self, path: &super::super::Foundation::BSTR, xmltext: &super::super::Foundation::BSTR, flags: i32, userid: &super::Com::VARIANT, password: &super::Com::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(&self, path: &super::super::Foundation::BSTR, pdefinition: &::core::option::Option<ITaskDefinition>, flags: i32, userid: &super::Com::VARIANT, password: &super::Com::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::Com::VARIANT) -> ::windows::core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskFolder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>() -> ITaskFolder_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolder(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolders(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFolder(::core::mem::transmute(&subfoldername), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteFolder(::core::mem::transmute(&subfoldername), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTask(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptasks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&name), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn RegisterTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterTask(::core::mem::transmute(&path), ::core::mem::transmute(&xmltext), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&userid), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: *mut ::core::ffi::c_void, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterTaskDefinition(::core::mem::transmute(&path), ::core::mem::transmute(&pdefinition), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&userid), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&logontype), ::core::mem::transmute(&sddl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityDescriptor(::core::mem::transmute_copy(&securityinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psddl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurityDescriptor(::core::mem::transmute(&sddl), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<ITaskFolder>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskFolderCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskFolderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>() -> ITaskFolderCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITaskFolderCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITaskHandler_Impl: Sized {
    fn Start(&self, phandlerservices: &::core::option::Option<::windows::core::IUnknown>, data: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITaskHandler {}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: isize>() -> ITaskHandler_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute(&phandlerservices), ::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Stop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskCompleted(&self, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITaskHandlerStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ITaskHandlerStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>() -> ITaskHandlerStatus_Vtbl {
        unsafe extern "system" fn UpdateStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateStatus(::core::mem::transmute_copy(&percentcomplete), ::core::mem::transmute(&statusmessage)).into()
        }
        unsafe extern "system" fn TaskCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TaskCompleted(::core::mem::transmute_copy(&taskerrcode)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn get_Item(&self, index: i32) -> ::windows::core::Result<ITaskNamedValuePair>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Create(&self, name: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskNamedValuePair>;
    fn Remove(&self, index: i32) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskNamedValueCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValueCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>() -> ITaskNamedValueCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppair, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&name), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppair, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetName(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&self, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetValue(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskNamedValuePair {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskNamedValuePair_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>() -> ITaskNamedValuePair_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Name(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Value(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
pub trait ITaskScheduler_Impl: Sized {
    fn SetTargetComputer(&self, pwszcomputer: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetTargetComputer(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Enum(&self) -> ::windows::core::Result<IEnumWorkItems>;
    fn Activate(&self, pwszname: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Delete(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn NewWorkItem(&self, pwsztaskname: &::windows::core::PCWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn AddWorkItem(&self, pwsztaskname: &::windows::core::PCWSTR, pworkitem: &::core::option::Option<IScheduledWorkItem>) -> ::windows::core::Result<()>;
    fn IsOfType(&self, pwszname: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITaskScheduler {}
impl ITaskScheduler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>() -> ITaskScheduler_Vtbl {
        unsafe extern "system" fn SetTargetComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszcomputer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetComputer(::core::mem::transmute(&pwszcomputer)).into()
        }
        unsafe extern "system" fn GetTargetComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszcomputer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumworkitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activate(::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn NewWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows::core::PCWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NewWorkItem(::core::mem::transmute(&pwsztaskname), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWorkItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows::core::PCWSTR, pworkitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWorkItem(::core::mem::transmute(&pwsztaskname), ::core::mem::transmute(&pworkitem)).into()
        }
        unsafe extern "system" fn IsOfType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskScheduler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsOfType(::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn GetFolder(&self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITaskFolder>;
    fn GetRunningTasks(&self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection>;
    fn NewTask(&self, flags: u32) -> ::windows::core::Result<ITaskDefinition>;
    fn Connect(&self, servername: &super::Com::VARIANT, user: &super::Com::VARIANT, domain: &super::Com::VARIANT, password: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Connected(&self) -> ::windows::core::Result<i16>;
    fn TargetServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectedUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ConnectedDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HighestVersion(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>() -> ITaskService_Vtbl {
        unsafe extern "system" fn GetFolder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFolder(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfolder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRunningTasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRunningTasks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprunningtasks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NewTask(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefinition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&servername), ::core::mem::transmute(&user), ::core::mem::transmute(&domain), ::core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn Connected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectedDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectedDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighestVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HighestVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AllowDemandStart(&self, pallowdemandstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetAllowDemandStart(&self, allowdemandstart: i16) -> ::windows::core::Result<()>;
    fn RestartInterval(&self, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRestartInterval(&self, restartinterval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RestartCount(&self, prestartcount: *mut i32) -> ::windows::core::Result<()>;
    fn SetRestartCount(&self, restartcount: i32) -> ::windows::core::Result<()>;
    fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::Result<()>;
    fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> ::windows::core::Result<()>;
    fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut i16) -> ::windows::core::Result<()>;
    fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: i16) -> ::windows::core::Result<()>;
    fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartIfOnBatteries(&self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn AllowHardTerminate(&self, pallowhardterminate: *mut i16) -> ::windows::core::Result<()>;
    fn SetAllowHardTerminate(&self, allowhardterminate: i16) -> ::windows::core::Result<()>;
    fn StartWhenAvailable(&self, pstartwhenavailable: *mut i16) -> ::windows::core::Result<()>;
    fn SetStartWhenAvailable(&self, startwhenavailable: i16) -> ::windows::core::Result<()>;
    fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetXmlText(&self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: i16) -> ::windows::core::Result<()>;
    fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExecutionTimeLimit(&self, executiontimelimit: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
    fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetDeleteExpiredTaskAfter(&self, expirationdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Priority(&self, ppriority: *mut i32) -> ::windows::core::Result<()>;
    fn SetPriority(&self, priority: i32) -> ::windows::core::Result<()>;
    fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::Result<()>;
    fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::Result<()>;
    fn Hidden(&self, phidden: *mut i16) -> ::windows::core::Result<()>;
    fn SetHidden(&self, hidden: i16) -> ::windows::core::Result<()>;
    fn IdleSettings(&self) -> ::windows::core::Result<IIdleSettings>;
    fn SetIdleSettings(&self, pidlesettings: &::core::option::Option<IIdleSettings>) -> ::windows::core::Result<()>;
    fn RunOnlyIfIdle(&self, prunonlyifidle: *mut i16) -> ::windows::core::Result<()>;
    fn SetRunOnlyIfIdle(&self, runonlyifidle: i16) -> ::windows::core::Result<()>;
    fn WakeToRun(&self, pwake: *mut i16) -> ::windows::core::Result<()>;
    fn SetWakeToRun(&self, wake: i16) -> ::windows::core::Result<()>;
    fn NetworkSettings(&self) -> ::windows::core::Result<INetworkSettings>;
    fn SetNetworkSettings(&self, pnetworksettings: &::core::option::Option<INetworkSettings>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>() -> ITaskSettings_Vtbl {
        unsafe extern "system" fn AllowDemandStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllowDemandStart(::core::mem::transmute_copy(&pallowdemandstart)).into()
        }
        unsafe extern "system" fn SetAllowDemandStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowDemandStart(::core::mem::transmute_copy(&allowdemandstart)).into()
        }
        unsafe extern "system" fn RestartInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartInterval(::core::mem::transmute_copy(&prestartinterval)).into()
        }
        unsafe extern "system" fn SetRestartInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRestartInterval(::core::mem::transmute(&restartinterval)).into()
        }
        unsafe extern "system" fn RestartCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestartCount(::core::mem::transmute_copy(&prestartcount)).into()
        }
        unsafe extern "system" fn SetRestartCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRestartCount(::core::mem::transmute_copy(&restartcount)).into()
        }
        unsafe extern "system" fn MultipleInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MultipleInstances(::core::mem::transmute_copy(&ppolicy)).into()
        }
        unsafe extern "system" fn SetMultipleInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultipleInstances(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn StopIfGoingOnBatteries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopIfGoingOnBatteries(::core::mem::transmute_copy(&pstopifonbatteries)).into()
        }
        unsafe extern "system" fn SetStopIfGoingOnBatteries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStopIfGoingOnBatteries(::core::mem::transmute_copy(&stopifonbatteries)).into()
        }
        unsafe extern "system" fn DisallowStartIfOnBatteries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisallowStartIfOnBatteries(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartIfOnBatteries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisallowStartIfOnBatteries(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn AllowHardTerminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllowHardTerminate(::core::mem::transmute_copy(&pallowhardterminate)).into()
        }
        unsafe extern "system" fn SetAllowHardTerminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowHardTerminate(::core::mem::transmute_copy(&allowhardterminate)).into()
        }
        unsafe extern "system" fn StartWhenAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartWhenAvailable(::core::mem::transmute_copy(&pstartwhenavailable)).into()
        }
        unsafe extern "system" fn SetStartWhenAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartWhenAvailable(::core::mem::transmute_copy(&startwhenavailable)).into()
        }
        unsafe extern "system" fn XmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.XmlText(::core::mem::transmute_copy(&ptext)).into()
        }
        unsafe extern "system" fn SetXmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetXmlText(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn RunOnlyIfNetworkAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&prunonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfNetworkAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunOnlyIfNetworkAvailable(::core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecutionTimeLimit(::core::mem::transmute_copy(&pexecutiontimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExecutionTimeLimit(::core::mem::transmute(&executiontimelimit)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn DeleteExpiredTaskAfter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteExpiredTaskAfter(::core::mem::transmute_copy(&pexpirationdelay)).into()
        }
        unsafe extern "system" fn SetDeleteExpiredTaskAfter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeleteExpiredTaskAfter(::core::mem::transmute(&expirationdelay)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Priority(::core::mem::transmute_copy(&ppriority)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Compatibility<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compatibility(::core::mem::transmute_copy(&pcompatlevel)).into()
        }
        unsafe extern "system" fn SetCompatibility<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompatibility(::core::mem::transmute_copy(&compatlevel)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hidden(::core::mem::transmute_copy(&phidden)).into()
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHidden(::core::mem::transmute_copy(&hidden)).into()
        }
        unsafe extern "system" fn IdleSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidlesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IdleSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidlesettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIdleSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidlesettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIdleSettings(::core::mem::transmute(&pidlesettings)).into()
        }
        unsafe extern "system" fn RunOnlyIfIdle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunOnlyIfIdle(::core::mem::transmute_copy(&prunonlyifidle)).into()
        }
        unsafe extern "system" fn SetRunOnlyIfIdle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRunOnlyIfIdle(::core::mem::transmute_copy(&runonlyifidle)).into()
        }
        unsafe extern "system" fn WakeToRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WakeToRun(::core::mem::transmute_copy(&pwake)).into()
        }
        unsafe extern "system" fn SetWakeToRun<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWakeToRun(::core::mem::transmute_copy(&wake)).into()
        }
        unsafe extern "system" fn NetworkSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NetworkSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworksettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkSettings(::core::mem::transmute(&pnetworksettings)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskSettings2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: isize>() -> ITaskSettings2_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()>;
    fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows::core::Result<()>;
    fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()>;
    fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows::core::Result<()>;
    fn MaintenanceSettings(&self) -> ::windows::core::Result<IMaintenanceSettings>;
    fn SetMaintenanceSettings(&self, pmaintenancesettings: &::core::option::Option<IMaintenanceSettings>) -> ::windows::core::Result<()>;
    fn CreateMaintenanceSettings(&self) -> ::windows::core::Result<IMaintenanceSettings>;
    fn Volatile(&self, pvolatile: *mut i16) -> ::windows::core::Result<()>;
    fn SetVolatile(&self, volatile: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITaskSettings3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITaskSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>() -> ITaskSettings3_Vtbl {
        unsafe extern "system" fn DisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&pdisallowstart)).into()
        }
        unsafe extern "system" fn SetDisallowStartOnRemoteAppSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisallowStartOnRemoteAppSession(::core::mem::transmute_copy(&disallowstart)).into()
        }
        unsafe extern "system" fn UseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UseUnifiedSchedulingEngine(::core::mem::transmute_copy(&puseunifiedengine)).into()
        }
        unsafe extern "system" fn SetUseUnifiedSchedulingEngine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseUnifiedSchedulingEngine(::core::mem::transmute_copy(&useunifiedengine)).into()
        }
        unsafe extern "system" fn MaintenanceSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmaintenancesettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaintenancesettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaintenanceSettings(::core::mem::transmute(&pmaintenancesettings)).into()
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateMaintenanceSettings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmaintenancesettings, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volatile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Volatile(::core::mem::transmute_copy(&pvolatile)).into()
        }
        unsafe extern "system" fn SetVolatile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolatile(::core::mem::transmute_copy(&volatile)).into()
        }
        Self {
            base__: ITaskSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
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
pub trait ITaskTrigger_Impl: Sized {
    fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> ::windows::core::Result<()>;
    fn GetTrigger(&self) -> ::windows::core::Result<TASK_TRIGGER>;
    fn GetTriggerString(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for ITaskTrigger {}
impl ITaskTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: isize>() -> ITaskTrigger_Vtbl {
        unsafe extern "system" fn SetTrigger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrigger(::core::mem::transmute_copy(&ptrigger)).into()
        }
        unsafe extern "system" fn GetTrigger<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTriggerString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTriggerString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwsztrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn GetInput(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOutput(&self, input: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetContext(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITaskVariables {}
#[cfg(feature = "Win32_Foundation")]
impl ITaskVariables_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: isize>() -> ITaskVariables_Vtbl {
        unsafe extern "system" fn GetInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinput: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutput(::core::mem::transmute(&input)).into()
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITaskVariables_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITimeTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: isize>() -> ITimeTrigger_Vtbl {
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRandomDelay(::core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()>;
    fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetId(&self, id: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern>;
    fn SetRepetition(&self, prepeat: &::core::option::Option<IRepetitionPattern>) -> ::windows::core::Result<()>;
    fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetExecutionTimeLimit(&self, timelimit: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetStartBoundary(&self, start: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetEndBoundary(&self, end: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()>;
    fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>() -> ITrigger_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Type(::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Id(::core::mem::transmute_copy(&pid)).into()
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id)).into()
        }
        unsafe extern "system" fn Repetition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprepeat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Repetition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprepeat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepetition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prepeat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRepetition(::core::mem::transmute(&prepeat)).into()
        }
        unsafe extern "system" fn ExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecutionTimeLimit(::core::mem::transmute_copy(&ptimelimit)).into()
        }
        unsafe extern "system" fn SetExecutionTimeLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExecutionTimeLimit(::core::mem::transmute(&timelimit)).into()
        }
        unsafe extern "system" fn StartBoundary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartBoundary(::core::mem::transmute_copy(&pstart)).into()
        }
        unsafe extern "system" fn SetStartBoundary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartBoundary(::core::mem::transmute(&start)).into()
        }
        unsafe extern "system" fn EndBoundary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndBoundary(::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetEndBoundary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndBoundary(::core::mem::transmute(&end)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enabled(::core::mem::transmute_copy(&penabled)).into()
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()>;
    fn get_Item(&self, index: i32) -> ::windows::core::Result<ITrigger>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> ::windows::core::Result<ITrigger>;
    fn Remove(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITriggerCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITriggerCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>() -> ITriggerCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrigger, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows::core::Result<()>;
    fn SetDaysOfWeek(&self, days: i16) -> ::windows::core::Result<()>;
    fn WeeksInterval(&self, pweeks: *mut i16) -> ::windows::core::Result<()>;
    fn SetWeeksInterval(&self, weeks: i16) -> ::windows::core::Result<()>;
    fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetRandomDelay(&self, randomdelay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWeeklyTrigger {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWeeklyTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>() -> IWeeklyTrigger_Vtbl {
        unsafe extern "system" fn DaysOfWeek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DaysOfWeek(::core::mem::transmute_copy(&pdays)).into()
        }
        unsafe extern "system" fn SetDaysOfWeek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaysOfWeek(::core::mem::transmute_copy(&days)).into()
        }
        unsafe extern "system" fn WeeksInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WeeksInterval(::core::mem::transmute_copy(&pweeks)).into()
        }
        unsafe extern "system" fn SetWeeksInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWeeksInterval(::core::mem::transmute_copy(&weeks)).into()
        }
        unsafe extern "system" fn RandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RandomDelay(::core::mem::transmute_copy(&prandomdelay)).into()
        }
        unsafe extern "system" fn SetRandomDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRandomDelay(::core::mem::transmute(&randomdelay)).into()
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, Impl, OFFSET>(),
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
