#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationActivation(::windows::core::IUnknown);
impl IWebApplicationActivation {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn CancelPendingActivation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelPendingActivation)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationActivation> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationActivation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationActivation> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationActivation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationActivation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationActivation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationActivation {}
impl ::core::fmt::Debug for IWebApplicationActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationActivation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationActivation {
    type Vtable = IWebApplicationActivation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcdcd0de_330e_481b_b843_4898a6a8ebac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationActivation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CancelPendingActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWebApplicationAuthoringMode(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWebApplicationAuthoringMode {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.QueryService)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthoringClientBinary(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AuthoringClientBinary)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebApplicationAuthoringMode> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationAuthoringMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebApplicationAuthoringMode> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationAuthoringMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Com::IServiceProvider> for &'a IWebApplicationAuthoringMode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Com::IServiceProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWebApplicationAuthoringMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebApplicationAuthoringMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebApplicationAuthoringMode {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebApplicationAuthoringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationAuthoringMode").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWebApplicationAuthoringMode {
    type Vtable = IWebApplicationAuthoringMode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x720aea93_1964_4db0_b005_29eb9e2b18a9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationAuthoringMode_Vtbl {
    pub base: super::super::super::Com::IServiceProvider_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthoringClientBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthoringClientBinary: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationHost(::windows::core::IUnknown);
impl IWebApplicationHost {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HWND(&self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HWND)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwnd)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn Document(&self) -> ::windows::core::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Document)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Web::MsHtml::IHTMLDocument2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn Advise<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, interfaceid: *const ::windows::core::GUID, callback: Param1, cookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Advise)(::core::mem::transmute_copy(self), ::core::mem::transmute(interfaceid), callback.into_param().abi(), ::core::mem::transmute(cookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn Unadvise(&self, cookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(cookie)).ok()
    }
}
impl ::core::convert::From<IWebApplicationHost> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationHost> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationHost {}
impl ::core::fmt::Debug for IWebApplicationHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationHost {
    type Vtable = IWebApplicationHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcecbd2c3_a3a5_4749_9681_20e9161c6794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationHost_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HWND: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub Document: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    Document: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationNavigationEvents(::windows::core::IUnknown);
impl IWebApplicationNavigationEvents {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn BeforeNavigate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1, navigationflags: u32, targetframename: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeforeNavigate)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), ::core::mem::transmute(navigationflags), targetframename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NavigateComplete)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn NavigateError<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1, targetframename: Param2, statuscode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NavigateError)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi(), targetframename.into_param().abi(), ::core::mem::transmute(statuscode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn DocumentComplete<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, htmlwindow: Param0, url: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DocumentComplete)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), url.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn DownloadBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DownloadBegin)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn DownloadComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DownloadComplete)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationNavigationEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationNavigationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationNavigationEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationNavigationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationNavigationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationNavigationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationNavigationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationNavigationEvents {}
impl ::core::fmt::Debug for IWebApplicationNavigationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationNavigationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationNavigationEvents {
    type Vtable = IWebApplicationNavigationEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc22615d2_d318_4da2_8422_1fcaf77b10e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationNavigationEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: ::windows::core::PCWSTR, navigationflags: u32, targetframename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeNavigate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateComplete: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub NavigateError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: ::windows::core::PCWSTR, targetframename: ::windows::core::PCWSTR, statuscode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    NavigateError: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub DocumentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    DocumentComplete: usize,
    pub DownloadBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationScriptEvents(::windows::core::IUnknown);
impl IWebApplicationScriptEvents {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn BeforeScriptExecute<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>>(&self, htmlwindow: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeforeScriptExecute)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub unsafe fn ScriptError<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Web::MsHtml::IHTMLWindow2>, Param1: ::windows::core::IntoParam<'a, super::IActiveScriptError>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>>(&self, htmlwindow: Param0, scripterror: Param1, url: Param2, errorhandled: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScriptError)(::core::mem::transmute_copy(self), htmlwindow.into_param().abi(), scripterror.into_param().abi(), url.into_param().abi(), errorhandled.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWebApplicationScriptEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationScriptEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationScriptEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationScriptEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationScriptEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationScriptEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationScriptEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationScriptEvents {}
impl ::core::fmt::Debug for IWebApplicationScriptEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationScriptEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationScriptEvents {
    type Vtable = IWebApplicationScriptEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c3f6998_1567_4bba_b52b_48d32141d613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationScriptEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub BeforeScriptExecute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    BeforeScriptExecute: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
    pub ScriptError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: ::windows::core::PCWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml")))]
    ScriptError: usize,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationUIEvents(::windows::core::IUnknown);
impl IWebApplicationUIEvents {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn SecurityProblem(&self, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SecurityProblem)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityproblem), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<IWebApplicationUIEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationUIEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationUIEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationUIEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationUIEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationUIEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationUIEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationUIEvents {}
impl ::core::fmt::Debug for IWebApplicationUIEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationUIEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationUIEvents {
    type Vtable = IWebApplicationUIEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b2b3f99_328c_41d5_a6f7_7483ed8e71dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUIEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SecurityProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
#[repr(transparent)]
pub struct IWebApplicationUpdateEvents(::windows::core::IUnknown);
impl IWebApplicationUpdateEvents {
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn OnPaint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPaint)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
    pub unsafe fn OnCssChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCssChanged)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWebApplicationUpdateEvents> for ::windows::core::IUnknown {
    fn from(value: IWebApplicationUpdateEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebApplicationUpdateEvents> for ::windows::core::IUnknown {
    fn from(value: &IWebApplicationUpdateEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebApplicationUpdateEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebApplicationUpdateEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebApplicationUpdateEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebApplicationUpdateEvents {}
impl ::core::fmt::Debug for IWebApplicationUpdateEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebApplicationUpdateEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWebApplicationUpdateEvents {
    type Vtable = IWebApplicationUpdateEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e59e6b7_c652_4daf_ad5e_16feb350cde3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebApplicationUpdateEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnPaint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnCssChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type RegisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(authoringmodeobject: ::core::option::Option<IWebApplicationAuthoringMode>, host: ::core::option::Option<IWebApplicationHost>) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug_WebApp\"`*"]
pub type UnregisterAuthoringClientFunctionType = ::core::option::Option<unsafe extern "system" fn(host: ::core::option::Option<IWebApplicationHost>) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
