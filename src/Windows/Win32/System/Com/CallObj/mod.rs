#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for CALLFRAMEINFO {
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
impl ::core::cmp::Eq for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CALLFRAMEINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAMEPARAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAMEPARAMINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CALLFRAMEPARAMINFO").field("fIn", &self.fIn).field("fOut", &self.fOut).field("stackOffset", &self.stackOffset).field("cbParam", &self.cbParam).finish()
    }
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
unsafe impl ::windows::core::Abi for CALLFRAMEPARAMINFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_COPY(pub i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
impl ::core::convert::From<i32> for CALLFRAME_COPY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CALLFRAME_COPY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_FREE(pub i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
impl ::core::convert::From<i32> for CALLFRAME_FREE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CALLFRAME_FREE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::core::option::Option<::windows::core::IUnknown>,
    pub guidTransferSyntax: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CALLFRAME_MARSHALCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CALLFRAME_MARSHALCONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CALLFRAME_MARSHALCONTEXT").field("fIn", &self.fIn).field("dwDestContext", &self.dwDestContext).field("pvDestContext", &self.pvDestContext).field("punkReserved", &self.punkReserved).field("guidTransferSyntax", &self.guidTransferSyntax).finish()
    }
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
unsafe impl ::windows::core::Abi for CALLFRAME_MARSHALCONTEXT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_NULL(pub i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
impl ::core::convert::From<i32> for CALLFRAME_NULL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CALLFRAME_NULL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLFRAME_WALK(pub i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
impl ::core::convert::From<i32> for CALLFRAME_WALK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CALLFRAME_WALK {
    type Abi = Self;
}
#[inline]
pub unsafe fn CoGetInterceptor<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(iidintercepted: *const ::windows::core::GUID, punkouter: Param1, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptor(iidintercepted: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetInterceptor(::core::mem::transmute(iidintercepted), punkouter.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetInterceptorFromTypeInfo<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, super::ITypeInfo>>(iidintercepted: *const ::windows::core::GUID, punkouter: Param1, typeinfo: Param2, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, typeinfo: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetInterceptorFromTypeInfo(::core::mem::transmute(iidintercepted), punkouter.into_param().abi(), typeinfo.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallFrame(pub ::windows::core::IUnknown);
impl ICallFrame {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<CALLFRAMEINFO> {
        let mut result__: <CALLFRAMEINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CALLFRAMEINFO>(result__)
    }
    pub unsafe fn GetIIDAndMethod(&self, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(piid), ::core::mem::transmute(pimethod)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinterface), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackLocation(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetStackLocation(&self, pvstack: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvstack)))
    }
    pub unsafe fn SetReturnValue(&self, hr: ::windows::core::HRESULT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(hr)))
    }
    pub unsafe fn GetReturnValue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetParamInfo(&self, iparam: u32) -> ::windows::core::Result<CALLFRAMEPARAMINFO> {
        let mut result__: <CALLFRAMEPARAMINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iparam), &mut result__).from_abi::<CALLFRAMEPARAMINFO>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParam(&self, iparam: u32, pvar: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iparam), ::core::mem::transmute(pvar)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParam(&self, iparam: u32) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: <super::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(iparam), &mut result__).from_abi::<super::VARIANT>(result__)
    }
    pub unsafe fn Copy<'a, Param1: ::windows::core::IntoParam<'a, ICallFrameWalker>>(&self, copycontrol: CALLFRAME_COPY, pwalker: Param1) -> ::windows::core::Result<ICallFrame> {
        let mut result__: <ICallFrame as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(copycontrol), pwalker.into_param().abi(), &mut result__).from_abi::<ICallFrame>(result__)
    }
    pub unsafe fn Free<'a, Param0: ::windows::core::IntoParam<'a, ICallFrame>, Param1: ::windows::core::IntoParam<'a, ICallFrameWalker>, Param2: ::windows::core::IntoParam<'a, ICallFrameWalker>, Param4: ::windows::core::IntoParam<'a, ICallFrameWalker>>(&self, pframeargsdest: Param0, pwalkerdestfree: Param1, pwalkercopy: Param2, freeflags: u32, pwalkerfree: Param4, nullflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pframeargsdest.into_param().abi(), pwalkerdestfree.into_param().abi(), pwalkercopy.into_param().abi(), ::core::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::core::mem::transmute(nullflags)).ok()
    }
    pub unsafe fn FreeParam<'a, Param2: ::windows::core::IntoParam<'a, ICallFrameWalker>>(&self, iparam: u32, freeflags: u32, pwalkerfree: Param2, nullflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(iparam), ::core::mem::transmute(freeflags), pwalkerfree.into_param().abi(), ::core::mem::transmute(nullflags)).ok()
    }
    pub unsafe fn WalkFrame<'a, Param1: ::windows::core::IntoParam<'a, ICallFrameWalker>>(&self, walkwhat: u32, pwalker: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(walkwhat), pwalker.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmshlcontext), ::core::mem::transmute(mshlflags), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmshlcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcbbufferused), ::core::mem::transmute(pdatarep), ::core::mem::transmute(prpcflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal(&self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ibfirstrelease), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext)).ok()
    }
    pub unsafe fn Invoke(&self, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvreceiver)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICallFrame {
    type Vtable = ICallFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd573b4b0_894e_11d2_b8b6_00c04fb9618a);
}
impl ::core::convert::From<ICallFrame> for ::windows::core::IUnknown {
    fn from(value: ICallFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallFrame> for ::windows::core::IUnknown {
    fn from(value: &ICallFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *mut CALLFRAMEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID, pimethod: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszinterface: *mut super::super::super::Foundation::PWSTR, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvstack: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hr: ::windows::core::HRESULT),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iparam: u32, pvar: *const ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iparam: u32, pvar: *mut ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copycontrol: CALLFRAME_COPY, pwalker: ::windows::core::RawPtr, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pframeargsdest: ::windows::core::RawPtr, pwalkerdestfree: ::windows::core::RawPtr, pwalkercopy: ::windows::core::RawPtr, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iparam: u32, freeflags: u32, pwalkerfree: ::windows::core::RawPtr, nullflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, walkwhat: u32, pwalker: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmshlcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmshlcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, mshlflags: super::MSHLFLAGS, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, pcbunmarshalled: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvreceiver: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallFrameEvents(pub ::windows::core::IUnknown);
impl ICallFrameEvents {
    pub unsafe fn OnCall<'a, Param0: ::windows::core::IntoParam<'a, ICallFrame>>(&self, pframe: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pframe.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ICallFrameEvents {
    type Vtable = ICallFrameEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd5e0843_fc91_11d0_97d7_00c04fb9618a);
}
impl ::core::convert::From<ICallFrameEvents> for ::windows::core::IUnknown {
    fn from(value: ICallFrameEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallFrameEvents> for ::windows::core::IUnknown {
    fn from(value: &ICallFrameEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFrameEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallFrameEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pframe: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallFrameWalker(pub ::windows::core::IUnknown);
impl ICallFrameWalker {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWalkInterface<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: Param2, fout: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(ppvinterface), fin.into_param().abi(), fout.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ICallFrameWalker {
    type Vtable = ICallFrameWalker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08b23919_392d_11d2_b8a4_00c04fb9618a);
}
impl ::core::convert::From<ICallFrameWalker> for ::windows::core::IUnknown {
    fn from(value: ICallFrameWalker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallFrameWalker> for ::windows::core::IUnknown {
    fn from(value: &ICallFrameWalker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFrameWalker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallFrameWalker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFrameWalker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, ppvinterface: *const *const ::core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallIndirect(pub ::windows::core::IUnknown);
impl ICallIndirect {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrreturn), ::core::mem::transmute(imethod), ::core::mem::transmute(pvargs), ::core::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(imethod), ::core::mem::transmute(pinfo), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(imethod), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(piid), ::core::mem::transmute(pfderivesfromidispatch), ::core::mem::transmute(pcmethod), ::core::mem::transmute(pwszinterface)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICallIndirect {
    type Vtable = ICallIndirect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd573b4b1_894e_11d2_b8b6_00c04fb9618a);
}
impl ::core::convert::From<ICallIndirect> for ::windows::core::IUnknown {
    fn from(value: ICallIndirect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallIndirect> for ::windows::core::IUnknown {
    fn from(value: &ICallIndirect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallIndirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallIndirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallIndirect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallInterceptor(pub ::windows::core::IUnknown);
impl ICallInterceptor {
    pub unsafe fn CallIndirect(&self, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phrreturn), ::core::mem::transmute(imethod), ::core::mem::transmute(pvargs), ::core::mem::transmute(cbargs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(imethod), ::core::mem::transmute(pinfo), ::core::mem::transmute(pwszmethod)).ok()
    }
    pub unsafe fn GetStackSize(&self, imethod: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(imethod), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIID(&self, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(piid), ::core::mem::transmute(pfderivesfromidispatch), ::core::mem::transmute(pcmethod), ::core::mem::transmute(pwszinterface)).ok()
    }
    pub unsafe fn RegisterSink<'a, Param0: ::windows::core::IntoParam<'a, ICallFrameEvents>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn GetRegisteredSink(&self) -> ::windows::core::Result<ICallFrameEvents> {
        let mut result__: <ICallFrameEvents as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ICallFrameEvents>(result__)
    }
}
unsafe impl ::windows::core::Interface for ICallInterceptor {
    type Vtable = ICallInterceptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60c7ca75_896d_11d2_b8b6_00c04fb9618a);
}
impl ::core::convert::From<ICallInterceptor> for ::windows::core::IUnknown {
    fn from(value: ICallInterceptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallInterceptor> for ::windows::core::IUnknown {
    fn from(value: &ICallInterceptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ICallInterceptor> for ICallIndirect {
    fn from(value: ICallInterceptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallInterceptor> for ICallIndirect {
    fn from(value: &ICallInterceptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICallIndirect> for ICallInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ICallIndirect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICallIndirect> for &ICallInterceptor {
    fn into_param(self) -> ::windows::core::Param<'a, ICallIndirect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallInterceptor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phrreturn: *mut ::windows::core::HRESULT, imethod: u32, pvargs: *const ::core::ffi::c_void, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, cbargs: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallUnmarshal(pub ::windows::core::IUnknown);
impl ICallUnmarshal {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unmarshal<'a, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: Param3, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut ::core::option::Option<ICallFrame>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(imethod),
            ::core::mem::transmute(pbuffer),
            ::core::mem::transmute(cbbuffer),
            fforcebuffercopy.into_param().abi(),
            ::core::mem::transmute(datarep),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(pcbunmarshalled),
            ::core::mem::transmute(ppframe),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseMarshalData(&self, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(imethod), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ibfirstrelease), ::core::mem::transmute(datarep), ::core::mem::transmute(pcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICallUnmarshal {
    type Vtable = ICallUnmarshal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5333b003_2e42_11d2_b89d_00c04fb9618a);
}
impl ::core::convert::From<ICallUnmarshal> for ::windows::core::IUnknown {
    fn from(value: ICallUnmarshal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallUnmarshal> for ::windows::core::IUnknown {
    fn from(value: &ICallUnmarshal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallUnmarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallUnmarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallUnmarshal_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>, pcbunmarshalled: *mut u32, ppframe: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, imethod: u32, pbuffer: *const ::core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const ::core::mem::ManuallyDrop<CALLFRAME_MARSHALCONTEXT>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInterfaceRelated(pub ::windows::core::IUnknown);
impl IInterfaceRelated {
    pub unsafe fn SetIID(&self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid)).ok()
    }
    pub unsafe fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IInterfaceRelated {
    type Vtable = IInterfaceRelated_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1fb5a79_7706_11d1_adba_00c04fc2adc0);
}
impl ::core::convert::From<IInterfaceRelated> for ::windows::core::IUnknown {
    fn from(value: IInterfaceRelated) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInterfaceRelated> for ::windows::core::IUnknown {
    fn from(value: &IInterfaceRelated) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInterfaceRelated {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInterfaceRelated {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInterfaceRelated_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
