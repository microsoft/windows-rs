#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationActivation(pub ::windows::runtime::IUnknown);
impl IWebApplicationActivation {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn CancelPendingActivation(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationActivation {
    type Vtable = IWebApplicationActivation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3168587998, 13070, 18459, [184, 67, 72, 152, 166, 168, 235, 172]);
}
impl ::core::convert::From<IWebApplicationActivation> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationActivation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationActivation> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationActivation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationActivation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationActivation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationActivation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationAuthoringMode(pub ::windows::runtime::IUnknown);
impl IWebApplicationAuthoringMode {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`*"]
    pub unsafe fn AuthoringClientBinary(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationAuthoringMode {
    type Vtable = IWebApplicationAuthoringMode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1913318035, 6500, 19888, [176, 5, 41, 235, 158, 43, 24, 169]);
}
impl ::core::convert::From<IWebApplicationAuthoringMode> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationAuthoringMode> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebApplicationAuthoringMode> for super::super::super::Com::IServiceProvider {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebApplicationAuthoringMode> for super::super::super::Com::IServiceProvider {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Com::IServiceProvider> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Com::IServiceProvider> for &IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationAuthoringMode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, designmodedllpath: *mut ::core::mem::ManuallyDrop<super::super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationHost(pub ::windows::runtime::IUnknown);
impl IWebApplicationHost {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`*"]
    pub unsafe fn HWND(&self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[cfg(feature = "Win32_Web_MsHtml")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Web_MsHtml`*"]
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2> {
        let mut result__: <super::super::super::super::Web::MsHtml::IHTMLDocument2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Web::MsHtml::IHTMLDocument2>(result__)
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn Advise<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, interfaceid: *const ::windows::runtime::GUID, callback: Param1, cookie: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfaceid), callback.into_param().abi(), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn Unadvise(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationHost {
    type Vtable = IWebApplicationHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3469464259, 41893, 18249, [150, 129, 32, 233, 22, 28, 103, 148]);
}
impl ::core::convert::From<IWebApplicationHost> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationHost> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Web_MsHtml")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmldocument: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Web_MsHtml"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interfaceid: *const ::windows::runtime::GUID, callback: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationNavigationEvents(pub ::windows::runtime::IUnknown);
impl IWebApplicationNavigationEvents {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`, `Win32_Web_MsHtml`*"]
    pub unsafe fn BeforeNavigate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1, navigationflags: u32, targetframename: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), ::core::mem::transmute(navigationflags), targetframename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`, `Win32_Web_MsHtml`*"]
    pub unsafe fn NavigateComplete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`, `Win32_Web_MsHtml`*"]
    pub unsafe fn NavigateError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1, targetframename: Param2, statuscode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), targetframename.into_param().abi(), ::core::mem::transmute(statuscode)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`, `Win32_Web_MsHtml`*"]
    pub unsafe fn DocumentComplete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn DownloadBegin(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn DownloadComplete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationNavigationEvents {
    type Vtable = IWebApplicationNavigationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3257275858, 54040, 19874, [132, 34, 31, 202, 247, 123, 16, 228]);
}
impl ::core::convert::From<IWebApplicationNavigationEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationNavigationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationNavigationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationNavigationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationNavigationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationScriptEvents(pub ::windows::runtime::IUnknown);
impl IWebApplicationScriptEvents {
    #[cfg(feature = "Win32_Web_MsHtml")]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Web_MsHtml`*"]
    pub unsafe fn BeforeScriptExecute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>>(&self, htmlwindow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`, `Win32_Foundation`, `Win32_Web_MsHtml`*"]
    pub unsafe fn ScriptError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::runtime::IntoParam<'a, super::IActiveScriptError>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::BOOL>>(&self, htmlwindow: Param0, scripterror: Param1, url: Param2, errorhandled: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), scripterror.into_param().abi(), url.into_param().abi(), errorhandled.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationScriptEvents {
    type Vtable = IWebApplicationScriptEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2084530584, 5479, 19386, [181, 43, 72, 211, 33, 65, 214, 19]);
}
impl ::core::convert::From<IWebApplicationScriptEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationScriptEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationScriptEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationScriptEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationScriptEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Web_MsHtml")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Web_MsHtml"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htmlwindow: ::windows::runtime::RawPtr, scripterror: ::windows::runtime::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationUIEvents(pub ::windows::runtime::IUnknown);
impl IWebApplicationUIEvents {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn SecurityProblem(&self, securityproblem: u32, result: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityproblem), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationUIEvents {
    type Vtable = IWebApplicationUIEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1529560985, 12940, 16853, [166, 247, 116, 131, 237, 142, 113, 221]);
}
impl ::core::convert::From<IWebApplicationUIEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationUIEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationUIEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationUIEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUIEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, securityproblem: u32, result: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationUpdateEvents(pub ::windows::runtime::IUnknown);
impl IWebApplicationUpdateEvents {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn OnPaint(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
    pub unsafe fn OnCssChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebApplicationUpdateEvents {
    type Vtable = IWebApplicationUpdateEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1046079159, 50770, 19887, [173, 94, 22, 254, 179, 80, 205, 227]);
}
impl ::core::convert::From<IWebApplicationUpdateEvents> for ::windows::runtime::IUnknown {
    fn from(value: IWebApplicationUpdateEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationUpdateEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IWebApplicationUpdateEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUpdateEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
pub type RegisterAuthoringClientFunctionType = unsafe extern "system" fn(authoringmodeobject: ::windows::runtime::RawPtr, host: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug_WebApp`*"]
pub type UnregisterAuthoringClientFunctionType = unsafe extern "system" fn(host: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
