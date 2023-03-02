#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[inline]
pub unsafe fn CoGetInterceptor<P0>(iidintercepted: *const ::windows::core::GUID, punkouter: P0, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetInterceptor ( iidintercepted : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetInterceptor(iidintercepted, punkouter.into_param().abi(), iid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<P0, P1>(iidintercepted: *const ::windows::core::GUID, punkouter: P0, typeinfo: P1, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    P1: ::windows::core::IntoParam<super::ITypeInfo>,
{
    ::windows::imp::link ! ( "ole32.dll""system" fn CoGetInterceptorFromTypeInfo ( iidintercepted : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , typeinfo : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CoGetInterceptorFromTypeInfo(iidintercepted, punkouter.into_param().abi(), typeinfo.into_param().abi(), iid, ppv).ok()
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallFrame(::windows::core::IUnknown);
impl ICallFrame {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIIDAndMethod)(::windows::core::Interface::as_raw(self), piid, pimethod).ok()
    }
    pub unsafe fn GetNames(&self, pwszinterface: *mut ::windows::core::PWSTR, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNames)(::windows::core::Interface::as_raw(self), pwszinterface, pwszmethod).ok()
    }
    pub unsafe fn GetStackLocation(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Interface::vtable(self).GetStackLocation)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetStackLocation(&self, pvstack: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).SetStackLocation)(::windows::core::Interface::as_raw(self), pvstack)
    }
    pub unsafe fn SetReturnValue(&self, hr: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).SetReturnValue)(::windows::core::Interface::as_raw(self), hr)
    }
    pub unsafe fn GetReturnValue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReturnValue)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> ::windows::core::Result<CALLFRAMEPARAMINFO> {
        let mut result__ = ::windows::core::zeroed::<CALLFRAMEPARAMINFO>();
        (::windows::core::Interface::vtable(self).GetParamInfo)(::windows::core::Interface::as_raw(self), iparam, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParam)(::windows::core::Interface::as_raw(self), iparam, pvar).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParam(&self, iparam: u32) -> ::windows::core::Result<super::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::VARIANT>();
        (::windows::core::Interface::vtable(self).GetParam)(::windows::core::Interface::as_raw(self), iparam, &mut result__).from_abi(result__)
    }
    pub unsafe fn Copy<P0>(&self, copycontrol: CALLFRAME_COPY, pwalker: P0) -> ::windows::core::Result<ICallFrame>
    where
        P0: ::windows::core::IntoParam<ICallFrameWalker>,
    {
        let mut result__ = ::windows::core::zeroed::<ICallFrame>();
        (::windows::core::Interface::vtable(self).Copy)(::windows::core::Interface::as_raw(self), copycontrol, pwalker.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Free<P0, P1, P2, P3>(&self, pframeargsdest: P0, pwalkerdestfree: P1, pwalkercopy: P2, freeflags: u32, pwalkerfree: P3, nullflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICallFrame>,
        P1: ::windows::core::IntoParam<ICallFrameWalker>,
        P2: ::windows::core::IntoParam<ICallFrameWalker>,
        P3: ::windows::core::IntoParam<ICallFrameWalker>,
    {
        (::windows::core::Interface::vtable(self).Free)(::windows::core::Interface::as_raw(self), pframeargsdest.into_param().abi(), pwalkerdestfree.into_param().abi(), pwalkercopy.into_param().abi(), freeflags, pwalkerfree.into_param().abi(), nullflags).ok()
    }
    pub unsafe fn FreeParam<P0>(&self, iparam: u32, freeflags: u32, pwalkerfree: P0, nullflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICallFrameWalker>,
    {
        (::windows::core::Interface::vtable(self).FreeParam)(::windows::core::Interface::as_raw(self), iparam, freeflags, pwalkerfree.into_param().abi(), nullflags).ok()
    }
    pub unsafe fn WalkFrame<P0>(&self, walkwhat: u32, pwalker: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICallFrameWalker>,
    {
        (::windows::core::Interface::vtable(self).WalkFrame)(::windows::core::Interface::as_raw(self), walkwhat, pwalker.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMarshalSizeMax)(::windows::core::Interface::as_raw(self), pmshlcontext, mshlflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: &[u8], pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Marshal)(::windows::core::Interface::as_raw(self), pmshlcontext, mshlflags, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, pcbbufferused, pdatarep, prpcflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal(&self, pbuffer: &[u8], datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Unmarshal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, datarep, pcontext, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMarshalData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, ibfirstrelease, datarep, pcontext).ok()
    }
    pub unsafe fn Invoke(&self, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::windows::core::Interface::as_raw(self), pvreceiver).ok()
    }
}
::windows::imp::interface_hierarchy!(ICallFrame, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICallFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrame {}
impl ::core::fmt::Debug for ICallFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallFrame {
    type Vtable = ICallFrame_Vtbl;
}
impl ::core::clone::Clone for ICallFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd573b4b0_894e_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrame_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInfo: usize,
    pub GetIIDAndMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT,
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinterface: *mut ::windows::core::PWSTR, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetStackLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub SetStackLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvstack: *const ::core::ffi::c_void),
    pub SetReturnValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT),
    pub GetReturnValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetParamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetParamInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetParam: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, pvar: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetParam: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframeargsdest: *mut ::core::ffi::c_void, pwalkerdestfree: *mut ::core::ffi::c_void, pwalkercopy: *mut ::core::ffi::c_void, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows::core::HRESULT,
    pub FreeParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: *mut ::core::ffi::c_void, nullflags: u32) -> ::windows::core::HRESULT,
    pub WalkFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, walkwhat: u32, pwalker: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMarshalSizeMax: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Marshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Marshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallFrameEvents(::windows::core::IUnknown);
impl ICallFrameEvents {
    pub unsafe fn OnCall<P0>(&self, pframe: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICallFrame>,
    {
        (::windows::core::Interface::vtable(self).OnCall)(::windows::core::Interface::as_raw(self), pframe.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICallFrameEvents, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICallFrameEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrameEvents {}
impl ::core::fmt::Debug for ICallFrameEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrameEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallFrameEvents {
    type Vtable = ICallFrameEvents_Vtbl;
}
impl ::core::clone::Clone for ICallFrameEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallFrameEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd5e0843_fc91_11d0_97d7_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframe: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallFrameWalker(::windows::core::IUnknown);
impl ICallFrameWalker {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWalkInterface<P0, P1>(&self, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: P0, fout: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnWalkInterface)(::windows::core::Interface::as_raw(self), iid, ppvinterface, fin.into_param().abi(), fout.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ICallFrameWalker, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICallFrameWalker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFrameWalker {}
impl ::core::fmt::Debug for ICallFrameWalker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFrameWalker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallFrameWalker {
    type Vtable = ICallFrameWalker_Vtbl;
}
impl ::core::clone::Clone for ICallFrameWalker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallFrameWalker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08b23919_392d_11d2_b8a4_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameWalker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWalkInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWalkInterface: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallIndirect(::windows::core::IUnknown);
impl ICallIndirect {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CallIndirect)(::windows::core::Interface::as_raw(self), phrreturn, imethod, pvargs, cbargs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMethodInfo)(::windows::core::Interface::as_raw(self), imethod, pinfo, pwszmethod).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStackSize)(::windows::core::Interface::as_raw(self), imethod, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: ::core::option::Option<*mut ::windows::core::GUID>, pfderivesfromidispatch: ::core::option::Option<*mut super::super::super::Foundation::BOOL>, pcmethod: ::core::option::Option<*mut u32>, pwszinterface: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfderivesfromidispatch.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcmethod.unwrap_or(::std::ptr::null_mut())), pwszinterface).ok()
    }
}
::windows::imp::interface_hierarchy!(ICallIndirect, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICallIndirect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallIndirect {}
impl ::core::fmt::Debug for ICallIndirect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallIndirect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallIndirect {
    type Vtable = ICallIndirect_Vtbl;
}
impl ::core::clone::Clone for ICallIndirect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallIndirect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd573b4b1_894e_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallIndirect_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CallIndirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMethodInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMethodInfo: usize,
    pub GetStackSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIID: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallInterceptor(::windows::core::IUnknown);
impl ICallInterceptor {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CallIndirect)(::windows::core::Interface::as_raw(self), phrreturn, imethod, pvargs, cbargs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMethodInfo)(::windows::core::Interface::as_raw(self), imethod, pinfo, pwszmethod).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStackSize)(::windows::core::Interface::as_raw(self), imethod, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: ::core::option::Option<*mut ::windows::core::GUID>, pfderivesfromidispatch: ::core::option::Option<*mut super::super::super::Foundation::BOOL>, pcmethod: ::core::option::Option<*mut u32>, pwszinterface: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(piid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfderivesfromidispatch.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcmethod.unwrap_or(::std::ptr::null_mut())), pwszinterface).ok()
    }
    pub unsafe fn RegisterSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ICallFrameEvents>,
    {
        (::windows::core::Interface::vtable(self).RegisterSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisteredSink(&self) -> ::windows::core::Result<ICallFrameEvents> {
        let mut result__ = ::windows::core::zeroed::<ICallFrameEvents>();
        (::windows::core::Interface::vtable(self).GetRegisteredSink)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ICallInterceptor, ::windows::core::IUnknown, ICallIndirect);
impl ::core::cmp::PartialEq for ICallInterceptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallInterceptor {}
impl ::core::fmt::Debug for ICallInterceptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallInterceptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallInterceptor {
    type Vtable = ICallInterceptor_Vtbl;
}
impl ::core::clone::Clone for ICallInterceptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallInterceptor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60c7ca75_896d_11d2_b8b6_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallInterceptor_Vtbl {
    pub base__: ICallIndirect_Vtbl,
    pub RegisterSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRegisteredSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct ICallUnmarshal(::windows::core::IUnknown);
impl ICallUnmarshal {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal<P0>(&self, imethod: u32, pbuffer: &[u8], fforcebuffercopy: P0, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Unmarshal)(::windows::core::Interface::as_raw(self), imethod, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, fforcebuffercopy.into_param().abi(), datarep, pcontext, pcbunmarshalled, ::core::mem::transmute(ppframe)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: &[u8], ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMarshalData)(::windows::core::Interface::as_raw(self), imethod, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, ibfirstrelease, datarep, pcontext).ok()
    }
}
::windows::imp::interface_hierarchy!(ICallUnmarshal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICallUnmarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallUnmarshal {}
impl ::core::fmt::Debug for ICallUnmarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallUnmarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallUnmarshal {
    type Vtable = ICallUnmarshal_Vtbl;
}
impl ::core::clone::Clone for ICallUnmarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICallUnmarshal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5333b003_2e42_11d2_b89d_00c04fb9618a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallUnmarshal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Unmarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unmarshal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseMarshalData: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
pub struct IInterfaceRelated(::windows::core::IUnknown);
impl IInterfaceRelated {
    pub unsafe fn SetIID(&self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIID)(::windows::core::Interface::as_raw(self), iid).ok()
    }
    pub unsafe fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetIID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IInterfaceRelated, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInterfaceRelated {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInterfaceRelated {}
impl ::core::fmt::Debug for IInterfaceRelated {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInterfaceRelated").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInterfaceRelated {
    type Vtable = IInterfaceRelated_Vtbl;
}
impl ::core::clone::Clone for IInterfaceRelated {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInterfaceRelated {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1fb5a79_7706_11d1_adba_00c04fc2adc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterfaceRelated_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLFRAME_COPY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
impl ::core::marker::Copy for CALLFRAME_COPY {}
impl ::core::clone::Clone for CALLFRAME_COPY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_COPY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CALLFRAME_COPY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CALLFRAME_COPY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_COPY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLFRAME_FREE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
impl ::core::marker::Copy for CALLFRAME_FREE {}
impl ::core::clone::Clone for CALLFRAME_FREE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_FREE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CALLFRAME_FREE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CALLFRAME_FREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_FREE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLFRAME_NULL(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
impl ::core::marker::Copy for CALLFRAME_NULL {}
impl ::core::clone::Clone for CALLFRAME_NULL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_NULL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CALLFRAME_NULL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CALLFRAME_NULL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_NULL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CALLFRAME_WALK(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`*"]
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
impl ::core::marker::Copy for CALLFRAME_WALK {}
impl ::core::clone::Clone for CALLFRAME_WALK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLFRAME_WALK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CALLFRAME_WALK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CALLFRAME_WALK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLFRAME_WALK").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows::core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAMEINFO")
            .field("iMethod", &self.iMethod)
            .field("fHasInValues", &self.fHasInValues)
            .field("fHasInOutValues", &self.fHasInOutValues)
            .field("fHasOutValues", &self.fHasOutValues)
            .field("fDerivesFromIDispatch", &self.fDerivesFromIDispatch)
            .field("cInInterfacesMax", &self.cInInterfacesMax)
            .field("cInOutInterfacesMax", &self.cInOutInterfacesMax)
            .field("cOutInterfacesMax", &self.cOutInterfacesMax)
            .field("cTopLevelInInterfaces", &self.cTopLevelInInterfaces)
            .field("iid", &self.iid)
            .field("cMethod", &self.cMethod)
            .field("cParams", &self.cParams)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CALLFRAMEINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.iMethod == other.iMethod && self.fHasInValues == other.fHasInValues && self.fHasInOutValues == other.fHasInOutValues && self.fHasOutValues == other.fHasOutValues && self.fDerivesFromIDispatch == other.fDerivesFromIDispatch && self.cInInterfacesMax == other.cInInterfacesMax && self.cInOutInterfacesMax == other.cInOutInterfacesMax && self.cOutInterfacesMax == other.cOutInterfacesMax && self.cTopLevelInInterfaces == other.cTopLevelInInterfaces && self.iid == other.iid && self.cMethod == other.cMethod && self.cParams == other.cParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEPARAMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAMEPARAMINFO").field("fIn", &self.fIn).field("fOut", &self.fOut).field("stackOffset", &self.stackOffset).field("cbParam", &self.cbParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CALLFRAMEPARAMINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAMEPARAMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fIn == other.fIn && self.fOut == other.fOut && self.stackOffset == other.stackOffset && self.cbParam == other.cbParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_CallObj\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub guidTransferSyntax: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAME_MARSHALCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALLFRAME_MARSHALCONTEXT").field("fIn", &self.fIn).field("dwDestContext", &self.dwDestContext).field("pvDestContext", &self.pvDestContext).field("punkReserved", &self.punkReserved).field("guidTransferSyntax", &self.guidTransferSyntax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CALLFRAME_MARSHALCONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CALLFRAME_MARSHALCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.fIn == other.fIn && self.dwDestContext == other.dwDestContext && self.pvDestContext == other.pvDestContext && self.punkReserved == other.punkReserved && self.guidTransferSyntax == other.guidTransferSyntax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
