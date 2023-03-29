#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct AsyncIDebugApplicationNodeEvents(::windows::core::IUnknown);
impl AsyncIDebugApplicationNodeEvents {
    pub unsafe fn Begin_onAddChild<P0>(&self, prddpchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).Begin_onAddChild)(::windows::core::Interface::as_raw(self), prddpchild.into_param().abi()).ok()
    }
    pub unsafe fn Finish_onAddChild(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_onAddChild)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_onRemoveChild<P0>(&self, prddpchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).Begin_onRemoveChild)(::windows::core::Interface::as_raw(self), prddpchild.into_param().abi()).ok()
    }
    pub unsafe fn Finish_onRemoveChild(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_onRemoveChild)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_onDetach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_onDetach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_onDetach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_onDetach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_onAttach<P0>(&self, prddpparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).Begin_onAttach)(::windows::core::Interface::as_raw(self), prddpparent.into_param().abi()).ok()
    }
    pub unsafe fn Finish_onAttach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_onAttach)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(AsyncIDebugApplicationNodeEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for AsyncIDebugApplicationNodeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIDebugApplicationNodeEvents {}
impl ::core::fmt::Debug for AsyncIDebugApplicationNodeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIDebugApplicationNodeEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIDebugApplicationNodeEvents {
    type Vtable = AsyncIDebugApplicationNodeEvents_Vtbl;
}
impl ::core::clone::Clone for AsyncIDebugApplicationNodeEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIDebugApplicationNodeEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2e3aa3b_aa8d_4ebf_84cd_648b737b8c13);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIDebugApplicationNodeEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_onAddChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_onAddChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_onRemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_onRemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_onDetach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_onDetach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_onAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_onAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScript(::windows::core::IUnknown);
impl IActiveScript {
    pub unsafe fn SetScriptSite<P0>(&self, pass: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptSite>,
    {
        (::windows::core::Interface::vtable(self).SetScriptSite)(::windows::core::Interface::as_raw(self), pass.into_param().abi()).ok()
    }
    pub unsafe fn GetScriptSite<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetScriptSite)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScriptState(&self, ss: SCRIPTSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScriptState)(::windows::core::Interface::as_raw(self), ss).ok()
    }
    pub unsafe fn GetScriptState(&self) -> ::windows::core::Result<SCRIPTSTATE> {
        let mut result__ = ::windows::core::zeroed::<SCRIPTSTATE>();
        (::windows::core::Interface::vtable(self).GetScriptState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddNamedItem<P0>(&self, pstrname: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddNamedItem)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn AddTypeLib(&self, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTypeLib)(::windows::core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScriptDispatch<P0>(&self, pstritemname: P0) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).GetScriptDispatch)(::windows::core::Interface::as_raw(self), pstritemname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentScriptThreadID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentScriptThreadID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScriptThreadID(&self, dwwin32threadid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetScriptThreadID)(::windows::core::Interface::as_raw(self), dwwin32threadid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScriptThreadState(&self, stidthread: u32) -> ::windows::core::Result<SCRIPTTHREADSTATE> {
        let mut result__ = ::windows::core::zeroed::<SCRIPTTHREADSTATE>();
        (::windows::core::Interface::vtable(self).GetScriptThreadState)(::windows::core::Interface::as_raw(self), stidthread, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InterruptScriptThread(&self, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InterruptScriptThread)(::windows::core::Interface::as_raw(self), stidthread, pexcepinfo, dwflags).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IActiveScript> {
        let mut result__ = ::windows::core::zeroed::<IActiveScript>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScript, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScript {}
impl ::core::fmt::Debug for IActiveScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScript").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScript {
    type Vtable = IActiveScript_Vtbl;
}
impl ::core::clone::Clone for IActiveScript {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScript {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb1a2ae1_a4f9_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScript_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetScriptSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pass: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScriptSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScriptState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ss: SCRIPTSTATE) -> ::windows::core::HRESULT,
    pub GetScriptState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pssstate: *mut SCRIPTSTATE) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    pub AddTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetScriptDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstritemname: ::windows::core::PCWSTR, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetScriptDispatch: usize,
    pub GetCurrentScriptThreadID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstidthread: *mut u32) -> ::windows::core::HRESULT,
    pub GetScriptThreadID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwin32threadid: u32, pstidthread: *mut u32) -> ::windows::core::HRESULT,
    pub GetScriptThreadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stidthread: u32, pstsstate: *mut SCRIPTTHREADSTATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InterruptScriptThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InterruptScriptThread: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscript: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptAuthor(::windows::core::IUnknown);
impl IActiveScriptAuthor {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddNamedItem<P0, P1>(&self, pszname: P0, dwflags: u32, pdisp: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).AddNamedItem)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), dwflags, pdisp.into_param().abi()).ok()
    }
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pszdefaultname: P0, pszcode: P1, pszitemname: P2, pszsubitemname: P3, pszeventname: P4, pszdelimiter: P5, dwcookie: u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddScriptlet)(::windows::core::Interface::as_raw(self), pszdefaultname.into_param().abi(), pszcode.into_param().abi(), pszitemname.into_param().abi(), pszsubitemname.into_param().abi(), pszeventname.into_param().abi(), pszdelimiter.into_param().abi(), dwcookie, dwflags).ok()
    }
    pub unsafe fn ParseScriptText<P0, P1, P2>(&self, pszcode: P0, pszitemname: P1, pszdelimiter: P2, dwcookie: u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParseScriptText)(::windows::core::Interface::as_raw(self), pszcode.into_param().abi(), pszitemname.into_param().abi(), pszdelimiter.into_param().abi(), dwcookie, dwflags).ok()
    }
    pub unsafe fn GetScriptTextAttributes<P0, P1>(&self, pszcode: P0, cch: u32, pszdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptTextAttributes)(::windows::core::Interface::as_raw(self), pszcode.into_param().abi(), cch, pszdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P1>(&self, pszcode: P0, cch: u32, pszdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptletTextAttributes)(::windows::core::Interface::as_raw(self), pszcode.into_param().abi(), cch, pszdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn GetRoot(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).GetRoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguageFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLanguageFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEventHandler<P0, P1, P2, P3>(&self, pdisp: P0, pszitem: P1, pszsubitem: P2, pszevent: P3) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).GetEventHandler)(::windows::core::Interface::as_raw(self), pdisp.into_param().abi(), pszitem.into_param().abi(), pszsubitem.into_param().abi(), pszevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveNamedItem<P0>(&self, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveNamedItem)(::windows::core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn AddTypeLib(&self, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTypeLib)(::windows::core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor, dwflags).ok()
    }
    pub unsafe fn RemoveTypeLib(&self, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveTypeLib)(::windows::core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor).ok()
    }
    pub unsafe fn GetChars(&self, frequestedlist: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetChars)(::windows::core::Interface::as_raw(self), frequestedlist, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfoFromContext<P0>(&self, pszcode: P0, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetInfoFromContext)(::windows::core::Interface::as_raw(self), pszcode.into_param().abi(), cchcode, ichcurrentposition, dwlisttypesrequested, pdwlisttypesprovided, pichlistanchorposition, pichfuncanchorposition, pmemid, picurrentparameter, ::core::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCommitChar(&self, ch: u16) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsCommitChar)(::windows::core::Interface::as_raw(self), ch, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptAuthor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptAuthor {}
impl ::core::fmt::Debug for IActiveScriptAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptAuthor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptAuthor {
    type Vtable = IActiveScriptAuthor_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptAuthor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptAuthor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c109da0_7006_11d1_b36c_00a0c911e8b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptAuthor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, dwflags: u32, pdisp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddNamedItem: usize,
    pub AddScriptlet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows::core::PCWSTR, pszcode: ::windows::core::PCWSTR, pszitemname: ::windows::core::PCWSTR, pszsubitemname: ::windows::core::PCWSTR, pszeventname: ::windows::core::PCWSTR, pszdelimiter: ::windows::core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub ParseScriptText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcode: ::windows::core::PCWSTR, pszitemname: ::windows::core::PCWSTR, pszdelimiter: ::windows::core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetScriptTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcode: ::windows::core::PCWSTR, cch: u32, pszdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcode: ::windows::core::PCWSTR, cch: u32, pszdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLanguageFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrfasa: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pszitem: ::windows::core::PCWSTR, pszsubitem: ::windows::core::PCWSTR, pszevent: ::windows::core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEventHandler: usize,
    pub RemoveNamedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub RemoveTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows::core::GUID, dwmajor: u32, dwminor: u32) -> ::windows::core::HRESULT,
    pub GetChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequestedlist: u32, pbstrchars: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetInfoFromContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcode: ::windows::core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCommitChar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ch: u16, pfcommit: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCommitChar: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptAuthorProcedure(::windows::core::IUnknown);
impl IActiveScriptAuthorProcedure {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pszcode: P0, pszformalparams: P1, pszprocedurename: P2, pszitemname: P3, pszdelimiter: P4, dwcookie: u32, dwflags: u32, pdispfor: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ParseProcedureText)(::windows::core::Interface::as_raw(self), pszcode.into_param().abi(), pszformalparams.into_param().abi(), pszprocedurename.into_param().abi(), pszitemname.into_param().abi(), pszdelimiter.into_param().abi(), dwcookie, dwflags, pdispfor.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptAuthorProcedure, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptAuthorProcedure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptAuthorProcedure {}
impl ::core::fmt::Debug for IActiveScriptAuthorProcedure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptAuthorProcedure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptAuthorProcedure {
    type Vtable = IActiveScriptAuthorProcedure_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptAuthorProcedure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptAuthorProcedure {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e2d4b70_bd9a_11d0_9336_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptAuthorProcedure_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcode: ::windows::core::PCWSTR, pszformalparams: ::windows::core::PCWSTR, pszprocedurename: ::windows::core::PCWSTR, pszitemname: ::windows::core::PCWSTR, pszdelimiter: ::windows::core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptDebug32(::windows::core::IUnknown);
impl IActiveScriptDebug32 {
    pub unsafe fn GetScriptTextAttributes<P0, P1>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptTextAttributes)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), unumcodechars, pstrdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P1>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptletTextAttributes)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), unumcodechars, pstrdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows::core::Result<IEnumDebugCodeContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugCodeContexts>();
        (::windows::core::Interface::vtable(self).EnumCodeContextsOfPosition)(::windows::core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptDebug32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptDebug32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptDebug32 {}
impl ::core::fmt::Debug for IActiveScriptDebug32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptDebug32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptDebug32 {
    type Vtable = IActiveScriptDebug32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptDebug32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptDebug32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c10_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptDebug32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetScriptTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub EnumCodeContextsOfPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptDebug64(::windows::core::IUnknown);
impl IActiveScriptDebug64 {
    pub unsafe fn GetScriptTextAttributes<P0, P1>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptTextAttributes)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), unumcodechars, pstrdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P1>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptletTextAttributes)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), unumcodechars, pstrdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows::core::Result<IEnumDebugCodeContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugCodeContexts>();
        (::windows::core::Interface::vtable(self).EnumCodeContextsOfPosition)(::windows::core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptDebug64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptDebug64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptDebug64 {}
impl ::core::fmt::Debug for IActiveScriptDebug64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptDebug64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptDebug64 {
    type Vtable = IActiveScriptDebug64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptDebug64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptDebug64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc437e23_f5b8_47f4_bb79_7d1ce5483b86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptDebug64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetScriptTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub EnumCodeContextsOfPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptEncode(::windows::core::IUnknown);
impl IActiveScriptEncode {
    pub unsafe fn EncodeSection<P0>(&self, pchin: P0, cchin: u32, pchout: ::windows::core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EncodeSection)(::windows::core::Interface::as_raw(self), pchin.into_param().abi(), cchin, ::core::mem::transmute(pchout), cchout, pcchret).ok()
    }
    pub unsafe fn DecodeScript<P0>(&self, pchin: P0, cchin: u32, pchout: ::windows::core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DecodeScript)(::windows::core::Interface::as_raw(self), pchin.into_param().abi(), cchin, ::core::mem::transmute(pchout), cchout, pcchret).ok()
    }
    pub unsafe fn GetEncodeProgId(&self, pbstrout: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEncodeProgId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrout)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptEncode, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptEncode {}
impl ::core::fmt::Debug for IActiveScriptEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptEncode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptEncode {
    type Vtable = IActiveScriptEncode_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptEncode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb1a2ae3_a4f9_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptEncode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EncodeSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchin: ::windows::core::PCWSTR, cchin: u32, pchout: ::windows::core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows::core::HRESULT,
    pub DecodeScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchin: ::windows::core::PCWSTR, cchin: u32, pchout: ::windows::core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows::core::HRESULT,
    pub GetEncodeProgId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrout: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptError(::windows::core::IUnknown);
impl IActiveScriptError {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExceptionInfo)(::windows::core::Interface::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSourcePosition)(::windows::core::Interface::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSourceLineText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptError, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptError {}
impl ::core::fmt::Debug for IActiveScriptError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptError {
    type Vtable = IActiveScriptError_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeae1ba61_a4ed_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptError_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExceptionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExceptionInfo: usize,
    pub GetSourcePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::HRESULT,
    pub GetSourceLineText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsourceline: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptError64(::windows::core::IUnknown);
impl IActiveScriptError64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetExceptionInfo)(::windows::core::Interface::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSourcePosition)(::windows::core::Interface::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSourceLineText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourcePosition64(&self, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSourcePosition64)(::windows::core::Interface::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptError64, ::windows::core::IUnknown, IActiveScriptError);
impl ::core::cmp::PartialEq for IActiveScriptError64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptError64 {}
impl ::core::fmt::Debug for IActiveScriptError64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptError64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptError64 {
    type Vtable = IActiveScriptError64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptError64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptError64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb21fb2a1_5b8f_4963_8c21_21450f84ed7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptError64_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetSourcePosition64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptErrorDebug(::windows::core::IUnknown);
impl IActiveScriptErrorDebug {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetExceptionInfo)(::windows::core::Interface::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSourcePosition)(::windows::core::Interface::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSourceLineText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentContext(&self) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).GetDocumentContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStackFrame(&self) -> ::windows::core::Result<IDebugStackFrame> {
        let mut result__ = ::windows::core::zeroed::<IDebugStackFrame>();
        (::windows::core::Interface::vtable(self).GetStackFrame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptErrorDebug, ::windows::core::IUnknown, IActiveScriptError);
impl ::core::cmp::PartialEq for IActiveScriptErrorDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptErrorDebug {}
impl ::core::fmt::Debug for IActiveScriptErrorDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptErrorDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptErrorDebug {
    type Vtable = IActiveScriptErrorDebug_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptErrorDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptErrorDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c12_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptErrorDebug_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetDocumentContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppssc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStackFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptErrorDebug110(::windows::core::IUnknown);
impl IActiveScriptErrorDebug110 {
    pub unsafe fn GetExceptionThrownKind(&self) -> ::windows::core::Result<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND> {
        let mut result__ = ::windows::core::zeroed::<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND>();
        (::windows::core::Interface::vtable(self).GetExceptionThrownKind)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptErrorDebug110, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptErrorDebug110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptErrorDebug110 {}
impl ::core::fmt::Debug for IActiveScriptErrorDebug110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptErrorDebug110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptErrorDebug110 {
    type Vtable = IActiveScriptErrorDebug110_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptErrorDebug110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptErrorDebug110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x516e42b6_89a8_4530_937b_5f0708431442);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptErrorDebug110_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetExceptionThrownKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexceptionkind: *mut SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptGarbageCollector(::windows::core::IUnknown);
impl IActiveScriptGarbageCollector {
    pub unsafe fn CollectGarbage(&self, scriptgctype: SCRIPTGCTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CollectGarbage)(::windows::core::Interface::as_raw(self), scriptgctype).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptGarbageCollector, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptGarbageCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptGarbageCollector {}
impl ::core::fmt::Debug for IActiveScriptGarbageCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptGarbageCollector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptGarbageCollector {
    type Vtable = IActiveScriptGarbageCollector_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptGarbageCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptGarbageCollector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aa2c4a0_2b53_11d4_a2a0_00104bd35090);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptGarbageCollector_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CollectGarbage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptgctype: SCRIPTGCTYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptHostEncode(::windows::core::IUnknown);
impl IActiveScriptHostEncode {
    pub unsafe fn EncodeScriptHostFile<P0, P1>(&self, bstrinfile: P0, pbstroutfile: *mut ::windows::core::BSTR, cflags: u32, bstrdefaultlang: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).EncodeScriptHostFile)(::windows::core::Interface::as_raw(self), bstrinfile.into_param().abi(), ::core::mem::transmute(pbstroutfile), cflags, bstrdefaultlang.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptHostEncode, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptHostEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptHostEncode {}
impl ::core::fmt::Debug for IActiveScriptHostEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptHostEncode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptHostEncode {
    type Vtable = IActiveScriptHostEncode_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptHostEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptHostEncode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee9b76e_cfe3_11d1_b747_00c04fc2b085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptHostEncode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EncodeScriptHostFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinfile: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstroutfile: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, cflags: u32, bstrdefaultlang: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParse32(::windows::core::IUnknown);
impl IActiveScriptParse32 {
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitNew)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pstrdefaultname: P0, pstrcode: P1, pstritemname: P2, pstrsubitemname: P3, pstreventname: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows::core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddScriptlet)(::windows::core::Interface::as_raw(self), pstrdefaultname.into_param().abi(), pstrcode.into_param().abi(), pstritemname.into_param().abi(), pstrsubitemname.into_param().abi(), pstreventname.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, ::core::mem::transmute(pbstrname), pexcepinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParseScriptText<P0, P1, P2, P3>(&self, pstrcode: P0, pstritemname: P1, punkcontext: P2, pstrdelimiter: P3, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Com::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParseScriptText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, pvarresult, pexcepinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParse32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParse32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParse32 {}
impl ::core::fmt::Debug for IActiveScriptParse32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParse32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParse32 {
    type Vtable = IActiveScriptParse32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParse32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParse32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb1a2ae2_a4f9_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParse32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddScriptlet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows::core::PCWSTR, pstrcode: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, pstrsubitemname: ::windows::core::PCWSTR, pstreventname: ::windows::core::PCWSTR, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddScriptlet: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ParseScriptText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Com::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ParseScriptText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParse64(::windows::core::IUnknown);
impl IActiveScriptParse64 {
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitNew)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pstrdefaultname: P0, pstrcode: P1, pstritemname: P2, pstrsubitemname: P3, pstreventname: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows::core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddScriptlet)(::windows::core::Interface::as_raw(self), pstrdefaultname.into_param().abi(), pstrcode.into_param().abi(), pstritemname.into_param().abi(), pstrsubitemname.into_param().abi(), pstreventname.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, ::core::mem::transmute(pbstrname), pexcepinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParseScriptText<P0, P1, P2, P3>(&self, pstrcode: P0, pstritemname: P1, punkcontext: P2, pstrdelimiter: P3, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Com::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ParseScriptText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, pvarresult, pexcepinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParse64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParse64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParse64 {}
impl ::core::fmt::Debug for IActiveScriptParse64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParse64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParse64 {
    type Vtable = IActiveScriptParse64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParse64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParse64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7ef7658_e1ee_480e_97ea_d52cb4d76d17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParse64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddScriptlet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows::core::PCWSTR, pstrcode: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, pstrsubitemname: ::windows::core::PCWSTR, pstreventname: ::windows::core::PCWSTR, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddScriptlet: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ParseScriptText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Com::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ParseScriptText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedure2_32(::windows::core::IUnknown);
impl IActiveScriptParseProcedure2_32 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).base__.ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstrprocedurename.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedure2_32, ::windows::core::IUnknown, IActiveScriptParseProcedure32);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure2_32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure2_32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure2_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure2_32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedure2_32 {
    type Vtable = IActiveScriptParseProcedure2_32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedure2_32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedure2_32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71ee5b20_fb04_11d1_b3a8_00a0c911e8b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure2_32_Vtbl {
    pub base__: IActiveScriptParseProcedure32_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedure2_64(::windows::core::IUnknown);
impl IActiveScriptParseProcedure2_64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).base__.ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstrprocedurename.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedure2_64, ::windows::core::IUnknown, IActiveScriptParseProcedure64);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure2_64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure2_64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure2_64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure2_64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedure2_64 {
    type Vtable = IActiveScriptParseProcedure2_64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedure2_64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedure2_64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe7c4271_210c_448d_9f54_76dab7047b28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure2_64_Vtbl {
    pub base__: IActiveScriptParseProcedure64_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedure32(::windows::core::IUnknown);
impl IActiveScriptParseProcedure32 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstrprocedurename.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedure32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedure32 {
    type Vtable = IActiveScriptParseProcedure32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedure32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedure32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa5b6a80_b834_11d0_932f_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstrformalparams: ::windows::core::PCWSTR, pstrprocedurename: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedure64(::windows::core::IUnknown);
impl IActiveScriptParseProcedure64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P5: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstrprocedurename.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedure64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedure64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedure64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedure64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedure64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedure64 {
    type Vtable = IActiveScriptParseProcedure64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedure64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedure64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc64713b6_e029_4cc5_9200_438b72890b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstrformalparams: ::windows::core::PCWSTR, pstrprocedurename: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedureOld32(::windows::core::IUnknown);
impl IActiveScriptParseProcedureOld32 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4>(&self, pstrcode: P0, pstrformalparams: P1, pstritemname: P2, punkcontext: P3, pstrdelimiter: P4, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedureOld32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedureOld32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedureOld32 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedureOld32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedureOld32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedureOld32 {
    type Vtable = IActiveScriptParseProcedureOld32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedureOld32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedureOld32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cff0050_6fdd_11d0_9328_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedureOld32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstrformalparams: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptParseProcedureOld64(::windows::core::IUnknown);
impl IActiveScriptParseProcedureOld64 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4>(&self, pstrcode: P0, pstrformalparams: P1, pstritemname: P2, punkcontext: P3, pstrdelimiter: P4, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows::core::Result<super::super::super::Com::IDispatch>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).ParseProcedureText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), pstrformalparams.into_param().abi(), pstritemname.into_param().abi(), punkcontext.into_param().abi(), pstrdelimiter.into_param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptParseProcedureOld64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptParseProcedureOld64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptParseProcedureOld64 {}
impl ::core::fmt::Debug for IActiveScriptParseProcedureOld64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptParseProcedureOld64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptParseProcedureOld64 {
    type Vtable = IActiveScriptParseProcedureOld64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptParseProcedureOld64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptParseProcedureOld64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21f57128_08c9_4638_ba12_22d15d88dc5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedureOld64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, pstrformalparams: ::windows::core::PCWSTR, pstritemname: ::windows::core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows::core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback(::windows::core::IUnknown);
impl IActiveScriptProfilerCallback {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), dwcontext).ok()
    }
    pub unsafe fn Shutdown(&self, hrreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self), hrreason).ok()
    }
    pub unsafe fn ScriptCompiled<P0>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).ScriptCompiled)(::windows::core::Interface::as_raw(self), scriptid, r#type, pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn FunctionCompiled<P0, P1, P2>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P0, pwszfunctionnamehint: P1, pidebugdocumentcontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).FunctionCompiled)(::windows::core::Interface::as_raw(self), functionid, scriptid, pwszfunctionname.into_param().abi(), pwszfunctionnamehint.into_param().abi(), pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnFunctionEnter)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnFunctionExit)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerCallback {
    type Vtable = IActiveScriptProfilerCallback_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x740eca23_7d9d_42e5_ba9d_f8b24b1c7a9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcontext: u32) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ScriptCompiled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FunctionCompiled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, functionid: i32, scriptid: i32, pwszfunctionname: ::windows::core::PCWSTR, pwszfunctionnamehint: ::windows::core::PCWSTR, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnFunctionEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows::core::HRESULT,
    pub OnFunctionExit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback2(::windows::core::IUnknown);
impl IActiveScriptProfilerCallback2 {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), dwcontext).ok()
    }
    pub unsafe fn Shutdown(&self, hrreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Shutdown)(::windows::core::Interface::as_raw(self), hrreason).ok()
    }
    pub unsafe fn ScriptCompiled<P0>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.ScriptCompiled)(::windows::core::Interface::as_raw(self), scriptid, r#type, pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn FunctionCompiled<P0, P1, P2>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P0, pwszfunctionnamehint: P1, pidebugdocumentcontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.FunctionCompiled)(::windows::core::Interface::as_raw(self), functionid, scriptid, pwszfunctionname.into_param().abi(), pwszfunctionnamehint.into_param().abi(), pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnFunctionEnter)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnFunctionExit)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionEnterByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnFunctionEnterByName)(::windows::core::Interface::as_raw(self), pwszfunctionname.into_param().abi(), r#type).ok()
    }
    pub unsafe fn OnFunctionExitByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnFunctionExitByName)(::windows::core::Interface::as_raw(self), pwszfunctionname.into_param().abi(), r#type).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerCallback2, ::windows::core::IUnknown, IActiveScriptProfilerCallback);
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback2 {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerCallback2 {
    type Vtable = IActiveScriptProfilerCallback2_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerCallback2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31b7f8ad_a637_409c_b22f_040995b6103d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback2_Vtbl {
    pub base__: IActiveScriptProfilerCallback_Vtbl,
    pub OnFunctionEnterByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows::core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::HRESULT,
    pub OnFunctionExitByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows::core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback3(::windows::core::IUnknown);
impl IActiveScriptProfilerCallback3 {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Initialize)(::windows::core::Interface::as_raw(self), dwcontext).ok()
    }
    pub unsafe fn Shutdown(&self, hrreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Shutdown)(::windows::core::Interface::as_raw(self), hrreason).ok()
    }
    pub unsafe fn ScriptCompiled<P0>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ScriptCompiled)(::windows::core::Interface::as_raw(self), scriptid, r#type, pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn FunctionCompiled<P0, P1, P2>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P0, pwszfunctionnamehint: P1, pidebugdocumentcontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.FunctionCompiled)(::windows::core::Interface::as_raw(self), functionid, scriptid, pwszfunctionname.into_param().abi(), pwszfunctionnamehint.into_param().abi(), pidebugdocumentcontext.into_param().abi()).ok()
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OnFunctionEnter)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.OnFunctionExit)(::windows::core::Interface::as_raw(self), scriptid, functionid).ok()
    }
    pub unsafe fn OnFunctionEnterByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.OnFunctionEnterByName)(::windows::core::Interface::as_raw(self), pwszfunctionname.into_param().abi(), r#type).ok()
    }
    pub unsafe fn OnFunctionExitByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.OnFunctionExitByName)(::windows::core::Interface::as_raw(self), pwszfunctionname.into_param().abi(), r#type).ok()
    }
    pub unsafe fn SetWebWorkerId(&self, webworkerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWebWorkerId)(::windows::core::Interface::as_raw(self), webworkerid).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerCallback3, ::windows::core::IUnknown, IActiveScriptProfilerCallback, IActiveScriptProfilerCallback2);
impl ::core::cmp::PartialEq for IActiveScriptProfilerCallback3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerCallback3 {}
impl ::core::fmt::Debug for IActiveScriptProfilerCallback3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerCallback3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerCallback3 {
    type Vtable = IActiveScriptProfilerCallback3_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerCallback3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerCallback3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac5ad25_2037_4687_91df_b59979d93d73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback3_Vtbl {
    pub base__: IActiveScriptProfilerCallback2_Vtbl,
    pub SetWebWorkerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webworkerid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerControl(::windows::core::IUnknown);
impl IActiveScriptProfilerControl {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartProfiling)(::windows::core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProfilerEventMask)(::windows::core::Interface::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopProfiling)(::windows::core::Interface::as_raw(self), hrshutdownreason).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerControl {
    type Vtable = IActiveScriptProfilerControl_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x784b5ff0_69b0_47d1_a7dc_2518f4230e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartProfiling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::HRESULT,
    pub SetProfilerEventMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweventmask: u32) -> ::windows::core::HRESULT,
    pub StopProfiling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerControl2(::windows::core::IUnknown);
impl IActiveScriptProfilerControl2 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartProfiling)(::windows::core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProfilerEventMask)(::windows::core::Interface::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopProfiling)(::windows::core::Interface::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteProfilerStart)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareProfilerStop)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerControl2, ::windows::core::IUnknown, IActiveScriptProfilerControl);
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl2 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerControl2 {
    type Vtable = IActiveScriptProfilerControl2_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47810165_498f_40be_94f1_653557e9e7da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl2_Vtbl {
    pub base__: IActiveScriptProfilerControl_Vtbl,
    pub CompleteProfilerStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrepareProfilerStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerControl3(::windows::core::IUnknown);
impl IActiveScriptProfilerControl3 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StartProfiling)(::windows::core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProfilerEventMask)(::windows::core::Interface::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.StopProfiling)(::windows::core::Interface::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CompleteProfilerStart)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.PrepareProfilerStop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumHeap(&self) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::windows::core::zeroed::<IActiveScriptProfilerHeapEnum>();
        (::windows::core::Interface::vtable(self).EnumHeap)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerControl3, ::windows::core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2);
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl3 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerControl3 {
    type Vtable = IActiveScriptProfilerControl3_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerControl3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerControl3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b403015_f381_4023_a5d0_6fed076de716);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl3_Vtbl {
    pub base__: IActiveScriptProfilerControl2_Vtbl,
    pub EnumHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerControl4(::windows::core::IUnknown);
impl IActiveScriptProfilerControl4 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StartProfiling)(::windows::core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetProfilerEventMask)(::windows::core::Interface::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.StopProfiling)(::windows::core::Interface::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CompleteProfilerStart)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.PrepareProfilerStop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumHeap(&self) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::windows::core::zeroed::<IActiveScriptProfilerHeapEnum>();
        (::windows::core::Interface::vtable(self).base__.EnumHeap)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SummarizeHeap)(::windows::core::Interface::as_raw(self), heapsummary).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerControl4, ::windows::core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2, IActiveScriptProfilerControl3);
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl4 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerControl4 {
    type Vtable = IActiveScriptProfilerControl4_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerControl4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerControl4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x160f94fd_9dbc_40d4_9eac_2b71db3132f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl4_Vtbl {
    pub base__: IActiveScriptProfilerControl3_Vtbl,
    pub SummarizeHeap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerControl5(::windows::core::IUnknown);
impl IActiveScriptProfilerControl5 {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const ::windows::core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.StartProfiling)(::windows::core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok()
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetProfilerEventMask)(::windows::core::Interface::as_raw(self), dweventmask).ok()
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.StopProfiling)(::windows::core::Interface::as_raw(self), hrshutdownreason).ok()
    }
    pub unsafe fn CompleteProfilerStart(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.CompleteProfilerStart)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrepareProfilerStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.PrepareProfilerStop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumHeap(&self) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::windows::core::zeroed::<IActiveScriptProfilerHeapEnum>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumHeap)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SummarizeHeap)(::windows::core::Interface::as_raw(self), heapsummary).ok()
    }
    pub unsafe fn EnumHeap2(&self, enumflags: PROFILER_HEAP_ENUM_FLAGS) -> ::windows::core::Result<IActiveScriptProfilerHeapEnum> {
        let mut result__ = ::windows::core::zeroed::<IActiveScriptProfilerHeapEnum>();
        (::windows::core::Interface::vtable(self).EnumHeap2)(::windows::core::Interface::as_raw(self), enumflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerControl5, ::windows::core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2, IActiveScriptProfilerControl3, IActiveScriptProfilerControl4);
impl ::core::cmp::PartialEq for IActiveScriptProfilerControl5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerControl5 {}
impl ::core::fmt::Debug for IActiveScriptProfilerControl5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerControl5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerControl5 {
    type Vtable = IActiveScriptProfilerControl5_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerControl5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerControl5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c01a2d1_8f0f_46a5_9720_0d7ed2c62f0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl5_Vtbl {
    pub base__: IActiveScriptProfilerControl4_Vtbl,
    pub EnumHeap2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumflags: PROFILER_HEAP_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProfilerHeapEnum(::windows::core::IUnknown);
impl IActiveScriptProfilerHeapEnum {
    pub unsafe fn Next(&self, heapobjects: &mut [*mut PROFILER_HEAP_OBJECT], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), heapobjects.len() as _, ::core::mem::transmute(heapobjects.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn GetOptionalInfo(&self, heapobject: *const PROFILER_HEAP_OBJECT, optionalinfo: &mut [PROFILER_HEAP_OBJECT_OPTIONAL_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOptionalInfo)(::windows::core::Interface::as_raw(self), heapobject, optionalinfo.len() as _, ::core::mem::transmute(optionalinfo.as_ptr())).ok()
    }
    pub unsafe fn FreeObjectAndOptionalInfo(&self, heapobjects: &[*const PROFILER_HEAP_OBJECT]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeObjectAndOptionalInfo)(::windows::core::Interface::as_raw(self), heapobjects.len() as _, ::core::mem::transmute(heapobjects.as_ptr())).ok()
    }
    pub unsafe fn GetNameIdMap(&self, pnamelist: *mut *mut *mut ::windows::core::PCWSTR, pcelt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNameIdMap)(::windows::core::Interface::as_raw(self), pnamelist, pcelt).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProfilerHeapEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptProfilerHeapEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProfilerHeapEnum {}
impl ::core::fmt::Debug for IActiveScriptProfilerHeapEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProfilerHeapEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProfilerHeapEnum {
    type Vtable = IActiveScriptProfilerHeapEnum_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProfilerHeapEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProfilerHeapEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32e4694e_0d37_419b_b93d_fa20ded6e8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerHeapEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptionalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> ::windows::core::HRESULT,
    pub FreeObjectAndOptionalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> ::windows::core::HRESULT,
    pub GetNameIdMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamelist: *mut *mut *mut ::windows::core::PCWSTR, pcelt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptProperty(::windows::core::IUnknown);
impl IActiveScriptProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Com::VARIANT) -> ::windows::core::Result<super::super::super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), dwproperty, pvarindex, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Com::VARIANT, pvarvalue: *const super::super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), dwproperty, pvarindex, pvarvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptProperty {}
impl ::core::fmt::Debug for IActiveScriptProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptProperty {
    type Vtable = IActiveScriptProperty_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4954e0d0_fbc7_11d1_8410_006008c3fbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Com::VARIANT, pvarvalue: *mut super::super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Com::VARIANT, pvarvalue: *const super::super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSIPInfo(::windows::core::IUnknown);
impl IActiveScriptSIPInfo {
    pub unsafe fn GetSIPOID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetSIPOID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSIPInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSIPInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSIPInfo {}
impl ::core::fmt::Debug for IActiveScriptSIPInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSIPInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSIPInfo {
    type Vtable = IActiveScriptSIPInfo_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSIPInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSIPInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x764651d0_38de_11d4_a2a3_00104bd35090);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSIPInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSIPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poid_sip: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSite(::windows::core::IUnknown);
impl IActiveScriptSite {
    pub unsafe fn GetLCID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLCID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItemInfo<P0>(&self, pstrname: P0, dwreturnmask: u32, ppiunkitem: *mut ::core::option::Option<::windows::core::IUnknown>, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetItemInfo)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi(), dwreturnmask, ::core::mem::transmute(ppiunkitem), ::core::mem::transmute(ppti)).ok()
    }
    pub unsafe fn GetDocVersionString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDocVersionString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OnScriptTerminate(&self, pvarresult: *const super::super::super::Com::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnScriptTerminate)(::windows::core::Interface::as_raw(self), pvarresult, pexcepinfo).ok()
    }
    pub unsafe fn OnStateChange(&self, ssscriptstate: SCRIPTSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStateChange)(::windows::core::Interface::as_raw(self), ssscriptstate).ok()
    }
    pub unsafe fn OnScriptError<P0>(&self, pscripterror: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptError>,
    {
        (::windows::core::Interface::vtable(self).OnScriptError)(::windows::core::Interface::as_raw(self), pscripterror.into_param().abi()).ok()
    }
    pub unsafe fn OnEnterScript(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEnterScript)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLeaveScript(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLeaveScript)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSite, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSite {}
impl ::core::fmt::Debug for IActiveScriptSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSite {
    type Vtable = IActiveScriptSite_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb01a1e3_a42b_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLCID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItemInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItemInfo: usize,
    pub GetDocVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrversion: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OnScriptTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarresult: *const super::super::super::Com::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OnScriptTerminate: usize,
    pub OnStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ssscriptstate: SCRIPTSTATE) -> ::windows::core::HRESULT,
    pub OnScriptError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscripterror: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEnterScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnLeaveScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteDebug32(::windows::core::IUnknown);
impl IActiveScriptSiteDebug32 {
    pub unsafe fn GetDocumentContextFromPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).GetDocumentContextFromPosition)(::windows::core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IDebugApplication32> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication32>();
        (::windows::core::Interface::vtable(self).GetApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetRootApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnScriptErrorDebug<P0>(&self, perrordebug: P0, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
    {
        (::windows::core::Interface::vtable(self).OnScriptErrorDebug)(::windows::core::Interface::as_raw(self), perrordebug.into_param().abi(), pfenterdebugger, pfcallonscripterrorwhencontinuing).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteDebug32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteDebug32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebug32 {}
impl ::core::fmt::Debug for IActiveScriptSiteDebug32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebug32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteDebug32 {
    type Vtable = IActiveScriptSiteDebug32_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteDebug32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteDebug32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c11_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebug32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentContextFromPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRootApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnScriptErrorDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnScriptErrorDebug: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteDebug64(::windows::core::IUnknown);
impl IActiveScriptSiteDebug64 {
    pub unsafe fn GetDocumentContextFromPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).GetDocumentContextFromPosition)(::windows::core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IDebugApplication64> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication64>();
        (::windows::core::Interface::vtable(self).GetApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetRootApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnScriptErrorDebug<P0>(&self, perrordebug: P0, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
    {
        (::windows::core::Interface::vtable(self).OnScriptErrorDebug)(::windows::core::Interface::as_raw(self), perrordebug.into_param().abi(), pfenterdebugger, pfcallonscripterrorwhencontinuing).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteDebug64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteDebug64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebug64 {}
impl ::core::fmt::Debug for IActiveScriptSiteDebug64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebug64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteDebug64 {
    type Vtable = IActiveScriptSiteDebug64_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteDebug64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteDebug64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b96b0a_7463_402c_92ac_89984226942f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebug64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentContextFromPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRootApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnScriptErrorDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnScriptErrorDebug: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteDebugEx(::windows::core::IUnknown);
impl IActiveScriptSiteDebugEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCanNotJITScriptErrorDebug<P0>(&self, perrordebug: P0) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnCanNotJITScriptErrorDebug)(::windows::core::Interface::as_raw(self), perrordebug.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteDebugEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteDebugEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteDebugEx {}
impl ::core::fmt::Debug for IActiveScriptSiteDebugEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteDebugEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteDebugEx {
    type Vtable = IActiveScriptSiteDebugEx_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteDebugEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteDebugEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb722ccb_6ad2_41c6_b780_af9c03ee69f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebugEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnCanNotJITScriptErrorDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnCanNotJITScriptErrorDebug: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteInterruptPoll(::windows::core::IUnknown);
impl IActiveScriptSiteInterruptPoll {
    pub unsafe fn QueryContinue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryContinue)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteInterruptPoll, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteInterruptPoll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteInterruptPoll {}
impl ::core::fmt::Debug for IActiveScriptSiteInterruptPoll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteInterruptPoll").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteInterruptPoll {
    type Vtable = IActiveScriptSiteInterruptPoll_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteInterruptPoll {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteInterruptPoll {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x539698a0_cdca_11cf_a5eb_00aa0047a063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteInterruptPoll_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteTraceInfo(::windows::core::IUnknown);
impl IActiveScriptSiteTraceInfo {
    pub unsafe fn SendScriptTraceInfo(&self, stieventtype: SCRIPTTRACEINFO, guidcontextid: ::windows::core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendScriptTraceInfo)(::windows::core::Interface::as_raw(self), stieventtype, ::core::mem::transmute(guidcontextid), dwscriptcontextcookie, lscriptstatementstart, lscriptstatementend, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteTraceInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteTraceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteTraceInfo {}
impl ::core::fmt::Debug for IActiveScriptSiteTraceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteTraceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteTraceInfo {
    type Vtable = IActiveScriptSiteTraceInfo_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteTraceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteTraceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b7272ae_1955_4bfe_98b0_780621888569);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteTraceInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SendScriptTraceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stieventtype: SCRIPTTRACEINFO, guidcontextid: ::windows::core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteUIControl(::windows::core::IUnknown);
impl IActiveScriptSiteUIControl {
    pub unsafe fn GetUIBehavior(&self, uicitem: SCRIPTUICITEM) -> ::windows::core::Result<SCRIPTUICHANDLING> {
        let mut result__ = ::windows::core::zeroed::<SCRIPTUICHANDLING>();
        (::windows::core::Interface::vtable(self).GetUIBehavior)(::windows::core::Interface::as_raw(self), uicitem, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteUIControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteUIControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteUIControl {}
impl ::core::fmt::Debug for IActiveScriptSiteUIControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteUIControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteUIControl {
    type Vtable = IActiveScriptSiteUIControl_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteUIControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteUIControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaedae97e_d7ee_4796_b960_7f092ae844ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteUIControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUIBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uicitem: SCRIPTUICITEM, puichandling: *mut SCRIPTUICHANDLING) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptSiteWindow(::windows::core::IUnknown);
impl IActiveScriptSiteWindow {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableModeless)(::windows::core::Interface::as_raw(self), fenable.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptSiteWindow, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptSiteWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptSiteWindow {}
impl ::core::fmt::Debug for IActiveScriptSiteWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptSiteWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptSiteWindow {
    type Vtable = IActiveScriptSiteWindow_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptSiteWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptSiteWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd10f6761_83e9_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteWindow_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptStats(::windows::core::IUnknown);
impl IActiveScriptStats {
    pub unsafe fn GetStat(&self, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStat)(::windows::core::Interface::as_raw(self), stid, pluhi, plulo).ok()
    }
    pub unsafe fn GetStatEx(&self, guid: *const ::windows::core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatEx)(::windows::core::Interface::as_raw(self), guid, pluhi, plulo).ok()
    }
    pub unsafe fn ResetStats(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetStats)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptStats, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptStats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptStats {}
impl ::core::fmt::Debug for IActiveScriptStats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptStats").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptStats {
    type Vtable = IActiveScriptStats_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptStats {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptStats {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8da6310_e19b_11d0_933c_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptStats_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows::core::HRESULT,
    pub ResetStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptStringCompare(::windows::core::IUnknown);
impl IActiveScriptStringCompare {
    pub unsafe fn StrComp<P0, P1>(&self, bszstr1: P0, bszstr2: P1) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StrComp)(::windows::core::Interface::as_raw(self), bszstr1.into_param().abi(), bszstr2.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptStringCompare, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptStringCompare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptStringCompare {}
impl ::core::fmt::Debug for IActiveScriptStringCompare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptStringCompare").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptStringCompare {
    type Vtable = IActiveScriptStringCompare_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptStringCompare {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptStringCompare {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58562769_ed52_42f7_8403_4963514e1f11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptStringCompare_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StrComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bszstr1: ::std::mem::MaybeUninit<::windows::core::BSTR>, bszstr2: ::std::mem::MaybeUninit<::windows::core::BSTR>, iret: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptTraceInfo(::windows::core::IUnknown);
impl IActiveScriptTraceInfo {
    pub unsafe fn StartScriptTracing<P0>(&self, psitetraceinfo: P0, guidcontextid: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptSiteTraceInfo>,
    {
        (::windows::core::Interface::vtable(self).StartScriptTracing)(::windows::core::Interface::as_raw(self), psitetraceinfo.into_param().abi(), ::core::mem::transmute(guidcontextid)).ok()
    }
    pub unsafe fn StopScriptTracing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopScriptTracing)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptTraceInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IActiveScriptTraceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptTraceInfo {}
impl ::core::fmt::Debug for IActiveScriptTraceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptTraceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptTraceInfo {
    type Vtable = IActiveScriptTraceInfo_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptTraceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptTraceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc35456e7_bebf_4a1b_86a9_24d56be8b369);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptTraceInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartScriptTracing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psitetraceinfo: *mut ::core::ffi::c_void, guidcontextid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub StopScriptTracing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IActiveScriptWinRTErrorDebug(::windows::core::IUnknown);
impl IActiveScriptWinRTErrorDebug {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetExceptionInfo)(::windows::core::Interface::as_raw(self), pexcepinfo).ok()
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSourcePosition)(::windows::core::Interface::as_raw(self), pdwsourcecontext, pullinenumber, plcharacterposition).ok()
    }
    pub unsafe fn GetSourceLineText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSourceLineText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRestrictedErrorString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetRestrictedErrorString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRestrictedErrorReference(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetRestrictedErrorReference)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCapabilitySid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetCapabilitySid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IActiveScriptWinRTErrorDebug, ::windows::core::IUnknown, IActiveScriptError);
impl ::core::cmp::PartialEq for IActiveScriptWinRTErrorDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveScriptWinRTErrorDebug {}
impl ::core::fmt::Debug for IActiveScriptWinRTErrorDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveScriptWinRTErrorDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActiveScriptWinRTErrorDebug {
    type Vtable = IActiveScriptWinRTErrorDebug_Vtbl;
}
impl ::core::clone::Clone for IActiveScriptWinRTErrorDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActiveScriptWinRTErrorDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73a3f82a_0fe9_4b33_ba3b_fe095f697e0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptWinRTErrorDebug_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetRestrictedErrorString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetRestrictedErrorReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencestring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetCapabilitySid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilitysid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IApplicationDebugger(::windows::core::IUnknown);
impl IApplicationDebugger {
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryAlive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateInstanceAtDebugger<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateInstanceAtDebugger)(::windows::core::Interface::as_raw(self), rclsid, punkouter.into_param().abi(), dwclscontext, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn onDebugOutput<P0>(&self, pstr: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).onDebugOutput)(::windows::core::Interface::as_raw(self), pstr.into_param().abi()).ok()
    }
    pub unsafe fn onHandleBreakPoint<P0, P1>(&self, prpt: P0, br: BREAKREASON, perror: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
        P1: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
    {
        (::windows::core::Interface::vtable(self).onHandleBreakPoint)(::windows::core::Interface::as_raw(self), prpt.into_param().abi(), br, perror.into_param().abi()).ok()
    }
    pub unsafe fn onClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onClose)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn onDebuggerEvent<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).onDebuggerEvent)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IApplicationDebugger, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApplicationDebugger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDebugger {}
impl ::core::fmt::Debug for IApplicationDebugger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDebugger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApplicationDebugger {
    type Vtable = IApplicationDebugger_Vtbl;
}
impl ::core::clone::Clone for IApplicationDebugger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApplicationDebugger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2a_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDebugger_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceAtDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub onHandleBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prpt: *mut ::core::ffi::c_void, br: BREAKREASON, perror: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onDebuggerEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IApplicationDebuggerUI(::windows::core::IUnknown);
impl IApplicationDebuggerUI {
    pub unsafe fn BringDocumentToTop<P0>(&self, pddt: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentText>,
    {
        (::windows::core::Interface::vtable(self).BringDocumentToTop)(::windows::core::Interface::as_raw(self), pddt.into_param().abi()).ok()
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentContext>,
    {
        (::windows::core::Interface::vtable(self).BringDocumentContextToTop)(::windows::core::Interface::as_raw(self), pddc.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IApplicationDebuggerUI, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IApplicationDebuggerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationDebuggerUI {}
impl ::core::fmt::Debug for IApplicationDebuggerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationDebuggerUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApplicationDebuggerUI {
    type Vtable = IApplicationDebuggerUI_Vtbl;
}
impl ::core::clone::Clone for IApplicationDebuggerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApplicationDebuggerUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2b_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDebuggerUI_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub BringDocumentToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddt: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IBindEventHandler(::windows::core::IUnknown);
impl IBindEventHandler {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindHandler<P0, P1>(&self, pstrevent: P0, pdisp: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).BindHandler)(::windows::core::Interface::as_raw(self), pstrevent.into_param().abi(), pdisp.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IBindEventHandler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IBindEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindEventHandler {}
impl ::core::fmt::Debug for IBindEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindEventHandler {
    type Vtable = IBindEventHandler_Vtbl;
}
impl ::core::clone::Clone for IBindEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBindEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63cdbcb0_c1b1_11d0_9336_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BindHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrevent: ::windows::core::PCWSTR, pdisp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindHandler: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplication11032(::windows::core::IUnknown);
impl IDebugApplication11032 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDebuggerOptions)(::windows::core::Interface::as_raw(self), mask, value).ok()
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> ::windows::core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        let mut result__ = ::windows::core::zeroed::<SCRIPT_DEBUGGER_OPTIONS>();
        (::windows::core::Interface::vtable(self).base__.GetCurrentDebuggerOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMainThread(&self) -> ::windows::core::Result<IRemoteDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).base__.GetMainThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallInMainThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn AsynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).AsynchronousCallInMainThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallableWaitForHandles(&self, phandles: &[super::super::super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CallableWaitForHandles)(::windows::core::Interface::as_raw(self), phandles.len() as _, ::core::mem::transmute(phandles.as_ptr()), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugApplication11032, ::windows::core::IUnknown, IRemoteDebugApplication110);
impl ::core::cmp::PartialEq for IDebugApplication11032 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication11032 {}
impl ::core::fmt::Debug for IDebugApplication11032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication11032").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplication11032 {
    type Vtable = IDebugApplication11032_Vtbl;
}
impl ::core::clone::Clone for IDebugApplication11032 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplication11032 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdb3b5de_89f2_4e11_84a5_97445f941c7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication11032_Vtbl {
    pub base__: IRemoteDebugApplication110_Vtbl,
    pub SynchronousCallInMainThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
    pub AsynchronousCallInMainThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CallableWaitForHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallableWaitForHandles: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplication11064(::windows::core::IUnknown);
impl IDebugApplication11064 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDebuggerOptions)(::windows::core::Interface::as_raw(self), mask, value).ok()
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> ::windows::core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        let mut result__ = ::windows::core::zeroed::<SCRIPT_DEBUGGER_OPTIONS>();
        (::windows::core::Interface::vtable(self).base__.GetCurrentDebuggerOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMainThread(&self) -> ::windows::core::Result<IRemoteDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).base__.GetMainThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall64>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallInMainThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn AsynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall64>,
    {
        (::windows::core::Interface::vtable(self).AsynchronousCallInMainThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CallableWaitForHandles(&self, phandles: &[super::super::super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).CallableWaitForHandles)(::windows::core::Interface::as_raw(self), phandles.len() as _, ::core::mem::transmute(phandles.as_ptr()), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugApplication11064, ::windows::core::IUnknown, IRemoteDebugApplication110);
impl ::core::cmp::PartialEq for IDebugApplication11064 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication11064 {}
impl ::core::fmt::Debug for IDebugApplication11064 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication11064").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplication11064 {
    type Vtable = IDebugApplication11064_Vtbl;
}
impl ::core::clone::Clone for IDebugApplication11064 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplication11064 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2039d958_4eeb_496a_87bb_2e5201eadeef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication11064_Vtbl {
    pub base__: IRemoteDebugApplication110_Vtbl,
    pub SynchronousCallInMainThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
    pub AsynchronousCallInMainThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CallableWaitForHandles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CallableWaitForHandles: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplication32(::windows::core::IUnknown);
impl IDebugApplication32 {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).base__.ResumeFromBreakPoint)(::windows::core::Interface::as_raw(self), prptfocus.into_param().abi(), bra, era).ok()
    }
    pub unsafe fn CauseBreak(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CauseBreak)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IApplicationDebugger>,
    {
        (::windows::core::Interface::vtable(self).base__.ConnectDebugger)(::windows::core::Interface::as_raw(self), pad.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisconnectDebugger)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDebugger(&self) -> ::windows::core::Result<IApplicationDebugger> {
        let mut result__ = ::windows::core::zeroed::<IApplicationDebugger>();
        (::windows::core::Interface::vtable(self).base__.GetDebugger)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstanceAtApplication<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.CreateInstanceAtApplication)(::windows::core::Interface::as_raw(self), rclsid, punkouter.into_param().abi(), dwclscontext, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryAlive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplicationThreads>();
        (::windows::core::Interface::vtable(self).base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).base__.GetRootNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExpressionContexts>();
        (::windows::core::Interface::vtable(self).base__.EnumGlobalExpressionContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, pstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi()).ok()
    }
    pub unsafe fn StepOutComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StepOutComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DebugOutput<P0>(&self, pstr: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DebugOutput)(::windows::core::Interface::as_raw(self), pstr.into_param().abi()).ok()
    }
    pub unsafe fn StartDebugSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartDebugSession)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HandleBreakPoint(&self, br: BREAKREASON) -> ::windows::core::Result<BREAKRESUMEACTION> {
        let mut result__ = ::windows::core::zeroed::<BREAKRESUMEACTION>();
        (::windows::core::Interface::vtable(self).HandleBreakPoint)(::windows::core::Interface::as_raw(self), br, &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBreakFlags)(::windows::core::Interface::as_raw(self), pabf, ::core::mem::transmute(pprdatsteppingthread)).ok()
    }
    pub unsafe fn GetCurrentThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).GetCurrentThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAsyncDebugOperation<P0>(&self, psdo: P0) -> ::windows::core::Result<IDebugAsyncOperation>
    where
        P0: ::windows::core::IntoParam<IDebugSyncOperation>,
    {
        let mut result__ = ::windows::core::zeroed::<IDebugAsyncOperation>();
        (::windows::core::Interface::vtable(self).CreateAsyncDebugOperation)(::windows::core::Interface::as_raw(self), psdo.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddStackFrameSniffer<P0>(&self, pdsfs: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IDebugStackFrameSniffer>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddStackFrameSniffer)(::windows::core::Interface::as_raw(self), pdsfs.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStackFrameSniffer)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn QueryCurrentThreadIsDebuggerThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryCurrentThreadIsDebuggerThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SynchronousCallInDebuggerThread<P0>(&self, pptc: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallInDebuggerThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn CreateApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).CreateApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FireDebuggerEvent<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).FireDebuggerEvent)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandleRuntimeError<P0, P1>(&self, perrordebug: P0, pscriptsite: P1, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
        P1: ::windows::core::IntoParam<IActiveScriptSite>,
    {
        (::windows::core::Interface::vtable(self).HandleRuntimeError)(::windows::core::Interface::as_raw(self), perrordebug.into_param().abi(), pscriptsite.into_param().abi(), pbra, perra, pfcallonscripterror).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FCanJitDebug(&self) -> super::super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).FCanJitDebug)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FIsAutoJitDebugEnabled(&self) -> super::super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).FIsAutoJitDebugEnabled)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AddGlobalExpressionContextProvider<P0>(&self, pdsfs: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IProvideExpressionContexts>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddGlobalExpressionContextProvider)(::windows::core::Interface::as_raw(self), pdsfs.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveGlobalExpressionContextProvider)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplication32, ::windows::core::IUnknown, IRemoteDebugApplication);
impl ::core::cmp::PartialEq for IDebugApplication32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication32 {}
impl ::core::fmt::Debug for IDebugApplication32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplication32 {
    type Vtable = IDebugApplication32_Vtbl;
}
impl ::core::clone::Clone for IDebugApplication32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplication32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c32_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication32_Vtbl {
    pub base__: IRemoteDebugApplication_Vtbl,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub StepOutComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub StartDebugSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HandleBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBreakFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAsyncDebugOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddStackFrameSniffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveStackFrameSniffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub QueryCurrentThreadIsDebuggerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SynchronousCallInDebuggerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::HRESULT,
    pub CreateApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDebuggerEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HandleRuntimeError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandleRuntimeError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FCanJitDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    FCanJitDebug: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FIsAutoJitDebugEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    FIsAutoJitDebugEnabled: usize,
    pub AddGlobalExpressionContextProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveGlobalExpressionContextProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplication64(::windows::core::IUnknown);
impl IDebugApplication64 {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).base__.ResumeFromBreakPoint)(::windows::core::Interface::as_raw(self), prptfocus.into_param().abi(), bra, era).ok()
    }
    pub unsafe fn CauseBreak(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CauseBreak)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IApplicationDebugger>,
    {
        (::windows::core::Interface::vtable(self).base__.ConnectDebugger)(::windows::core::Interface::as_raw(self), pad.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisconnectDebugger)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDebugger(&self) -> ::windows::core::Result<IApplicationDebugger> {
        let mut result__ = ::windows::core::zeroed::<IApplicationDebugger>();
        (::windows::core::Interface::vtable(self).base__.GetDebugger)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstanceAtApplication<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.CreateInstanceAtApplication)(::windows::core::Interface::as_raw(self), rclsid, punkouter.into_param().abi(), dwclscontext, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryAlive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplicationThreads>();
        (::windows::core::Interface::vtable(self).base__.EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).base__.GetRootNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExpressionContexts>();
        (::windows::core::Interface::vtable(self).base__.EnumGlobalExpressionContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, pstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi()).ok()
    }
    pub unsafe fn StepOutComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StepOutComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DebugOutput<P0>(&self, pstr: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DebugOutput)(::windows::core::Interface::as_raw(self), pstr.into_param().abi()).ok()
    }
    pub unsafe fn StartDebugSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartDebugSession)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn HandleBreakPoint(&self, br: BREAKREASON) -> ::windows::core::Result<BREAKRESUMEACTION> {
        let mut result__ = ::windows::core::zeroed::<BREAKRESUMEACTION>();
        (::windows::core::Interface::vtable(self).HandleBreakPoint)(::windows::core::Interface::as_raw(self), br, &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBreakFlags)(::windows::core::Interface::as_raw(self), pabf, ::core::mem::transmute(pprdatsteppingthread)).ok()
    }
    pub unsafe fn GetCurrentThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).GetCurrentThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAsyncDebugOperation<P0>(&self, psdo: P0) -> ::windows::core::Result<IDebugAsyncOperation>
    where
        P0: ::windows::core::IntoParam<IDebugSyncOperation>,
    {
        let mut result__ = ::windows::core::zeroed::<IDebugAsyncOperation>();
        (::windows::core::Interface::vtable(self).CreateAsyncDebugOperation)(::windows::core::Interface::as_raw(self), psdo.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddStackFrameSniffer<P0>(&self, pdsfs: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IDebugStackFrameSniffer>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddStackFrameSniffer)(::windows::core::Interface::as_raw(self), pdsfs.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStackFrameSniffer)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn QueryCurrentThreadIsDebuggerThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryCurrentThreadIsDebuggerThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SynchronousCallInDebuggerThread<P0>(&self, pptc: P0, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall64>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallInDebuggerThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn CreateApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).CreateApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FireDebuggerEvent<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).FireDebuggerEvent)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandleRuntimeError<P0, P1>(&self, perrordebug: P0, pscriptsite: P1, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IActiveScriptErrorDebug>,
        P1: ::windows::core::IntoParam<IActiveScriptSite>,
    {
        (::windows::core::Interface::vtable(self).HandleRuntimeError)(::windows::core::Interface::as_raw(self), perrordebug.into_param().abi(), pscriptsite.into_param().abi(), pbra, perra, pfcallonscripterror).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FCanJitDebug(&self) -> super::super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).FCanJitDebug)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FIsAutoJitDebugEnabled(&self) -> super::super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).FIsAutoJitDebugEnabled)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AddGlobalExpressionContextProvider<P0>(&self, pdsfs: P0) -> ::windows::core::Result<u64>
    where
        P0: ::windows::core::IntoParam<IProvideExpressionContexts>,
    {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).AddGlobalExpressionContextProvider)(::windows::core::Interface::as_raw(self), pdsfs.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveGlobalExpressionContextProvider)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplication64, ::windows::core::IUnknown, IRemoteDebugApplication);
impl ::core::cmp::PartialEq for IDebugApplication64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplication64 {}
impl ::core::fmt::Debug for IDebugApplication64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplication64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplication64 {
    type Vtable = IDebugApplication64_Vtbl;
}
impl ::core::clone::Clone for IDebugApplication64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplication64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dedc754_04c7_4f10_9e60_16a390fe6e62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication64_Vtbl {
    pub base__: IRemoteDebugApplication_Vtbl,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub StepOutComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub StartDebugSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HandleBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBreakFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAsyncDebugOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddStackFrameSniffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveStackFrameSniffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub QueryCurrentThreadIsDebuggerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SynchronousCallInDebuggerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::HRESULT,
    pub CreateApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FireDebuggerEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HandleRuntimeError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandleRuntimeError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FCanJitDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    FCanJitDebug: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FIsAutoJitDebugEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    FIsAutoJitDebugEnabled: usize,
    pub AddGlobalExpressionContextProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u64) -> ::windows::core::HRESULT,
    pub RemoveGlobalExpressionContextProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationNode(::windows::core::IUnknown);
impl IDebugApplicationNode {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<IDebugDocument> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocument>();
        (::windows::core::Interface::vtable(self).base__.GetDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumChildren(&self) -> ::windows::core::Result<IEnumDebugApplicationNodes> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugApplicationNodes>();
        (::windows::core::Interface::vtable(self).EnumChildren)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentProvider<P0>(&self, pddp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentProvider>,
    {
        (::windows::core::Interface::vtable(self).SetDocumentProvider)(::windows::core::Interface::as_raw(self), pddp.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Attach<P0>(&self, pdanparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), pdanparent.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationNode, ::windows::core::IUnknown, IDebugDocumentInfo, IDebugDocumentProvider);
impl ::core::cmp::PartialEq for IDebugApplicationNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNode {}
impl ::core::fmt::Debug for IDebugApplicationNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationNode {
    type Vtable = IDebugApplicationNode_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c34_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNode_Vtbl {
    pub base__: IDebugDocumentProvider_Vtbl,
    pub EnumChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprddp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDocumentProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdanparent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationNode100(::windows::core::IUnknown);
impl IDebugApplicationNode100 {
    pub unsafe fn SetFilterForEventSink(&self, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFilterForEventSink)(::windows::core::Interface::as_raw(self), dwcookie, filter).ok()
    }
    pub unsafe fn GetExcludedDocuments(&self, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows::core::Result<TEXT_DOCUMENT_ARRAY> {
        let mut result__ = ::windows::core::zeroed::<TEXT_DOCUMENT_ARRAY>();
        (::windows::core::Interface::vtable(self).GetExcludedDocuments)(::windows::core::Interface::as_raw(self), filter, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryIsChildNode<P0>(&self, psearchkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocument>,
    {
        (::windows::core::Interface::vtable(self).QueryIsChildNode)(::windows::core::Interface::as_raw(self), psearchkey.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationNode100, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugApplicationNode100 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNode100 {}
impl ::core::fmt::Debug for IDebugApplicationNode100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNode100").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationNode100 {
    type Vtable = IDebugApplicationNode100_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationNode100 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationNode100 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a7734e_841b_4f77_9384_a2891e76e7e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNode100_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFilterForEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows::core::HRESULT,
    pub GetExcludedDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: APPLICATION_NODE_EVENT_FILTER, pdocuments: *mut TEXT_DOCUMENT_ARRAY) -> ::windows::core::HRESULT,
    pub QueryIsChildNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psearchkey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationNodeEvents(::windows::core::IUnknown);
impl IDebugApplicationNodeEvents {
    pub unsafe fn onAddChild<P0>(&self, prddpchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).onAddChild)(::windows::core::Interface::as_raw(self), prddpchild.into_param().abi()).ok()
    }
    pub unsafe fn onRemoveChild<P0>(&self, prddpchild: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).onRemoveChild)(::windows::core::Interface::as_raw(self), prddpchild.into_param().abi()).ok()
    }
    pub unsafe fn onDetach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onDetach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn onAttach<P0>(&self, prddpparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplicationNode>,
    {
        (::windows::core::Interface::vtable(self).onAttach)(::windows::core::Interface::as_raw(self), prddpparent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationNodeEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugApplicationNodeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationNodeEvents {}
impl ::core::fmt::Debug for IDebugApplicationNodeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationNodeEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationNodeEvents {
    type Vtable = IDebugApplicationNodeEvents_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationNodeEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationNodeEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c35_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNodeEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onAddChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onRemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onDetach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onAttach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationThread(::windows::core::IUnknown);
impl IDebugApplicationThread {
    pub unsafe fn GetSystemThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSystemThreadId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IRemoteDebugApplication> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplication>();
        (::windows::core::Interface::vtable(self).base__.GetApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).base__.EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut ::windows::core::BSTR, pbstrstate: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription), ::core::mem::transmute(pbstrstate)).ok()
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugStackFrame>,
        P1: ::windows::core::IntoParam<IDebugCodeContext>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNextStatement)(::windows::core::Interface::as_raw(self), pstackframe.into_param().abi(), pcodecontext.into_param().abi()).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.Suspend)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.Resume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSuspendCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSuspendCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SynchronousCallIntoThread32<P0>(&self, pstcb: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallIntoThread32)(::windows::core::Interface::as_raw(self), pstcb.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn QueryIsCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryIsCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryIsDebuggerThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryIsDebuggerThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), pstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn SetStateString<P0>(&self, pstrstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStateString)(::windows::core::Interface::as_raw(self), pstrstate.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationThread, ::windows::core::IUnknown, IRemoteDebugApplicationThread);
impl ::core::cmp::PartialEq for IDebugApplicationThread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread {}
impl ::core::fmt::Debug for IDebugApplicationThread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationThread {
    type Vtable = IDebugApplicationThread_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationThread {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationThread {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c38_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread_Vtbl {
    pub base__: IRemoteDebugApplicationThread_Vtbl,
    pub SynchronousCallIntoThread32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::HRESULT,
    pub QueryIsCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryIsDebuggerThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetStateString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrstate: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationThread11032(::windows::core::IUnknown);
impl IDebugApplicationThread11032 {
    pub unsafe fn GetActiveThreadRequestCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveThreadRequestCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuspendedForBreakPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsSuspendedForBreakPoint)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadCallable(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsThreadCallable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AsynchronousCallIntoThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).AsynchronousCallIntoThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationThread11032, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugApplicationThread11032 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread11032 {}
impl ::core::fmt::Debug for IDebugApplicationThread11032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread11032").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationThread11032 {
    type Vtable = IDebugApplicationThread11032_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationThread11032 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationThread11032 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2194ac5c_6561_404a_a2e9_f57d72de3702);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread11032_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetActiveThreadRequestCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSuspendedForBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSuspendedForBreakPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadCallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadCallable: usize,
    pub AsynchronousCallIntoThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationThread11064(::windows::core::IUnknown);
impl IDebugApplicationThread11064 {
    pub unsafe fn GetActiveThreadRequestCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveThreadRequestCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuspendedForBreakPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsSuspendedForBreakPoint)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadCallable(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsThreadCallable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AsynchronousCallIntoThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall64>,
    {
        (::windows::core::Interface::vtable(self).AsynchronousCallIntoThread)(::windows::core::Interface::as_raw(self), pptc.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationThread11064, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugApplicationThread11064 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread11064 {}
impl ::core::fmt::Debug for IDebugApplicationThread11064 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread11064").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationThread11064 {
    type Vtable = IDebugApplicationThread11064_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationThread11064 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationThread11064 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x420aa4cc_efd8_4dac_983b_47127826917d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread11064_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetActiveThreadRequestCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSuspendedForBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSuspendedForBreakPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadCallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadCallable: usize,
    pub AsynchronousCallIntoThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationThread64(::windows::core::IUnknown);
impl IDebugApplicationThread64 {
    pub unsafe fn GetSystemThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSystemThreadId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IRemoteDebugApplication> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplication>();
        (::windows::core::Interface::vtable(self).base__.base__.GetApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut ::windows::core::BSTR, pbstrstate: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription), ::core::mem::transmute(pbstrstate)).ok()
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugStackFrame>,
        P1: ::windows::core::IntoParam<IDebugCodeContext>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNextStatement)(::windows::core::Interface::as_raw(self), pstackframe.into_param().abi(), pcodecontext.into_param().abi()).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.Suspend)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.Resume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSuspendCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSuspendCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SynchronousCallIntoThread32<P0>(&self, pstcb: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall32>,
    {
        (::windows::core::Interface::vtable(self).base__.SynchronousCallIntoThread32)(::windows::core::Interface::as_raw(self), pstcb.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
    pub unsafe fn QueryIsCurrentThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryIsCurrentThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryIsDebuggerThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryIsDebuggerThread)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pstrdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), pstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn SetStateString<P0>(&self, pstrstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStateString)(::windows::core::Interface::as_raw(self), pstrstate.into_param().abi()).ok()
    }
    pub unsafe fn SynchronousCallIntoThread64<P0>(&self, pstcb: P0, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugThreadCall64>,
    {
        (::windows::core::Interface::vtable(self).SynchronousCallIntoThread64)(::windows::core::Interface::as_raw(self), pstcb.into_param().abi(), dwparam1, dwparam2, dwparam3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationThread64, ::windows::core::IUnknown, IRemoteDebugApplicationThread, IDebugApplicationThread);
impl ::core::cmp::PartialEq for IDebugApplicationThread64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThread64 {}
impl ::core::fmt::Debug for IDebugApplicationThread64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThread64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationThread64 {
    type Vtable = IDebugApplicationThread64_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationThread64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationThread64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9dac5886_dbad_456d_9dee_5dec39ab3dda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread64_Vtbl {
    pub base__: IDebugApplicationThread_Vtbl,
    pub SynchronousCallIntoThread64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugApplicationThreadEvents110(::windows::core::IUnknown);
impl IDebugApplicationThreadEvents110 {
    pub unsafe fn OnSuspendForBreakPoint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSuspendForBreakPoint)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnResumeFromBreakPoint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnResumeFromBreakPoint)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnThreadRequestComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadRequestComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnBeginThreadRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBeginThreadRequest)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugApplicationThreadEvents110, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugApplicationThreadEvents110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugApplicationThreadEvents110 {}
impl ::core::fmt::Debug for IDebugApplicationThreadEvents110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugApplicationThreadEvents110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugApplicationThreadEvents110 {
    type Vtable = IDebugApplicationThreadEvents110_Vtbl;
}
impl ::core::clone::Clone for IDebugApplicationThreadEvents110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugApplicationThreadEvents110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84e5e468_d5da_48a8_83f4_40366429007b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThreadEvents110_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSuspendForBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnResumeFromBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnThreadRequestComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnBeginThreadRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugAsyncOperation(::windows::core::IUnknown);
impl IDebugAsyncOperation {
    pub unsafe fn GetSyncDebugOperation(&self) -> ::windows::core::Result<IDebugSyncOperation> {
        let mut result__ = ::windows::core::zeroed::<IDebugSyncOperation>();
        (::windows::core::Interface::vtable(self).GetSyncDebugOperation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Start<P0>(&self, padocb: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugAsyncOperationCallBack>,
    {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), padocb.into_param().abi()).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryIsComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryIsComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetResult(&self, phrresult: *mut ::windows::core::HRESULT, ppunkresult: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResult)(::windows::core::Interface::as_raw(self), phrresult, ::core::mem::transmute(ppunkresult)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugAsyncOperation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAsyncOperation {}
impl ::core::fmt::Debug for IDebugAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAsyncOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugAsyncOperation {
    type Vtable = IDebugAsyncOperation_Vtbl;
}
impl ::core::clone::Clone for IDebugAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugAsyncOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1b_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugAsyncOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSyncDebugOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padocb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryIsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugAsyncOperationCallBack(::windows::core::IUnknown);
impl IDebugAsyncOperationCallBack {
    pub unsafe fn onComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugAsyncOperationCallBack, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugAsyncOperationCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugAsyncOperationCallBack {}
impl ::core::fmt::Debug for IDebugAsyncOperationCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugAsyncOperationCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugAsyncOperationCallBack {
    type Vtable = IDebugAsyncOperationCallBack_Vtbl;
}
impl ::core::clone::Clone for IDebugAsyncOperationCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugAsyncOperationCallBack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1c_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugAsyncOperationCallBack_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugCodeContext(::windows::core::IUnknown);
impl IDebugCodeContext {
    pub unsafe fn GetDocumentContext(&self) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).GetDocumentContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBreakPoint(&self, bps: BREAKPOINT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBreakPoint)(::windows::core::Interface::as_raw(self), bps).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugCodeContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugCodeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugCodeContext {}
impl ::core::fmt::Debug for IDebugCodeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugCodeContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugCodeContext {
    type Vtable = IDebugCodeContext_Vtbl;
}
impl ::core::clone::Clone for IDebugCodeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugCodeContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c13_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugCodeContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bps: BREAKPOINT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugCookie(::windows::core::IUnknown);
impl IDebugCookie {
    pub unsafe fn SetDebugCookie(&self, dwdebugappcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDebugCookie)(::windows::core::Interface::as_raw(self), dwdebugappcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugCookie, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugCookie {}
impl ::core::fmt::Debug for IDebugCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugCookie").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugCookie {
    type Vtable = IDebugCookie_Vtbl;
}
impl ::core::clone::Clone for IDebugCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c39_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugCookie_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDebugCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocument(::windows::core::IUnknown);
impl IDebugDocument {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugDocument, ::windows::core::IUnknown, IDebugDocumentInfo);
impl ::core::cmp::PartialEq for IDebugDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocument {}
impl ::core::fmt::Debug for IDebugDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocument {
    type Vtable = IDebugDocument_Vtbl;
}
impl ::core::clone::Clone for IDebugDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c21_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocument_Vtbl {
    pub base__: IDebugDocumentInfo_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentContext(::windows::core::IUnknown);
impl IDebugDocumentContext {
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<IDebugDocument> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocument>();
        (::windows::core::Interface::vtable(self).GetDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumCodeContexts(&self) -> ::windows::core::Result<IEnumDebugCodeContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugCodeContexts>();
        (::windows::core::Interface::vtable(self).EnumCodeContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentContext {}
impl ::core::fmt::Debug for IDebugDocumentContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentContext {
    type Vtable = IDebugDocumentContext_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c28_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumCodeContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentHelper32(::windows::core::IUnknown);
impl IDebugDocumentHelper32 {
    pub unsafe fn Init<P0, P1, P2>(&self, pda: P0, pszshortname: P1, pszlongname: P2, docattr: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplication32>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), pszshortname.into_param().abi(), pszlongname.into_param().abi(), docattr).ok()
    }
    pub unsafe fn Attach<P0>(&self, pddhparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentHelper32>,
    {
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), pddhparent.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddUnicodeText<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddUnicodeText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn AddDBCSText<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDBCSText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn SetDebugDocumentHost<P0>(&self, pddh: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentHost>,
    {
        (::windows::core::Interface::vtable(self).SetDebugDocumentHost)(::windows::core::Interface::as_raw(self), pddh.into_param().abi()).ok()
    }
    pub unsafe fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDeferredText)(::windows::core::Interface::as_raw(self), cchars, dwtextstartcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefineScriptBlock<P0, P1>(&self, ulcharoffset: u32, cchars: u32, pas: P0, fscriptlet: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IActiveScript>,
        P1: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).DefineScriptBlock)(::windows::core::Interface::as_raw(self), ulcharoffset, cchars, pas.into_param().abi(), fscriptlet.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTextAttr(&self, statextattr: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultTextAttr)(::windows::core::Interface::as_raw(self), statextattr).ok()
    }
    pub unsafe fn SetTextAttributes(&self, ulcharoffset: u32, pstatextattr: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTextAttributes)(::windows::core::Interface::as_raw(self), ulcharoffset, pstatextattr.len() as _, ::core::mem::transmute(pstatextattr.as_ptr())).ok()
    }
    pub unsafe fn SetLongName<P0>(&self, pszlongname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLongName)(::windows::core::Interface::as_raw(self), pszlongname.into_param().abi()).ok()
    }
    pub unsafe fn SetShortName<P0>(&self, pszshortname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetShortName)(::windows::core::Interface::as_raw(self), pszshortname.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentAttr(&self, pszattributes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentAttr)(::windows::core::Interface::as_raw(self), pszattributes).ok()
    }
    pub unsafe fn GetDebugApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetDebugApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScriptBlockInfo(&self, dwsourcecontext: u32, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScriptBlockInfo)(::windows::core::Interface::as_raw(self), dwsourcecontext, ::core::mem::transmute(ppasd), picharpos, pcchars).ok()
    }
    pub unsafe fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).CreateDebugDocumentContext)(::windows::core::Interface::as_raw(self), icharpos, cchars, &mut result__).from_abi(result__)
    }
    pub unsafe fn BringDocumentToTop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BringDocumentToTop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentContext>,
    {
        (::windows::core::Interface::vtable(self).BringDocumentContextToTop)(::windows::core::Interface::as_raw(self), pddc.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentHelper32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentHelper32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHelper32 {}
impl ::core::fmt::Debug for IDebugDocumentHelper32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHelper32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentHelper32 {
    type Vtable = IDebugDocumentHelper32_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentHelper32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentHelper32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c26_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHelper32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows::core::PCWSTR, pszlongname: ::windows::core::PCWSTR, docattr: u32) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddUnicodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddDBCSText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub SetDebugDocumentHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddDeferredText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DefineScriptBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DefineScriptBlock: usize,
    pub SetDefaultTextAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows::core::HRESULT,
    pub SetTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows::core::HRESULT,
    pub SetLongName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlongname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszshortname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetDocumentAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows::core::HRESULT,
    pub GetDebugApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScriptBlockInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows::core::HRESULT,
    pub CreateDebugDocumentContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BringDocumentToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentHelper64(::windows::core::IUnknown);
impl IDebugDocumentHelper64 {
    pub unsafe fn Init<P0, P1, P2>(&self, pda: P0, pszshortname: P1, pszlongname: P2, docattr: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugApplication64>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), pszshortname.into_param().abi(), pszlongname.into_param().abi(), docattr).ok()
    }
    pub unsafe fn Attach<P0>(&self, pddhparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentHelper64>,
    {
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), pddhparent.into_param().abi()).ok()
    }
    pub unsafe fn Detach(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddUnicodeText<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddUnicodeText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn AddDBCSText<P0>(&self, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDBCSText)(::windows::core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn SetDebugDocumentHost<P0>(&self, pddh: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentHost>,
    {
        (::windows::core::Interface::vtable(self).SetDebugDocumentHost)(::windows::core::Interface::as_raw(self), pddh.into_param().abi()).ok()
    }
    pub unsafe fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDeferredText)(::windows::core::Interface::as_raw(self), cchars, dwtextstartcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefineScriptBlock<P0, P1>(&self, ulcharoffset: u32, cchars: u32, pas: P0, fscriptlet: P1) -> ::windows::core::Result<u64>
    where
        P0: ::windows::core::IntoParam<IActiveScript>,
        P1: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).DefineScriptBlock)(::windows::core::Interface::as_raw(self), ulcharoffset, cchars, pas.into_param().abi(), fscriptlet.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTextAttr(&self, statextattr: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultTextAttr)(::windows::core::Interface::as_raw(self), statextattr).ok()
    }
    pub unsafe fn SetTextAttributes(&self, ulcharoffset: u32, pstatextattr: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTextAttributes)(::windows::core::Interface::as_raw(self), ulcharoffset, pstatextattr.len() as _, ::core::mem::transmute(pstatextattr.as_ptr())).ok()
    }
    pub unsafe fn SetLongName<P0>(&self, pszlongname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLongName)(::windows::core::Interface::as_raw(self), pszlongname.into_param().abi()).ok()
    }
    pub unsafe fn SetShortName<P0>(&self, pszshortname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetShortName)(::windows::core::Interface::as_raw(self), pszshortname.into_param().abi()).ok()
    }
    pub unsafe fn SetDocumentAttr(&self, pszattributes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDocumentAttr)(::windows::core::Interface::as_raw(self), pszattributes).ok()
    }
    pub unsafe fn GetDebugApplicationNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetDebugApplicationNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScriptBlockInfo(&self, dwsourcecontext: u64, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScriptBlockInfo)(::windows::core::Interface::as_raw(self), dwsourcecontext, ::core::mem::transmute(ppasd), picharpos, pcchars).ok()
    }
    pub unsafe fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).CreateDebugDocumentContext)(::windows::core::Interface::as_raw(self), icharpos, cchars, &mut result__).from_abi(result__)
    }
    pub unsafe fn BringDocumentToTop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BringDocumentToTop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentContext>,
    {
        (::windows::core::Interface::vtable(self).BringDocumentContextToTop)(::windows::core::Interface::as_raw(self), pddc.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentHelper64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentHelper64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHelper64 {}
impl ::core::fmt::Debug for IDebugDocumentHelper64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHelper64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentHelper64 {
    type Vtable = IDebugDocumentHelper64_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentHelper64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentHelper64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c7363c_20fd_47f9_bd82_4855e0150871);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHelper64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows::core::PCWSTR, pszlongname: ::windows::core::PCWSTR, docattr: u32) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddUnicodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddDBCSText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCSTR) -> ::windows::core::HRESULT,
    pub SetDebugDocumentHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddDeferredText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DefineScriptBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DefineScriptBlock: usize,
    pub SetDefaultTextAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows::core::HRESULT,
    pub SetTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows::core::HRESULT,
    pub SetLongName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlongname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszshortname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetDocumentAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows::core::HRESULT,
    pub GetDebugApplicationNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScriptBlockInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows::core::HRESULT,
    pub CreateDebugDocumentContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BringDocumentToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentHost(::windows::core::IUnknown);
impl IDebugDocumentHost {
    pub unsafe fn GetDeferredText(&self, dwtextstartcookie: u32, pchartext: ::windows::core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeferredText)(::windows::core::Interface::as_raw(self), dwtextstartcookie, ::core::mem::transmute(pchartext), pstatextattr, pcnumchars, cmaxchars).ok()
    }
    pub unsafe fn GetScriptTextAttributes<P0, P1>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P1, dwflags: u32, pattr: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetScriptTextAttributes)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), unumcodechars, pstrdelimiter.into_param().abi(), dwflags, pattr).ok()
    }
    pub unsafe fn OnCreateDocumentContext(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).OnCreateDocumentContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathName(&self, pbstrlongname: *mut ::windows::core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrlongname), pfisoriginalfile).ok()
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFileName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyChanged)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentHost, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentHost {}
impl ::core::fmt::Debug for IDebugDocumentHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentHost {
    type Vtable = IDebugDocumentHost_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentHost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c27_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeferredText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtextstartcookie: u32, pchartext: ::windows::core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetScriptTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows::core::HRESULT,
    pub OnCreateDocumentContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkouter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPathName: usize,
    pub GetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentInfo(::windows::core::IUnknown);
impl IDebugDocumentInfo {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentInfo {}
impl ::core::fmt::Debug for IDebugDocumentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentInfo {
    type Vtable = IDebugDocumentInfo_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1f_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dnt: DOCUMENTNAMETYPE, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDocumentClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsiddocument: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentProvider(::windows::core::IUnknown);
impl IDebugDocumentProvider {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<IDebugDocument> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocument>();
        (::windows::core::Interface::vtable(self).GetDocument)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentProvider, ::windows::core::IUnknown, IDebugDocumentInfo);
impl ::core::cmp::PartialEq for IDebugDocumentProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentProvider {}
impl ::core::fmt::Debug for IDebugDocumentProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentProvider {
    type Vtable = IDebugDocumentProvider_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c20_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentProvider_Vtbl {
    pub base__: IDebugDocumentInfo_Vtbl,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppssd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentText(::windows::core::IUnknown);
impl IDebugDocumentText {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentAttributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDocumentAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), pcnumlines, pcnumchars).ok()
    }
    pub unsafe fn GetPositionOfLine(&self, clinenumber: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPositionOfLine)(::windows::core::Interface::as_raw(self), clinenumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLineOfPosition)(::windows::core::Interface::as_raw(self), ccharacterposition, pclinenumber, pccharacteroffsetinline).ok()
    }
    pub unsafe fn GetText(&self, ccharacterposition: u32, pchartext: &mut [u16], pstatextattr: ::core::option::Option<*mut u16>, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), ccharacterposition, ::core::mem::transmute(pchartext.as_ptr()), ::core::mem::transmute(pstatextattr.unwrap_or(::std::ptr::null_mut())), pcnumchars, pchartext.len() as _).ok()
    }
    pub unsafe fn GetPositionOfContext<P0>(&self, psc: P0, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentContext>,
    {
        (::windows::core::Interface::vtable(self).GetPositionOfContext)(::windows::core::Interface::as_raw(self), psc.into_param().abi(), pccharacterposition, cnumchars).ok()
    }
    pub unsafe fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).GetContextOfPosition)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumchars, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentText, ::windows::core::IUnknown, IDebugDocumentInfo, IDebugDocument);
impl ::core::cmp::PartialEq for IDebugDocumentText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentText {}
impl ::core::fmt::Debug for IDebugDocumentText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentText {
    type Vtable = IDebugDocumentText_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c22_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentText_Vtbl {
    pub base__: IDebugDocument_Vtbl,
    pub GetDocumentAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptextdocattr: *mut u32) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows::core::HRESULT,
    pub GetPositionOfLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clinenumber: u32, pccharacterposition: *mut u32) -> ::windows::core::HRESULT,
    pub GetLineOfPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pchartext: ::windows::core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows::core::HRESULT,
    pub GetPositionOfContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psc: *mut ::core::ffi::c_void, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows::core::HRESULT,
    pub GetContextOfPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentTextAuthor(::windows::core::IUnknown);
impl IDebugDocumentTextAuthor {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetName)(::windows::core::Interface::as_raw(self), dnt, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentClassId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDocumentClassId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentAttributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDocumentAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), pcnumlines, pcnumchars).ok()
    }
    pub unsafe fn GetPositionOfLine(&self, clinenumber: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetPositionOfLine)(::windows::core::Interface::as_raw(self), clinenumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLineOfPosition)(::windows::core::Interface::as_raw(self), ccharacterposition, pclinenumber, pccharacteroffsetinline).ok()
    }
    pub unsafe fn GetText(&self, ccharacterposition: u32, pchartext: &mut [u16], pstatextattr: ::core::option::Option<*mut u16>, pcnumchars: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), ccharacterposition, ::core::mem::transmute(pchartext.as_ptr()), ::core::mem::transmute(pstatextattr.unwrap_or(::std::ptr::null_mut())), pcnumchars, pchartext.len() as _).ok()
    }
    pub unsafe fn GetPositionOfContext<P0>(&self, psc: P0, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugDocumentContext>,
    {
        (::windows::core::Interface::vtable(self).base__.GetPositionOfContext)(::windows::core::Interface::as_raw(self), psc.into_param().abi(), pccharacterposition, cnumchars).ok()
    }
    pub unsafe fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> ::windows::core::Result<IDebugDocumentContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentContext>();
        (::windows::core::Interface::vtable(self).base__.GetContextOfPosition)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumchars, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertText(&self, ccharacterposition: u32, pchartext: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertText)(::windows::core::Interface::as_raw(self), ccharacterposition, pchartext.len() as _, ::core::mem::transmute(pchartext.as_ptr())).ok()
    }
    pub unsafe fn RemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveText)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumtoremove).ok()
    }
    pub unsafe fn ReplaceText(&self, ccharacterposition: u32, pchartext: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReplaceText)(::windows::core::Interface::as_raw(self), ccharacterposition, pchartext.len() as _, ::core::mem::transmute(pchartext.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentTextAuthor, ::windows::core::IUnknown, IDebugDocumentInfo, IDebugDocument, IDebugDocumentText);
impl ::core::cmp::PartialEq for IDebugDocumentTextAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextAuthor {}
impl ::core::fmt::Debug for IDebugDocumentTextAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextAuthor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentTextAuthor {
    type Vtable = IDebugDocumentTextAuthor_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentTextAuthor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentTextAuthor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c24_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextAuthor_Vtbl {
    pub base__: IDebugDocumentText_Vtbl,
    pub InsertText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32, pchartext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows::core::HRESULT,
    pub ReplaceText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32, pchartext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentTextEvents(::windows::core::IUnknown);
impl IDebugDocumentTextEvents {
    pub unsafe fn onDestroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onDestroy)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn onInsertText(&self, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onInsertText)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumtoinsert).ok()
    }
    pub unsafe fn onRemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onRemoveText)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumtoremove).ok()
    }
    pub unsafe fn onReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onReplaceText)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumtoreplace).ok()
    }
    pub unsafe fn onUpdateTextAttributes(&self, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onUpdateTextAttributes)(::windows::core::Interface::as_raw(self), ccharacterposition, cnumtoupdate).ok()
    }
    pub unsafe fn onUpdateDocumentAttributes(&self, textdocattr: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onUpdateDocumentAttributes)(::windows::core::Interface::as_raw(self), textdocattr).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentTextEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentTextEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextEvents {}
impl ::core::fmt::Debug for IDebugDocumentTextEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentTextEvents {
    type Vtable = IDebugDocumentTextEvents_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentTextEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentTextEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c23_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub onInsertText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows::core::HRESULT,
    pub onRemoveText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows::core::HRESULT,
    pub onReplaceText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows::core::HRESULT,
    pub onUpdateTextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows::core::HRESULT,
    pub onUpdateDocumentAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textdocattr: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugDocumentTextExternalAuthor(::windows::core::IUnknown);
impl IDebugDocumentTextExternalAuthor {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathName(&self, pbstrlongname: *mut ::windows::core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrlongname), pfisoriginalfile).ok()
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFileName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NotifyChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyChanged)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugDocumentTextExternalAuthor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugDocumentTextExternalAuthor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugDocumentTextExternalAuthor {}
impl ::core::fmt::Debug for IDebugDocumentTextExternalAuthor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugDocumentTextExternalAuthor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugDocumentTextExternalAuthor {
    type Vtable = IDebugDocumentTextExternalAuthor_Vtbl;
}
impl ::core::clone::Clone for IDebugDocumentTextExternalAuthor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugDocumentTextExternalAuthor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c25_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextExternalAuthor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPathName: usize,
    pub GetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugExpression(::windows::core::IUnknown);
impl IDebugExpression {
    pub unsafe fn Start<P0>(&self, pdecb: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugExpressionCallBack>,
    {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), pdecb.into_param().abi()).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryIsComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryIsComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetResultAsString(&self, phrresult: *mut ::windows::core::HRESULT, pbstrresult: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResultAsString)(::windows::core::Interface::as_raw(self), phrresult, ::core::mem::transmute(pbstrresult)).ok()
    }
    pub unsafe fn GetResultAsDebugProperty(&self, phrresult: *mut ::windows::core::HRESULT, ppdp: *mut ::core::option::Option<super::IDebugProperty>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResultAsDebugProperty)(::windows::core::Interface::as_raw(self), phrresult, ::core::mem::transmute(ppdp)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugExpression, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugExpression {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpression {}
impl ::core::fmt::Debug for IDebugExpression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpression").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugExpression {
    type Vtable = IDebugExpression_Vtbl;
}
impl ::core::clone::Clone for IDebugExpression {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugExpression {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c14_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpression_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdecb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryIsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResultAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetResultAsDebugProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, ppdp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugExpressionCallBack(::windows::core::IUnknown);
impl IDebugExpressionCallBack {
    pub unsafe fn onComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).onComplete)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugExpressionCallBack, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugExpressionCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpressionCallBack {}
impl ::core::fmt::Debug for IDebugExpressionCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpressionCallBack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugExpressionCallBack {
    type Vtable = IDebugExpressionCallBack_Vtbl;
}
impl ::core::clone::Clone for IDebugExpressionCallBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugExpressionCallBack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c16_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpressionCallBack_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugExpressionContext(::windows::core::IUnknown);
impl IDebugExpressionContext {
    pub unsafe fn ParseLanguageText<P0, P1>(&self, pstrcode: P0, nradix: u32, pstrdelimiter: P1, dwflags: u32) -> ::windows::core::Result<IDebugExpression>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IDebugExpression>();
        (::windows::core::Interface::vtable(self).ParseLanguageText)(::windows::core::Interface::as_raw(self), pstrcode.into_param().abi(), nradix, pstrdelimiter.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguageInfo(&self, pbstrlanguagename: *mut ::windows::core::BSTR, planguageid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguageInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrlanguagename), planguageid).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugExpressionContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugExpressionContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExpressionContext {}
impl ::core::fmt::Debug for IDebugExpressionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExpressionContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugExpressionContext {
    type Vtable = IDebugExpressionContext_Vtbl;
}
impl ::core::clone::Clone for IDebugExpressionContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugExpressionContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c15_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpressionContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ParseLanguageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrcode: ::windows::core::PCWSTR, nradix: u32, pstrdelimiter: ::windows::core::PCWSTR, dwflags: u32, ppe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLanguageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlanguagename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, planguageid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugFormatter(::windows::core::IUnknown);
impl IDebugFormatter {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetStringForVariant(&self, pvar: *const super::super::super::Com::VARIANT, nradix: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStringForVariant)(::windows::core::Interface::as_raw(self), pvar, nradix, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVariantForString<P0>(&self, pwstrvalue: P0) -> ::windows::core::Result<super::super::super::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetVariantForString)(::windows::core::Interface::as_raw(self), pwstrvalue.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetStringForVarType(&self, vt: super::super::super::Com::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStringForVarType)(::windows::core::Interface::as_raw(self), vt, ptdescarraytype, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugFormatter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugFormatter {}
impl ::core::fmt::Debug for IDebugFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugFormatter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugFormatter {
    type Vtable = IDebugFormatter_Vtbl;
}
impl ::core::clone::Clone for IDebugFormatter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugFormatter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c05_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugFormatter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetStringForVariant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Com::VARIANT, nradix: u32, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetStringForVariant: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetVariantForString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrvalue: ::windows::core::PCWSTR, pvar: *mut super::super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetVariantForString: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetStringForVarType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vt: super::super::super::Com::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetStringForVarType: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugHelper(::windows::core::IUnknown);
impl IDebugHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyBrowser<P0, P1>(&self, pvar: *const super::super::super::Com::VARIANT, bstrname: P0, pdat: P1) -> ::windows::core::Result<super::IDebugProperty>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IDebugApplicationThread>,
    {
        let mut result__ = ::windows::core::zeroed::<super::IDebugProperty>();
        (::windows::core::Interface::vtable(self).CreatePropertyBrowser)(::windows::core::Interface::as_raw(self), pvar, bstrname.into_param().abi(), pdat.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreatePropertyBrowserEx<P0, P1, P2>(&self, pvar: *const super::super::super::Com::VARIANT, bstrname: P0, pdat: P1, pdf: P2) -> ::windows::core::Result<super::IDebugProperty>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IDebugApplicationThread>,
        P2: ::windows::core::IntoParam<IDebugFormatter>,
    {
        let mut result__ = ::windows::core::zeroed::<super::IDebugProperty>();
        (::windows::core::Interface::vtable(self).CreatePropertyBrowserEx)(::windows::core::Interface::as_raw(self), pvar, bstrname.into_param().abi(), pdat.into_param().abi(), pdf.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSimpleConnectionPoint<P0>(&self, pdisp: P0) -> ::windows::core::Result<ISimpleConnectionPoint>
    where
        P0: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISimpleConnectionPoint>();
        (::windows::core::Interface::vtable(self).CreateSimpleConnectionPoint)(::windows::core::Interface::as_raw(self), pdisp.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugHelper {}
impl ::core::fmt::Debug for IDebugHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugHelper {
    type Vtable = IDebugHelper_Vtbl;
}
impl ::core::clone::Clone for IDebugHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c3f_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreatePropertyBrowser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Com::VARIANT, bstrname: ::windows::core::PCWSTR, pdat: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreatePropertyBrowser: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreatePropertyBrowserEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Com::VARIANT, bstrname: ::windows::core::PCWSTR, pdat: *mut ::core::ffi::c_void, pdf: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreatePropertyBrowserEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSimpleConnectionPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, ppscp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSimpleConnectionPoint: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugSessionProvider(::windows::core::IUnknown);
impl IDebugSessionProvider {
    pub unsafe fn StartDebugSession<P0>(&self, pda: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplication>,
    {
        (::windows::core::Interface::vtable(self).StartDebugSession)(::windows::core::Interface::as_raw(self), pda.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugSessionProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugSessionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSessionProvider {}
impl ::core::fmt::Debug for IDebugSessionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSessionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugSessionProvider {
    type Vtable = IDebugSessionProvider_Vtbl;
}
impl ::core::clone::Clone for IDebugSessionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugSessionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c29_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSessionProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartDebugSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugStackFrame(::windows::core::IUnknown);
impl IDebugStackFrame {
    pub unsafe fn GetCodeContext(&self) -> ::windows::core::Result<IDebugCodeContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugCodeContext>();
        (::windows::core::Interface::vtable(self).GetCodeContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescriptionString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescriptionString)(::windows::core::Interface::as_raw(self), flong.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetLanguageString)(::windows::core::Interface::as_raw(self), flong.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).GetThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDebugProperty(&self) -> ::windows::core::Result<super::IDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<super::IDebugProperty>();
        (::windows::core::Interface::vtable(self).GetDebugProperty)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugStackFrame, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugStackFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrame {}
impl ::core::fmt::Debug for IDebugStackFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugStackFrame {
    type Vtable = IDebugStackFrame_Vtbl;
}
impl ::core::clone::Clone for IDebugStackFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugStackFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c17_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrame_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCodeContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescriptionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescriptionString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLanguageString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrlanguage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLanguageString: usize,
    pub GetThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDebugProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugStackFrame110(::windows::core::IUnknown);
impl IDebugStackFrame110 {
    pub unsafe fn GetCodeContext(&self) -> ::windows::core::Result<IDebugCodeContext> {
        let mut result__ = ::windows::core::zeroed::<IDebugCodeContext>();
        (::windows::core::Interface::vtable(self).base__.GetCodeContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescriptionString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescriptionString)(::windows::core::Interface::as_raw(self), flong.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageString<P0>(&self, flong: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetLanguageString)(::windows::core::Interface::as_raw(self), flong.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).base__.GetThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDebugProperty(&self) -> ::windows::core::Result<super::IDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<super::IDebugProperty>();
        (::windows::core::Interface::vtable(self).base__.GetDebugProperty)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStackFrameType(&self) -> ::windows::core::Result<DEBUG_STACKFRAME_TYPE> {
        let mut result__ = ::windows::core::zeroed::<DEBUG_STACKFRAME_TYPE>();
        (::windows::core::Interface::vtable(self).GetStackFrameType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetScriptInvocationContext(&self) -> ::windows::core::Result<IScriptInvocationContext> {
        let mut result__ = ::windows::core::zeroed::<IScriptInvocationContext>();
        (::windows::core::Interface::vtable(self).GetScriptInvocationContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugStackFrame110, ::windows::core::IUnknown, IDebugStackFrame);
impl ::core::cmp::PartialEq for IDebugStackFrame110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrame110 {}
impl ::core::fmt::Debug for IDebugStackFrame110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrame110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugStackFrame110 {
    type Vtable = IDebugStackFrame110_Vtbl;
}
impl ::core::clone::Clone for IDebugStackFrame110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugStackFrame110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b509611_b6ea_4b24_adcb_d0ccfd1a7e33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrame110_Vtbl {
    pub base__: IDebugStackFrame_Vtbl,
    pub GetStackFrameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstackframekind: *mut DEBUG_STACKFRAME_TYPE) -> ::windows::core::HRESULT,
    pub GetScriptInvocationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinvocationcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugStackFrameSniffer(::windows::core::IUnknown);
impl IDebugStackFrameSniffer {
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugStackFrameSniffer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugStackFrameSniffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSniffer {}
impl ::core::fmt::Debug for IDebugStackFrameSniffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSniffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugStackFrameSniffer {
    type Vtable = IDebugStackFrameSniffer_Vtbl;
}
impl ::core::clone::Clone for IDebugStackFrameSniffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugStackFrameSniffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c18_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSniffer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumStackFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugStackFrameSnifferEx32(::windows::core::IUnknown);
impl IDebugStackFrameSnifferEx32 {
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).base__.EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStackFramesEx32(&self, dwspmin: u32) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).EnumStackFramesEx32)(::windows::core::Interface::as_raw(self), dwspmin, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugStackFrameSnifferEx32, ::windows::core::IUnknown, IDebugStackFrameSniffer);
impl ::core::cmp::PartialEq for IDebugStackFrameSnifferEx32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSnifferEx32 {}
impl ::core::fmt::Debug for IDebugStackFrameSnifferEx32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSnifferEx32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugStackFrameSnifferEx32 {
    type Vtable = IDebugStackFrameSnifferEx32_Vtbl;
}
impl ::core::clone::Clone for IDebugStackFrameSnifferEx32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugStackFrameSnifferEx32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c19_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSnifferEx32_Vtbl {
    pub base__: IDebugStackFrameSniffer_Vtbl,
    pub EnumStackFramesEx32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspmin: u32, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugStackFrameSnifferEx64(::windows::core::IUnknown);
impl IDebugStackFrameSnifferEx64 {
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).base__.EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStackFramesEx64(&self, dwspmin: u64) -> ::windows::core::Result<IEnumDebugStackFrames64> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames64>();
        (::windows::core::Interface::vtable(self).EnumStackFramesEx64)(::windows::core::Interface::as_raw(self), dwspmin, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugStackFrameSnifferEx64, ::windows::core::IUnknown, IDebugStackFrameSniffer);
impl ::core::cmp::PartialEq for IDebugStackFrameSnifferEx64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugStackFrameSnifferEx64 {}
impl ::core::fmt::Debug for IDebugStackFrameSnifferEx64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugStackFrameSnifferEx64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugStackFrameSnifferEx64 {
    type Vtable = IDebugStackFrameSnifferEx64_Vtbl;
}
impl ::core::clone::Clone for IDebugStackFrameSnifferEx64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugStackFrameSnifferEx64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cd12af4_49c1_4d52_8d8a_c146f47581aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSnifferEx64_Vtbl {
    pub base__: IDebugStackFrameSniffer_Vtbl,
    pub EnumStackFramesEx64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspmin: u64, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugSyncOperation(::windows::core::IUnknown);
impl IDebugSyncOperation {
    pub unsafe fn GetTargetThread(&self) -> ::windows::core::Result<IDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).GetTargetThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Execute(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).Execute)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InProgressAbort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InProgressAbort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugSyncOperation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugSyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugSyncOperation {}
impl ::core::fmt::Debug for IDebugSyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugSyncOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugSyncOperation {
    type Vtable = IDebugSyncOperation_Vtbl;
}
impl ::core::clone::Clone for IDebugSyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugSyncOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1a_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSyncOperation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTargetThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppattarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InProgressAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugThreadCall32(::windows::core::IUnknown);
impl IDebugThreadCall32 {
    pub unsafe fn ThreadCallHandler(&self, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadCallHandler)(::windows::core::Interface::as_raw(self), dwparam1, dwparam2, dwparam3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugThreadCall32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugThreadCall32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugThreadCall32 {}
impl ::core::fmt::Debug for IDebugThreadCall32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugThreadCall32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugThreadCall32 {
    type Vtable = IDebugThreadCall32_Vtbl;
}
impl ::core::clone::Clone for IDebugThreadCall32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugThreadCall32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c36_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugThreadCall32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ThreadCallHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IDebugThreadCall64(::windows::core::IUnknown);
impl IDebugThreadCall64 {
    pub unsafe fn ThreadCallHandler(&self, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ThreadCallHandler)(::windows::core::Interface::as_raw(self), dwparam1, dwparam2, dwparam3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDebugThreadCall64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugThreadCall64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugThreadCall64 {}
impl ::core::fmt::Debug for IDebugThreadCall64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugThreadCall64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugThreadCall64 {
    type Vtable = IDebugThreadCall64_Vtbl;
}
impl ::core::clone::Clone for IDebugThreadCall64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugThreadCall64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb3fa335_e979_42fd_9fcf_a7546a0f3905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugThreadCall64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ThreadCallHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumDebugApplicationNodes(::windows::core::IUnknown);
impl IEnumDebugApplicationNodes {
    pub unsafe fn Next(&self, celt: u32, pprddp: *mut ::core::option::Option<IDebugApplicationNode>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(pprddp), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugApplicationNodes> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugApplicationNodes>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugApplicationNodes, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugApplicationNodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugApplicationNodes {}
impl ::core::fmt::Debug for IEnumDebugApplicationNodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugApplicationNodes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugApplicationNodes {
    type Vtable = IEnumDebugApplicationNodes_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugApplicationNodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugApplicationNodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c3a_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugApplicationNodes_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pprddp: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumDebugCodeContexts(::windows::core::IUnknown);
impl IEnumDebugCodeContexts {
    pub unsafe fn Next(&self, celt: u32, pscc: *mut ::core::option::Option<IDebugCodeContext>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(pscc), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugCodeContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugCodeContexts>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugCodeContexts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugCodeContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugCodeContexts {}
impl ::core::fmt::Debug for IEnumDebugCodeContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugCodeContexts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugCodeContexts {
    type Vtable = IEnumDebugCodeContexts_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugCodeContexts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugCodeContexts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1d_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugCodeContexts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pscc: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumDebugExpressionContexts(::windows::core::IUnknown);
impl IEnumDebugExpressionContexts {
    pub unsafe fn Next(&self, celt: u32, ppdec: *mut ::core::option::Option<IDebugExpressionContext>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppdec), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExpressionContexts>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugExpressionContexts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugExpressionContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugExpressionContexts {}
impl ::core::fmt::Debug for IEnumDebugExpressionContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugExpressionContexts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugExpressionContexts {
    type Vtable = IEnumDebugExpressionContexts_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugExpressionContexts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugExpressionContexts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c40_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugExpressionContexts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppdec: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumDebugStackFrames(::windows::core::IUnknown);
impl IEnumDebugStackFrames {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, prgdsfd, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugStackFrames, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugStackFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugStackFrames {}
impl ::core::fmt::Debug for IEnumDebugStackFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugStackFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugStackFrames {
    type Vtable = IEnumDebugStackFrames_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugStackFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugStackFrames {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c1e_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugStackFrames_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumDebugStackFrames64(::windows::core::IUnknown);
impl IEnumDebugStackFrames64 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Next)(::windows::core::Interface::as_raw(self), celt, prgdsfd, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next64(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next64)(::windows::core::Interface::as_raw(self), celt, prgdsfd, pceltfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugStackFrames64, ::windows::core::IUnknown, IEnumDebugStackFrames);
impl ::core::cmp::PartialEq for IEnumDebugStackFrames64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugStackFrames64 {}
impl ::core::fmt::Debug for IEnumDebugStackFrames64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugStackFrames64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugStackFrames64 {
    type Vtable = IEnumDebugStackFrames64_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugStackFrames64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugStackFrames64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dc38853_c1b0_4176_a984_b298361027af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugStackFrames64_Vtbl {
    pub base__: IEnumDebugStackFrames_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Next64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next64: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumJsStackFrames(::windows::core::IUnknown);
impl IEnumJsStackFrames {
    pub unsafe fn Next(&self, pframes: &mut [JS_NATIVE_FRAME], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pframes.len() as _, ::core::mem::transmute(pframes.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumJsStackFrames, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumJsStackFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumJsStackFrames {}
impl ::core::fmt::Debug for IEnumJsStackFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumJsStackFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumJsStackFrames {
    type Vtable = IEnumJsStackFrames_Vtbl;
}
impl ::core::clone::Clone for IEnumJsStackFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumJsStackFrames {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e7da34b_fb51_4791_abe7_cb5bdf419755);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumJsStackFrames_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumRemoteDebugApplicationThreads(::windows::core::IUnknown);
impl IEnumRemoteDebugApplicationThreads {
    pub unsafe fn Next(&self, celt: u32, pprdat: *mut ::core::option::Option<IRemoteDebugApplicationThread>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(pprdat), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplicationThreads>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumRemoteDebugApplicationThreads, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumRemoteDebugApplicationThreads {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRemoteDebugApplicationThreads {}
impl ::core::fmt::Debug for IEnumRemoteDebugApplicationThreads {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRemoteDebugApplicationThreads").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumRemoteDebugApplicationThreads {
    type Vtable = IEnumRemoteDebugApplicationThreads_Vtbl;
}
impl ::core::clone::Clone for IEnumRemoteDebugApplicationThreads {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumRemoteDebugApplicationThreads {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c3c_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRemoteDebugApplicationThreads_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pprdat: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IEnumRemoteDebugApplications(::windows::core::IUnknown);
impl IEnumRemoteDebugApplications {
    pub unsafe fn Next(&self, celt: u32, ppda: *mut ::core::option::Option<IRemoteDebugApplication>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppda), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumRemoteDebugApplications> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplications>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumRemoteDebugApplications, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumRemoteDebugApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRemoteDebugApplications {}
impl ::core::fmt::Debug for IEnumRemoteDebugApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRemoteDebugApplications").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumRemoteDebugApplications {
    type Vtable = IEnumRemoteDebugApplications_Vtbl;
}
impl ::core::clone::Clone for IEnumRemoteDebugApplications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumRemoteDebugApplications {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c3b_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRemoteDebugApplications_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppda: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppessd: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebug(::windows::core::IUnknown);
impl IJsDebug {
    pub unsafe fn OpenVirtualProcess<P0>(&self, processid: u32, runtimejsbaseaddress: u64, pdatatarget: P0) -> ::windows::core::Result<IJsDebugProcess>
    where
        P0: ::windows::core::IntoParam<IJsDebugDataTarget>,
    {
        let mut result__ = ::windows::core::zeroed::<IJsDebugProcess>();
        (::windows::core::Interface::vtable(self).OpenVirtualProcess)(::windows::core::Interface::as_raw(self), processid, runtimejsbaseaddress, pdatatarget.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IJsDebug, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebug {}
impl ::core::fmt::Debug for IJsDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebug {
    type Vtable = IJsDebug_Vtbl;
}
impl ::core::clone::Clone for IJsDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe0e89da_2ac5_4c04_ac5e_59956aae3613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebug_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenVirtualProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processid: u32, runtimejsbaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, ppprocess: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugBreakPoint(::windows::core::IUnknown);
impl IJsDebugBreakPoint {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Enable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disable)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDocumentPosition(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentPosition)(::windows::core::Interface::as_raw(self), pdocumentid, pcharacteroffset, pstatementcharcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IJsDebugBreakPoint, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugBreakPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugBreakPoint {}
impl ::core::fmt::Debug for IJsDebugBreakPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugBreakPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugBreakPoint {
    type Vtable = IJsDebugBreakPoint_Vtbl;
}
impl ::core::clone::Clone for IJsDebugBreakPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugBreakPoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf6773e3_ed8d_488b_8a3e_5812577d1542);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugBreakPoint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisenabled: *mut super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocumentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugDataTarget(::windows::core::IUnknown);
impl IJsDebugDataTarget {
    pub unsafe fn ReadMemory(&self, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: &mut [u8], pbytesread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadMemory)(::windows::core::Interface::as_raw(self), address, flags, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, pbytesread).ok()
    }
    pub unsafe fn WriteMemory(&self, address: u64, pmemory: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteMemory)(::windows::core::Interface::as_raw(self), address, ::core::mem::transmute(pmemory.as_ptr()), pmemory.len() as _).ok()
    }
    pub unsafe fn AllocateVirtualMemory(&self, address: u64, size: u32, allocationtype: u32, pageprotection: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).AllocateVirtualMemory)(::windows::core::Interface::as_raw(self), address, size, allocationtype, pageprotection, &mut result__).from_abi(result__)
    }
    pub unsafe fn FreeVirtualMemory(&self, address: u64, size: u32, freetype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeVirtualMemory)(::windows::core::Interface::as_raw(self), address, size, freetype).ok()
    }
    pub unsafe fn GetTlsValue(&self, threadid: u32, tlsindex: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetTlsValue)(::windows::core::Interface::as_raw(self), threadid, tlsindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReadBSTR(&self, address: u64) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReadBSTR)(::windows::core::Interface::as_raw(self), address, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReadNullTerminatedString(&self, address: u64, charactersize: u16, maxcharacters: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ReadNullTerminatedString)(::windows::core::Interface::as_raw(self), address, charactersize, maxcharacters, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateStackFrameEnumerator(&self, threadid: u32) -> ::windows::core::Result<IEnumJsStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumJsStackFrames>();
        (::windows::core::Interface::vtable(self).CreateStackFrameEnumerator)(::windows::core::Interface::as_raw(self), threadid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadContext(&self, threadid: u32, contextflags: u32, pcontext: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadContext)(::windows::core::Interface::as_raw(self), threadid, contextflags, pcontext.len() as _, ::core::mem::transmute(pcontext.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IJsDebugDataTarget, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugDataTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugDataTarget {}
impl ::core::fmt::Debug for IJsDebugDataTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugDataTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugDataTarget {
    type Vtable = IJsDebugDataTarget_Vtbl;
}
impl ::core::clone::Clone for IJsDebugDataTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugDataTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b28977_53a1_48e5_9000_5d0dfa893931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugDataTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReadMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> ::windows::core::HRESULT,
    pub WriteMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, pmemory: *const u8, size: u32) -> ::windows::core::HRESULT,
    pub AllocateVirtualMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, size: u32, allocationtype: u32, pageprotection: u32, pallocatedaddress: *mut u64) -> ::windows::core::HRESULT,
    pub FreeVirtualMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, size: u32, freetype: u32) -> ::windows::core::HRESULT,
    pub GetTlsValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32, tlsindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT,
    pub ReadBSTR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, pstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReadNullTerminatedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u64, charactersize: u16, maxcharacters: u32, pstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CreateStackFrameEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugFrame(::windows::core::IUnknown);
impl IJsDebugFrame {
    pub unsafe fn GetStackRange(&self, pstart: *mut u64, pend: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStackRange)(::windows::core::Interface::as_raw(self), pstart, pend).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentPositionWithId(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentPositionWithId)(::windows::core::Interface::as_raw(self), pdocumentid, pcharacteroffset, pstatementcharcount).ok()
    }
    pub unsafe fn GetDocumentPositionWithName(&self, pdocumentname: *mut ::windows::core::BSTR, pline: *mut u32, pcolumn: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentPositionWithName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdocumentname), pline, pcolumn).ok()
    }
    pub unsafe fn GetDebugProperty(&self) -> ::windows::core::Result<IJsDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<IJsDebugProperty>();
        (::windows::core::Interface::vtable(self).GetDebugProperty)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReturnAddress(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetReturnAddress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Evaluate<P0>(&self, pexpressiontext: P0, ppdebugproperty: *mut ::core::option::Option<IJsDebugProperty>, perror: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Evaluate)(::windows::core::Interface::as_raw(self), pexpressiontext.into_param().abi(), ::core::mem::transmute(ppdebugproperty), ::core::mem::transmute(perror)).ok()
    }
}
::windows::imp::interface_hierarchy!(IJsDebugFrame, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugFrame {}
impl ::core::fmt::Debug for IJsDebugFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugFrame {
    type Vtable = IJsDebugFrame_Vtbl;
}
impl ::core::clone::Clone for IJsDebugFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9196637_ab9d_44b2_bad2_13b95b3f390e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugFrame_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStackRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstart: *mut u64, pend: *mut u64) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetDocumentPositionWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetDocumentPositionWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdocumentname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pline: *mut u32, pcolumn: *mut u32) -> ::windows::core::HRESULT,
    pub GetDebugProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdebugproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReturnAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preturnaddress: *mut u64) -> ::windows::core::HRESULT,
    pub Evaluate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexpressiontext: ::windows::core::PCWSTR, ppdebugproperty: *mut *mut ::core::ffi::c_void, perror: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugProcess(::windows::core::IUnknown);
impl IJsDebugProcess {
    pub unsafe fn CreateStackWalker(&self, threadid: u32) -> ::windows::core::Result<IJsDebugStackWalker> {
        let mut result__ = ::windows::core::zeroed::<IJsDebugStackWalker>();
        (::windows::core::Interface::vtable(self).CreateStackWalker)(::windows::core::Interface::as_raw(self), threadid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBreakPoint<P0>(&self, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: P0) -> ::windows::core::Result<IJsDebugBreakPoint>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IJsDebugBreakPoint>();
        (::windows::core::Interface::vtable(self).CreateBreakPoint)(::windows::core::Interface::as_raw(self), documentid, characteroffset, charactercount, isenabled.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn PerformAsyncBreak(&self, threadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PerformAsyncBreak)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn GetExternalStepAddress(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetExternalStepAddress)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IJsDebugProcess, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugProcess {}
impl ::core::fmt::Debug for IJsDebugProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugProcess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugProcess {
    type Vtable = IJsDebugProcess_Vtbl;
}
impl ::core::clone::Clone for IJsDebugProcess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugProcess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d587168_6a2d_4041_bd3b_0de674502862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugProcess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateStackWalker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32, ppstackwalker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: super::super::super::super::Foundation::BOOL, ppdebugbreakpoint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateBreakPoint: usize,
    pub PerformAsyncBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub GetExternalStepAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcodeaddress: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugProperty(::windows::core::IUnknown);
impl IJsDebugProperty {
    pub unsafe fn GetPropertyInfo(&self, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyInfo)(::windows::core::Interface::as_raw(self), nradix, ppropertyinfo).ok()
    }
    pub unsafe fn GetMembers(&self, members: JS_PROPERTY_MEMBERS) -> ::windows::core::Result<IJsEnumDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<IJsEnumDebugProperty>();
        (::windows::core::Interface::vtable(self).GetMembers)(::windows::core::Interface::as_raw(self), members, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IJsDebugProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugProperty {}
impl ::core::fmt::Debug for IJsDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugProperty {
    type Vtable = IJsDebugProperty_Vtbl;
}
impl ::core::clone::Clone for IJsDebugProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8ffcf2b_3aa4_4320_85c3_52a312ba9633);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows::core::HRESULT,
    pub GetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: JS_PROPERTY_MEMBERS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsDebugStackWalker(::windows::core::IUnknown);
impl IJsDebugStackWalker {
    pub unsafe fn GetNext(&self) -> ::windows::core::Result<IJsDebugFrame> {
        let mut result__ = ::windows::core::zeroed::<IJsDebugFrame>();
        (::windows::core::Interface::vtable(self).GetNext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IJsDebugStackWalker, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsDebugStackWalker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsDebugStackWalker {}
impl ::core::fmt::Debug for IJsDebugStackWalker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsDebugStackWalker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsDebugStackWalker {
    type Vtable = IJsDebugStackWalker_Vtbl;
}
impl ::core::clone::Clone for IJsDebugStackWalker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsDebugStackWalker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb24b094_73c4_456c_a4ec_e90ea00bdfe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugStackWalker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IJsEnumDebugProperty(::windows::core::IUnknown);
impl IJsEnumDebugProperty {
    pub unsafe fn Next(&self, ppdebugproperty: &mut [::core::option::Option<IJsDebugProperty>], pactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppdebugproperty.len() as _, ::core::mem::transmute(ppdebugproperty.as_ptr()), pactualcount).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IJsEnumDebugProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IJsEnumDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsEnumDebugProperty {}
impl ::core::fmt::Debug for IJsEnumDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsEnumDebugProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJsEnumDebugProperty {
    type Vtable = IJsEnumDebugProperty_Vtbl;
}
impl ::core::clone::Clone for IJsEnumDebugProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IJsEnumDebugProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4092432f_2f0f_4fe1_b638_5b74a52cdcbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsEnumDebugProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppdebugproperty: *mut *mut ::core::ffi::c_void, pactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IMachineDebugManager(::windows::core::IUnknown);
impl IMachineDebugManager {
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplication>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveApplication)(::windows::core::Interface::as_raw(self), dwappcookie).ok()
    }
    pub unsafe fn EnumApplications(&self) -> ::windows::core::Result<IEnumRemoteDebugApplications> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplications>();
        (::windows::core::Interface::vtable(self).EnumApplications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMachineDebugManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMachineDebugManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManager {}
impl ::core::fmt::Debug for IMachineDebugManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMachineDebugManager {
    type Vtable = IMachineDebugManager_Vtbl;
}
impl ::core::clone::Clone for IMachineDebugManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMachineDebugManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2c_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows::core::HRESULT,
    pub EnumApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IMachineDebugManagerCookie(::windows::core::IUnknown);
impl IMachineDebugManagerCookie {
    pub unsafe fn AddApplication<P0>(&self, pda: P0, dwdebugappcookie: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplication>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), dwdebugappcookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveApplication(&self, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveApplication)(::windows::core::Interface::as_raw(self), dwdebugappcookie, dwappcookie).ok()
    }
    pub unsafe fn EnumApplications(&self) -> ::windows::core::Result<IEnumRemoteDebugApplications> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplications>();
        (::windows::core::Interface::vtable(self).EnumApplications)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IMachineDebugManagerCookie, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMachineDebugManagerCookie {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManagerCookie {}
impl ::core::fmt::Debug for IMachineDebugManagerCookie {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManagerCookie").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMachineDebugManagerCookie {
    type Vtable = IMachineDebugManagerCookie_Vtbl;
}
impl ::core::clone::Clone for IMachineDebugManagerCookie {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMachineDebugManagerCookie {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2d_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManagerCookie_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwdebugappcookie: u32, pdwappcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows::core::HRESULT,
    pub EnumApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IMachineDebugManagerEvents(::windows::core::IUnknown);
impl IMachineDebugManagerEvents {
    pub unsafe fn onAddApplication<P0>(&self, pda: P0, dwappcookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplication>,
    {
        (::windows::core::Interface::vtable(self).onAddApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), dwappcookie).ok()
    }
    pub unsafe fn onRemoveApplication<P0>(&self, pda: P0, dwappcookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplication>,
    {
        (::windows::core::Interface::vtable(self).onRemoveApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), dwappcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IMachineDebugManagerEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMachineDebugManagerEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineDebugManagerEvents {}
impl ::core::fmt::Debug for IMachineDebugManagerEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineDebugManagerEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMachineDebugManagerEvents {
    type Vtable = IMachineDebugManagerEvents_Vtbl;
}
impl ::core::clone::Clone for IMachineDebugManagerEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMachineDebugManagerEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2e_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManagerEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub onAddApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows::core::HRESULT,
    pub onRemoveApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IProcessDebugManager32(::windows::core::IUnknown);
impl IProcessDebugManager32 {
    pub unsafe fn CreateApplication(&self) -> ::windows::core::Result<IDebugApplication32> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication32>();
        (::windows::core::Interface::vtable(self).CreateApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultApplication(&self) -> ::windows::core::Result<IDebugApplication32> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication32>();
        (::windows::core::Interface::vtable(self).GetDefaultApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IDebugApplication32>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveApplication)(::windows::core::Interface::as_raw(self), dwappcookie).ok()
    }
    pub unsafe fn CreateDebugDocumentHelper<P0>(&self, punkouter: P0) -> ::windows::core::Result<IDebugDocumentHelper32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentHelper32>();
        (::windows::core::Interface::vtable(self).CreateDebugDocumentHelper)(::windows::core::Interface::as_raw(self), punkouter.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IProcessDebugManager32, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProcessDebugManager32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessDebugManager32 {}
impl ::core::fmt::Debug for IProcessDebugManager32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessDebugManager32").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessDebugManager32 {
    type Vtable = IProcessDebugManager32_Vtbl;
}
impl ::core::clone::Clone for IProcessDebugManager32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProcessDebugManager32 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c2f_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDebugManager32_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows::core::HRESULT,
    pub CreateDebugDocumentHelper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IProcessDebugManager64(::windows::core::IUnknown);
impl IProcessDebugManager64 {
    pub unsafe fn CreateApplication(&self) -> ::windows::core::Result<IDebugApplication64> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication64>();
        (::windows::core::Interface::vtable(self).CreateApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultApplication(&self) -> ::windows::core::Result<IDebugApplication64> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplication64>();
        (::windows::core::Interface::vtable(self).GetDefaultApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IDebugApplication64>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddApplication)(::windows::core::Interface::as_raw(self), pda.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveApplication)(::windows::core::Interface::as_raw(self), dwappcookie).ok()
    }
    pub unsafe fn CreateDebugDocumentHelper<P0>(&self, punkouter: P0) -> ::windows::core::Result<IDebugDocumentHelper64>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDebugDocumentHelper64>();
        (::windows::core::Interface::vtable(self).CreateDebugDocumentHelper)(::windows::core::Interface::as_raw(self), punkouter.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IProcessDebugManager64, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProcessDebugManager64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessDebugManager64 {}
impl ::core::fmt::Debug for IProcessDebugManager64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessDebugManager64").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessDebugManager64 {
    type Vtable = IProcessDebugManager64_Vtbl;
}
impl ::core::clone::Clone for IProcessDebugManager64 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProcessDebugManager64 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b9fc1c_63a9_4cc1_ac21_087d69a17fab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDebugManager64_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefaultApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows::core::HRESULT,
    pub CreateDebugDocumentHelper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IProvideExpressionContexts(::windows::core::IUnknown);
impl IProvideExpressionContexts {
    pub unsafe fn EnumExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExpressionContexts>();
        (::windows::core::Interface::vtable(self).EnumExpressionContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IProvideExpressionContexts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IProvideExpressionContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideExpressionContexts {}
impl ::core::fmt::Debug for IProvideExpressionContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideExpressionContexts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProvideExpressionContexts {
    type Vtable = IProvideExpressionContexts_Vtbl;
}
impl ::core::clone::Clone for IProvideExpressionContexts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProvideExpressionContexts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c41_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideExpressionContexts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumExpressionContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugApplication(::windows::core::IUnknown);
impl IRemoteDebugApplication {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).ResumeFromBreakPoint)(::windows::core::Interface::as_raw(self), prptfocus.into_param().abi(), bra, era).ok()
    }
    pub unsafe fn CauseBreak(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CauseBreak)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IApplicationDebugger>,
    {
        (::windows::core::Interface::vtable(self).ConnectDebugger)(::windows::core::Interface::as_raw(self), pad.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectDebugger)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDebugger(&self) -> ::windows::core::Result<IApplicationDebugger> {
        let mut result__ = ::windows::core::zeroed::<IApplicationDebugger>();
        (::windows::core::Interface::vtable(self).GetDebugger)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstanceAtApplication<P0>(&self, rclsid: *const ::windows::core::GUID, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateInstanceAtApplication)(::windows::core::Interface::as_raw(self), rclsid, punkouter.into_param().abi(), dwclscontext, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAlive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryAlive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumThreads(&self) -> ::windows::core::Result<IEnumRemoteDebugApplicationThreads> {
        let mut result__ = ::windows::core::zeroed::<IEnumRemoteDebugApplicationThreads>();
        (::windows::core::Interface::vtable(self).EnumThreads)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRootNode(&self) -> ::windows::core::Result<IDebugApplicationNode> {
        let mut result__ = ::windows::core::zeroed::<IDebugApplicationNode>();
        (::windows::core::Interface::vtable(self).GetRootNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> ::windows::core::Result<IEnumDebugExpressionContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExpressionContexts>();
        (::windows::core::Interface::vtable(self).EnumGlobalExpressionContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugApplication, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplication {}
impl ::core::fmt::Debug for IRemoteDebugApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugApplication {
    type Vtable = IRemoteDebugApplication_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c30_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplication_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ResumeFromBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prptfocus: *mut ::core::ffi::c_void, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows::core::HRESULT,
    pub CauseBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pad: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceAtApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAlive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumThreads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetRootNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumGlobalExpressionContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugApplication110(::windows::core::IUnknown);
impl IRemoteDebugApplication110 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDebuggerOptions)(::windows::core::Interface::as_raw(self), mask, value).ok()
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> ::windows::core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        let mut result__ = ::windows::core::zeroed::<SCRIPT_DEBUGGER_OPTIONS>();
        (::windows::core::Interface::vtable(self).GetCurrentDebuggerOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMainThread(&self) -> ::windows::core::Result<IRemoteDebugApplicationThread> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplicationThread>();
        (::windows::core::Interface::vtable(self).GetMainThread)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugApplication110, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugApplication110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplication110 {}
impl ::core::fmt::Debug for IRemoteDebugApplication110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplication110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugApplication110 {
    type Vtable = IRemoteDebugApplication110_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugApplication110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugApplication110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5fe005b_2836_485e_b1f9_89d91aa24fd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplication110_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDebuggerOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::HRESULT,
    pub GetCurrentDebuggerOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcurrentoptions: *mut SCRIPT_DEBUGGER_OPTIONS) -> ::windows::core::HRESULT,
    pub GetMainThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppthread: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugApplicationEvents(::windows::core::IUnknown);
impl IRemoteDebugApplicationEvents {
    pub unsafe fn OnConnectDebugger<P0>(&self, pad: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IApplicationDebugger>,
    {
        (::windows::core::Interface::vtable(self).OnConnectDebugger)(::windows::core::Interface::as_raw(self), pad.into_param().abi()).ok()
    }
    pub unsafe fn OnDisconnectDebugger(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisconnectDebugger)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnSetName<P0>(&self, pstrname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnSetName)(::windows::core::Interface::as_raw(self), pstrname.into_param().abi()).ok()
    }
    pub unsafe fn OnDebugOutput<P0>(&self, pstr: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnDebugOutput)(::windows::core::Interface::as_raw(self), pstr.into_param().abi()).ok()
    }
    pub unsafe fn OnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClose)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEnterBreakPoint<P0>(&self, prdat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).OnEnterBreakPoint)(::windows::core::Interface::as_raw(self), prdat.into_param().abi()).ok()
    }
    pub unsafe fn OnLeaveBreakPoint<P0>(&self, prdat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).OnLeaveBreakPoint)(::windows::core::Interface::as_raw(self), prdat.into_param().abi()).ok()
    }
    pub unsafe fn OnCreateThread<P0>(&self, prdat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).OnCreateThread)(::windows::core::Interface::as_raw(self), prdat.into_param().abi()).ok()
    }
    pub unsafe fn OnDestroyThread<P0>(&self, prdat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).OnDestroyThread)(::windows::core::Interface::as_raw(self), prdat.into_param().abi()).ok()
    }
    pub unsafe fn OnBreakFlagChange<P0>(&self, abf: u32, prdatsteppingthread: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IRemoteDebugApplicationThread>,
    {
        (::windows::core::Interface::vtable(self).OnBreakFlagChange)(::windows::core::Interface::as_raw(self), abf, prdatsteppingthread.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugApplicationEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugApplicationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplicationEvents {}
impl ::core::fmt::Debug for IRemoteDebugApplicationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplicationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugApplicationEvents {
    type Vtable = IRemoteDebugApplicationEvents_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugApplicationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugApplicationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c33_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplicationEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnectDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnDisconnectDebugger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEnterBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnLeaveBreakPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnCreateThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnDestroyThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnBreakFlagChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, abf: u32, prdatsteppingthread: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugApplicationThread(::windows::core::IUnknown);
impl IRemoteDebugApplicationThread {
    pub unsafe fn GetSystemThreadId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSystemThreadId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<IRemoteDebugApplication> {
        let mut result__ = ::windows::core::zeroed::<IRemoteDebugApplication>();
        (::windows::core::Interface::vtable(self).GetApplication)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumStackFrames(&self) -> ::windows::core::Result<IEnumDebugStackFrames> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugStackFrames>();
        (::windows::core::Interface::vtable(self).EnumStackFrames)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut ::windows::core::BSTR, pbstrstate: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdescription), ::core::mem::transmute(pbstrstate)).ok()
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDebugStackFrame>,
        P1: ::windows::core::IntoParam<IDebugCodeContext>,
    {
        (::windows::core::Interface::vtable(self).SetNextStatement)(::windows::core::Interface::as_raw(self), pstackframe.into_param().abi(), pcodecontext.into_param().abi()).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Suspend)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSuspendCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSuspendCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugApplicationThread, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugApplicationThread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugApplicationThread {}
impl ::core::fmt::Debug for IRemoteDebugApplicationThread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugApplicationThread").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugApplicationThread {
    type Vtable = IRemoteDebugApplicationThread_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugApplicationThread {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugApplicationThread {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c37_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplicationThread_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSystemThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: *mut u32) -> ::windows::core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprda: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumStackFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrstate: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetNextStatement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstackframe: *mut ::core::ffi::c_void, pcodecontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut u32) -> ::windows::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSuspendCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugCriticalErrorEvent110(::windows::core::IUnknown);
impl IRemoteDebugCriticalErrorEvent110 {
    pub unsafe fn GetErrorInfo(&self, pbstrsource: *mut ::windows::core::BSTR, pmessageid: *mut i32, pbstrmessage: *mut ::windows::core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetErrorInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrsource), pmessageid, ::core::mem::transmute(pbstrmessage), ::core::mem::transmute(pplocation)).ok()
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugCriticalErrorEvent110, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugCriticalErrorEvent110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugCriticalErrorEvent110 {}
impl ::core::fmt::Debug for IRemoteDebugCriticalErrorEvent110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugCriticalErrorEvent110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugCriticalErrorEvent110 {
    type Vtable = IRemoteDebugCriticalErrorEvent110_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugCriticalErrorEvent110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugCriticalErrorEvent110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f69c611_6b14_47e8_9260_4bb7c52f504b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugCriticalErrorEvent110_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pmessageid: *mut i32, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IRemoteDebugInfoEvent110(::windows::core::IUnknown);
impl IRemoteDebugInfoEvent110 {
    pub unsafe fn GetEventInfo(&self, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::windows::core::BSTR, pbstrurl: *mut ::windows::core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventInfo)(::windows::core::Interface::as_raw(self), pmessagetype, ::core::mem::transmute(pbstrmessage), ::core::mem::transmute(pbstrurl), ::core::mem::transmute(pplocation)).ok()
    }
}
::windows::imp::interface_hierarchy!(IRemoteDebugInfoEvent110, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRemoteDebugInfoEvent110 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteDebugInfoEvent110 {}
impl ::core::fmt::Debug for IRemoteDebugInfoEvent110 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDebugInfoEvent110").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRemoteDebugInfoEvent110 {
    type Vtable = IRemoteDebugInfoEvent110_Vtbl;
}
impl ::core::clone::Clone for IRemoteDebugInfoEvent110 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoteDebugInfoEvent110 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff56bb6_eb89_4c0f_8823_cc2a4c0b7f26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugInfoEvent110_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetEventInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrurl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IScriptEntry(::windows::core::IUnknown);
impl IScriptEntry {
    pub unsafe fn Alive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Alive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndexInParent(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetIndexInParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumberOfChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetNumberOfChildren)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChild(&self, isn: u32) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).base__.GetChild)(::windows::core::Interface::as_raw(self), isn, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateChildEntry<P0>(&self, isn: u32, dwcookie: u32, pszdelimiter: P0) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).base__.CreateChildEntry)(::windows::core::Interface::as_raw(self), isn, dwcookie, pszdelimiter.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P1, P2, P3>(&self, pszdefaultname: P0, prgpsznames: &[::windows::core::PCWSTR], pszevent: P1, pszdelimiter: P2, ptisignature: P3, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::super::Com::ITypeInfo>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).base__.CreateChildHandler)(::windows::core::Interface::as_raw(self), pszdefaultname.into_param().abi(), ::core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len() as _, pszevent.into_param().abi(), pszdelimiter.into_param().abi(), ptisignature.into_param().abi(), imethodsignature, isn, dwcookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetBody(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetBody)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBody<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetItemName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetItemName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetItemName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignature(&self, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppti), pimethod).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignature<P0>(&self, pti: P0, imethod: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Com::ITypeInfo>,
    {
        (::windows::core::Interface::vtable(self).SetSignature)(::windows::core::Interface::as_raw(self), pti.into_param().abi(), imethod).ok()
    }
    pub unsafe fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRange)(::windows::core::Interface::as_raw(self), pichmin, pcch).ok()
    }
}
::windows::imp::interface_hierarchy!(IScriptEntry, ::windows::core::IUnknown, IScriptNode);
impl ::core::cmp::PartialEq for IScriptEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptEntry {}
impl ::core::fmt::Debug for IScriptEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScriptEntry {
    type Vtable = IScriptEntry_Vtbl;
}
impl ::core::clone::Clone for IScriptEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScriptEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aee2a95_bcbb_11d0_8c72_00c04fc2b085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptEntry_Vtbl {
    pub base__: IScriptNode_Vtbl,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void, pimethod: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pti: *mut ::core::ffi::c_void, imethod: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignature: usize,
    pub GetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pichmin: *mut u32, pcch: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IScriptInvocationContext(::windows::core::IUnknown);
impl IScriptInvocationContext {
    pub unsafe fn GetContextType(&self) -> ::windows::core::Result<SCRIPT_INVOCATION_CONTEXT_TYPE> {
        let mut result__ = ::windows::core::zeroed::<SCRIPT_INVOCATION_CONTEXT_TYPE>();
        (::windows::core::Interface::vtable(self).GetContextType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContextDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetContextDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContextObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetContextObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IScriptInvocationContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IScriptInvocationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptInvocationContext {}
impl ::core::fmt::Debug for IScriptInvocationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptInvocationContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScriptInvocationContext {
    type Vtable = IScriptInvocationContext_Vtbl;
}
impl ::core::clone::Clone for IScriptInvocationContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScriptInvocationContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d7741b7_af7e_4a2a_85e5_c77f4d0659fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptInvocationContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetContextType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinvocationcontexttype: *mut SCRIPT_INVOCATION_CONTEXT_TYPE) -> ::windows::core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetContextObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontextobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IScriptNode(::windows::core::IUnknown);
impl IScriptNode {
    pub unsafe fn Alive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Alive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndexInParent(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetIndexInParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumberOfChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNumberOfChildren)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChild(&self, isn: u32) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).GetChild)(::windows::core::Interface::as_raw(self), isn, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateChildEntry<P0>(&self, isn: u32, dwcookie: u32, pszdelimiter: P0) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).CreateChildEntry)(::windows::core::Interface::as_raw(self), isn, dwcookie, pszdelimiter.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P1, P2, P3>(&self, pszdefaultname: P0, prgpsznames: &[::windows::core::PCWSTR], pszevent: P1, pszdelimiter: P2, ptisignature: P3, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::super::Com::ITypeInfo>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).CreateChildHandler)(::windows::core::Interface::as_raw(self), pszdefaultname.into_param().abi(), ::core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len() as _, pszevent.into_param().abi(), pszdelimiter.into_param().abi(), ptisignature.into_param().abi(), imethodsignature, isn, dwcookie, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IScriptNode, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IScriptNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptNode {}
impl ::core::fmt::Debug for IScriptNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScriptNode {
    type Vtable = IScriptNode_Vtbl;
}
impl ::core::clone::Clone for IScriptNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScriptNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aee2a94_bcbb_11d0_8c72_00c04fc2b085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptNode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Alive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsnparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIndexInParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisn: *mut u32) -> ::windows::core::HRESULT,
    pub GetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub GetNumberOfChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsn: *mut u32) -> ::windows::core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isn: u32, ppsn: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CreateChildEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isn: u32, dwcookie: u32, pszdelimiter: ::windows::core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateChildHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows::core::PCWSTR, prgpsznames: *const ::windows::core::PCWSTR, cpsznames: u32, pszevent: ::windows::core::PCWSTR, pszdelimiter: ::windows::core::PCWSTR, ptisignature: *mut ::core::ffi::c_void, imethodsignature: u32, isn: u32, dwcookie: u32, ppse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateChildHandler: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IScriptScriptlet(::windows::core::IUnknown);
impl IScriptScriptlet {
    pub unsafe fn Alive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Alive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).base__.base__.GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndexInParent(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetIndexInParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCookie(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCookie)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumberOfChildren(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetNumberOfChildren)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChild(&self, isn: u32) -> ::windows::core::Result<IScriptNode> {
        let mut result__ = ::windows::core::zeroed::<IScriptNode>();
        (::windows::core::Interface::vtable(self).base__.base__.GetChild)(::windows::core::Interface::as_raw(self), isn, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateChildEntry<P0>(&self, isn: u32, dwcookie: u32, pszdelimiter: P0) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateChildEntry)(::windows::core::Interface::as_raw(self), isn, dwcookie, pszdelimiter.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P1, P2, P3>(&self, pszdefaultname: P0, prgpsznames: &[::windows::core::PCWSTR], pszevent: P1, pszdelimiter: P2, ptisignature: P3, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows::core::Result<IScriptEntry>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::super::Com::ITypeInfo>,
    {
        let mut result__ = ::windows::core::zeroed::<IScriptEntry>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateChildHandler)(::windows::core::Interface::as_raw(self), pszdefaultname.into_param().abi(), ::core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len() as _, pszevent.into_param().abi(), pszdelimiter.into_param().abi(), ptisignature.into_param().abi(), imethodsignature, isn, dwcookie, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetText)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetBody(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetBody)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBody<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBody)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetItemName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetItemName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetItemName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignature(&self, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSignature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppti), pimethod).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignature<P0>(&self, pti: P0, imethod: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Com::ITypeInfo>,
    {
        (::windows::core::Interface::vtable(self).base__.SetSignature)(::windows::core::Interface::as_raw(self), pti.into_param().abi(), imethod).ok()
    }
    pub unsafe fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRange)(::windows::core::Interface::as_raw(self), pichmin, pcch).ok()
    }
    pub unsafe fn GetSubItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSubItemName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubItemName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSubItemName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetEventName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetEventName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetEventName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
    pub unsafe fn GetSimpleEventName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSimpleEventName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSimpleEventName<P0>(&self, psz: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSimpleEventName)(::windows::core::Interface::as_raw(self), psz.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IScriptScriptlet, ::windows::core::IUnknown, IScriptNode, IScriptEntry);
impl ::core::cmp::PartialEq for IScriptScriptlet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScriptScriptlet {}
impl ::core::fmt::Debug for IScriptScriptlet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScriptScriptlet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScriptScriptlet {
    type Vtable = IScriptScriptlet_Vtbl;
}
impl ::core::clone::Clone for IScriptScriptlet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IScriptScriptlet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aee2a96_bcbb_11d0_8c72_00c04fc2b085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptScriptlet_Vtbl {
    pub base__: IScriptEntry_Vtbl,
    pub GetSubItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSubItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetEventName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetEventName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSimpleEventName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetSimpleEventName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psz: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct ISimpleConnectionPoint(::windows::core::IUnknown);
impl ISimpleConnectionPoint {
    pub unsafe fn GetEventCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetEventCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DescribeEvents(&self, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::windows::core::BSTR, pceventsfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DescribeEvents)(::windows::core::Interface::as_raw(self), ievent, cevents, prgid, ::core::mem::transmute(prgbstr), pceventsfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, pdisp: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), pdisp.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(ISimpleConnectionPoint, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISimpleConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleConnectionPoint {}
impl ::core::fmt::Debug for ISimpleConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleConnectionPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimpleConnectionPoint {
    type Vtable = ISimpleConnectionPoint_Vtbl;
}
impl ::core::clone::Clone for ISimpleConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISimpleConnectionPoint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c3e_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleConnectionPoint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetEventCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub DescribeEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pceventsfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct ITridentEventSink(::windows::core::IUnknown);
impl ITridentEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FireEvent<P0>(&self, pstrevent: P0, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Com::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).FireEvent)(::windows::core::Interface::as_raw(self), pstrevent.into_param().abi(), pdp, pvarres, pei).ok()
    }
}
::windows::imp::interface_hierarchy!(ITridentEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITridentEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITridentEventSink {}
impl ::core::fmt::Debug for ITridentEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITridentEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITridentEventSink {
    type Vtable = ITridentEventSink_Vtbl;
}
impl ::core::clone::Clone for ITridentEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITridentEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dc9ca50_06ef_11d2_8415_006008c3fbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FireEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrevent: ::windows::core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Com::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FireEvent: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IWebAppDiagnosticsObjectInitialization(::windows::core::IUnknown);
impl IWebAppDiagnosticsObjectInitialization {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0, P1>(&self, hpassedhandle: P0, pdebugapplication: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::super::Foundation::HANDLE_PTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), hpassedhandle.into_param().abi(), pdebugapplication.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWebAppDiagnosticsObjectInitialization, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWebAppDiagnosticsObjectInitialization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAppDiagnosticsObjectInitialization {}
impl ::core::fmt::Debug for IWebAppDiagnosticsObjectInitialization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAppDiagnosticsObjectInitialization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebAppDiagnosticsObjectInitialization {
    type Vtable = IWebAppDiagnosticsObjectInitialization_Vtbl;
}
impl ::core::clone::Clone for IWebAppDiagnosticsObjectInitialization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAppDiagnosticsObjectInitialization {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ff3a42_a5f5_432b_b625_8e8e16f57e15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAppDiagnosticsObjectInitialization_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
pub struct IWebAppDiagnosticsSetup(::windows::core::IUnknown);
impl IWebAppDiagnosticsSetup {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiagnosticsSupported(&self) -> ::windows::core::Result<super::super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).DiagnosticsSupported)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateObjectWithSiteAtWebApp(&self, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, hpasstoobject: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateObjectWithSiteAtWebApp)(::windows::core::Interface::as_raw(self), rclsid, dwclscontext, riid, hpasstoobject).ok()
    }
}
::windows::imp::interface_hierarchy!(IWebAppDiagnosticsSetup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWebAppDiagnosticsSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAppDiagnosticsSetup {}
impl ::core::fmt::Debug for IWebAppDiagnosticsSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAppDiagnosticsSetup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebAppDiagnosticsSetup {
    type Vtable = IWebAppDiagnosticsSetup_Vtbl;
}
impl ::core::clone::Clone for IWebAppDiagnosticsSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAppDiagnosticsSetup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x379bfbe1_c6c9_432a_93e1_6d17656c538c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAppDiagnosticsSetup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DiagnosticsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DiagnosticsSupported: usize,
    pub CreateObjectWithSiteAtWebApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, hpasstoobject: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ACTIVPROF_E_PROFILER_ABSENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ACTIVPROF_E_PROFILER_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ACTIVPROF_E_UNABLE_TO_APPLY_ACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_DEBUGGER_BLOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_DEBUGGER_HALT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_IN_BREAKPOINT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_NESTED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_STEP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_STEPTYPE_BYTECODE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_STEPTYPE_MACHINE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_STEPTYPE_MASK: u32 = 15728640u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const APPBREAKFLAG_STEPTYPE_SOURCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const CATID_ActiveScript: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0b7a1a1_9847_11cf_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const CATID_ActiveScriptAuthor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aee2a92_bcbb_11d0_8c72_00c04fc2b085);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const CATID_ActiveScriptEncode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0b7a1a3_9847_11cf_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const CATID_ActiveScriptParse: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0b7a1a2_9847_11cf_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const CDebugDocumentHelper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83b8bca6_687c_11d0_a405_00aa0060275c);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_ALLOWBREAKPOINTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_ALLOWERRORREPORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_EVALUATETOCODECONTEXT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_ISEXPRESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_ISNONUSERCODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_NOSIDEEFFECTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEBUG_TEXT_RETURNVALUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DebugHelper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bfcc060_8c1d_11d0_accd_00aa0060275c);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DefaultDebugSessionProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x834128a2_51f4_11d0_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_INVALID_MEMORY_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338171i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_MISMATCHED_RUNTIME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338175i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_OUTSIDE_OF_VM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338172i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338169i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338170i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const E_JsDEBUG_UNKNOWN_THREAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1916338174i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const FACILITY_JsDEBUG: u32 = 3527u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const GETATTRFLAG_HUMANTEXT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const GETATTRFLAG_THIS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const GETATTRTYPE_DEPSCAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const GETATTRTYPE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const MachineDebugManager_DEBUG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49769cec_3a55_4bb0_b697_88fede77e8ea);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const MachineDebugManager_RETAIL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c0a3666_30c9_11d0_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const OID_JSSIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06c9e010_38ce_11d4_a2a3_00104bd35090);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const OID_VBSSIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1629f04e_2799_4db5_8fe5_ace10f17ebab);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const OID_WSFSIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a610570_38ce_11d4_a2a3_00104bd35090);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ProcessDebugManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78a51822_51f4_11d0_8f20_00805f2cd064);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTINFO_ITYPEINFO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTINFO_IUNKNOWN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTINTERRUPT_DEBUG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTINTERRUPT_RAISEEXCEPTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_CODEONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_GLOBALMEMBERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_ISPERSISTENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_ISSOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_ISVISIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTITEM_NOCODE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROC_HOSTMANAGESSOURCE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROC_IMPLICIT_PARENTS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROC_IMPLICIT_THIS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROC_ISEXPRESSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROC_ISXDOMAIN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION: u32 = 1879048194u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_BUILDNUMBER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_CATCHEXCEPTION: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_CONVERSIONLCID: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_DEBUGGER: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_DELAYEDEVENTSINKING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_GCCONTROLSOFTCLOSE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_HACK_FIBERSUPPORT: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_HACK_TRIDENTEVENTSINK: u32 = 1879048193u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_HOSTKEEPALIVE: u32 = 1879048196u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_HOSTSTACKREQUIRED: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_INTEGERMODE: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_INVOKEVERSIONING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_JITDEBUG: u32 = 4353u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_MAJORVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_MINORVERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_SCRIPTSAREFULLYTRUSTED: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTPROP_STRINGCOMPAREINSTANCE: u32 = 12289u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTAT_INSTRUCTION_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTAT_INTSTRUCTION_TIME: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTAT_STATEMENT_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTAT_TOTAL_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_DELAYEXECUTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_HOSTMANAGESSOURCE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_ISEXPRESSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_ISNONUSERCODE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_ISPERSISTENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_ISVISIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTEXT_ISXDOMAIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTYPELIB_ISCONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTYPELIB_ISPERSISTENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_COMMIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_ENUMLIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_ENUM_TRIGGER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_GLOBALLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_MEMBERLIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_MEMBER_TRIGGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_NOLIST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_PARAMTIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_CMPL_PARAM_TRIGGER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_ENCODE_DEFAULT_LANGUAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_ENCODE_NO_ASP_LANGUAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_ENCODE_SECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_E_PROPAGATE: i32 = -2147352318i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_E_RECORDED: i32 = -2040119292i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPT_E_REPORTED: i32 = -2147352319i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_COMMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_FUNCTION_START: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_HUMANTEXT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_IDENTIFIER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_KEYWORD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_MEMBERLOOKUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_NONSOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_NUMBER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_OPERATOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_STRING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SOURCETEXT_ATTR_THIS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const TEXT_DOC_ATTR_READONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const TEXT_DOC_ATTR_TYPE_PRIMARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const TEXT_DOC_ATTR_TYPE_SCRIPT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const TEXT_DOC_ATTR_TYPE_WORKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const THREAD_BLOCKED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const THREAD_OUT_OF_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const THREAD_STATE_RUNNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const THREAD_STATE_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const fasaCaseSensitive: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const fasaPreferInternalHandler: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const fasaSupportInternalHandler: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPLICATION_NODE_EVENT_FILTER(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const FILTER_EXCLUDE_NOTHING: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const FILTER_EXCLUDE_ANONYMOUS_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const FILTER_EXCLUDE_EVAL_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(2i32);
impl ::core::marker::Copy for APPLICATION_NODE_EVENT_FILTER {}
impl ::core::clone::Clone for APPLICATION_NODE_EVENT_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPLICATION_NODE_EVENT_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for APPLICATION_NODE_EVENT_FILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for APPLICATION_NODE_EVENT_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATION_NODE_EVENT_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BREAKPOINT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKPOINT_DELETED: BREAKPOINT_STATE = BREAKPOINT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKPOINT_DISABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKPOINT_ENABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(2i32);
impl ::core::marker::Copy for BREAKPOINT_STATE {}
impl ::core::clone::Clone for BREAKPOINT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BREAKPOINT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BREAKPOINT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BREAKPOINT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKPOINT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BREAKREASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_STEP: BREAKREASON = BREAKREASON(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_BREAKPOINT: BREAKREASON = BREAKREASON(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_DEBUGGER_BLOCK: BREAKREASON = BREAKREASON(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_HOST_INITIATED: BREAKREASON = BREAKREASON(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_LANGUAGE_INITIATED: BREAKREASON = BREAKREASON(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_DEBUGGER_HALT: BREAKREASON = BREAKREASON(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_ERROR: BREAKREASON = BREAKREASON(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_JIT: BREAKREASON = BREAKREASON(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKREASON_MUTATION_BREAKPOINT: BREAKREASON = BREAKREASON(8i32);
impl ::core::marker::Copy for BREAKREASON {}
impl ::core::clone::Clone for BREAKREASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BREAKREASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BREAKREASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BREAKREASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKREASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BREAKRESUMEACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_ABORT: BREAKRESUMEACTION = BREAKRESUMEACTION(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_CONTINUE: BREAKRESUMEACTION = BREAKRESUMEACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_STEP_INTO: BREAKRESUMEACTION = BREAKRESUMEACTION(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_STEP_OVER: BREAKRESUMEACTION = BREAKRESUMEACTION(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_STEP_OUT: BREAKRESUMEACTION = BREAKRESUMEACTION(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_IGNORE: BREAKRESUMEACTION = BREAKRESUMEACTION(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const BREAKRESUMEACTION_STEP_DOCUMENT: BREAKRESUMEACTION = BREAKRESUMEACTION(6i32);
impl ::core::marker::Copy for BREAKRESUMEACTION {}
impl ::core::clone::Clone for BREAKRESUMEACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BREAKRESUMEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BREAKRESUMEACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BREAKRESUMEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BREAKRESUMEACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEBUG_EVENT_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEIT_GENERAL: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEIT_ASMJS_IN_DEBUGGING: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEIT_ASMJS_SUCCEEDED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DEIT_ASMJS_FAILED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(3i32);
impl ::core::marker::Copy for DEBUG_EVENT_INFO_TYPE {}
impl ::core::clone::Clone for DEBUG_EVENT_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEBUG_EVENT_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEBUG_EVENT_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEBUG_EVENT_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_EVENT_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEBUG_STACKFRAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DST_SCRIPT_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DST_INTERNAL_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DST_INVOCATION_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(2i32);
impl ::core::marker::Copy for DEBUG_STACKFRAME_TYPE {}
impl ::core::clone::Clone for DEBUG_STACKFRAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEBUG_STACKFRAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEBUG_STACKFRAME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEBUG_STACKFRAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_STACKFRAME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOCUMENTNAMETYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_APPNODE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_FILE_TAIL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_UNIQUE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const DOCUMENTNAMETYPE_SOURCE_MAP_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(5i32);
impl ::core::marker::Copy for DOCUMENTNAMETYPE {}
impl ::core::clone::Clone for DOCUMENTNAMETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOCUMENTNAMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOCUMENTNAMETYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOCUMENTNAMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOCUMENTNAMETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ERRORRESUMEACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ERRORRESUMEACTION_ReexecuteErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller: ERRORRESUMEACTION = ERRORRESUMEACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ERRORRESUMEACTION_SkipErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(2i32);
impl ::core::marker::Copy for ERRORRESUMEACTION {}
impl ::core::clone::Clone for ERRORRESUMEACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERRORRESUMEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ERRORRESUMEACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ERRORRESUMEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERRORRESUMEACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JS_PROPERTY_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_ATTRIBUTE_NONE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_HAS_CHILDREN: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_FAKE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_METHOD: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_READONLY: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_NATIVE_WINRT_POINTER: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_FRAME_INTRYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_FRAME_INCATCHBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_FRAME_INFINALLYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(128i32);
impl ::core::marker::Copy for JS_PROPERTY_ATTRIBUTES {}
impl ::core::clone::Clone for JS_PROPERTY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JS_PROPERTY_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for JS_PROPERTY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for JS_PROPERTY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JS_PROPERTY_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JS_PROPERTY_MEMBERS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_MEMBERS_ALL: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const JS_PROPERTY_MEMBERS_ARGUMENTS: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(1i32);
impl ::core::marker::Copy for JS_PROPERTY_MEMBERS {}
impl ::core::clone::Clone for JS_PROPERTY_MEMBERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JS_PROPERTY_MEMBERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for JS_PROPERTY_MEMBERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for JS_PROPERTY_MEMBERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JS_PROPERTY_MEMBERS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JsDebugReadMemoryFlags(pub i32);
impl JsDebugReadMemoryFlags {
    pub const None: Self = Self(0i32);
    pub const JsDebugAllowPartialRead: Self = Self(1i32);
}
impl ::core::marker::Copy for JsDebugReadMemoryFlags {}
impl ::core::clone::Clone for JsDebugReadMemoryFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JsDebugReadMemoryFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for JsDebugReadMemoryFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for JsDebugReadMemoryFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsDebugReadMemoryFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_EVENT_MASK(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_EVENT_MASK_TRACE_ALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_EVENT_MASK_TRACE_ALL_WITH_DOM: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(7i32);
impl ::core::marker::Copy for PROFILER_EVENT_MASK {}
impl ::core::clone::Clone for PROFILER_EVENT_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_EVENT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_EVENT_MASK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_EVENT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_EVENT_MASK").field(&self.0).finish()
    }
}
impl PROFILER_EVENT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_HEAP_ENUM_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_ENUM_FLAGS_NONE: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_ENUM_FLAGS_RELATIONSHIP_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(3i32);
impl ::core::marker::Copy for PROFILER_HEAP_ENUM_FLAGS {}
impl ::core::clone::Clone for PROFILER_HEAP_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_HEAP_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_ENUM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_HEAP_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_ENUM_FLAGS").field(&self.0).finish()
    }
}
impl PROFILER_HEAP_ENUM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_HEAP_OBJECT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_OBJECT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_IS_ROOT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_SITE_CLOSED: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_UNKNOWN: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_DISPATCH: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_APPROXIMATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_STATE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_INSTANCE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_RUNTIMECLASS: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_DELEGATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_NAMESPACE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4096i32);
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_FLAGS {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl PROFILER_HEAP_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_PROTOTYPE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_FUNCTION_NAME: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SCOPE_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INTERNAL_PROPERTY: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_NAME_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INDEX_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_ATTRIBUTES_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_TEXT_CHILDREN_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_RELATIONSHIPS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WINRTEVENTS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WEAKMAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAX_VALUE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_NONE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_GET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_SET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(131072i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_LET_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(262144i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_CONST_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(524288i32);
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS").field(&self.0).finish()
    }
}
impl PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_HEAP_SUMMARY_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_HEAP_SUMMARY_VERSION_1: PROFILER_HEAP_SUMMARY_VERSION = PROFILER_HEAP_SUMMARY_VERSION(1i32);
impl ::core::marker::Copy for PROFILER_HEAP_SUMMARY_VERSION {}
impl ::core::clone::Clone for PROFILER_HEAP_SUMMARY_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_HEAP_SUMMARY_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_SUMMARY_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_HEAP_SUMMARY_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_HEAP_SUMMARY_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_RELATIONSHIP_INFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_NUMBER: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_STRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_HEAP_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_EXTERNAL_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_BSTR: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_PROPERTY_TYPE_SUBSTRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(6i32);
impl ::core::marker::Copy for PROFILER_RELATIONSHIP_INFO {}
impl ::core::clone::Clone for PROFILER_RELATIONSHIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_RELATIONSHIP_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_RELATIONSHIP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_RELATIONSHIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_RELATIONSHIP_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROFILER_SCRIPT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_SCRIPT_TYPE_USER: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_SCRIPT_TYPE_DYNAMIC: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_SCRIPT_TYPE_NATIVE: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const PROFILER_SCRIPT_TYPE_DOM: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(3i32);
impl ::core::marker::Copy for PROFILER_SCRIPT_TYPE {}
impl ::core::clone::Clone for PROFILER_SCRIPT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILER_SCRIPT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROFILER_SCRIPT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROFILER_SCRIPT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILER_SCRIPT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTGCTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTGCTYPE_NORMAL: SCRIPTGCTYPE = SCRIPTGCTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTGCTYPE_EXHAUSTIVE: SCRIPTGCTYPE = SCRIPTGCTYPE(1i32);
impl ::core::marker::Copy for SCRIPTGCTYPE {}
impl ::core::clone::Clone for SCRIPTGCTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTGCTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTGCTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTGCTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTGCTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTLANGUAGEVERSION(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTLANGUAGEVERSION_DEFAULT: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTLANGUAGEVERSION_5_7: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTLANGUAGEVERSION_5_8: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTLANGUAGEVERSION_MAX: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(255i32);
impl ::core::marker::Copy for SCRIPTLANGUAGEVERSION {}
impl ::core::clone::Clone for SCRIPTLANGUAGEVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTLANGUAGEVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTLANGUAGEVERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTLANGUAGEVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTLANGUAGEVERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_UNINITIALIZED: SCRIPTSTATE = SCRIPTSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_INITIALIZED: SCRIPTSTATE = SCRIPTSTATE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_STARTED: SCRIPTSTATE = SCRIPTSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_CONNECTED: SCRIPTSTATE = SCRIPTSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_DISCONNECTED: SCRIPTSTATE = SCRIPTSTATE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTSTATE_CLOSED: SCRIPTSTATE = SCRIPTSTATE(4i32);
impl ::core::marker::Copy for SCRIPTSTATE {}
impl ::core::clone::Clone for SCRIPTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTTHREADSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTHREADSTATE_NOTINSCRIPT: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTHREADSTATE_RUNNING: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(1i32);
impl ::core::marker::Copy for SCRIPTTHREADSTATE {}
impl ::core::clone::Clone for SCRIPTTHREADSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTTHREADSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTTHREADSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTTHREADSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTTHREADSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTTRACEINFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_SCRIPTSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_SCRIPTEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_COMCALLSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_COMCALLEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_CREATEOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_CREATEOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_GETOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTTRACEINFO_GETOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(7i32);
impl ::core::marker::Copy for SCRIPTTRACEINFO {}
impl ::core::clone::Clone for SCRIPTTRACEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTTRACEINFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTTRACEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTTRACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTTRACEINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTUICHANDLING(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTUICHANDLING_ALLOW: SCRIPTUICHANDLING = SCRIPTUICHANDLING(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTUICHANDLING_NOUIERROR: SCRIPTUICHANDLING = SCRIPTUICHANDLING(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTUICHANDLING_NOUIDEFAULT: SCRIPTUICHANDLING = SCRIPTUICHANDLING(2i32);
impl ::core::marker::Copy for SCRIPTUICHANDLING {}
impl ::core::clone::Clone for SCRIPTUICHANDLING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTUICHANDLING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTUICHANDLING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTUICHANDLING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTUICHANDLING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTUICITEM(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTUICITEM_INPUTBOX: SCRIPTUICITEM = SCRIPTUICITEM(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SCRIPTUICITEM_MSGBOX: SCRIPTUICITEM = SCRIPTUICITEM(2i32);
impl ::core::marker::Copy for SCRIPTUICITEM {}
impl ::core::clone::Clone for SCRIPTUICITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTUICITEM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPTUICITEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTUICITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTUICITEM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPT_DEBUGGER_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SDO_NONE: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SDO_ENABLE_FIRST_CHANCE_EXCEPTIONS: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SDO_ENABLE_WEB_WORKER_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SDO_ENABLE_NONUSER_CODE_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SDO_ENABLE_LIBRARY_STACK_FRAME: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(8i32);
impl ::core::marker::Copy for SCRIPT_DEBUGGER_OPTIONS {}
impl ::core::clone::Clone for SCRIPT_DEBUGGER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPT_DEBUGGER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPT_DEBUGGER_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPT_DEBUGGER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_DEBUGGER_OPTIONS").field(&self.0).finish()
    }
}
impl SCRIPT_DEBUGGER_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SCRIPT_DEBUGGER_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SCRIPT_DEBUGGER_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ETK_FIRST_CHANCE: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ETK_USER_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const ETK_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(2i32);
impl ::core::marker::Copy for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {}
impl ::core::clone::Clone for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPT_INVOCATION_CONTEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_Event: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_SetTimeout: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_SetInterval: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_SetImmediate: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_RequestAnimationFrame: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_ToString: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_MutationObserverCheckpoint: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_WWAExecUnsafeLocalFunction: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub const SICT_WWAExecAtPriority: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(8i32);
impl ::core::marker::Copy for SCRIPT_INVOCATION_CONTEXT_TYPE {}
impl ::core::clone::Clone for SCRIPT_INVOCATION_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPT_INVOCATION_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SCRIPT_INVOCATION_CONTEXT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SCRIPT_INVOCATION_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPT_INVOCATION_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DebugStackFrameDescriptor {
    pub pdsf: ::std::mem::ManuallyDrop<::core::option::Option<IDebugStackFrame>>,
    pub dwMin: u32,
    pub dwLim: u32,
    pub fFinal: super::super::super::super::Foundation::BOOL,
    pub punkFinal: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DebugStackFrameDescriptor {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DebugStackFrameDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugStackFrameDescriptor").field("pdsf", &self.pdsf).field("dwMin", &self.dwMin).field("dwLim", &self.dwLim).field("fFinal", &self.fFinal).field("punkFinal", &self.punkFinal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DebugStackFrameDescriptor {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DebugStackFrameDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.pdsf == other.pdsf && self.dwMin == other.dwMin && self.dwLim == other.dwLim && self.fFinal == other.fFinal && self.punkFinal == other.punkFinal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DebugStackFrameDescriptor {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DebugStackFrameDescriptor {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DebugStackFrameDescriptor64 {
    pub pdsf: ::std::mem::ManuallyDrop<::core::option::Option<IDebugStackFrame>>,
    pub dwMin: u64,
    pub dwLim: u64,
    pub fFinal: super::super::super::super::Foundation::BOOL,
    pub punkFinal: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DebugStackFrameDescriptor64 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DebugStackFrameDescriptor64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugStackFrameDescriptor64").field("pdsf", &self.pdsf).field("dwMin", &self.dwMin).field("dwLim", &self.dwLim).field("fFinal", &self.fFinal).field("punkFinal", &self.punkFinal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DebugStackFrameDescriptor64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DebugStackFrameDescriptor64 {
    fn eq(&self, other: &Self) -> bool {
        self.pdsf == other.pdsf && self.dwMin == other.dwMin && self.dwLim == other.dwLim && self.fFinal == other.fFinal && self.punkFinal == other.punkFinal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DebugStackFrameDescriptor64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DebugStackFrameDescriptor64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct JS_NATIVE_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
}
impl ::core::marker::Copy for JS_NATIVE_FRAME {}
impl ::core::clone::Clone for JS_NATIVE_FRAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JS_NATIVE_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JS_NATIVE_FRAME").field("InstructionOffset", &self.InstructionOffset).field("ReturnOffset", &self.ReturnOffset).field("FrameOffset", &self.FrameOffset).field("StackOffset", &self.StackOffset).finish()
    }
}
impl ::windows::core::TypeKind for JS_NATIVE_FRAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for JS_NATIVE_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset
    }
}
impl ::core::cmp::Eq for JS_NATIVE_FRAME {}
impl ::core::default::Default for JS_NATIVE_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct JsDebugPropertyInfo {
    pub name: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub r#type: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub value: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub fullName: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub attr: JS_PROPERTY_ATTRIBUTES,
}
impl ::core::clone::Clone for JsDebugPropertyInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for JsDebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JsDebugPropertyInfo").field("name", &self.name).field("type", &self.r#type).field("value", &self.value).field("fullName", &self.fullName).field("attr", &self.attr).finish()
    }
}
impl ::windows::core::TypeKind for JsDebugPropertyInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for JsDebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.value == other.value && self.fullName == other.fullName && self.attr == other.attr
    }
}
impl ::core::cmp::Eq for JsDebugPropertyInfo {}
impl ::core::default::Default for JsDebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_OBJECT {
    pub size: u32,
    pub Anonymous: PROFILER_HEAP_OBJECT_0,
    pub typeNameId: u32,
    pub flags: u32,
    pub unused: u16,
    pub optionalInfoCount: u16,
}
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub union PROFILER_HEAP_OBJECT_0 {
    pub objectId: usize,
    pub externalObjectAddress: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_0 {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    pub infoType: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE,
    pub Anonymous: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0,
}
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub union PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    pub prototype: usize,
    pub functionName: ::windows::core::PCWSTR,
    pub elementAttributesSize: u32,
    pub elementTextChildrenSize: u32,
    pub scopeList: *mut PROFILER_HEAP_OBJECT_SCOPE_LIST,
    pub internalProperty: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP,
    pub namePropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub indexPropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub relationshipList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub eventList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub weakMapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub mapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub setCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
}
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP {
    pub relationshipId: u32,
    pub relationshipInfo: PROFILER_RELATIONSHIP_INFO,
    pub Anonymous: PROFILER_HEAP_OBJECT_RELATIONSHIP_0,
}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub union PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    pub numberValue: f64,
    pub stringValue: ::windows::core::PCWSTR,
    pub bstrValue: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub objectId: usize,
    pub externalObjectAddress: *mut ::core::ffi::c_void,
    pub subString: *mut PROFILER_PROPERTY_TYPE_SUBSTRING_INFO,
}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    pub count: u32,
    pub elements: [PROFILER_HEAP_OBJECT_RELATIONSHIP; 1],
}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_OBJECT_SCOPE_LIST {
    pub count: u32,
    pub scopes: [usize; 1],
}
impl ::core::marker::Copy for PROFILER_HEAP_OBJECT_SCOPE_LIST {}
impl ::core::clone::Clone for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_HEAP_OBJECT_SCOPE_LIST").field("count", &self.count).field("scopes", &self.scopes).finish()
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.scopes == other.scopes
    }
}
impl ::core::cmp::Eq for PROFILER_HEAP_OBJECT_SCOPE_LIST {}
impl ::core::default::Default for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_HEAP_SUMMARY {
    pub version: PROFILER_HEAP_SUMMARY_VERSION,
    pub totalHeapSize: u32,
}
impl ::core::marker::Copy for PROFILER_HEAP_SUMMARY {}
impl ::core::clone::Clone for PROFILER_HEAP_SUMMARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROFILER_HEAP_SUMMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_HEAP_SUMMARY").field("version", &self.version).field("totalHeapSize", &self.totalHeapSize).finish()
    }
}
impl ::windows::core::TypeKind for PROFILER_HEAP_SUMMARY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROFILER_HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.totalHeapSize == other.totalHeapSize
    }
}
impl ::core::cmp::Eq for PROFILER_HEAP_SUMMARY {}
impl ::core::default::Default for PROFILER_HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    pub length: u32,
    pub value: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {}
impl ::core::clone::Clone for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILER_PROPERTY_TYPE_SUBSTRING_INFO").field("length", &self.length).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::core::cmp::Eq for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {}
impl ::core::default::Default for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_ActiveScript\"`*"]
pub struct TEXT_DOCUMENT_ARRAY {
    pub dwCount: u32,
    pub Members: *mut ::core::option::Option<IDebugDocumentText>,
}
impl ::core::marker::Copy for TEXT_DOCUMENT_ARRAY {}
impl ::core::clone::Clone for TEXT_DOCUMENT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TEXT_DOCUMENT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXT_DOCUMENT_ARRAY").field("dwCount", &self.dwCount).field("Members", &self.Members).finish()
    }
}
impl ::windows::core::TypeKind for TEXT_DOCUMENT_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TEXT_DOCUMENT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.Members == other.Members
    }
}
impl ::core::cmp::Eq for TEXT_DOCUMENT_ARRAY {}
impl ::core::default::Default for TEXT_DOCUMENT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
