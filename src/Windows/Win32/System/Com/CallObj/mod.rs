#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
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
    pub iid: ::windows::runtime::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALLFRAMEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALLFRAMEINFO")
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
impl ::std::cmp::PartialEq for CALLFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.iMethod == other.iMethod
            && self.fHasInValues == other.fHasInValues
            && self.fHasInOutValues == other.fHasInOutValues
            && self.fHasOutValues == other.fHasOutValues
            && self.fDerivesFromIDispatch == other.fDerivesFromIDispatch
            && self.cInInterfacesMax == other.cInInterfacesMax
            && self.cInOutInterfacesMax == other.cInOutInterfacesMax
            && self.cOutInterfacesMax == other.cOutInterfacesMax
            && self.cTopLevelInInterfaces == other.cTopLevelInInterfaces
            && self.iid == other.iid
            && self.cMethod == other.cMethod
            && self.cParams == other.cParams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALLFRAMEINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALLFRAMEPARAMINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALLFRAMEPARAMINFO").field("fIn", &self.fIn).field("fOut", &self.fOut).field("stackOffset", &self.stackOffset).field("cbParam", &self.cbParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CALLFRAMEPARAMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fIn == other.fIn && self.fOut == other.fOut && self.stackOffset == other.stackOffset && self.cbParam == other.cbParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALLFRAMEPARAMINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_COPY(pub i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
impl ::std::convert::From<i32> for CALLFRAME_COPY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLFRAME_COPY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_FREE(pub i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
impl ::std::convert::From<i32> for CALLFRAME_FREE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLFRAME_FREE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::std::ffi::c_void,
    pub punkReserved: ::std::option::Option<::windows::runtime::IUnknown>,
    pub guidTransferSyntax: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CALLFRAME_MARSHALCONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CALLFRAME_MARSHALCONTEXT").field("fIn", &self.fIn).field("dwDestContext", &self.dwDestContext).field("pvDestContext", &self.pvDestContext).field("punkReserved", &self.punkReserved).field("guidTransferSyntax", &self.guidTransferSyntax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CALLFRAME_MARSHALCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.fIn == other.fIn && self.dwDestContext == other.dwDestContext && self.pvDestContext == other.pvDestContext && self.punkReserved == other.punkReserved && self.guidTransferSyntax == other.guidTransferSyntax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CALLFRAME_MARSHALCONTEXT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_NULL(pub i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
impl ::std::convert::From<i32> for CALLFRAME_NULL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLFRAME_NULL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_WALK(pub i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
impl ::std::convert::From<i32> for CALLFRAME_WALK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLFRAME_WALK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[inline]
pub unsafe fn CoGetInterceptor<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(iidintercepted: *const ::windows::runtime::GUID, punkouter: Param1, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptor(iidintercepted: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetInterceptor(::std::mem::transmute(iidintercepted), punkouter.into_param().abi(), ::std::mem::transmute(iid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Ole_Automation")]
#[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_System_Ole_Automation`*"]
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, super::super::Ole::Automation::ITypeInfo>>(iidintercepted: *const ::windows::runtime::GUID, punkouter: Param1, typeinfo: Param2, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, typeinfo: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetInterceptorFromTypeInfo(::std::mem::transmute(iidintercepted), punkouter.into_param().abi(), typeinfo.into_param().abi(), ::std::mem::transmute(iid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallFrame(::windows::runtime::IUnknown);
impl ICallFrame {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<CALLFRAMEINFO> {
        let mut result__: <CALLFRAMEINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<CALLFRAMEINFO>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut ::windows::runtime::GUID, pimethod: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(piid), ::std::mem::transmute(pimethod)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetNames(&self, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwszinterface), ::std::mem::transmute(pwszmethod)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetStackLocation(&self) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn SetStackLocation(&self, pvstack: *const ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvstack)))
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn SetReturnValue(&self, hr: ::windows::runtime::HRESULT) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(hr)))
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetReturnValue(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> ::windows::runtime::Result<CALLFRAMEPARAMINFO> {
        let mut result__: <CALLFRAMEPARAMINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(iparam), &mut result__).from_abi::<CALLFRAMEPARAMINFO>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const super::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(iparam), ::std::mem::transmute(pvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetParam(&self, iparam: u32) -> ::windows::runtime::Result<super::VARIANT> {
        let mut result__: <super::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(iparam), &mut result__).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn Copy<'a, Param1: ::windows::runtime::IntoParam<'a, ICallFrameWalker>>(&self, copycontrol: CALLFRAME_COPY, pwalker: Param1) -> ::windows::runtime::Result<ICallFrame> {
        let mut result__: <ICallFrame as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(copycontrol), pwalker.into_param().abi(), &mut result__).from_abi::<ICallFrame>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn Free<'a, Param0: ::windows::runtime::IntoParam<'a, ICallFrame>, Param1: ::windows::runtime::IntoParam<'a, ICallFrameWalker>, Param2: ::windows::runtime::IntoParam<'a, ICallFrameWalker>, Param4: ::windows::runtime::IntoParam<'a, ICallFrameWalker>>(&self, pframeargsdest: Param0, pwalkerdestfree: Param1, pwalkercopy: Param2, freeflags: u32, pwalkerfree: Param4, nullflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pframeargsdest.into_param().abi(), pwalkerdestfree.into_param().abi(), pwalkercopy.into_param().abi(), ::std::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::std::mem::transmute(nullflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn FreeParam<'a, Param2: ::windows::runtime::IntoParam<'a, ICallFrameWalker>>(&self, iparam: u32, freeflags: u32, pwalkerfree: Param2, nullflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(iparam), ::std::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::std::mem::transmute(nullflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn WalkFrame<'a, Param1: ::windows::runtime::IntoParam<'a, ICallFrameWalker>>(&self, walkwhat: u32, pwalker: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(walkwhat), pwalker.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmshlcontext), ::std::mem::transmute(mshlflags), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmshlcontext), ::std::mem::transmute(mshlflags), ::std::mem::transmute(pbuffer), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(pcbbufferused), ::std::mem::transmute(pdatarep), ::std::mem::transmute(prpcflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn Unmarshal(&self, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(datarep), ::std::mem::transmute(pcontext), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ibfirstrelease), ::std::mem::transmute(datarep), ::std::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn Invoke(&self, pvreceiver: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvreceiver)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICallFrame {
    type Vtable = ICallFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3581129904, 35150, 4562, [184, 182, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallFrame> for ::windows::runtime::IUnknown {
    fn from(value: ICallFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallFrame> for ::windows::runtime::IUnknown {
    fn from(value: &ICallFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinfo: *mut CALLFRAMEINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID, pimethod: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvstack: *const ::std::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hr: ::windows::runtime::HRESULT),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iparam: u32, pvar: *const ::std::mem::ManuallyDrop<super::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iparam: u32, pvar: *mut ::std::mem::ManuallyDrop<super::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copycontrol: CALLFRAME_COPY, pwalker: ::windows::runtime::RawPtr, ppframe: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pframeargsdest: ::windows::runtime::RawPtr, pwalkerdestfree: ::windows::runtime::RawPtr, pwalkercopy: ::windows::runtime::RawPtr, freeflags: u32, pwalkerfree: ::windows::runtime::RawPtr, nullflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iparam: u32, freeflags: u32, pwalkerfree: ::windows::runtime::RawPtr, nullflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, walkwhat: u32, pwalker: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmshlcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmshlcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, mshlflags: super::MSHLFLAGS, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, pcbunmarshalled: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvreceiver: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallFrameEvents(::windows::runtime::IUnknown);
impl ICallFrameEvents {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn OnCall<'a, Param0: ::windows::runtime::IntoParam<'a, ICallFrame>>(&self, pframe: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pframe.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICallFrameEvents {
    type Vtable = ICallFrameEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4250798147, 64657, 4560, [151, 215, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallFrameEvents> for ::windows::runtime::IUnknown {
    fn from(value: ICallFrameEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallFrameEvents> for ::windows::runtime::IUnknown {
    fn from(value: &ICallFrameEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallFrameEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallFrameEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pframe: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallFrameWalker(::windows::runtime::IUnknown);
impl ICallFrameWalker {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn OnWalkInterface<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, iid: *const ::windows::runtime::GUID, ppvinterface: *const *const ::std::ffi::c_void, fin: Param2, fout: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid), ::std::mem::transmute(ppvinterface), fin.into_param().abi(), fout.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICallFrameWalker {
    type Vtable = ICallFrameWalker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(145897753, 14637, 4562, [184, 164, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallFrameWalker> for ::windows::runtime::IUnknown {
    fn from(value: ICallFrameWalker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallFrameWalker> for ::windows::runtime::IUnknown {
    fn from(value: &ICallFrameWalker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallFrameWalker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallFrameWalker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameWalker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppvinterface: *const *const ::std::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallIndirect(::windows::runtime::IUnknown);
impl ICallIndirect {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::runtime::HRESULT, imethod: u32, pvargs: *const ::std::ffi::c_void, cbargs: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrreturn), ::std::mem::transmute(imethod), ::std::mem::transmute(pvargs), ::std::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(imethod), ::std::mem::transmute(pinfo), ::std::mem::transmute(pwszmethod)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(imethod), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetIID(&self, piid: *mut ::windows::runtime::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(piid), ::std::mem::transmute(pfderivesfromidispatch), ::std::mem::transmute(pcmethod), ::std::mem::transmute(pwszinterface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICallIndirect {
    type Vtable = ICallIndirect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3581129905, 35150, 4562, [184, 182, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallIndirect> for ::windows::runtime::IUnknown {
    fn from(value: ICallIndirect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallIndirect> for ::windows::runtime::IUnknown {
    fn from(value: &ICallIndirect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallIndirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallIndirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallIndirect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrreturn: *mut ::windows::runtime::HRESULT, imethod: u32, pvargs: *const ::std::ffi::c_void, cbargs: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, cbargs: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallInterceptor(::windows::runtime::IUnknown);
impl ICallInterceptor {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::runtime::HRESULT, imethod: u32, pvargs: *const ::std::ffi::c_void, cbargs: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrreturn), ::std::mem::transmute(imethod), ::std::mem::transmute(pvargs), ::std::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(imethod), ::std::mem::transmute(pinfo), ::std::mem::transmute(pwszmethod)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(imethod), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn GetIID(&self, piid: *mut ::windows::runtime::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(piid), ::std::mem::transmute(pfderivesfromidispatch), ::std::mem::transmute(pcmethod), ::std::mem::transmute(pwszinterface)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn RegisterSink<'a, Param0: ::windows::runtime::IntoParam<'a, ICallFrameEvents>>(&self, psink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetRegisteredSink(&self) -> ::windows::runtime::Result<ICallFrameEvents> {
        let mut result__: <ICallFrameEvents as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ICallFrameEvents>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICallInterceptor {
    type Vtable = ICallInterceptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1623706229, 35181, 4562, [184, 182, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallInterceptor> for ::windows::runtime::IUnknown {
    fn from(value: ICallInterceptor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallInterceptor> for ::windows::runtime::IUnknown {
    fn from(value: &ICallInterceptor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallInterceptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallInterceptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ICallInterceptor> for ICallIndirect {
    fn from(value: ICallInterceptor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallInterceptor> for ICallIndirect {
    fn from(value: &ICallInterceptor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICallIndirect> for ICallInterceptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICallIndirect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICallIndirect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICallIndirect> for &ICallInterceptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICallIndirect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICallIndirect>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallInterceptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrreturn: *mut ::windows::runtime::HRESULT, imethod: u32, pvargs: *const ::std::ffi::c_void, cbargs: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, cbargs: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallUnmarshal(::windows::runtime::IUnknown);
impl ICallUnmarshal {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn Unmarshal<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, imethod: u32, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, fforcebuffercopy: Param3, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::std::option::Option<ICallFrame>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(imethod),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(cbbuffer),
            fforcebuffercopy.into_param().abi(),
            ::std::mem::transmute(datarep),
            ::std::mem::transmute(pcontext),
            ::std::mem::transmute(pcbunmarshalled),
            ::std::mem::transmute(ppframe),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_CallObj`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(imethod), ::std::mem::transmute(pbuffer), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(ibfirstrelease), ::std::mem::transmute(datarep), ::std::mem::transmute(pcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICallUnmarshal {
    type Vtable = ICallUnmarshal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1395896323, 11842, 4562, [184, 157, 0, 192, 79, 185, 97, 138]);
}
impl ::std::convert::From<ICallUnmarshal> for ::windows::runtime::IUnknown {
    fn from(value: ICallUnmarshal) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallUnmarshal> for ::windows::runtime::IUnknown {
    fn from(value: &ICallUnmarshal) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallUnmarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallUnmarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallUnmarshal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, pcbunmarshalled: *mut u32, ppframe: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, imethod: u32, pbuffer: *const ::std::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const ::std::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com_CallObj`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInterfaceRelated(::windows::runtime::IUnknown);
impl IInterfaceRelated {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn SetIID(&self, iid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(iid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub unsafe fn GetIID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInterfaceRelated {
    type Vtable = IInterfaceRelated_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3522910841, 30470, 4561, [173, 186, 0, 192, 79, 194, 173, 192]);
}
impl ::std::convert::From<IInterfaceRelated> for ::windows::runtime::IUnknown {
    fn from(value: IInterfaceRelated) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInterfaceRelated> for ::windows::runtime::IUnknown {
    fn from(value: &IInterfaceRelated) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInterfaceRelated {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInterfaceRelated {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterfaceRelated_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
