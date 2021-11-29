#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationActivation(pub ::windows::core::IUnknown);
impl IWebApplicationActivation {
    pub unsafe fn CancelPendingActivation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationActivation {
    type Vtable = IWebApplicationActivation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcdcd0de_330e_481b_b843_4898a6a8ebac);
}
impl ::core::convert::From<IWebApplicationActivation> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationActivation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationActivation> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationActivation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationActivation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationAuthoringMode(pub ::windows::core::IUnknown);
impl IWebApplicationAuthoringMode {
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthoringClientBinary(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationAuthoringMode {
    type Vtable = IWebApplicationAuthoringMode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720aea93_1964_4db0_b005_29eb9e2b18a9);
}
impl ::core::convert::From<IWebApplicationAuthoringMode> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationAuthoringMode> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Com::IServiceProvider> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Com::IServiceProvider> for &IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationAuthoringMode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, designmodedllpath: *mut ::core::mem::ManuallyDrop<super::super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationHost(pub ::windows::core::IUnknown);
impl IWebApplicationHost {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HWND(&self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[cfg(feature = "Win32_Web_MsHtml")]
    pub unsafe fn Document(&self) -> ::windows::core::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2> {
        let mut result__: <super::super::super::super::Web::MsHtml::IHTMLDocument2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Web::MsHtml::IHTMLDocument2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Advise<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, interfaceid: *const ::windows::core::GUID, callback: Param1, cookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfaceid), callback.into_param().abi(), ::core::mem::transmute(cookie)).ok()
    }
    pub unsafe fn Unadvise(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationHost {
    type Vtable = IWebApplicationHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcecbd2c3_a3a5_4749_9681_20e9161c6794);
}
impl ::core::convert::From<IWebApplicationHost> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationHost> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Web_MsHtml")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Web_MsHtml"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interfaceid: *const ::windows::core::GUID, callback: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationNavigationEvents(pub ::windows::core::IUnknown);
impl IWebApplicationNavigationEvents {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn BeforeNavigate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1, navigationflags: u32, targetframename: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), ::core::mem::transmute(navigationflags), targetframename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateError<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1, targetframename: Param2, statuscode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), targetframename.into_param().abi(), ::core::mem::transmute(statuscode)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn DocumentComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    pub unsafe fn DownloadBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DownloadComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationNavigationEvents {
    type Vtable = IWebApplicationNavigationEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc22615d2_d318_4da2_8422_1fcaf77b10e4);
}
impl ::core::convert::From<IWebApplicationNavigationEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationNavigationEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationNavigationEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationNavigationEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationNavigationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationScriptEvents(pub ::windows::core::IUnknown);
impl IWebApplicationScriptEvents {
    #[cfg(feature = "Win32_Web_MsHtml")]
    pub unsafe fn BeforeScriptExecute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>>(&self, htmlwindow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn ScriptError<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::IActiveScriptError>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>>(&self, htmlwindow: Param0, scripterror: Param1, url: Param2, errorhandled: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), scripterror.into_param().abi(), url.into_param().abi(), errorhandled.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationScriptEvents {
    type Vtable = IWebApplicationScriptEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c3f6998_1567_4bba_b52b_48d32141d613);
}
impl ::core::convert::From<IWebApplicationScriptEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationScriptEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationScriptEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationScriptEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationScriptEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Web_MsHtml")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Web_MsHtml"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationUIEvents(pub ::windows::core::IUnknown);
impl IWebApplicationUIEvents {
    pub unsafe fn SecurityProblem(&self, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityproblem), ::core::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationUIEvents {
    type Vtable = IWebApplicationUIEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b2b3f99_328c_41d5_a6f7_7483ed8e71dd);
}
impl ::core::convert::From<IWebApplicationUIEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationUIEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationUIEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationUIEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUIEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWebApplicationUpdateEvents(pub ::windows::core::IUnknown);
impl IWebApplicationUpdateEvents {
    pub unsafe fn OnPaint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OnCssChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationUpdateEvents {
    type Vtable = IWebApplicationUpdateEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e59e6b7_c652_4daf_ad5e_16feb350cde3);
}
impl ::core::convert::From<IWebApplicationUpdateEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationUpdateEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebApplicationUpdateEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationUpdateEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUpdateEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: ::windows::core::RawPtr, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT>;
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: ::windows::core::RawPtr) -> ::windows::core::HRESULT>;
