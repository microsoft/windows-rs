pub trait AsyncIDebugApplicationNodeEvents_Impl: Sized {
    fn Begin_onAddChild(&self, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onAddChild(&self) -> ::windows_core::Result<()>;
    fn Begin_onRemoveChild(&self, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onRemoveChild(&self) -> ::windows_core::Result<()>;
    fn Begin_onDetach(&self) -> ::windows_core::Result<()>;
    fn Finish_onDetach(&self) -> ::windows_core::Result<()>;
    fn Begin_onAttach(&self, prddpparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onAttach(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for AsyncIDebugApplicationNodeEvents {}
impl AsyncIDebugApplicationNodeEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>() -> AsyncIDebugApplicationNodeEvents_Vtbl {
        unsafe extern "system" fn Begin_onAddChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_onAddChild(::windows_core::from_raw_borrowed(&prddpchild)).into()
        }
        unsafe extern "system" fn Finish_onAddChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_onAddChild().into()
        }
        unsafe extern "system" fn Begin_onRemoveChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_onRemoveChild(::windows_core::from_raw_borrowed(&prddpchild)).into()
        }
        unsafe extern "system" fn Finish_onRemoveChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_onRemoveChild().into()
        }
        unsafe extern "system" fn Begin_onDetach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_onDetach().into()
        }
        unsafe extern "system" fn Finish_onDetach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_onDetach().into()
        }
        unsafe extern "system" fn Begin_onAttach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_onAttach(::windows_core::from_raw_borrowed(&prddpparent)).into()
        }
        unsafe extern "system" fn Finish_onAttach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_onAttach().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_onAddChild: Begin_onAddChild::<Identity, Impl, OFFSET>,
            Finish_onAddChild: Finish_onAddChild::<Identity, Impl, OFFSET>,
            Begin_onRemoveChild: Begin_onRemoveChild::<Identity, Impl, OFFSET>,
            Finish_onRemoveChild: Finish_onRemoveChild::<Identity, Impl, OFFSET>,
            Begin_onDetach: Begin_onDetach::<Identity, Impl, OFFSET>,
            Finish_onDetach: Finish_onDetach::<Identity, Impl, OFFSET>,
            Begin_onAttach: Begin_onAttach::<Identity, Impl, OFFSET>,
            Finish_onAttach: Finish_onAttach::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <AsyncIDebugApplicationNodeEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScript_Impl: Sized {
    fn SetScriptSite(&self, pass: ::core::option::Option<&IActiveScriptSite>) -> ::windows_core::Result<()>;
    fn GetScriptSite(&self, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetScriptState(&self, ss: SCRIPTSTATE) -> ::windows_core::Result<()>;
    fn GetScriptState(&self) -> ::windows_core::Result<SCRIPTSTATE>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn AddNamedItem(&self, pstrname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn AddTypeLib(&self, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScriptDispatch(&self, pstritemname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
    fn GetCurrentScriptThreadID(&self) -> ::windows_core::Result<u32>;
    fn GetScriptThreadID(&self, dwwin32threadid: u32) -> ::windows_core::Result<u32>;
    fn GetScriptThreadState(&self, stidthread: u32) -> ::windows_core::Result<SCRIPTTHREADSTATE>;
    fn InterruptScriptThread(&self, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IActiveScript>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScript {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScript_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>() -> IActiveScript_Vtbl {
        unsafe extern "system" fn SetScriptSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pass: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScriptSite(::windows_core::from_raw_borrowed(&pass)).into()
        }
        unsafe extern "system" fn GetScriptSite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptSite(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn SetScriptState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ss: SCRIPTSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScriptState(::core::mem::transmute_copy(&ss)).into()
        }
        unsafe extern "system" fn GetScriptState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pssstate: *mut SCRIPTSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pssstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn AddNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNamedItem(::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn AddTypeLib<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTypeLib(::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScriptDispatch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstritemname: ::windows_core::PCWSTR, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptDispatch(::core::mem::transmute(&pstritemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentScriptThreadID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstidthread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentScriptThreadID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstidthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptThreadID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwwin32threadid: u32, pstidthread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptThreadID(::core::mem::transmute_copy(&dwwin32threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstidthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptThreadState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stidthread: u32, pstsstate: *mut SCRIPTTHREADSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptThreadState(::core::mem::transmute_copy(&stidthread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstsstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterruptScriptThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InterruptScriptThread(::core::mem::transmute_copy(&stidthread), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscript: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscript, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScriptSite: SetScriptSite::<Identity, Impl, OFFSET>,
            GetScriptSite: GetScriptSite::<Identity, Impl, OFFSET>,
            SetScriptState: SetScriptState::<Identity, Impl, OFFSET>,
            GetScriptState: GetScriptState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            AddNamedItem: AddNamedItem::<Identity, Impl, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, Impl, OFFSET>,
            GetScriptDispatch: GetScriptDispatch::<Identity, Impl, OFFSET>,
            GetCurrentScriptThreadID: GetCurrentScriptThreadID::<Identity, Impl, OFFSET>,
            GetScriptThreadID: GetScriptThreadID::<Identity, Impl, OFFSET>,
            GetScriptThreadState: GetScriptThreadState::<Identity, Impl, OFFSET>,
            InterruptScriptThread: InterruptScriptThread::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScript as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IActiveScriptAuthor_Impl: Sized {
    fn AddNamedItem(&self, pszname: &::windows_core::PCWSTR, dwflags: u32, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddScriptlet(&self, pszdefaultname: &::windows_core::PCWSTR, pszcode: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszsubitemname: &::windows_core::PCWSTR, pszeventname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn ParseScriptText(&self, pszcode: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScriptTextAttributes(&self, pszcode: &::windows_core::PCWSTR, cch: u32, pszdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pszcode: &::windows_core::PCWSTR, cch: u32, pszdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetRoot(&self) -> ::windows_core::Result<IScriptNode>;
    fn GetLanguageFlags(&self) -> ::windows_core::Result<u32>;
    fn GetEventHandler(&self, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>, pszitem: &::windows_core::PCWSTR, pszsubitem: &::windows_core::PCWSTR, pszevent: &::windows_core::PCWSTR) -> ::windows_core::Result<IScriptEntry>;
    fn RemoveNamedItem(&self, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddTypeLib(&self, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn RemoveTypeLib(&self, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32) -> ::windows_core::Result<()>;
    fn GetChars(&self, frequestedlist: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInfoFromContext(&self, pszcode: &::windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn IsCommitChar(&self, ch: u16) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IActiveScriptAuthor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IActiveScriptAuthor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>() -> IActiveScriptAuthor_Vtbl {
        unsafe extern "system" fn AddNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, dwflags: u32, pdisp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNamedItem(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdisp)).into()
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows_core::PCWSTR, pszcode: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszsubitemname: ::windows_core::PCWSTR, pszeventname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddScriptlet(::core::mem::transmute(&pszdefaultname), ::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszsubitemname), ::core::mem::transmute(&pszeventname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseScriptText(::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cch: u32, pszdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptTextAttributes(::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cch: u32, pszdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptletTextAttributes(::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn GetRoot<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRoot() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrfasa: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrfasa, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, pszsubitem: ::windows_core::PCWSTR, pszevent: ::windows_core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventHandler(::windows_core::from_raw_borrowed(&pdisp), ::core::mem::transmute(&pszitem), ::core::mem::transmute(&pszsubitem), ::core::mem::transmute(&pszevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNamedItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNamedItem(::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn AddTypeLib<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTypeLib(::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RemoveTypeLib<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveTypeLib(::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor)).into()
        }
        unsafe extern "system" fn GetChars<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequestedlist: u32, pbstrchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChars(::core::mem::transmute_copy(&frequestedlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfoFromContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfoFromContext(::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cchcode), ::core::mem::transmute_copy(&ichcurrentposition), ::core::mem::transmute_copy(&dwlisttypesrequested), ::core::mem::transmute_copy(&pdwlisttypesprovided), ::core::mem::transmute_copy(&pichlistanchorposition), ::core::mem::transmute_copy(&pichfuncanchorposition), ::core::mem::transmute_copy(&pmemid), ::core::mem::transmute_copy(&picurrentparameter), ::core::mem::transmute_copy(&ppunk))
                .into()
        }
        unsafe extern "system" fn IsCommitChar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ch: u16, pfcommit: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCommitChar(::core::mem::transmute_copy(&ch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcommit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamedItem: AddNamedItem::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            GetRoot: GetRoot::<Identity, Impl, OFFSET>,
            GetLanguageFlags: GetLanguageFlags::<Identity, Impl, OFFSET>,
            GetEventHandler: GetEventHandler::<Identity, Impl, OFFSET>,
            RemoveNamedItem: RemoveNamedItem::<Identity, Impl, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, Impl, OFFSET>,
            RemoveTypeLib: RemoveTypeLib::<Identity, Impl, OFFSET>,
            GetChars: GetChars::<Identity, Impl, OFFSET>,
            GetInfoFromContext: GetInfoFromContext::<Identity, Impl, OFFSET>,
            IsCommitChar: IsCommitChar::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptAuthor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptAuthorProcedure_Impl: Sized {
    fn ParseProcedureText(&self, pszcode: &::windows_core::PCWSTR, pszformalparams: &::windows_core::PCWSTR, pszprocedurename: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptAuthorProcedure {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptAuthorProcedure_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthorProcedure_Impl, const OFFSET: isize>() -> IActiveScriptAuthorProcedure_Vtbl {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptAuthorProcedure_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, pszformalparams: ::windows_core::PCWSTR, pszprocedurename: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseProcedureText(::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszformalparams), ::core::mem::transmute(&pszprocedurename), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdispfor)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptAuthorProcedure as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptDebug32_Impl: Sized {
    fn GetScriptTextAttributes(&self, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::RuntimeName for IActiveScriptDebug32 {}
impl IActiveScriptDebug32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: isize>() -> IActiveScriptDebug32_Vtbl {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptTextAttributes(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptletTextAttributes(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCodeContextsOfPosition(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptDebug32 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptDebug64_Impl: Sized {
    fn GetScriptTextAttributes(&self, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::RuntimeName for IActiveScriptDebug64 {}
impl IActiveScriptDebug64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: isize>() -> IActiveScriptDebug64_Vtbl {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptTextAttributes(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptletTextAttributes(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCodeContextsOfPosition(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptDebug64 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptEncode_Impl: Sized {
    fn EncodeSection(&self, pchin: &::windows_core::PCWSTR, cchin: u32, pchout: &::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::Result<()>;
    fn DecodeScript(&self, pchin: &::windows_core::PCWSTR, cchin: u32, pchout: &::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::Result<()>;
    fn GetEncodeProgId(&self, pbstrout: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptEncode {}
impl IActiveScriptEncode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: isize>() -> IActiveScriptEncode_Vtbl {
        unsafe extern "system" fn EncodeSection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchin: ::windows_core::PCWSTR, cchin: u32, pchout: ::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncodeSection(::core::mem::transmute(&pchin), ::core::mem::transmute_copy(&cchin), ::core::mem::transmute(&pchout), ::core::mem::transmute_copy(&cchout), ::core::mem::transmute_copy(&pcchret)).into()
        }
        unsafe extern "system" fn DecodeScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pchin: ::windows_core::PCWSTR, cchin: u32, pchout: ::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DecodeScript(::core::mem::transmute(&pchin), ::core::mem::transmute_copy(&cchin), ::core::mem::transmute(&pchout), ::core::mem::transmute_copy(&cchout), ::core::mem::transmute_copy(&pcchret)).into()
        }
        unsafe extern "system" fn GetEncodeProgId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrout: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEncodeProgId(::core::mem::transmute_copy(&pbstrout)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncodeSection: EncodeSection::<Identity, Impl, OFFSET>,
            DecodeScript: DecodeScript::<Identity, Impl, OFFSET>,
            GetEncodeProgId: GetEncodeProgId::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptEncode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError_Impl: Sized {
    fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::Result<()>;
    fn GetSourceLineText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptError {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptError_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: isize>() -> IActiveScriptError_Vtbl {
        unsafe extern "system" fn GetExceptionInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExceptionInfo(::core::mem::transmute_copy(&pexcepinfo)).into()
        }
        unsafe extern "system" fn GetSourcePosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSourcePosition(::core::mem::transmute_copy(&pdwsourcecontext), ::core::mem::transmute_copy(&pullinenumber), ::core::mem::transmute_copy(&plcharacterposition)).into()
        }
        unsafe extern "system" fn GetSourceLineText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsourceline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceLineText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsourceline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetExceptionInfo: GetExceptionInfo::<Identity, Impl, OFFSET>,
            GetSourcePosition: GetSourcePosition::<Identity, Impl, OFFSET>,
            GetSourceLineText: GetSourceLineText::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptError as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError64_Impl: Sized + IActiveScriptError_Impl {
    fn GetSourcePosition64(&self, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptError64 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptError64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError64_Impl, const OFFSET: isize>() -> IActiveScriptError64_Vtbl {
        unsafe extern "system" fn GetSourcePosition64<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptError64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSourcePosition64(::core::mem::transmute_copy(&pdwsourcecontext), ::core::mem::transmute_copy(&pullinenumber), ::core::mem::transmute_copy(&plcharacterposition)).into()
        }
        Self { base__: IActiveScriptError_Vtbl::new::<Identity, Impl, OFFSET>(), GetSourcePosition64: GetSourcePosition64::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptError64 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptError as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptErrorDebug_Impl: Sized + IActiveScriptError_Impl {
    fn GetDocumentContext(&self) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetStackFrame(&self) -> ::windows_core::Result<IDebugStackFrame>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptErrorDebug {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptErrorDebug_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: isize>() -> IActiveScriptErrorDebug_Vtbl {
        unsafe extern "system" fn GetDocumentContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppssc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppssc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStackFrame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStackFrame() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IActiveScriptError_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentContext: GetDocumentContext::<Identity, Impl, OFFSET>,
            GetStackFrame: GetStackFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptErrorDebug as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptError as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptErrorDebug110_Impl: Sized {
    fn GetExceptionThrownKind(&self) -> ::windows_core::Result<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND>;
}
impl ::windows_core::RuntimeName for IActiveScriptErrorDebug110 {}
impl IActiveScriptErrorDebug110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptErrorDebug110_Impl, const OFFSET: isize>() -> IActiveScriptErrorDebug110_Vtbl {
        unsafe extern "system" fn GetExceptionThrownKind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptErrorDebug110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexceptionkind: *mut SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExceptionThrownKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexceptionkind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetExceptionThrownKind: GetExceptionThrownKind::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptErrorDebug110 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptGarbageCollector_Impl: Sized {
    fn CollectGarbage(&self, scriptgctype: SCRIPTGCTYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptGarbageCollector {}
impl IActiveScriptGarbageCollector_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptGarbageCollector_Impl, const OFFSET: isize>() -> IActiveScriptGarbageCollector_Vtbl {
        unsafe extern "system" fn CollectGarbage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptGarbageCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptgctype: SCRIPTGCTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CollectGarbage(::core::mem::transmute_copy(&scriptgctype)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CollectGarbage: CollectGarbage::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptGarbageCollector as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptHostEncode_Impl: Sized {
    fn EncodeScriptHostFile(&self, bstrinfile: &::windows_core::BSTR, pbstroutfile: *mut ::windows_core::BSTR, cflags: u32, bstrdefaultlang: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptHostEncode {}
impl IActiveScriptHostEncode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptHostEncode_Impl, const OFFSET: isize>() -> IActiveScriptHostEncode_Vtbl {
        unsafe extern "system" fn EncodeScriptHostFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptHostEncode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, cflags: u32, bstrdefaultlang: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EncodeScriptHostFile(::core::mem::transmute(&bstrinfile), ::core::mem::transmute_copy(&pbstroutfile), ::core::mem::transmute_copy(&cflags), ::core::mem::transmute(&bstrdefaultlang)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EncodeScriptHostFile: EncodeScriptHostFile::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptHostEncode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse32_Impl: Sized {
    fn InitNew(&self) -> ::windows_core::Result<()>;
    fn AddScriptlet(&self, pstrdefaultname: &::windows_core::PCWSTR, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, pstrsubitemname: &::windows_core::PCWSTR, pstreventname: &::windows_core::PCWSTR, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn ParseScriptText(&self, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IActiveScriptParse32 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptParse32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: isize>() -> IActiveScriptParse32_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows_core::PCWSTR, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, pstrsubitemname: ::windows_core::PCWSTR, pstreventname: ::windows_core::PCWSTR, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddScriptlet(::core::mem::transmute(&pstrdefaultname), ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::core::mem::transmute(&pstrsubitemname), ::core::mem::transmute(&pstreventname), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pexcepinfo))
                .into()
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseScriptText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParse32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse64_Impl: Sized {
    fn InitNew(&self) -> ::windows_core::Result<()>;
    fn AddScriptlet(&self, pstrdefaultname: &::windows_core::PCWSTR, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, pstrsubitemname: &::windows_core::PCWSTR, pstreventname: &::windows_core::PCWSTR, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn ParseScriptText(&self, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IActiveScriptParse64 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptParse64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: isize>() -> IActiveScriptParse64_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows_core::PCWSTR, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, pstrsubitemname: ::windows_core::PCWSTR, pstreventname: ::windows_core::PCWSTR, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddScriptlet(::core::mem::transmute(&pstrdefaultname), ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::core::mem::transmute(&pstrsubitemname), ::core::mem::transmute(&pstreventname), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pexcepinfo))
                .into()
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseScriptText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParse64 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_32_Impl: Sized + IActiveScriptParseProcedure32_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedure2_32 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure2_32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure2_32_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedure2_32_Vtbl {
        Self { base__: IActiveScriptParseProcedure32_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedure2_32 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptParseProcedure32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_64_Impl: Sized + IActiveScriptParseProcedure64_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedure2_64 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure2_64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure2_64_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedure2_64_Vtbl {
        Self { base__: IActiveScriptParseProcedure64_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedure2_64 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptParseProcedure64 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure32_Impl: Sized {
    fn ParseProcedureText(&self, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstrprocedurename: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedure32 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure32_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedure32_Vtbl {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstrprocedurename: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParseProcedureText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstrprocedurename), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedure32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure64_Impl: Sized {
    fn ParseProcedureText(&self, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstrprocedurename: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedure64 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure64_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedure64_Vtbl {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedure64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstrprocedurename: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParseProcedureText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstrprocedurename), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedure64 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld32_Impl: Sized {
    fn ParseProcedureText(&self, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedureOld32 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedureOld32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedureOld32_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedureOld32_Vtbl {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedureOld32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParseProcedureText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedureOld32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld64_Impl: Sized {
    fn ParseProcedureText(&self, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptParseProcedureOld64 {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedureOld64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedureOld64_Impl, const OFFSET: isize>() -> IActiveScriptParseProcedureOld64_Vtbl {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptParseProcedureOld64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParseProcedureText(::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptParseProcedureOld64 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerCallback_Impl: Sized {
    fn Initialize(&self, dwcontext: u32) -> ::windows_core::Result<()>;
    fn Shutdown(&self, hrreason: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ScriptCompiled(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn FunctionCompiled(&self, functionid: i32, scriptid: i32, pwszfunctionname: &::windows_core::PCWSTR, pwszfunctionnamehint: &::windows_core::PCWSTR, pidebugdocumentcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows_core::Result<()>;
    fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerCallback {}
impl IActiveScriptProfilerCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>() -> IActiveScriptProfilerCallback_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&dwcontext)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown(::core::mem::transmute_copy(&hrreason)).into()
        }
        unsafe extern "system" fn ScriptCompiled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScriptCompiled(::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&r#type), ::windows_core::from_raw_borrowed(&pidebugdocumentcontext)).into()
        }
        unsafe extern "system" fn FunctionCompiled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, functionid: i32, scriptid: i32, pwszfunctionname: ::windows_core::PCWSTR, pwszfunctionnamehint: ::windows_core::PCWSTR, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FunctionCompiled(::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&scriptid), ::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute(&pwszfunctionnamehint), ::windows_core::from_raw_borrowed(&pidebugdocumentcontext)).into()
        }
        unsafe extern "system" fn OnFunctionEnter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFunctionEnter(::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn OnFunctionExit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFunctionExit(::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&functionid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            ScriptCompiled: ScriptCompiled::<Identity, Impl, OFFSET>,
            FunctionCompiled: FunctionCompiled::<Identity, Impl, OFFSET>,
            OnFunctionEnter: OnFunctionEnter::<Identity, Impl, OFFSET>,
            OnFunctionExit: OnFunctionExit::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerCallback as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerCallback2_Impl: Sized + IActiveScriptProfilerCallback_Impl {
    fn OnFunctionEnterByName(&self, pwszfunctionname: &::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::Result<()>;
    fn OnFunctionExitByName(&self, pwszfunctionname: &::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerCallback2 {}
impl IActiveScriptProfilerCallback2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>() -> IActiveScriptProfilerCallback2_Vtbl {
        unsafe extern "system" fn OnFunctionEnterByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFunctionEnterByName(::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn OnFunctionExitByName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFunctionExitByName(::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base__: IActiveScriptProfilerCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnFunctionEnterByName: OnFunctionEnterByName::<Identity, Impl, OFFSET>,
            OnFunctionExitByName: OnFunctionExitByName::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerCallback2 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerCallback as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerCallback3_Impl: Sized + IActiveScriptProfilerCallback2_Impl {
    fn SetWebWorkerId(&self, webworkerid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerCallback3 {}
impl IActiveScriptProfilerCallback3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback3_Impl, const OFFSET: isize>() -> IActiveScriptProfilerCallback3_Vtbl {
        unsafe extern "system" fn SetWebWorkerId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerCallback3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webworkerid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWebWorkerId(::core::mem::transmute_copy(&webworkerid)).into()
        }
        Self { base__: IActiveScriptProfilerCallback2_Vtbl::new::<Identity, Impl, OFFSET>(), SetWebWorkerId: SetWebWorkerId::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerCallback3 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerCallback as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerCallback2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerControl_Impl: Sized {
    fn StartProfiling(&self, clsidprofilerobject: *const ::windows_core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows_core::Result<()>;
    fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows_core::Result<()>;
    fn StopProfiling(&self, hrshutdownreason: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerControl {}
impl IActiveScriptProfilerControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: isize>() -> IActiveScriptProfilerControl_Vtbl {
        unsafe extern "system" fn StartProfiling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidprofilerobject: *const ::windows_core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartProfiling(::core::mem::transmute_copy(&clsidprofilerobject), ::core::mem::transmute_copy(&dweventmask), ::core::mem::transmute_copy(&dwcontext)).into()
        }
        unsafe extern "system" fn SetProfilerEventMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProfilerEventMask(::core::mem::transmute_copy(&dweventmask)).into()
        }
        unsafe extern "system" fn StopProfiling<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrshutdownreason: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopProfiling(::core::mem::transmute_copy(&hrshutdownreason)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartProfiling: StartProfiling::<Identity, Impl, OFFSET>,
            SetProfilerEventMask: SetProfilerEventMask::<Identity, Impl, OFFSET>,
            StopProfiling: StopProfiling::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerControl as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerControl2_Impl: Sized + IActiveScriptProfilerControl_Impl {
    fn CompleteProfilerStart(&self) -> ::windows_core::Result<()>;
    fn PrepareProfilerStop(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerControl2 {}
impl IActiveScriptProfilerControl2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>() -> IActiveScriptProfilerControl2_Vtbl {
        unsafe extern "system" fn CompleteProfilerStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteProfilerStart().into()
        }
        unsafe extern "system" fn PrepareProfilerStop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareProfilerStop().into()
        }
        Self {
            base__: IActiveScriptProfilerControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            CompleteProfilerStart: CompleteProfilerStart::<Identity, Impl, OFFSET>,
            PrepareProfilerStop: PrepareProfilerStop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerControl2 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerControl3_Impl: Sized + IActiveScriptProfilerControl2_Impl {
    fn EnumHeap(&self) -> ::windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerControl3 {}
impl IActiveScriptProfilerControl3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl3_Impl, const OFFSET: isize>() -> IActiveScriptProfilerControl3_Vtbl {
        unsafe extern "system" fn EnumHeap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumHeap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IActiveScriptProfilerControl2_Vtbl::new::<Identity, Impl, OFFSET>(), EnumHeap: EnumHeap::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerControl3 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerControl4_Impl: Sized + IActiveScriptProfilerControl3_Impl {
    fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerControl4 {}
impl IActiveScriptProfilerControl4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl4_Impl, const OFFSET: isize>() -> IActiveScriptProfilerControl4_Vtbl {
        unsafe extern "system" fn SummarizeHeap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SummarizeHeap(::core::mem::transmute_copy(&heapsummary)).into()
        }
        Self { base__: IActiveScriptProfilerControl3_Vtbl::new::<Identity, Impl, OFFSET>(), SummarizeHeap: SummarizeHeap::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerControl4 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl2 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl3 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerControl5_Impl: Sized + IActiveScriptProfilerControl4_Impl {
    fn EnumHeap2(&self, enumflags: PROFILER_HEAP_ENUM_FLAGS) -> ::windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerControl5 {}
impl IActiveScriptProfilerControl5_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl5_Impl, const OFFSET: isize>() -> IActiveScriptProfilerControl5_Vtbl {
        unsafe extern "system" fn EnumHeap2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerControl5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumflags: PROFILER_HEAP_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumHeap2(::core::mem::transmute_copy(&enumflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IActiveScriptProfilerControl4_Vtbl::new::<Identity, Impl, OFFSET>(), EnumHeap2: EnumHeap2::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerControl5 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl2 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl3 as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptProfilerControl4 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptProfilerHeapEnum_Impl: Sized {
    fn Next(&self, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionalInfo(&self, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> ::windows_core::Result<()>;
    fn FreeObjectAndOptionalInfo(&self, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> ::windows_core::Result<()>;
    fn GetNameIdMap(&self, pnamelist: *mut *mut *mut ::windows_core::PCWSTR, pcelt: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptProfilerHeapEnum {}
impl IActiveScriptProfilerHeapEnum_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>() -> IActiveScriptProfilerHeapEnum_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&heapobjects), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn GetOptionalInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOptionalInfo(::core::mem::transmute_copy(&heapobject), ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&optionalinfo)).into()
        }
        unsafe extern "system" fn FreeObjectAndOptionalInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeObjectAndOptionalInfo(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&heapobjects)).into()
        }
        unsafe extern "system" fn GetNameIdMap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamelist: *mut *mut *mut ::windows_core::PCWSTR, pcelt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNameIdMap(::core::mem::transmute_copy(&pnamelist), ::core::mem::transmute_copy(&pcelt)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            GetOptionalInfo: GetOptionalInfo::<Identity, Impl, OFFSET>,
            FreeObjectAndOptionalInfo: FreeObjectAndOptionalInfo::<Identity, Impl, OFFSET>,
            GetNameIdMap: GetNameIdMap::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProfilerHeapEnum as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptProperty_Impl: Sized {
    fn GetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Variant::VARIANT>;
    fn SetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IActiveScriptProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: isize>() -> IActiveScriptProperty_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *mut super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&pvarindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&pvarindex), ::core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptProperty as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptSIPInfo_Impl: Sized {
    fn GetSIPOID(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IActiveScriptSIPInfo {}
impl IActiveScriptSIPInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSIPInfo_Impl, const OFFSET: isize>() -> IActiveScriptSIPInfo_Vtbl {
        unsafe extern "system" fn GetSIPOID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSIPInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poid_sip: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSIPOID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poid_sip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSIPOID: GetSIPOID::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSIPInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptSite_Impl: Sized {
    fn GetLCID(&self) -> ::windows_core::Result<u32>;
    fn GetItemInfo(&self, pstrname: &::windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut ::core::option::Option<::windows_core::IUnknown>, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>) -> ::windows_core::Result<()>;
    fn GetDocVersionString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn OnScriptTerminate(&self, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn OnStateChange(&self, ssscriptstate: SCRIPTSTATE) -> ::windows_core::Result<()>;
    fn OnScriptError(&self, pscripterror: ::core::option::Option<&IActiveScriptError>) -> ::windows_core::Result<()>;
    fn OnEnterScript(&self) -> ::windows_core::Result<()>;
    fn OnLeaveScript(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IActiveScriptSite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptSite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>() -> IActiveScriptSite_Vtbl {
        unsafe extern "system" fn GetLCID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLCID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetItemInfo(::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwreturnmask), ::core::mem::transmute_copy(&ppiunkitem), ::core::mem::transmute_copy(&ppti)).into()
        }
        unsafe extern "system" fn GetDocVersionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocVersionString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnScriptTerminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScriptTerminate(::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into()
        }
        unsafe extern "system" fn OnStateChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ssscriptstate: SCRIPTSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChange(::core::mem::transmute_copy(&ssscriptstate)).into()
        }
        unsafe extern "system" fn OnScriptError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscripterror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScriptError(::windows_core::from_raw_borrowed(&pscripterror)).into()
        }
        unsafe extern "system" fn OnEnterScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnterScript().into()
        }
        unsafe extern "system" fn OnLeaveScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLeaveScript().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLCID: GetLCID::<Identity, Impl, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, Impl, OFFSET>,
            GetDocVersionString: GetDocVersionString::<Identity, Impl, OFFSET>,
            OnScriptTerminate: OnScriptTerminate::<Identity, Impl, OFFSET>,
            OnStateChange: OnStateChange::<Identity, Impl, OFFSET>,
            OnScriptError: OnScriptError::<Identity, Impl, OFFSET>,
            OnEnterScript: OnEnterScript::<Identity, Impl, OFFSET>,
            OnLeaveScript: OnLeaveScript::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSite as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebug32_Impl: Sized {
    fn GetDocumentContextFromPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(&self) -> ::windows_core::Result<IDebugApplication32>;
    fn GetRootApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(&self, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IActiveScriptSiteDebug32 {}
#[cfg(feature = "Win32_Foundation")]
impl IActiveScriptSiteDebug32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>() -> IActiveScriptSiteDebug32_Vtbl {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentContextFromPosition(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScriptErrorDebug(::windows_core::from_raw_borrowed(&perrordebug), ::core::mem::transmute_copy(&pfenterdebugger), ::core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, Impl, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteDebug32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebug64_Impl: Sized {
    fn GetDocumentContextFromPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(&self) -> ::windows_core::Result<IDebugApplication64>;
    fn GetRootApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(&self, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IActiveScriptSiteDebug64 {}
#[cfg(feature = "Win32_Foundation")]
impl IActiveScriptSiteDebug64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>() -> IActiveScriptSiteDebug64_Vtbl {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentContextFromPosition(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnScriptErrorDebug(::windows_core::from_raw_borrowed(&perrordebug), ::core::mem::transmute_copy(&pfenterdebugger), ::core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, Impl, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteDebug64 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebugEx_Impl: Sized {
    fn OnCanNotJITScriptErrorDebug(&self, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IActiveScriptSiteDebugEx {}
#[cfg(feature = "Win32_Foundation")]
impl IActiveScriptSiteDebugEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebugEx_Impl, const OFFSET: isize>() -> IActiveScriptSiteDebugEx_Vtbl {
        unsafe extern "system" fn OnCanNotJITScriptErrorDebug<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteDebugEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnCanNotJITScriptErrorDebug(::windows_core::from_raw_borrowed(&perrordebug)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcallonscripterrorwhencontinuing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCanNotJITScriptErrorDebug: OnCanNotJITScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteDebugEx as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptSiteInterruptPoll_Impl: Sized {
    fn QueryContinue(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptSiteInterruptPoll {}
impl IActiveScriptSiteInterruptPoll_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: isize>() -> IActiveScriptSiteInterruptPoll_Vtbl {
        unsafe extern "system" fn QueryContinue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryContinue().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryContinue: QueryContinue::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteInterruptPoll as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptSiteTraceInfo_Impl: Sized {
    fn SendScriptTraceInfo(&self, stieventtype: SCRIPTTRACEINFO, guidcontextid: &::windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptSiteTraceInfo {}
impl IActiveScriptSiteTraceInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteTraceInfo_Impl, const OFFSET: isize>() -> IActiveScriptSiteTraceInfo_Vtbl {
        unsafe extern "system" fn SendScriptTraceInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteTraceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stieventtype: SCRIPTTRACEINFO, guidcontextid: ::windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendScriptTraceInfo(::core::mem::transmute_copy(&stieventtype), ::core::mem::transmute(&guidcontextid), ::core::mem::transmute_copy(&dwscriptcontextcookie), ::core::mem::transmute_copy(&lscriptstatementstart), ::core::mem::transmute_copy(&lscriptstatementend), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SendScriptTraceInfo: SendScriptTraceInfo::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteTraceInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptSiteUIControl_Impl: Sized {
    fn GetUIBehavior(&self, uicitem: SCRIPTUICITEM) -> ::windows_core::Result<SCRIPTUICHANDLING>;
}
impl ::windows_core::RuntimeName for IActiveScriptSiteUIControl {}
impl IActiveScriptSiteUIControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteUIControl_Impl, const OFFSET: isize>() -> IActiveScriptSiteUIControl_Vtbl {
        unsafe extern "system" fn GetUIBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uicitem: SCRIPTUICITEM, puichandling: *mut SCRIPTUICHANDLING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUIBehavior(::core::mem::transmute_copy(&uicitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puichandling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUIBehavior: GetUIBehavior::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteUIControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteWindow_Impl: Sized {
    fn GetWindow(&self) -> ::windows_core::Result<super::super::super::super::Foundation::HWND>;
    fn EnableModeless(&self, fenable: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IActiveScriptSiteWindow {}
#[cfg(feature = "Win32_Foundation")]
impl IActiveScriptSiteWindow_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: isize>() -> IActiveScriptSiteWindow_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptSiteWindow as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptStats_Impl: Sized {
    fn GetStat(&self, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatEx(&self, guid: *const ::windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::Result<()>;
    fn ResetStats(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptStats {}
impl IActiveScriptStats_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: isize>() -> IActiveScriptStats_Vtbl {
        unsafe extern "system" fn GetStat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStat(::core::mem::transmute_copy(&stid), ::core::mem::transmute_copy(&pluhi), ::core::mem::transmute_copy(&plulo)).into()
        }
        unsafe extern "system" fn GetStatEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatEx(::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pluhi), ::core::mem::transmute_copy(&plulo)).into()
        }
        unsafe extern "system" fn ResetStats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetStats().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStat: GetStat::<Identity, Impl, OFFSET>,
            GetStatEx: GetStatEx::<Identity, Impl, OFFSET>,
            ResetStats: ResetStats::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptStats as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptStringCompare_Impl: Sized {
    fn StrComp(&self, bszstr1: &::windows_core::BSTR, bszstr2: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IActiveScriptStringCompare {}
impl IActiveScriptStringCompare_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStringCompare_Impl, const OFFSET: isize>() -> IActiveScriptStringCompare_Vtbl {
        unsafe extern "system" fn StrComp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptStringCompare_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bszstr1: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszstr2: ::std::mem::MaybeUninit<::windows_core::BSTR>, iret: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrComp(::core::mem::transmute(&bszstr1), ::core::mem::transmute(&bszstr2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iret, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StrComp: StrComp::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptStringCompare as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveScriptTraceInfo_Impl: Sized {
    fn StartScriptTracing(&self, psitetraceinfo: ::core::option::Option<&IActiveScriptSiteTraceInfo>, guidcontextid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn StopScriptTracing(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveScriptTraceInfo {}
impl IActiveScriptTraceInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: isize>() -> IActiveScriptTraceInfo_Vtbl {
        unsafe extern "system" fn StartScriptTracing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psitetraceinfo: *mut ::core::ffi::c_void, guidcontextid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartScriptTracing(::windows_core::from_raw_borrowed(&psitetraceinfo), ::core::mem::transmute(&guidcontextid)).into()
        }
        unsafe extern "system" fn StopScriptTracing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopScriptTracing().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartScriptTracing: StartScriptTracing::<Identity, Impl, OFFSET>,
            StopScriptTracing: StopScriptTracing::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptTraceInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptWinRTErrorDebug_Impl: Sized + IActiveScriptError_Impl {
    fn GetRestrictedErrorString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRestrictedErrorReference(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCapabilitySid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IActiveScriptWinRTErrorDebug {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptWinRTErrorDebug_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>() -> IActiveScriptWinRTErrorDebug_Vtbl {
        unsafe extern "system" fn GetRestrictedErrorString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictedErrorString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictedErrorReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, referencestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictedErrorReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referencestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilitySid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilitysid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCapabilitySid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilitysid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IActiveScriptError_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRestrictedErrorString: GetRestrictedErrorString::<Identity, Impl, OFFSET>,
            GetRestrictedErrorReference: GetRestrictedErrorReference::<Identity, Impl, OFFSET>,
            GetCapabilitySid: GetCapabilitySid::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveScriptWinRTErrorDebug as ::windows_core::ComInterface>::IID || *iid == <IActiveScriptError as ::windows_core::ComInterface>::IID
    }
}
pub trait IApplicationDebugger_Impl: Sized {
    fn QueryAlive(&self) -> ::windows_core::Result<()>;
    fn CreateInstanceAtDebugger(&self, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn onDebugOutput(&self, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn onHandleBreakPoint(&self, prpt: ::core::option::Option<&IRemoteDebugApplicationThread>, br: BREAKREASON, perror: ::core::option::Option<&IActiveScriptErrorDebug>) -> ::windows_core::Result<()>;
    fn onClose(&self) -> ::windows_core::Result<()>;
    fn onDebuggerEvent(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IApplicationDebugger {}
impl IApplicationDebugger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>() -> IApplicationDebugger_Vtbl {
        unsafe extern "system" fn QueryAlive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryAlive().into()
        }
        unsafe extern "system" fn CreateInstanceAtDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstanceAtDebugger(::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn onDebugOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onDebugOutput(::core::mem::transmute(&pstr)).into()
        }
        unsafe extern "system" fn onHandleBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prpt: *mut ::core::ffi::c_void, br: BREAKREASON, perror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onHandleBreakPoint(::windows_core::from_raw_borrowed(&prpt), ::core::mem::transmute_copy(&br), ::windows_core::from_raw_borrowed(&perror)).into()
        }
        unsafe extern "system" fn onClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onClose().into()
        }
        unsafe extern "system" fn onDebuggerEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onDebuggerEvent(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryAlive: QueryAlive::<Identity, Impl, OFFSET>,
            CreateInstanceAtDebugger: CreateInstanceAtDebugger::<Identity, Impl, OFFSET>,
            onDebugOutput: onDebugOutput::<Identity, Impl, OFFSET>,
            onHandleBreakPoint: onHandleBreakPoint::<Identity, Impl, OFFSET>,
            onClose: onClose::<Identity, Impl, OFFSET>,
            onDebuggerEvent: onDebuggerEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IApplicationDebugger as ::windows_core::ComInterface>::IID
    }
}
pub trait IApplicationDebuggerUI_Impl: Sized {
    fn BringDocumentToTop(&self, pddt: ::core::option::Option<&IDebugDocumentText>) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IApplicationDebuggerUI {}
impl IApplicationDebuggerUI_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: isize>() -> IApplicationDebuggerUI_Vtbl {
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddt: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentToTop(::windows_core::from_raw_borrowed(&pddt)).into()
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentContextToTop(::windows_core::from_raw_borrowed(&pddc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IApplicationDebuggerUI as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBindEventHandler_Impl: Sized {
    fn BindHandler(&self, pstrevent: &::windows_core::PCWSTR, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IBindEventHandler {}
#[cfg(feature = "Win32_System_Com")]
impl IBindEventHandler_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBindEventHandler_Impl, const OFFSET: isize>() -> IBindEventHandler_Vtbl {
        unsafe extern "system" fn BindHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBindEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrevent: ::windows_core::PCWSTR, pdisp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindHandler(::core::mem::transmute(&pstrevent), ::windows_core::from_raw_borrowed(&pdisp)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BindHandler: BindHandler::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindEventHandler as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication11032_Impl: Sized + IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(&self, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn AsynchronousCallInMainThread(&self, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn CallableWaitForHandles(&self, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplication11032 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplication11032_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: isize>() -> IDebugApplication11032_Vtbl {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallInMainThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCallInMainThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallableWaitForHandles(::core::mem::transmute_copy(&handlecount), ::core::mem::transmute_copy(&phandles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRemoteDebugApplication110_Vtbl::new::<Identity, Impl, OFFSET>(),
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplication11032 as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplication110 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication11064_Impl: Sized + IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(&self, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn AsynchronousCallInMainThread(&self, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn CallableWaitForHandles(&self, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplication11064 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplication11064_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: isize>() -> IDebugApplication11064_Vtbl {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallInMainThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCallInMainThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CallableWaitForHandles(::core::mem::transmute_copy(&handlecount), ::core::mem::transmute_copy(&phandles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IRemoteDebugApplication110_Vtbl::new::<Identity, Impl, OFFSET>(),
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplication11064 as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplication110 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication32_Impl: Sized + IRemoteDebugApplication_Impl {
    fn SetName(&self, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StepOutComplete(&self) -> ::windows_core::Result<()>;
    fn DebugOutput(&self, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StartDebugSession(&self) -> ::windows_core::Result<()>;
    fn HandleBreakPoint(&self, br: BREAKREASON) -> ::windows_core::Result<BREAKRESUMEACTION>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn GetCurrentThread(&self) -> ::windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(&self, psdo: ::core::option::Option<&IDebugSyncOperation>) -> ::windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(&self, pdsfs: ::core::option::Option<&IDebugStackFrameSniffer>) -> ::windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(&self) -> ::windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(&self, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
    fn CreateApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn HandleRuntimeError(&self, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pscriptsite: ::core::option::Option<&IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FCanJitDebug(&self) -> super::super::super::super::Foundation::BOOL;
    fn FIsAutoJitDebugEnabled(&self) -> super::super::super::super::Foundation::BOOL;
    fn AddGlobalExpressionContextProvider(&self, pdsfs: ::core::option::Option<&IProvideExpressionContexts>) -> ::windows_core::Result<u32>;
    fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplication32 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplication32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>() -> IDebugApplication32_Vtbl {
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&pstrname)).into()
        }
        unsafe extern "system" fn StepOutComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StepOutComplete().into()
        }
        unsafe extern "system" fn DebugOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugOutput(::core::mem::transmute(&pstr)).into()
        }
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartDebugSession().into()
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HandleBreakPoint(::core::mem::transmute_copy(&br)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbra, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn GetBreakFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakFlags(::core::mem::transmute_copy(&pabf), ::core::mem::transmute_copy(&pprdatsteppingthread)).into()
        }
        unsafe extern "system" fn GetCurrentThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAsyncDebugOperation(::windows_core::from_raw_borrowed(&psdo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppado, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddStackFrameSniffer(::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStackFrameSniffer(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryCurrentThreadIsDebuggerThread().into()
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallInDebuggerThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdannew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDebuggerEvent(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleRuntimeError(::windows_core::from_raw_borrowed(&perrordebug), ::windows_core::from_raw_borrowed(&pscriptsite), ::core::mem::transmute_copy(&pbra), ::core::mem::transmute_copy(&perra), ::core::mem::transmute_copy(&pfcallonscripterror)).into()
        }
        unsafe extern "system" fn FCanJitDebug<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FCanJitDebug()
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FIsAutoJitDebugEnabled()
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddGlobalExpressionContextProvider(::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveGlobalExpressionContextProvider(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: IRemoteDebugApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetName: SetName::<Identity, Impl, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, Impl, OFFSET>,
            DebugOutput: DebugOutput::<Identity, Impl, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, Impl, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, Impl, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, Impl, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, Impl, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, Impl, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, Impl, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, Impl, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, Impl, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, Impl, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, Impl, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, Impl, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, Impl, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplication32 as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplication as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication64_Impl: Sized + IRemoteDebugApplication_Impl {
    fn SetName(&self, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StepOutComplete(&self) -> ::windows_core::Result<()>;
    fn DebugOutput(&self, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StartDebugSession(&self) -> ::windows_core::Result<()>;
    fn HandleBreakPoint(&self, br: BREAKREASON) -> ::windows_core::Result<BREAKRESUMEACTION>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn GetCurrentThread(&self) -> ::windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(&self, psdo: ::core::option::Option<&IDebugSyncOperation>) -> ::windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(&self, pdsfs: ::core::option::Option<&IDebugStackFrameSniffer>) -> ::windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> ::windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(&self) -> ::windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(&self, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
    fn CreateApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(&self, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn HandleRuntimeError(&self, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pscriptsite: ::core::option::Option<&IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FCanJitDebug(&self) -> super::super::super::super::Foundation::BOOL;
    fn FIsAutoJitDebugEnabled(&self) -> super::super::super::super::Foundation::BOOL;
    fn AddGlobalExpressionContextProvider(&self, pdsfs: ::core::option::Option<&IProvideExpressionContexts>) -> ::windows_core::Result<u64>;
    fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplication64 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplication64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>() -> IDebugApplication64_Vtbl {
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&pstrname)).into()
        }
        unsafe extern "system" fn StepOutComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StepOutComplete().into()
        }
        unsafe extern "system" fn DebugOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugOutput(::core::mem::transmute(&pstr)).into()
        }
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartDebugSession().into()
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HandleBreakPoint(::core::mem::transmute_copy(&br)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbra, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn GetBreakFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakFlags(::core::mem::transmute_copy(&pabf), ::core::mem::transmute_copy(&pprdatsteppingthread)).into()
        }
        unsafe extern "system" fn GetCurrentThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAsyncDebugOperation(::windows_core::from_raw_borrowed(&psdo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppado, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddStackFrameSniffer(::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStackFrameSniffer(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryCurrentThreadIsDebuggerThread().into()
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallInDebuggerThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdannew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireDebuggerEvent(::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleRuntimeError(::windows_core::from_raw_borrowed(&perrordebug), ::windows_core::from_raw_borrowed(&pscriptsite), ::core::mem::transmute_copy(&pbra), ::core::mem::transmute_copy(&perra), ::core::mem::transmute_copy(&pfcallonscripterror)).into()
        }
        unsafe extern "system" fn FCanJitDebug<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FCanJitDebug()
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FIsAutoJitDebugEnabled()
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddGlobalExpressionContextProvider(::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveGlobalExpressionContextProvider(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: IRemoteDebugApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetName: SetName::<Identity, Impl, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, Impl, OFFSET>,
            DebugOutput: DebugOutput::<Identity, Impl, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, Impl, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, Impl, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, Impl, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, Impl, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, Impl, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, Impl, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, Impl, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, Impl, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, Impl, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, Impl, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, Impl, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, Impl, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplication64 as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplication as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationNode_Impl: Sized + IDebugDocumentProvider_Impl {
    fn EnumChildren(&self) -> ::windows_core::Result<IEnumDebugApplicationNodes>;
    fn GetParent(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn SetDocumentProvider(&self, pddp: ::core::option::Option<&IDebugDocumentProvider>) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn Attach(&self, pdanparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Detach(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationNode {}
impl IDebugApplicationNode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>() -> IDebugApplicationNode_Vtbl {
        unsafe extern "system" fn EnumChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDocumentProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentProvider(::windows_core::from_raw_borrowed(&pddp)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdanparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::windows_core::from_raw_borrowed(&pdanparent)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Detach().into()
        }
        Self {
            base__: IDebugDocumentProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumChildren: EnumChildren::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            SetDocumentProvider: SetDocumentProvider::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationNode as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationNode100_Impl: Sized {
    fn SetFilterForEventSink(&self, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::Result<()>;
    fn GetExcludedDocuments(&self, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::Result<TEXT_DOCUMENT_ARRAY>;
    fn QueryIsChildNode(&self, psearchkey: ::core::option::Option<&IDebugDocument>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationNode100 {}
impl IDebugApplicationNode100_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: isize>() -> IDebugApplicationNode100_Vtbl {
        unsafe extern "system" fn SetFilterForEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilterForEventSink(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&filter)).into()
        }
        unsafe extern "system" fn GetExcludedDocuments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: APPLICATION_NODE_EVENT_FILTER, pdocuments: *mut TEXT_DOCUMENT_ARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExcludedDocuments(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdocuments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryIsChildNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psearchkey: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryIsChildNode(::windows_core::from_raw_borrowed(&psearchkey)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFilterForEventSink: SetFilterForEventSink::<Identity, Impl, OFFSET>,
            GetExcludedDocuments: GetExcludedDocuments::<Identity, Impl, OFFSET>,
            QueryIsChildNode: QueryIsChildNode::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationNode100 as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationNodeEvents_Impl: Sized {
    fn onAddChild(&self, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn onRemoveChild(&self, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn onDetach(&self) -> ::windows_core::Result<()>;
    fn onAttach(&self, prddpparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationNodeEvents {}
impl IDebugApplicationNodeEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>() -> IDebugApplicationNodeEvents_Vtbl {
        unsafe extern "system" fn onAddChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onAddChild(::windows_core::from_raw_borrowed(&prddpchild)).into()
        }
        unsafe extern "system" fn onRemoveChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onRemoveChild(::windows_core::from_raw_borrowed(&prddpchild)).into()
        }
        unsafe extern "system" fn onDetach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onDetach().into()
        }
        unsafe extern "system" fn onAttach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onAttach(::windows_core::from_raw_borrowed(&prddpparent)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onAddChild: onAddChild::<Identity, Impl, OFFSET>,
            onRemoveChild: onRemoveChild::<Identity, Impl, OFFSET>,
            onDetach: onDetach::<Identity, Impl, OFFSET>,
            onAttach: onAttach::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationNodeEvents as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationThread_Impl: Sized + IRemoteDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread32(&self, pstcb: ::core::option::Option<&IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
    fn QueryIsCurrentThread(&self) -> ::windows_core::Result<()>;
    fn QueryIsDebuggerThread(&self) -> ::windows_core::Result<()>;
    fn SetDescription(&self, pstrdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetStateString(&self, pstrstate: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationThread {}
impl IDebugApplicationThread_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>() -> IDebugApplicationThread_Vtbl {
        unsafe extern "system" fn SynchronousCallIntoThread32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallIntoThread32(::windows_core::from_raw_borrowed(&pstcb), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        unsafe extern "system" fn QueryIsCurrentThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryIsCurrentThread().into()
        }
        unsafe extern "system" fn QueryIsDebuggerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryIsDebuggerThread().into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&pstrdescription)).into()
        }
        unsafe extern "system" fn SetStateString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrstate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStateString(::core::mem::transmute(&pstrstate)).into()
        }
        Self {
            base__: IRemoteDebugApplicationThread_Vtbl::new::<Identity, Impl, OFFSET>(),
            SynchronousCallIntoThread32: SynchronousCallIntoThread32::<Identity, Impl, OFFSET>,
            QueryIsCurrentThread: QueryIsCurrentThread::<Identity, Impl, OFFSET>,
            QueryIsDebuggerThread: QueryIsDebuggerThread::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetStateString: SetStateString::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationThread as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplicationThread as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplicationThread11032_Impl: Sized {
    fn GetActiveThreadRequestCount(&self) -> ::windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn IsThreadCallable(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn AsynchronousCallIntoThread(&self, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplicationThread11032 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplicationThread11032_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: isize>() -> IDebugApplicationThread11032_Vtbl {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveThreadRequestCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puithreadrequests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSuspendedForBreakPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissuspended, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadCallable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThreadCallable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiscallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCallIntoThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, Impl, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, Impl, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, Impl, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationThread11032 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplicationThread11064_Impl: Sized {
    fn GetActiveThreadRequestCount(&self) -> ::windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn IsThreadCallable(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn AsynchronousCallIntoThread(&self, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugApplicationThread11064 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugApplicationThread11064_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: isize>() -> IDebugApplicationThread11064_Vtbl {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveThreadRequestCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puithreadrequests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSuspendedForBreakPoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissuspended, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreadCallable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsThreadCallable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiscallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCallIntoThread(::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, Impl, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, Impl, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, Impl, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationThread11064 as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationThread64_Impl: Sized + IDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread64(&self, pstcb: ::core::option::Option<&IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationThread64 {}
impl IDebugApplicationThread64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread64_Impl, const OFFSET: isize>() -> IDebugApplicationThread64_Vtbl {
        unsafe extern "system" fn SynchronousCallIntoThread64<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThread64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCallIntoThread64(::windows_core::from_raw_borrowed(&pstcb), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        Self {
            base__: IDebugApplicationThread_Vtbl::new::<Identity, Impl, OFFSET>(),
            SynchronousCallIntoThread64: SynchronousCallIntoThread64::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationThread64 as ::windows_core::ComInterface>::IID || *iid == <IRemoteDebugApplicationThread as ::windows_core::ComInterface>::IID || *iid == <IDebugApplicationThread as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugApplicationThreadEvents110_Impl: Sized {
    fn OnSuspendForBreakPoint(&self) -> ::windows_core::Result<()>;
    fn OnResumeFromBreakPoint(&self) -> ::windows_core::Result<()>;
    fn OnThreadRequestComplete(&self) -> ::windows_core::Result<()>;
    fn OnBeginThreadRequest(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugApplicationThreadEvents110 {}
impl IDebugApplicationThreadEvents110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>() -> IDebugApplicationThreadEvents110_Vtbl {
        unsafe extern "system" fn OnSuspendForBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSuspendForBreakPoint().into()
        }
        unsafe extern "system" fn OnResumeFromBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResumeFromBreakPoint().into()
        }
        unsafe extern "system" fn OnThreadRequestComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadRequestComplete().into()
        }
        unsafe extern "system" fn OnBeginThreadRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBeginThreadRequest().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSuspendForBreakPoint: OnSuspendForBreakPoint::<Identity, Impl, OFFSET>,
            OnResumeFromBreakPoint: OnResumeFromBreakPoint::<Identity, Impl, OFFSET>,
            OnThreadRequestComplete: OnThreadRequestComplete::<Identity, Impl, OFFSET>,
            OnBeginThreadRequest: OnBeginThreadRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugApplicationThreadEvents110 as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugAsyncOperation_Impl: Sized {
    fn GetSyncDebugOperation(&self) -> ::windows_core::Result<IDebugSyncOperation>;
    fn Start(&self, padocb: ::core::option::Option<&IDebugAsyncOperationCallBack>) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn QueryIsComplete(&self) -> ::windows_core::Result<()>;
    fn GetResult(&self, phrresult: *mut ::windows_core::HRESULT, ppunkresult: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugAsyncOperation {}
impl IDebugAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>() -> IDebugAsyncOperation_Vtbl {
        unsafe extern "system" fn GetSyncDebugOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSyncDebugOperation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padocb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::windows_core::from_raw_borrowed(&padocb)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn QueryIsComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryIsComplete().into()
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResult(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&ppunkresult)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSyncDebugOperation: GetSyncDebugOperation::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugAsyncOperation as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugAsyncOperationCallBack_Impl: Sized {
    fn onComplete(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugAsyncOperationCallBack {}
impl IDebugAsyncOperationCallBack_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperationCallBack_Impl, const OFFSET: isize>() -> IDebugAsyncOperationCallBack_Vtbl {
        unsafe extern "system" fn onComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugAsyncOperationCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onComplete().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugAsyncOperationCallBack as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugCodeContext_Impl: Sized {
    fn GetDocumentContext(&self) -> ::windows_core::Result<IDebugDocumentContext>;
    fn SetBreakPoint(&self, bps: BREAKPOINT_STATE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugCodeContext {}
impl IDebugCodeContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: isize>() -> IDebugCodeContext_Vtbl {
        unsafe extern "system" fn GetDocumentContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bps: BREAKPOINT_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakPoint(::core::mem::transmute_copy(&bps)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContext: GetDocumentContext::<Identity, Impl, OFFSET>,
            SetBreakPoint: SetBreakPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugCodeContext as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugCookie_Impl: Sized {
    fn SetDebugCookie(&self, dwdebugappcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugCookie {}
impl IDebugCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugCookie_Impl, const OFFSET: isize>() -> IDebugCookie_Vtbl {
        unsafe extern "system" fn SetDebugCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugCookie(::core::mem::transmute_copy(&dwdebugappcookie)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetDebugCookie: SetDebugCookie::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocument_Impl: Sized + IDebugDocumentInfo_Impl {}
impl ::windows_core::RuntimeName for IDebugDocument {}
impl IDebugDocument_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocument_Impl, const OFFSET: isize>() -> IDebugDocument_Vtbl {
        Self { base__: IDebugDocumentInfo_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocument as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentContext_Impl: Sized {
    fn GetDocument(&self) -> ::windows_core::Result<IDebugDocument>;
    fn EnumCodeContexts(&self) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::RuntimeName for IDebugDocumentContext {}
impl IDebugDocumentContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: isize>() -> IDebugDocumentContext_Vtbl {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCodeContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCodeContexts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            EnumCodeContexts: EnumCodeContexts::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHelper32_Impl: Sized {
    fn Init(&self, pda: ::core::option::Option<&IDebugApplication32>, pszshortname: &::windows_core::PCWSTR, pszlongname: &::windows_core::PCWSTR, docattr: u32) -> ::windows_core::Result<()>;
    fn Attach(&self, pddhparent: ::core::option::Option<&IDebugDocumentHelper32>) -> ::windows_core::Result<()>;
    fn Detach(&self) -> ::windows_core::Result<()>;
    fn AddUnicodeText(&self, psztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDBCSText(&self, psztext: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetDebugDocumentHost(&self, pddh: ::core::option::Option<&IDebugDocumentHost>) -> ::windows_core::Result<()>;
    fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::Result<()>;
    fn DefineScriptBlock(&self, ulcharoffset: u32, cchars: u32, pas: ::core::option::Option<&IActiveScript>, fscriptlet: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<u32>;
    fn SetDefaultTextAttr(&self, statextattr: u16) -> ::windows_core::Result<()>;
    fn SetTextAttributes(&self, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::Result<()>;
    fn SetLongName(&self, pszlongname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetShortName(&self, pszshortname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDocumentAttr(&self, pszattributes: u32) -> ::windows_core::Result<()>;
    fn GetDebugApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(&self, dwsourcecontext: u32, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(&self) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugDocumentHelper32 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugDocumentHelper32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>() -> IDebugDocumentHelper32_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR, pszlongname: ::windows_core::PCWSTR, docattr: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute(&pszshortname), ::core::mem::transmute(&pszlongname), ::core::mem::transmute_copy(&docattr)).into()
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::windows_core::from_raw_borrowed(&pddhparent)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Detach().into()
        }
        unsafe extern "system" fn AddUnicodeText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddUnicodeText(::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn AddDBCSText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDBCSText(::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugDocumentHost(::windows_core::from_raw_borrowed(&pddh)).into()
        }
        unsafe extern "system" fn AddDeferredText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDeferredText(::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&dwtextstartcookie)).into()
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefineScriptBlock(::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::windows_core::from_raw_borrowed(&pas), ::core::mem::transmute_copy(&fscriptlet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsourcecontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultTextAttr(::core::mem::transmute_copy(&statextattr)).into()
        }
        unsafe extern "system" fn SetTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextAttributes(::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&pstatextattr)).into()
        }
        unsafe extern "system" fn SetLongName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlongname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLongName(::core::mem::transmute(&pszlongname)).into()
        }
        unsafe extern "system" fn SetShortName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetShortName(::core::mem::transmute(&pszshortname)).into()
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentAttr(::core::mem::transmute_copy(&pszattributes)).into()
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDebugApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptBlockInfo(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ppasd), ::core::mem::transmute_copy(&picharpos), ::core::mem::transmute_copy(&pcchars)).into()
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDebugDocumentContext(::core::mem::transmute_copy(&icharpos), ::core::mem::transmute_copy(&cchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppddc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentToTop().into()
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentContextToTop(::windows_core::from_raw_borrowed(&pddc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, Impl, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, Impl, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, Impl, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, Impl, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, Impl, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, Impl, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, Impl, OFFSET>,
            SetLongName: SetLongName::<Identity, Impl, OFFSET>,
            SetShortName: SetShortName::<Identity, Impl, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, Impl, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, Impl, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, Impl, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, Impl, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentHelper32 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHelper64_Impl: Sized {
    fn Init(&self, pda: ::core::option::Option<&IDebugApplication64>, pszshortname: &::windows_core::PCWSTR, pszlongname: &::windows_core::PCWSTR, docattr: u32) -> ::windows_core::Result<()>;
    fn Attach(&self, pddhparent: ::core::option::Option<&IDebugDocumentHelper64>) -> ::windows_core::Result<()>;
    fn Detach(&self) -> ::windows_core::Result<()>;
    fn AddUnicodeText(&self, psztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDBCSText(&self, psztext: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetDebugDocumentHost(&self, pddh: ::core::option::Option<&IDebugDocumentHost>) -> ::windows_core::Result<()>;
    fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::Result<()>;
    fn DefineScriptBlock(&self, ulcharoffset: u32, cchars: u32, pas: ::core::option::Option<&IActiveScript>, fscriptlet: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<u64>;
    fn SetDefaultTextAttr(&self, statextattr: u16) -> ::windows_core::Result<()>;
    fn SetTextAttributes(&self, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::Result<()>;
    fn SetLongName(&self, pszlongname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetShortName(&self, pszshortname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDocumentAttr(&self, pszattributes: u32) -> ::windows_core::Result<()>;
    fn GetDebugApplicationNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(&self, dwsourcecontext: u64, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(&self) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugDocumentHelper64 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugDocumentHelper64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>() -> IDebugDocumentHelper64_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR, pszlongname: ::windows_core::PCWSTR, docattr: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute(&pszshortname), ::core::mem::transmute(&pszlongname), ::core::mem::transmute_copy(&docattr)).into()
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::windows_core::from_raw_borrowed(&pddhparent)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Detach().into()
        }
        unsafe extern "system" fn AddUnicodeText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddUnicodeText(::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn AddDBCSText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDBCSText(::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebugDocumentHost(::windows_core::from_raw_borrowed(&pddh)).into()
        }
        unsafe extern "system" fn AddDeferredText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDeferredText(::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&dwtextstartcookie)).into()
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefineScriptBlock(::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::windows_core::from_raw_borrowed(&pas), ::core::mem::transmute_copy(&fscriptlet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsourcecontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultTextAttr(::core::mem::transmute_copy(&statextattr)).into()
        }
        unsafe extern "system" fn SetTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextAttributes(::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&pstatextattr)).into()
        }
        unsafe extern "system" fn SetLongName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlongname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLongName(::core::mem::transmute(&pszlongname)).into()
        }
        unsafe extern "system" fn SetShortName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetShortName(::core::mem::transmute(&pszshortname)).into()
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDocumentAttr(::core::mem::transmute_copy(&pszattributes)).into()
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDebugApplicationNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptBlockInfo(::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ppasd), ::core::mem::transmute_copy(&picharpos), ::core::mem::transmute_copy(&pcchars)).into()
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDebugDocumentContext(::core::mem::transmute_copy(&icharpos), ::core::mem::transmute_copy(&cchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppddc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentToTop().into()
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringDocumentContextToTop(::windows_core::from_raw_borrowed(&pddc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, Impl, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, Impl, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, Impl, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, Impl, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, Impl, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, Impl, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, Impl, OFFSET>,
            SetLongName: SetLongName::<Identity, Impl, OFFSET>,
            SetShortName: SetShortName::<Identity, Impl, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, Impl, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, Impl, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, Impl, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, Impl, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentHelper64 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHost_Impl: Sized {
    fn GetDeferredText(&self, dwtextstartcookie: u32, pchartext: &::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetScriptTextAttributes(&self, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn OnCreateDocumentContext(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetPathName(&self, pbstrlongname: *mut ::windows_core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFileName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NotifyChanged(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugDocumentHost {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugDocumentHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>() -> IDebugDocumentHost_Vtbl {
        unsafe extern "system" fn GetDeferredText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtextstartcookie: u32, pchartext: ::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeferredText(::core::mem::transmute_copy(&dwtextstartcookie), ::core::mem::transmute(&pchartext), ::core::mem::transmute_copy(&pstatextattr), ::core::mem::transmute_copy(&pcnumchars), ::core::mem::transmute_copy(&cmaxchars)).into()
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScriptTextAttributes(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into()
        }
        unsafe extern "system" fn OnCreateDocumentContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkouter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnCreateDocumentContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkouter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPathName(::core::mem::transmute_copy(&pbstrlongname), ::core::mem::transmute_copy(&pfisoriginalfile)).into()
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrshortname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyChanged().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeferredText: GetDeferredText::<Identity, Impl, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            OnCreateDocumentContext: OnCreateDocumentContext::<Identity, Impl, OFFSET>,
            GetPathName: GetPathName::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentHost as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentInfo_Impl: Sized {
    fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocumentClassId(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IDebugDocumentInfo {}
impl IDebugDocumentInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: isize>() -> IDebugDocumentInfo_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnt: DOCUMENTNAMETYPE, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName(::core::mem::transmute_copy(&dnt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentClassId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsiddocument: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentClassId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsiddocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDocumentClassId: GetDocumentClassId::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentProvider_Impl: Sized + IDebugDocumentInfo_Impl {
    fn GetDocument(&self) -> ::windows_core::Result<IDebugDocument>;
}
impl ::windows_core::RuntimeName for IDebugDocumentProvider {}
impl IDebugDocumentProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentProvider_Impl, const OFFSET: isize>() -> IDebugDocumentProvider_Vtbl {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppssd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppssd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDebugDocumentInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetDocument: GetDocument::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentProvider as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentText_Impl: Sized + IDebugDocument_Impl {
    fn GetDocumentAttributes(&self) -> ::windows_core::Result<u32>;
    fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows_core::Result<()>;
    fn GetPositionOfLine(&self, clinenumber: u32) -> ::windows_core::Result<u32>;
    fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows_core::Result<()>;
    fn GetText(&self, ccharacterposition: u32, pchartext: &::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetPositionOfContext(&self, psc: ::core::option::Option<&IDebugDocumentContext>, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows_core::Result<()>;
    fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
}
impl ::windows_core::RuntimeName for IDebugDocumentText {}
impl IDebugDocumentText_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>() -> IDebugDocumentText_Vtbl {
        unsafe extern "system" fn GetDocumentAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptextdocattr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDocumentAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptextdocattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&pcnumlines), ::core::mem::transmute_copy(&pcnumchars)).into()
        }
        unsafe extern "system" fn GetPositionOfLine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clinenumber: u32, pccharacterposition: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPositionOfLine(::core::mem::transmute_copy(&clinenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccharacterposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLineOfPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineOfPosition(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&pclinenumber), ::core::mem::transmute_copy(&pccharacteroffsetinline)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pchartext: ::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute(&pchartext), ::core::mem::transmute_copy(&pstatextattr), ::core::mem::transmute_copy(&pcnumchars), ::core::mem::transmute_copy(&cmaxchars)).into()
        }
        unsafe extern "system" fn GetPositionOfContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psc: *mut ::core::ffi::c_void, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPositionOfContext(::windows_core::from_raw_borrowed(&psc), ::core::mem::transmute_copy(&pccharacterposition), ::core::mem::transmute_copy(&cnumchars)).into()
        }
        unsafe extern "system" fn GetContextOfPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextOfPosition(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDebugDocument_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDocumentAttributes: GetDocumentAttributes::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPositionOfLine: GetPositionOfLine::<Identity, Impl, OFFSET>,
            GetLineOfPosition: GetLineOfPosition::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetPositionOfContext: GetPositionOfContext::<Identity, Impl, OFFSET>,
            GetContextOfPosition: GetContextOfPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentText as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID || *iid == <IDebugDocument as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentTextAuthor_Impl: Sized + IDebugDocumentText_Impl {
    fn InsertText(&self, ccharacterposition: u32, cnumtoinsert: u32, pchartext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::Result<()>;
    fn ReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32, pchartext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugDocumentTextAuthor {}
impl IDebugDocumentTextAuthor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>() -> IDebugDocumentTextAuthor_Vtbl {
        unsafe extern "system" fn InsertText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32, pchartext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoinsert), ::core::mem::transmute(&pchartext)).into()
        }
        unsafe extern "system" fn RemoveText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoremove)).into()
        }
        unsafe extern "system" fn ReplaceText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32, pchartext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplaceText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoreplace), ::core::mem::transmute(&pchartext)).into()
        }
        Self {
            base__: IDebugDocumentText_Vtbl::new::<Identity, Impl, OFFSET>(),
            InsertText: InsertText::<Identity, Impl, OFFSET>,
            RemoveText: RemoveText::<Identity, Impl, OFFSET>,
            ReplaceText: ReplaceText::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentTextAuthor as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentInfo as ::windows_core::ComInterface>::IID || *iid == <IDebugDocument as ::windows_core::ComInterface>::IID || *iid == <IDebugDocumentText as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugDocumentTextEvents_Impl: Sized {
    fn onDestroy(&self) -> ::windows_core::Result<()>;
    fn onInsertText(&self, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows_core::Result<()>;
    fn onRemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::Result<()>;
    fn onReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows_core::Result<()>;
    fn onUpdateTextAttributes(&self, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows_core::Result<()>;
    fn onUpdateDocumentAttributes(&self, textdocattr: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugDocumentTextEvents {}
impl IDebugDocumentTextEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>() -> IDebugDocumentTextEvents_Vtbl {
        unsafe extern "system" fn onDestroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onDestroy().into()
        }
        unsafe extern "system" fn onInsertText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onInsertText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoinsert)).into()
        }
        unsafe extern "system" fn onRemoveText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onRemoveText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoremove)).into()
        }
        unsafe extern "system" fn onReplaceText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onReplaceText(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoreplace)).into()
        }
        unsafe extern "system" fn onUpdateTextAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onUpdateTextAttributes(::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoupdate)).into()
        }
        unsafe extern "system" fn onUpdateDocumentAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textdocattr: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onUpdateDocumentAttributes(::core::mem::transmute_copy(&textdocattr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onDestroy: onDestroy::<Identity, Impl, OFFSET>,
            onInsertText: onInsertText::<Identity, Impl, OFFSET>,
            onRemoveText: onRemoveText::<Identity, Impl, OFFSET>,
            onReplaceText: onReplaceText::<Identity, Impl, OFFSET>,
            onUpdateTextAttributes: onUpdateTextAttributes::<Identity, Impl, OFFSET>,
            onUpdateDocumentAttributes: onUpdateDocumentAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentTextEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentTextExternalAuthor_Impl: Sized {
    fn GetPathName(&self, pbstrlongname: *mut ::windows_core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFileName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NotifyChanged(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugDocumentTextExternalAuthor {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugDocumentTextExternalAuthor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>() -> IDebugDocumentTextExternalAuthor_Vtbl {
        unsafe extern "system" fn GetPathName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPathName(::core::mem::transmute_copy(&pbstrlongname), ::core::mem::transmute_copy(&pfisoriginalfile)).into()
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrshortname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyChanged().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPathName: GetPathName::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugDocumentTextExternalAuthor as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugExpression_Impl: Sized {
    fn Start(&self, pdecb: ::core::option::Option<&IDebugExpressionCallBack>) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn QueryIsComplete(&self) -> ::windows_core::Result<()>;
    fn GetResultAsString(&self, phrresult: *mut ::windows_core::HRESULT, pbstrresult: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetResultAsDebugProperty(&self, phrresult: *mut ::windows_core::HRESULT, ppdp: *mut ::core::option::Option<super::IDebugProperty>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugExpression {}
impl IDebugExpression_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>() -> IDebugExpression_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdecb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::windows_core::from_raw_borrowed(&pdecb)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn QueryIsComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryIsComplete().into()
        }
        unsafe extern "system" fn GetResultAsString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResultAsString(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pbstrresult)).into()
        }
        unsafe extern "system" fn GetResultAsDebugProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, ppdp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResultAsDebugProperty(::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&ppdp)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, Impl, OFFSET>,
            GetResultAsString: GetResultAsString::<Identity, Impl, OFFSET>,
            GetResultAsDebugProperty: GetResultAsDebugProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugExpression as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugExpressionCallBack_Impl: Sized {
    fn onComplete(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugExpressionCallBack {}
impl IDebugExpressionCallBack_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpressionCallBack_Impl, const OFFSET: isize>() -> IDebugExpressionCallBack_Vtbl {
        unsafe extern "system" fn onComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpressionCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onComplete().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugExpressionCallBack as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugExpressionContext_Impl: Sized {
    fn ParseLanguageText(&self, pstrcode: &::windows_core::PCWSTR, nradix: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<IDebugExpression>;
    fn GetLanguageInfo(&self, pbstrlanguagename: *mut ::windows_core::BSTR, planguageid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugExpressionContext {}
impl IDebugExpressionContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: isize>() -> IDebugExpressionContext_Vtbl {
        unsafe extern "system" fn ParseLanguageText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, nradix: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, ppe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParseLanguageText(::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlanguagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, planguageid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLanguageInfo(::core::mem::transmute_copy(&pbstrlanguagename), ::core::mem::transmute_copy(&planguageid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseLanguageText: ParseLanguageText::<Identity, Impl, OFFSET>,
            GetLanguageInfo: GetLanguageInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugExpressionContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugFormatter_Impl: Sized {
    fn GetStringForVariant(&self, pvar: *const super::super::super::Variant::VARIANT, nradix: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetVariantForString(&self, pwstrvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Variant::VARIANT>;
    fn GetStringForVarType(&self, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDebugFormatter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDebugFormatter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: isize>() -> IDebugFormatter_Vtbl {
        unsafe extern "system" fn GetStringForVariant<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, nradix: u32, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringForVariant(::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&nradix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVariantForString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrvalue: ::windows_core::PCWSTR, pvar: *mut super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVariantForString(::core::mem::transmute(&pwstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringForVarType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringForVarType(::core::mem::transmute_copy(&vt), ::core::mem::transmute_copy(&ptdescarraytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringForVariant: GetStringForVariant::<Identity, Impl, OFFSET>,
            GetVariantForString: GetVariantForString::<Identity, Impl, OFFSET>,
            GetStringForVarType: GetStringForVarType::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugFormatter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugHelper_Impl: Sized {
    fn CreatePropertyBrowser(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: &::windows_core::PCWSTR, pdat: ::core::option::Option<&IDebugApplicationThread>) -> ::windows_core::Result<super::IDebugProperty>;
    fn CreatePropertyBrowserEx(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: &::windows_core::PCWSTR, pdat: ::core::option::Option<&IDebugApplicationThread>, pdf: ::core::option::Option<&IDebugFormatter>) -> ::windows_core::Result<super::IDebugProperty>;
    fn CreateSimpleConnectionPoint(&self, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<ISimpleConnectionPoint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDebugHelper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDebugHelper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: isize>() -> IDebugHelper_Vtbl {
        unsafe extern "system" fn CreatePropertyBrowser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: ::windows_core::PCWSTR, pdat: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyBrowser(::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pdat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyBrowserEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: ::windows_core::PCWSTR, pdat: *mut ::core::ffi::c_void, pdf: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePropertyBrowserEx(::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pdat), ::windows_core::from_raw_borrowed(&pdf)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSimpleConnectionPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, ppscp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSimpleConnectionPoint(::windows_core::from_raw_borrowed(&pdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyBrowser: CreatePropertyBrowser::<Identity, Impl, OFFSET>,
            CreatePropertyBrowserEx: CreatePropertyBrowserEx::<Identity, Impl, OFFSET>,
            CreateSimpleConnectionPoint: CreateSimpleConnectionPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugHelper as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugSessionProvider_Impl: Sized {
    fn StartDebugSession(&self, pda: ::core::option::Option<&IRemoteDebugApplication>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugSessionProvider {}
impl IDebugSessionProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSessionProvider_Impl, const OFFSET: isize>() -> IDebugSessionProvider_Vtbl {
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSessionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartDebugSession(::windows_core::from_raw_borrowed(&pda)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugSessionProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugStackFrame_Impl: Sized {
    fn GetCodeContext(&self) -> ::windows_core::Result<IDebugCodeContext>;
    fn GetDescriptionString(&self, flong: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLanguageString(&self, flong: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetThread(&self) -> ::windows_core::Result<IDebugApplicationThread>;
    fn GetDebugProperty(&self) -> ::windows_core::Result<super::IDebugProperty>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugStackFrame {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugStackFrame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>() -> IDebugStackFrame_Vtbl {
        unsafe extern "system" fn GetCodeContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodeContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptionString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescriptionString(::core::mem::transmute_copy(&flong)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrlanguage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageString(::core::mem::transmute_copy(&flong)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlanguage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDebugProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDebugProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodeContext: GetCodeContext::<Identity, Impl, OFFSET>,
            GetDescriptionString: GetDescriptionString::<Identity, Impl, OFFSET>,
            GetLanguageString: GetLanguageString::<Identity, Impl, OFFSET>,
            GetThread: GetThread::<Identity, Impl, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugStackFrame as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugStackFrame110_Impl: Sized + IDebugStackFrame_Impl {
    fn GetStackFrameType(&self) -> ::windows_core::Result<DEBUG_STACKFRAME_TYPE>;
    fn GetScriptInvocationContext(&self) -> ::windows_core::Result<IScriptInvocationContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDebugStackFrame110 {}
#[cfg(feature = "Win32_Foundation")]
impl IDebugStackFrame110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: isize>() -> IDebugStackFrame110_Vtbl {
        unsafe extern "system" fn GetStackFrameType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstackframekind: *mut DEBUG_STACKFRAME_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStackFrameType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstackframekind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScriptInvocationContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinvocationcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptInvocationContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinvocationcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDebugStackFrame_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStackFrameType: GetStackFrameType::<Identity, Impl, OFFSET>,
            GetScriptInvocationContext: GetScriptInvocationContext::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugStackFrame110 as ::windows_core::ComInterface>::IID || *iid == <IDebugStackFrame as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugStackFrameSniffer_Impl: Sized {
    fn EnumStackFrames(&self) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
impl ::windows_core::RuntimeName for IDebugStackFrameSniffer {}
impl IDebugStackFrameSniffer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSniffer_Impl, const OFFSET: isize>() -> IDebugStackFrameSniffer_Vtbl {
        unsafe extern "system" fn EnumStackFrames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSniffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStackFrames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumStackFrames: EnumStackFrames::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugStackFrameSniffer as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugStackFrameSnifferEx32_Impl: Sized + IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx32(&self, dwspmin: u32) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
impl ::windows_core::RuntimeName for IDebugStackFrameSnifferEx32 {}
impl IDebugStackFrameSnifferEx32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSnifferEx32_Impl, const OFFSET: isize>() -> IDebugStackFrameSnifferEx32_Vtbl {
        unsafe extern "system" fn EnumStackFramesEx32<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSnifferEx32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspmin: u32, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStackFramesEx32(::core::mem::transmute_copy(&dwspmin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDebugStackFrameSniffer_Vtbl::new::<Identity, Impl, OFFSET>(), EnumStackFramesEx32: EnumStackFramesEx32::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugStackFrameSnifferEx32 as ::windows_core::ComInterface>::IID || *iid == <IDebugStackFrameSniffer as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugStackFrameSnifferEx64_Impl: Sized + IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx64(&self, dwspmin: u64) -> ::windows_core::Result<IEnumDebugStackFrames64>;
}
impl ::windows_core::RuntimeName for IDebugStackFrameSnifferEx64 {}
impl IDebugStackFrameSnifferEx64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSnifferEx64_Impl, const OFFSET: isize>() -> IDebugStackFrameSnifferEx64_Vtbl {
        unsafe extern "system" fn EnumStackFramesEx64<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugStackFrameSnifferEx64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspmin: u64, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStackFramesEx64(::core::mem::transmute_copy(&dwspmin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDebugStackFrameSniffer_Vtbl::new::<Identity, Impl, OFFSET>(), EnumStackFramesEx64: EnumStackFramesEx64::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugStackFrameSnifferEx64 as ::windows_core::ComInterface>::IID || *iid == <IDebugStackFrameSniffer as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugSyncOperation_Impl: Sized {
    fn GetTargetThread(&self) -> ::windows_core::Result<IDebugApplicationThread>;
    fn Execute(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InProgressAbort(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugSyncOperation {}
impl IDebugSyncOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: isize>() -> IDebugSyncOperation_Vtbl {
        unsafe extern "system" fn GetTargetThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppattarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Execute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InProgressAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InProgressAbort().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTargetThread: GetTargetThread::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            InProgressAbort: InProgressAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugSyncOperation as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugThreadCall32_Impl: Sized {
    fn ThreadCallHandler(&self, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugThreadCall32 {}
impl IDebugThreadCall32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugThreadCall32_Impl, const OFFSET: isize>() -> IDebugThreadCall32_Vtbl {
        unsafe extern "system" fn ThreadCallHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugThreadCall32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadCallHandler(::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThreadCallHandler: ThreadCallHandler::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugThreadCall32 as ::windows_core::ComInterface>::IID
    }
}
pub trait IDebugThreadCall64_Impl: Sized {
    fn ThreadCallHandler(&self, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDebugThreadCall64 {}
impl IDebugThreadCall64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugThreadCall64_Impl, const OFFSET: isize>() -> IDebugThreadCall64_Vtbl {
        unsafe extern "system" fn ThreadCallHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDebugThreadCall64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ThreadCallHandler(::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThreadCallHandler: ThreadCallHandler::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDebugThreadCall64 as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumDebugApplicationNodes_Impl: Sized {
    fn Next(&self, celt: u32, pprddp: *mut ::core::option::Option<IDebugApplicationNode>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDebugApplicationNodes>;
}
impl ::windows_core::RuntimeName for IEnumDebugApplicationNodes {}
impl IEnumDebugApplicationNodes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>() -> IEnumDebugApplicationNodes_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pprddp: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprddp), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumDebugApplicationNodes as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumDebugCodeContexts_Impl: Sized {
    fn Next(&self, celt: u32, pscc: *mut ::core::option::Option<IDebugCodeContext>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::RuntimeName for IEnumDebugCodeContexts {}
impl IEnumDebugCodeContexts_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: isize>() -> IEnumDebugCodeContexts_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pscc: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pscc), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumDebugCodeContexts as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumDebugExpressionContexts_Impl: Sized {
    fn Next(&self, celt: u32, ppdec: *mut ::core::option::Option<IDebugExpressionContext>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::RuntimeName for IEnumDebugExpressionContexts {}
impl IEnumDebugExpressionContexts_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>() -> IEnumDebugExpressionContexts_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppdec: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdec), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumDebugExpressionContexts as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDebugStackFrames_Impl: Sized {
    fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnumDebugStackFrames {}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDebugStackFrames_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: isize>() -> IEnumDebugStackFrames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&prgdsfd), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumDebugStackFrames as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDebugStackFrames64_Impl: Sized + IEnumDebugStackFrames_Impl {
    fn Next64(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnumDebugStackFrames64 {}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDebugStackFrames64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames64_Impl, const OFFSET: isize>() -> IEnumDebugStackFrames64_Vtbl {
        unsafe extern "system" fn Next64<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDebugStackFrames64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next64(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&prgdsfd), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self { base__: IEnumDebugStackFrames_Vtbl::new::<Identity, Impl, OFFSET>(), Next64: Next64::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumDebugStackFrames64 as ::windows_core::ComInterface>::IID || *iid == <IEnumDebugStackFrames as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumJsStackFrames_Impl: Sized {
    fn Next(&self, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumJsStackFrames {}
impl IEnumJsStackFrames_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: isize>() -> IEnumJsStackFrames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cframecount), ::core::mem::transmute_copy(&pframes), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET>, Reset: Reset::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumJsStackFrames as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumRemoteDebugApplicationThreads_Impl: Sized {
    fn Next(&self, celt: u32, pprdat: *mut ::core::option::Option<IRemoteDebugApplicationThread>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumRemoteDebugApplicationThreads>;
}
impl ::windows_core::RuntimeName for IEnumRemoteDebugApplicationThreads {}
impl IEnumRemoteDebugApplicationThreads_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>() -> IEnumRemoteDebugApplicationThreads_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pprdat: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprdat), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperdat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumRemoteDebugApplicationThreads as ::windows_core::ComInterface>::IID
    }
}
pub trait IEnumRemoteDebugApplications_Impl: Sized {
    fn Next(&self, celt: u32, ppda: *mut ::core::option::Option<IRemoteDebugApplication>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::RuntimeName for IEnumRemoteDebugApplications {}
impl IEnumRemoteDebugApplications_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>() -> IEnumRemoteDebugApplications_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppda: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppda), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppessd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppessd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEnumRemoteDebugApplications as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsDebug_Impl: Sized {
    fn OpenVirtualProcess(&self, processid: u32, runtimejsbaseaddress: u64, pdatatarget: ::core::option::Option<&IJsDebugDataTarget>) -> ::windows_core::Result<IJsDebugProcess>;
}
impl ::windows_core::RuntimeName for IJsDebug {}
impl IJsDebug_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebug_Impl, const OFFSET: isize>() -> IJsDebug_Vtbl {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32, runtimejsbaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, ppprocess: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenVirtualProcess(::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&runtimejsbaseaddress), ::windows_core::from_raw_borrowed(&pdatatarget)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprocess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenVirtualProcess: OpenVirtualProcess::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebug as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IJsDebugBreakPoint_Impl: Sized {
    fn IsEnabled(&self) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn Enable(&self) -> ::windows_core::Result<()>;
    fn Disable(&self) -> ::windows_core::Result<()>;
    fn Delete(&self) -> ::windows_core::Result<()>;
    fn GetDocumentPosition(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IJsDebugBreakPoint {}
#[cfg(feature = "Win32_Foundation")]
impl IJsDebugBreakPoint_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>() -> IJsDebugBreakPoint_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisenabled: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable().into()
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disable().into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GetDocumentPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentPosition(::core::mem::transmute_copy(&pdocumentid), ::core::mem::transmute_copy(&pcharacteroffset), ::core::mem::transmute_copy(&pstatementcharcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetDocumentPosition: GetDocumentPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugBreakPoint as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsDebugDataTarget_Impl: Sized {
    fn ReadMemory(&self, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> ::windows_core::Result<()>;
    fn WriteMemory(&self, address: u64, pmemory: *const u8, size: u32) -> ::windows_core::Result<()>;
    fn AllocateVirtualMemory(&self, address: u64, size: u32, allocationtype: u32, pageprotection: u32) -> ::windows_core::Result<u64>;
    fn FreeVirtualMemory(&self, address: u64, size: u32, freetype: u32) -> ::windows_core::Result<()>;
    fn GetTlsValue(&self, threadid: u32, tlsindex: u32) -> ::windows_core::Result<u64>;
    fn ReadBSTR(&self, address: u64) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReadNullTerminatedString(&self, address: u64, charactersize: u16, maxcharacters: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateStackFrameEnumerator(&self, threadid: u32) -> ::windows_core::Result<IEnumJsStackFrames>;
    fn GetThreadContext(&self, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IJsDebugDataTarget {}
impl IJsDebugDataTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>() -> IJsDebugDataTarget_Vtbl {
        unsafe extern "system" fn ReadMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadMemory(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pbytesread)).into()
        }
        unsafe extern "system" fn WriteMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, pmemory: *const u8, size: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteMemory(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&pmemory), ::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn AllocateVirtualMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, size: u32, allocationtype: u32, pageprotection: u32, pallocatedaddress: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocateVirtualMemory(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&allocationtype), ::core::mem::transmute_copy(&pageprotection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallocatedaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeVirtualMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, size: u32, freetype: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeVirtualMemory(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&freetype)).into()
        }
        unsafe extern "system" fn GetTlsValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32, tlsindex: u32, pvalue: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTlsValue(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&tlsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBSTR<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, pstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadBSTR(::core::mem::transmute_copy(&address)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadNullTerminatedString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u64, charactersize: u16, maxcharacters: u32, pstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadNullTerminatedString(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&charactersize), ::core::mem::transmute_copy(&maxcharacters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStackFrameEnumerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStackFrameEnumerator(::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetThreadContext(::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&contextsize), ::core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMemory: ReadMemory::<Identity, Impl, OFFSET>,
            WriteMemory: WriteMemory::<Identity, Impl, OFFSET>,
            AllocateVirtualMemory: AllocateVirtualMemory::<Identity, Impl, OFFSET>,
            FreeVirtualMemory: FreeVirtualMemory::<Identity, Impl, OFFSET>,
            GetTlsValue: GetTlsValue::<Identity, Impl, OFFSET>,
            ReadBSTR: ReadBSTR::<Identity, Impl, OFFSET>,
            ReadNullTerminatedString: ReadNullTerminatedString::<Identity, Impl, OFFSET>,
            CreateStackFrameEnumerator: CreateStackFrameEnumerator::<Identity, Impl, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugDataTarget as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsDebugFrame_Impl: Sized {
    fn GetStackRange(&self, pstart: *mut u64, pend: *mut u64) -> ::windows_core::Result<()>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocumentPositionWithId(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetDocumentPositionWithName(&self, pdocumentname: *mut ::windows_core::BSTR, pline: *mut u32, pcolumn: *mut u32) -> ::windows_core::Result<()>;
    fn GetDebugProperty(&self) -> ::windows_core::Result<IJsDebugProperty>;
    fn GetReturnAddress(&self) -> ::windows_core::Result<u64>;
    fn Evaluate(&self, pexpressiontext: &::windows_core::PCWSTR, ppdebugproperty: *mut ::core::option::Option<IJsDebugProperty>, perror: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IJsDebugFrame {}
impl IJsDebugFrame_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>() -> IJsDebugFrame_Vtbl {
        unsafe extern "system" fn GetStackRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstart: *mut u64, pend: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStackRange(::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentPositionWithId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentPositionWithId(::core::mem::transmute_copy(&pdocumentid), ::core::mem::transmute_copy(&pcharacteroffset), ::core::mem::transmute_copy(&pstatementcharcount)).into()
        }
        unsafe extern "system" fn GetDocumentPositionWithName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pline: *mut u32, pcolumn: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentPositionWithName(::core::mem::transmute_copy(&pdocumentname), ::core::mem::transmute_copy(&pline), ::core::mem::transmute_copy(&pcolumn)).into()
        }
        unsafe extern "system" fn GetDebugProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdebugproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDebugProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReturnAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preturnaddress: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReturnAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Evaluate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pexpressiontext: ::windows_core::PCWSTR, ppdebugproperty: *mut *mut ::core::ffi::c_void, perror: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Evaluate(::core::mem::transmute(&pexpressiontext), ::core::mem::transmute_copy(&ppdebugproperty), ::core::mem::transmute_copy(&perror)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStackRange: GetStackRange::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDocumentPositionWithId: GetDocumentPositionWithId::<Identity, Impl, OFFSET>,
            GetDocumentPositionWithName: GetDocumentPositionWithName::<Identity, Impl, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, Impl, OFFSET>,
            GetReturnAddress: GetReturnAddress::<Identity, Impl, OFFSET>,
            Evaluate: Evaluate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugFrame as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IJsDebugProcess_Impl: Sized {
    fn CreateStackWalker(&self, threadid: u32) -> ::windows_core::Result<IJsDebugStackWalker>;
    fn CreateBreakPoint(&self, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<IJsDebugBreakPoint>;
    fn PerformAsyncBreak(&self, threadid: u32) -> ::windows_core::Result<()>;
    fn GetExternalStepAddress(&self) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IJsDebugProcess {}
#[cfg(feature = "Win32_Foundation")]
impl IJsDebugProcess_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: isize>() -> IJsDebugProcess_Vtbl {
        unsafe extern "system" fn CreateStackWalker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32, ppstackwalker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStackWalker(::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstackwalker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: super::super::super::super::Foundation::BOOL, ppdebugbreakpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBreakPoint(::core::mem::transmute_copy(&documentid), ::core::mem::transmute_copy(&characteroffset), ::core::mem::transmute_copy(&charactercount), ::core::mem::transmute_copy(&isenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugbreakpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerformAsyncBreak<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PerformAsyncBreak(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn GetExternalStepAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcodeaddress: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExternalStepAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcodeaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStackWalker: CreateStackWalker::<Identity, Impl, OFFSET>,
            CreateBreakPoint: CreateBreakPoint::<Identity, Impl, OFFSET>,
            PerformAsyncBreak: PerformAsyncBreak::<Identity, Impl, OFFSET>,
            GetExternalStepAddress: GetExternalStepAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugProcess as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsDebugProperty_Impl: Sized {
    fn GetPropertyInfo(&self, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows_core::Result<()>;
    fn GetMembers(&self, members: JS_PROPERTY_MEMBERS) -> ::windows_core::Result<IJsEnumDebugProperty>;
}
impl ::windows_core::RuntimeName for IJsDebugProperty {}
impl IJsDebugProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: isize>() -> IJsDebugProperty_Vtbl {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyInfo(::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&ppropertyinfo)).into()
        }
        unsafe extern "system" fn GetMembers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, members: JS_PROPERTY_MEMBERS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMembers(::core::mem::transmute_copy(&members)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetMembers: GetMembers::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugProperty as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsDebugStackWalker_Impl: Sized {
    fn GetNext(&self) -> ::windows_core::Result<IJsDebugFrame>;
}
impl ::windows_core::RuntimeName for IJsDebugStackWalker {}
impl IJsDebugStackWalker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugStackWalker_Impl, const OFFSET: isize>() -> IJsDebugStackWalker_Vtbl {
        unsafe extern "system" fn GetNext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsDebugStackWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNext: GetNext::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsDebugStackWalker as ::windows_core::ComInterface>::IID
    }
}
pub trait IJsEnumDebugProperty_Impl: Sized {
    fn Next(&self, count: u32, ppdebugproperty: *mut ::core::option::Option<IJsDebugProperty>, pactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IJsEnumDebugProperty {}
impl IJsEnumDebugProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: isize>() -> IJsEnumDebugProperty_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32, ppdebugproperty: *mut *mut ::core::ffi::c_void, pactualcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppdebugproperty), ::core::mem::transmute_copy(&pactualcount)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IJsEnumDebugProperty as ::windows_core::ComInterface>::IID
    }
}
pub trait IMachineDebugManager_Impl: Sized {
    fn AddApplication(&self, pda: ::core::option::Option<&IRemoteDebugApplication>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn EnumApplications(&self) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::RuntimeName for IMachineDebugManager {}
impl IMachineDebugManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: isize>() -> IMachineDebugManager_Vtbl {
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddApplication(::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplication(::core::mem::transmute_copy(&dwappcookie)).into()
        }
        unsafe extern "system" fn EnumApplications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumApplications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            EnumApplications: EnumApplications::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IMachineDebugManager as ::windows_core::ComInterface>::IID
    }
}
pub trait IMachineDebugManagerCookie_Impl: Sized {
    fn AddApplication(&self, pda: ::core::option::Option<&IRemoteDebugApplication>, dwdebugappcookie: u32) -> ::windows_core::Result<u32>;
    fn RemoveApplication(&self, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn EnumApplications(&self) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::RuntimeName for IMachineDebugManagerCookie {}
impl IMachineDebugManagerCookie_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: isize>() -> IMachineDebugManagerCookie_Vtbl {
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwdebugappcookie: u32, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddApplication(::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwdebugappcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplication(::core::mem::transmute_copy(&dwdebugappcookie), ::core::mem::transmute_copy(&dwappcookie)).into()
        }
        unsafe extern "system" fn EnumApplications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumApplications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            EnumApplications: EnumApplications::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IMachineDebugManagerCookie as ::windows_core::ComInterface>::IID
    }
}
pub trait IMachineDebugManagerEvents_Impl: Sized {
    fn onAddApplication(&self, pda: ::core::option::Option<&IRemoteDebugApplication>, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn onRemoveApplication(&self, pda: ::core::option::Option<&IRemoteDebugApplication>, dwappcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMachineDebugManagerEvents {}
impl IMachineDebugManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: isize>() -> IMachineDebugManagerEvents_Vtbl {
        unsafe extern "system" fn onAddApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onAddApplication(::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwappcookie)).into()
        }
        unsafe extern "system" fn onRemoveApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.onRemoveApplication(::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwappcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onAddApplication: onAddApplication::<Identity, Impl, OFFSET>,
            onRemoveApplication: onRemoveApplication::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IMachineDebugManagerEvents as ::windows_core::ComInterface>::IID
    }
}
pub trait IProcessDebugManager32_Impl: Sized {
    fn CreateApplication(&self) -> ::windows_core::Result<IDebugApplication32>;
    fn GetDefaultApplication(&self) -> ::windows_core::Result<IDebugApplication32>;
    fn AddApplication(&self, pda: ::core::option::Option<&IDebugApplication32>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentHelper(&self, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDebugDocumentHelper32>;
}
impl ::windows_core::RuntimeName for IProcessDebugManager32 {}
impl IProcessDebugManager32_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>() -> IProcessDebugManager32_Vtbl {
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddApplication(::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplication(::core::mem::transmute_copy(&dwappcookie)).into()
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDebugDocumentHelper(::windows_core::from_raw_borrowed(&punkouter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pddh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, Impl, OFFSET>,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IProcessDebugManager32 as ::windows_core::ComInterface>::IID
    }
}
pub trait IProcessDebugManager64_Impl: Sized {
    fn CreateApplication(&self) -> ::windows_core::Result<IDebugApplication64>;
    fn GetDefaultApplication(&self) -> ::windows_core::Result<IDebugApplication64>;
    fn AddApplication(&self, pda: ::core::option::Option<&IDebugApplication64>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentHelper(&self, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDebugDocumentHelper64>;
}
impl ::windows_core::RuntimeName for IProcessDebugManager64 {}
impl IProcessDebugManager64_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>() -> IProcessDebugManager64_Vtbl {
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddApplication(::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveApplication(::core::mem::transmute_copy(&dwappcookie)).into()
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDebugDocumentHelper(::windows_core::from_raw_borrowed(&punkouter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pddh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, Impl, OFFSET>,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IProcessDebugManager64 as ::windows_core::ComInterface>::IID
    }
}
pub trait IProvideExpressionContexts_Impl: Sized {
    fn EnumExpressionContexts(&self) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::RuntimeName for IProvideExpressionContexts {}
impl IProvideExpressionContexts_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideExpressionContexts_Impl, const OFFSET: isize>() -> IProvideExpressionContexts_Vtbl {
        unsafe extern "system" fn EnumExpressionContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvideExpressionContexts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumExpressionContexts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumExpressionContexts: EnumExpressionContexts::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IProvideExpressionContexts as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugApplication_Impl: Sized {
    fn ResumeFromBreakPoint(&self, prptfocus: ::core::option::Option<&IRemoteDebugApplicationThread>, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows_core::Result<()>;
    fn CauseBreak(&self) -> ::windows_core::Result<()>;
    fn ConnectDebugger(&self, pad: ::core::option::Option<&IApplicationDebugger>) -> ::windows_core::Result<()>;
    fn DisconnectDebugger(&self) -> ::windows_core::Result<()>;
    fn GetDebugger(&self) -> ::windows_core::Result<IApplicationDebugger>;
    fn CreateInstanceAtApplication(&self, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryAlive(&self) -> ::windows_core::Result<()>;
    fn EnumThreads(&self) -> ::windows_core::Result<IEnumRemoteDebugApplicationThreads>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRootNode(&self) -> ::windows_core::Result<IDebugApplicationNode>;
    fn EnumGlobalExpressionContexts(&self) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::RuntimeName for IRemoteDebugApplication {}
impl IRemoteDebugApplication_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>() -> IRemoteDebugApplication_Vtbl {
        unsafe extern "system" fn ResumeFromBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prptfocus: *mut ::core::ffi::c_void, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeFromBreakPoint(::windows_core::from_raw_borrowed(&prptfocus), ::core::mem::transmute_copy(&bra), ::core::mem::transmute_copy(&era)).into()
        }
        unsafe extern "system" fn CauseBreak<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CauseBreak().into()
        }
        unsafe extern "system" fn ConnectDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectDebugger(::windows_core::from_raw_borrowed(&pad)).into()
        }
        unsafe extern "system" fn DisconnectDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectDebugger().into()
        }
        unsafe extern "system" fn GetDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pad: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDebugger() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pad, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceAtApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstanceAtApplication(::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAlive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryAlive().into()
        }
        unsafe extern "system" fn EnumThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumThreads() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperdat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootNode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRootNode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumGlobalExpressionContexts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumGlobalExpressionContexts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResumeFromBreakPoint: ResumeFromBreakPoint::<Identity, Impl, OFFSET>,
            CauseBreak: CauseBreak::<Identity, Impl, OFFSET>,
            ConnectDebugger: ConnectDebugger::<Identity, Impl, OFFSET>,
            DisconnectDebugger: DisconnectDebugger::<Identity, Impl, OFFSET>,
            GetDebugger: GetDebugger::<Identity, Impl, OFFSET>,
            CreateInstanceAtApplication: CreateInstanceAtApplication::<Identity, Impl, OFFSET>,
            QueryAlive: QueryAlive::<Identity, Impl, OFFSET>,
            EnumThreads: EnumThreads::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetRootNode: GetRootNode::<Identity, Impl, OFFSET>,
            EnumGlobalExpressionContexts: EnumGlobalExpressionContexts::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugApplication as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugApplication110_Impl: Sized {
    fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::Result<()>;
    fn GetCurrentDebuggerOptions(&self) -> ::windows_core::Result<SCRIPT_DEBUGGER_OPTIONS>;
    fn GetMainThread(&self) -> ::windows_core::Result<IRemoteDebugApplicationThread>;
}
impl ::windows_core::RuntimeName for IRemoteDebugApplication110 {}
impl IRemoteDebugApplication110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: isize>() -> IRemoteDebugApplication110_Vtbl {
        unsafe extern "system" fn SetDebuggerOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDebuggerOptions(::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetCurrentDebuggerOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcurrentoptions: *mut SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentDebuggerOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcurrentoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMainThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMainThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDebuggerOptions: SetDebuggerOptions::<Identity, Impl, OFFSET>,
            GetCurrentDebuggerOptions: GetCurrentDebuggerOptions::<Identity, Impl, OFFSET>,
            GetMainThread: GetMainThread::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugApplication110 as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugApplicationEvents_Impl: Sized {
    fn OnConnectDebugger(&self, pad: ::core::option::Option<&IApplicationDebugger>) -> ::windows_core::Result<()>;
    fn OnDisconnectDebugger(&self) -> ::windows_core::Result<()>;
    fn OnSetName(&self, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDebugOutput(&self, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnClose(&self) -> ::windows_core::Result<()>;
    fn OnEnterBreakPoint(&self, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnLeaveBreakPoint(&self, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnCreateThread(&self, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnDestroyThread(&self, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnBreakFlagChange(&self, abf: u32, prdatsteppingthread: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRemoteDebugApplicationEvents {}
impl IRemoteDebugApplicationEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>() -> IRemoteDebugApplicationEvents_Vtbl {
        unsafe extern "system" fn OnConnectDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnectDebugger(::windows_core::from_raw_borrowed(&pad)).into()
        }
        unsafe extern "system" fn OnDisconnectDebugger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnectDebugger().into()
        }
        unsafe extern "system" fn OnSetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetName(::core::mem::transmute(&pstrname)).into()
        }
        unsafe extern "system" fn OnDebugOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDebugOutput(::core::mem::transmute(&pstr)).into()
        }
        unsafe extern "system" fn OnClose<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClose().into()
        }
        unsafe extern "system" fn OnEnterBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnterBreakPoint(::windows_core::from_raw_borrowed(&prdat)).into()
        }
        unsafe extern "system" fn OnLeaveBreakPoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLeaveBreakPoint(::windows_core::from_raw_borrowed(&prdat)).into()
        }
        unsafe extern "system" fn OnCreateThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCreateThread(::windows_core::from_raw_borrowed(&prdat)).into()
        }
        unsafe extern "system" fn OnDestroyThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDestroyThread(::windows_core::from_raw_borrowed(&prdat)).into()
        }
        unsafe extern "system" fn OnBreakFlagChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, abf: u32, prdatsteppingthread: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBreakFlagChange(::core::mem::transmute_copy(&abf), ::windows_core::from_raw_borrowed(&prdatsteppingthread)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectDebugger: OnConnectDebugger::<Identity, Impl, OFFSET>,
            OnDisconnectDebugger: OnDisconnectDebugger::<Identity, Impl, OFFSET>,
            OnSetName: OnSetName::<Identity, Impl, OFFSET>,
            OnDebugOutput: OnDebugOutput::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
            OnEnterBreakPoint: OnEnterBreakPoint::<Identity, Impl, OFFSET>,
            OnLeaveBreakPoint: OnLeaveBreakPoint::<Identity, Impl, OFFSET>,
            OnCreateThread: OnCreateThread::<Identity, Impl, OFFSET>,
            OnDestroyThread: OnDestroyThread::<Identity, Impl, OFFSET>,
            OnBreakFlagChange: OnBreakFlagChange::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugApplicationEvents as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugApplicationThread_Impl: Sized {
    fn GetSystemThreadId(&self) -> ::windows_core::Result<u32>;
    fn GetApplication(&self) -> ::windows_core::Result<IRemoteDebugApplication>;
    fn EnumStackFrames(&self) -> ::windows_core::Result<IEnumDebugStackFrames>;
    fn GetDescription(&self, pbstrdescription: *mut ::windows_core::BSTR, pbstrstate: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetNextStatement(&self, pstackframe: ::core::option::Option<&IDebugStackFrame>, pcodecontext: ::core::option::Option<&IDebugCodeContext>) -> ::windows_core::Result<()>;
    fn GetState(&self) -> ::windows_core::Result<u32>;
    fn Suspend(&self) -> ::windows_core::Result<u32>;
    fn Resume(&self) -> ::windows_core::Result<u32>;
    fn GetSuspendCount(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IRemoteDebugApplicationThread {}
impl IRemoteDebugApplicationThread_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>() -> IRemoteDebugApplicationThread_Vtbl {
        unsafe extern "system" fn GetSystemThreadId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwthreadid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumStackFrames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumStackFrames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrstate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDescription(::core::mem::transmute_copy(&pbstrdescription), ::core::mem::transmute_copy(&pbstrstate)).into()
        }
        unsafe extern "system" fn SetNextStatement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstackframe: *mut ::core::ffi::c_void, pcodecontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNextStatement(::windows_core::from_raw_borrowed(&pstackframe), ::windows_core::from_raw_borrowed(&pcodecontext)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Suspend() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuspendCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSuspendCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemThreadId: GetSystemThreadId::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            EnumStackFrames: EnumStackFrames::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetNextStatement: SetNextStatement::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            GetSuspendCount: GetSuspendCount::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugApplicationThread as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugCriticalErrorEvent110_Impl: Sized {
    fn GetErrorInfo(&self, pbstrsource: *mut ::windows_core::BSTR, pmessageid: *mut i32, pbstrmessage: *mut ::windows_core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRemoteDebugCriticalErrorEvent110 {}
impl IRemoteDebugCriticalErrorEvent110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: isize>() -> IRemoteDebugCriticalErrorEvent110_Vtbl {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pmessageid: *mut i32, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorInfo(::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pmessageid), ::core::mem::transmute_copy(&pbstrmessage), ::core::mem::transmute_copy(&pplocation)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugCriticalErrorEvent110 as ::windows_core::ComInterface>::IID
    }
}
pub trait IRemoteDebugInfoEvent110_Impl: Sized {
    fn GetEventInfo(&self, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::windows_core::BSTR, pbstrurl: *mut ::windows_core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRemoteDebugInfoEvent110 {}
impl IRemoteDebugInfoEvent110_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugInfoEvent110_Impl, const OFFSET: isize>() -> IRemoteDebugInfoEvent110_Vtbl {
        unsafe extern "system" fn GetEventInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteDebugInfoEvent110_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventInfo(::core::mem::transmute_copy(&pmessagetype), ::core::mem::transmute_copy(&pbstrmessage), ::core::mem::transmute_copy(&pbstrurl), ::core::mem::transmute_copy(&pplocation)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetEventInfo: GetEventInfo::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRemoteDebugInfoEvent110 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptEntry_Impl: Sized + IScriptNode_Impl {
    fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetText(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBody(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBody(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetItemName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetItemName(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignature(&self, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> ::windows_core::Result<()>;
    fn SetSignature(&self, pti: ::core::option::Option<&super::super::super::Com::ITypeInfo>, imethod: u32) -> ::windows_core::Result<()>;
    fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IScriptEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IScriptEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>() -> IScriptEntry_Vtbl {
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetBody<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBody() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetItemName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetItemName(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void, pimethod: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignature(::core::mem::transmute_copy(&ppti), ::core::mem::transmute_copy(&pimethod)).into()
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pti: *mut ::core::ffi::c_void, imethod: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignature(::windows_core::from_raw_borrowed(&pti), ::core::mem::transmute_copy(&imethod)).into()
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pichmin: *mut u32, pcch: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRange(::core::mem::transmute_copy(&pichmin), ::core::mem::transmute_copy(&pcch)).into()
        }
        Self {
            base__: IScriptNode_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetBody: GetBody::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            SetItemName: SetItemName::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScriptEntry as ::windows_core::ComInterface>::IID || *iid == <IScriptNode as ::windows_core::ComInterface>::IID
    }
}
pub trait IScriptInvocationContext_Impl: Sized {
    fn GetContextType(&self) -> ::windows_core::Result<SCRIPT_INVOCATION_CONTEXT_TYPE>;
    fn GetContextDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContextObject(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IScriptInvocationContext {}
impl IScriptInvocationContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: isize>() -> IScriptInvocationContext_Vtbl {
        unsafe extern "system" fn GetContextType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinvocationcontexttype: *mut SCRIPT_INVOCATION_CONTEXT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinvocationcontexttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontextobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContextType: GetContextType::<Identity, Impl, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, Impl, OFFSET>,
            GetContextObject: GetContextObject::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScriptInvocationContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptNode_Impl: Sized {
    fn Alive(&self) -> ::windows_core::Result<()>;
    fn Delete(&self) -> ::windows_core::Result<()>;
    fn GetParent(&self) -> ::windows_core::Result<IScriptNode>;
    fn GetIndexInParent(&self) -> ::windows_core::Result<u32>;
    fn GetCookie(&self) -> ::windows_core::Result<u32>;
    fn GetNumberOfChildren(&self) -> ::windows_core::Result<u32>;
    fn GetChild(&self, isn: u32) -> ::windows_core::Result<IScriptNode>;
    fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateChildEntry(&self, isn: u32, dwcookie: u32, pszdelimiter: &::windows_core::PCWSTR) -> ::windows_core::Result<IScriptEntry>;
    fn CreateChildHandler(&self, pszdefaultname: &::windows_core::PCWSTR, prgpsznames: *const ::windows_core::PCWSTR, cpsznames: u32, pszevent: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, ptisignature: ::core::option::Option<&super::super::super::Com::ITypeInfo>, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows_core::Result<IScriptEntry>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IScriptNode {}
#[cfg(feature = "Win32_System_Com")]
impl IScriptNode_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>() -> IScriptNode_Vtbl {
        unsafe extern "system" fn Alive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alive().into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsnparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsnparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexInParent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisn: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexInParent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCookie() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOfChildren<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsn: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumberOfChildren() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChild<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isn: u32, ppsn: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChild(::core::mem::transmute_copy(&isn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChildEntry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isn: u32, dwcookie: u32, pszdelimiter: ::windows_core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateChildEntry(::core::mem::transmute_copy(&isn), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute(&pszdelimiter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChildHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows_core::PCWSTR, prgpsznames: *const ::windows_core::PCWSTR, cpsznames: u32, pszevent: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, ptisignature: *mut ::core::ffi::c_void, imethodsignature: u32, isn: u32, dwcookie: u32, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateChildHandler(::core::mem::transmute(&pszdefaultname), ::core::mem::transmute_copy(&prgpsznames), ::core::mem::transmute_copy(&cpsznames), ::core::mem::transmute(&pszevent), ::core::mem::transmute(&pszdelimiter), ::windows_core::from_raw_borrowed(&ptisignature), ::core::mem::transmute_copy(&imethodsignature), ::core::mem::transmute_copy(&isn), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alive: Alive::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            GetIndexInParent: GetIndexInParent::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetNumberOfChildren: GetNumberOfChildren::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            CreateChildEntry: CreateChildEntry::<Identity, Impl, OFFSET>,
            CreateChildHandler: CreateChildHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScriptNode as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptScriptlet_Impl: Sized + IScriptEntry_Impl {
    fn GetSubItemName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubItemName(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetEventName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEventName(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSimpleEventName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSimpleEventName(&self, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IScriptScriptlet {}
#[cfg(feature = "Win32_System_Com")]
impl IScriptScriptlet_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>() -> IScriptScriptlet_Vtbl {
        unsafe extern "system" fn GetSubItemName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubItemName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubItemName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubItemName(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetEventName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventName(::core::mem::transmute(&psz)).into()
        }
        unsafe extern "system" fn GetSimpleEventName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSimpleEventName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimpleEventName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSimpleEventName(::core::mem::transmute(&psz)).into()
        }
        Self {
            base__: IScriptEntry_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSubItemName: GetSubItemName::<Identity, Impl, OFFSET>,
            SetSubItemName: SetSubItemName::<Identity, Impl, OFFSET>,
            GetEventName: GetEventName::<Identity, Impl, OFFSET>,
            SetEventName: SetEventName::<Identity, Impl, OFFSET>,
            GetSimpleEventName: GetSimpleEventName::<Identity, Impl, OFFSET>,
            SetSimpleEventName: SetSimpleEventName::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScriptScriptlet as ::windows_core::ComInterface>::IID || *iid == <IScriptNode as ::windows_core::ComInterface>::IID || *iid == <IScriptEntry as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISimpleConnectionPoint_Impl: Sized {
    fn GetEventCount(&self) -> ::windows_core::Result<u32>;
    fn DescribeEvents(&self, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::windows_core::BSTR, pceventsfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Advise(&self, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISimpleConnectionPoint {}
#[cfg(feature = "Win32_System_Com")]
impl ISimpleConnectionPoint_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: isize>() -> ISimpleConnectionPoint_Vtbl {
        unsafe extern "system" fn GetEventCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DescribeEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pceventsfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DescribeEvents(::core::mem::transmute_copy(&ievent), ::core::mem::transmute_copy(&cevents), ::core::mem::transmute_copy(&prgid), ::core::mem::transmute_copy(&prgbstr), ::core::mem::transmute_copy(&pceventsfetched)).into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::windows_core::from_raw_borrowed(&pdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEventCount: GetEventCount::<Identity, Impl, OFFSET>,
            DescribeEvents: DescribeEvents::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISimpleConnectionPoint as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITridentEventSink_Impl: Sized {
    fn FireEvent(&self, pstrevent: &::windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITridentEventSink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITridentEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentEventSink_Impl, const OFFSET: isize>() -> ITridentEventSink_Vtbl {
        unsafe extern "system" fn FireEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITridentEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrevent: ::windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireEvent(::core::mem::transmute(&pstrevent), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FireEvent: FireEvent::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ITridentEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAppDiagnosticsObjectInitialization_Impl: Sized {
    fn Initialize(&self, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWebAppDiagnosticsObjectInitialization {}
#[cfg(feature = "Win32_Foundation")]
impl IWebAppDiagnosticsObjectInitialization_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: isize>() -> IWebAppDiagnosticsObjectInitialization_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hpassedhandle), ::windows_core::from_raw_borrowed(&pdebugapplication)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWebAppDiagnosticsObjectInitialization as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAppDiagnosticsSetup_Impl: Sized {
    fn DiagnosticsSupported(&self) -> ::windows_core::Result<super::super::super::super::Foundation::VARIANT_BOOL>;
    fn CreateObjectWithSiteAtWebApp(&self, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, hpasstoobject: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWebAppDiagnosticsSetup {}
#[cfg(feature = "Win32_Foundation")]
impl IWebAppDiagnosticsSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>() -> IWebAppDiagnosticsSetup_Vtbl {
        unsafe extern "system" fn DiagnosticsSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiagnosticsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateObjectWithSiteAtWebApp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, hpasstoobject: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateObjectWithSiteAtWebApp(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&hpasstoobject)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DiagnosticsSupported: DiagnosticsSupported::<Identity, Impl, OFFSET>,
            CreateObjectWithSiteAtWebApp: CreateObjectWithSiteAtWebApp::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWebAppDiagnosticsSetup as ::windows_core::ComInterface>::IID
    }
}
