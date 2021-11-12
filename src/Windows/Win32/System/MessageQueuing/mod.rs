#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FOREIGN_STATUS(pub i32);
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
impl ::core::convert::From<i32> for FOREIGN_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FOREIGN_STATUS {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQApplication(pub ::windows::core::IUnknown);
impl IMSMQApplication {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, machinename: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), machinename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQApplication {
    type Vtable = IMSMQApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e085_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQApplication> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQApplication> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication> for super::Com::IDispatch {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQApplication2(pub ::windows::core::IUnknown);
impl IMSMQApplication2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, machinename: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), machinename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQApplication2 {
    type Vtable = IMSMQApplication2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12a30900_7300_11d2_b0e6_00e02c074f6b);
}
impl ::core::convert::From<IMSMQApplication2> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQApplication2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQApplication2> for IMSMQApplication {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQApplication2> for IMSMQApplication {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication> for IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication> for &IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQApplication2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, externalcertificate: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQApplication3(pub ::windows::core::IUnknown);
impl IMSMQApplication3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, machinename: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), machinename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ActiveQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrivateQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DirectoryServiceServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInAllQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMachine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmachine: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), bstrmachine.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Tidy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQApplication3 {
    type Vtable = IMSMQApplication3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1f_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQApplication3> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQApplication3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication2> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication2> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQApplication> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQApplication> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQApplication3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, externalcertificate: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvactivequeues: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvprivatequeues: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdirectoryserviceserver: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisconnected: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbytesinallqueues: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQCollection(pub ::windows::core::IUnknown);
impl IMSMQCollection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQCollection {
    type Vtable = IMSMQCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0188ac2f_ecb3_4173_9779_635ca2039c72);
}
impl ::core::convert::From<IMSMQCollection> for ::windows::core::IUnknown {
    fn from(value: IMSMQCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQCollection> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCollection> for super::Com::IDispatch {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCollection> for super::Com::IDispatch {
    fn from(value: &IMSMQCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pvarret: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQCoordinatedTransactionDispenser(pub ::windows::core::IUnknown);
impl IMSMQCoordinatedTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction> {
        let mut result__: <IMSMQTransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser {
    type Vtable = IMSMQCoordinatedTransactionDispenser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e081_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQCoordinatedTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQCoordinatedTransactionDispenser2(pub ::windows::core::IUnknown);
impl IMSMQCoordinatedTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2> {
        let mut result__: <IMSMQTransaction2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser2 {
    type Vtable = IMSMQCoordinatedTransactionDispenser2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b10_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQCoordinatedTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQCoordinatedTransactionDispenser3(pub ::windows::core::IUnknown);
impl IMSMQCoordinatedTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3> {
        let mut result__: <IMSMQTransaction3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser3 {
    type Vtable = IMSMQCoordinatedTransactionDispenser3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b14_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQCoordinatedTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQDestination(pub ::windows::core::IUnknown);
impl IMSMQDestination {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IADs(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_IADs<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, piads: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), piads.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetADsPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstradspath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstradspath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrpathname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), bstrformatname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destinations(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Destinations<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestinations: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pdestinations.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQDestination {
    type Vtable = IMSMQDestination_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b16_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQDestination> for ::windows::core::IUnknown {
    fn from(value: IMSMQDestination) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQDestination> for ::windows::core::IUnknown {
    fn from(value: &IMSMQDestination) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQDestination {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQDestination {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQDestination> for super::Com::IDispatch {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQDestination {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQDestination {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQDestination_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfisopen: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstradspath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQEvent(pub ::windows::core::IUnknown);
impl IMSMQEvent {}
unsafe impl ::windows::core::Interface for IMSMQEvent {
    type Vtable = IMSMQEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e077_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQEvent> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQEvent> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent> for super::Com::IDispatch {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQEvent2(pub ::windows::core::IUnknown);
impl IMSMQEvent2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQEvent2 {
    type Vtable = IMSMQEvent2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b12_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQEvent2> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQEvent2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQEvent2> for IMSMQEvent {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQEvent2> for IMSMQEvent {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent> for &IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQEvent2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQEvent3(pub ::windows::core::IUnknown);
impl IMSMQEvent3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQEvent3 {
    type Vtable = IMSMQEvent3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1c_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQEvent3> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQEvent3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent2> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent2> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQEvent> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQEvent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQEvent3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQManagement(pub ::windows::core::IUnknown);
impl IMSMQManagement {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQManagement {
    type Vtable = IMSMQManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe5f0241_e489_4957_8cc4_a452fcf3e23e);
}
impl ::core::convert::From<IMSMQManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQManagement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQManagement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQManagement> for super::Com::IDispatch {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machine: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pathname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, formatname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmessagecount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plforeignstatus: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plqueuetype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfislocal: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbytesinqueue: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQMessage(pub ::windows::core::IUnknown);
impl IMSMQMessage {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lappspecific)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), varbody.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(lack)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueue>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQMessage {
    type Vtable = IMSMQMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e074_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQMessage> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQMessage> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage> for super::Com::IDispatch {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQMessage {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ldelivery: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltrace: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidsrcmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarbody: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plack: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lhashalg: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenttime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plarrivedtime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsendercert: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenderid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationqueue: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQMessage2(pub ::windows::core::IUnknown);
impl IMSMQMessage2 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lappspecific)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), varbody.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(lack)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueue2>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), varextension.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQMessage2 {
    type Vtable = IMSMQMessage2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9933be0_a567_11d2_b0f3_00e02c074f6b);
}
impl ::core::convert::From<IMSMQMessage2> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQMessage2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQMessage2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ldelivery: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltrace: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidsrcmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarbody: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plack: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lhashalg: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenttime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plarrivedtime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsendercert: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenderid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationqueue: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarextension: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidconnectortype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvardestsymmkey: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsignature: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthprovname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarxactid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQMessage3(pub ::windows::core::IUnknown);
impl IMSMQMessage3 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lappspecific)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), varbody.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(lack)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), varextension.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v2<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo3>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo3>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestresponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), pdestresponse.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsoapheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), bstrsoapheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsoapbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), bstrsoapbody.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQMessage3 {
    type Vtable = IMSMQMessage3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1a_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQMessage3> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQMessage3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQMessage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQMessage3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQMessage3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ldelivery: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltrace: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidsrcmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarbody: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plack: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lhashalg: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenttime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plarrivedtime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsendercert: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenderid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationqueue: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarextension: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidconnectortype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvardestsymmkey: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsignature: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthprovname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarxactid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarlookupid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrsoapenvelope: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcompoundmessage: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQMessage4(pub ::windows::core::IUnknown);
impl IMSMQMessage4 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthlevel)).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(ldelivery)).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltrace)).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpriority)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lappspecific)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), varbody.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v1<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varmsgid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), varmsgid.into_param().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(lack)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreachqueue)).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmaxtimetoreceive)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhashalg)).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(lencryptalg)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsendercert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), varsendercert.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsenderidtype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, destinationqueue: Param0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), destinationqueue.into_param().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varextension: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), varextension.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidconnectortype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), bstrguidconnectortype.into_param().abi()).ok()
    }
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).61)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).62)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vardestsymmkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).63)(::core::mem::transmute_copy(self), vardestsymmkey.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).64)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsignature: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).65)(::core::mem::transmute_copy(self), varsignature.into_param().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).66)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).67)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthprovtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).68)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrauthprovname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).69)(::core::mem::transmute_copy(self), bstrauthprovname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsenderid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).70)(::core::mem::transmute_copy(self), varsenderid.into_param().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).71)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).72)(::core::mem::transmute_copy(self), ::core::mem::transmute(lmsgclass)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).73)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).74)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).75)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).76)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).77)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).78)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).79)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo_v2<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo2>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).80)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).81)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).82)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn putref_ResponseQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo4>>(&self, pqinforesponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).83)(::core::mem::transmute_copy(self), pqinforesponse.into_param().abi()).ok()
    }
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).84)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn putref_AdminQueueInfo<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueueInfo4>>(&self, pqinfoadmin: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).85)(::core::mem::transmute_copy(self), pqinfoadmin.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).86)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, pdestresponse: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).87)(::core::mem::transmute_copy(self), pdestresponse.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).88)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).89)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).90)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).91)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).92)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).93)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).94)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).95)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsoapheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).96)(::core::mem::transmute_copy(self), bstrsoapheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrsoapbody: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).97)(::core::mem::transmute_copy(self), bstrsoapbody.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQMessage4 {
    type Vtable = IMSMQMessage4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b23_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQMessage4> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQMessage4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQMessage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQMessage4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQMessage4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQMessage4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ldelivery: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ltrace: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidsrcmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarbody: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmsgid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plack: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lhashalg: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenttime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plarrivedtime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsendercert: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsenderid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destinationqueue: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarextension: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidconnectortype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvardestsymmkey: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsignature: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthprovname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarxactid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarlookupid: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrsoapenvelope: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcompoundmessage: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQOutgoingQueueManagement(pub ::windows::core::IUnknown);
impl IMSMQOutgoingQueueManagement {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NextHops(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn EodGetSendInfo(&self) -> ::windows::core::Result<IMSMQCollection> {
        let mut result__: <IMSMQCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQCollection>(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EodResend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQOutgoingQueueManagement {
    type Vtable = IMSMQOutgoingQueueManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c478fb_f9b0_4695_8a7f_439ac94326d3);
}
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQManagement> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQManagement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQManagement> for &IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQManagement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQOutgoingQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQOutgoingQueueManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machine: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pathname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, formatname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmessagecount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plforeignstatus: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plqueuetype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfislocal: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbytesinqueue: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plstate: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvnexthops: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQPrivateDestination(pub ::windows::core::IUnknown);
impl IMSMQPrivateDestination {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHandle<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), varhandle.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQPrivateDestination {
    type Vtable = IMSMQPrivateDestination_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b17_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQPrivateDestination> for ::windows::core::IUnknown {
    fn from(value: IMSMQPrivateDestination) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQPrivateDestination> for ::windows::core::IUnknown {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQPrivateDestination {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQPrivateDestination {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQPrivateDestination {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQPrivateDestination {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateDestination_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarhandle: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQPrivateEvent(pub ::windows::core::IUnknown);
impl IMSMQPrivateEvent {
    pub unsafe fn Hwnd(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn FireArrivedEvent<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueue>>(&self, pq: Param0, msgcursor: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pq.into_param().abi(), ::core::mem::transmute(msgcursor)).ok()
    }
    pub unsafe fn FireArrivedErrorEvent<'a, Param0: ::windows::core::IntoParam<'a, IMSMQQueue>>(&self, pq: Param0, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pq.into_param().abi(), ::core::mem::transmute(hrstatus), ::core::mem::transmute(msgcursor)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQPrivateEvent {
    type Vtable = IMSMQPrivateEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7ab3341_c9d3_11d1_bb47_0080c7c5a2c0);
}
impl ::core::convert::From<IMSMQPrivateEvent> for ::windows::core::IUnknown {
    fn from(value: IMSMQPrivateEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQPrivateEvent> for ::windows::core::IUnknown {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQPrivateEvent {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phwnd: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQuery(pub ::windows::core::IUnknown);
impl IMSMQQuery {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos> {
        let mut result__: <IMSMQQueueInfos as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQuery {
    type Vtable = IMSMQQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e072_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQQuery> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQuery> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery> for super::Com::IDispatch {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQuery {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQuery2(pub ::windows::core::IUnknown);
impl IMSMQQuery2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2> {
        let mut result__: <IMSMQQueueInfos2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQuery2 {
    type Vtable = IMSMQQuery2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b0e_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQuery2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQuery2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQuery2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQuery3(pub ::windows::core::IUnknown);
impl IMSMQQuery3 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3> {
        let mut result__: <IMSMQQueueInfos3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
        multicastaddress: *const super::Com::VARIANT,
        relmulticastaddress: *const super::Com::VARIANT,
    ) -> ::windows::core::Result<IMSMQQueueInfos3> {
        let mut result__: <IMSMQQueueInfos3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQuery3 {
    type Vtable = IMSMQQuery3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b19_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQuery3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQuery3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQuery3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        multicastaddress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmulticastaddress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQuery4(pub ::windows::core::IUnknown);
impl IMSMQQuery4 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4> {
        let mut result__: <IMSMQQueueInfos4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(
        &self,
        queueguid: *const super::Com::VARIANT,
        servicetypeguid: *const super::Com::VARIANT,
        label: *const super::Com::VARIANT,
        createtime: *const super::Com::VARIANT,
        modifytime: *const super::Com::VARIANT,
        relservicetype: *const super::Com::VARIANT,
        rellabel: *const super::Com::VARIANT,
        relcreatetime: *const super::Com::VARIANT,
        relmodifytime: *const super::Com::VARIANT,
        multicastaddress: *const super::Com::VARIANT,
        relmulticastaddress: *const super::Com::VARIANT,
    ) -> ::windows::core::Result<IMSMQQueueInfos4> {
        let mut result__: <IMSMQQueueInfos4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            &mut result__,
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQuery4 {
    type Vtable = IMSMQQuery4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b24_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQuery4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQuery4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQuery4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQuery4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQuery4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQuery4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        queueguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        servicetypeguid: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        label: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        createtime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        modifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relservicetype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        rellabel: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relcreatetime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmodifytime: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        multicastaddress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        relmulticastaddress: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>,
        ppqinfos: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueue(pub ::windows::core::IUnknown);
impl IMSMQQueue {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::core::IntoParam<'a, IMSMQEvent>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueue {
    type Vtable = IMSMQQueue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e076_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQQueue> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueue> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue> for super::Com::IDispatch {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr, cursor: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueue2(pub ::windows::core::IUnknown);
impl IMSMQQueue2 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::core::IntoParam<'a, IMSMQEvent2>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__: <IMSMQMessage2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueue2 {
    type Vtable = IMSMQQueue2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef0574e0_06d8_11d3_b100_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueue2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueue2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueue2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueue2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueue2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr, cursor: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueue3(pub ::windows::core::IUnknown);
impl IMSMQQueue3 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::core::IntoParam<'a, IMSMQEvent3>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveNextByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNextByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekPreviousByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__: <IMSMQMessage3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage3>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueue3 {
    type Vtable = IMSMQQueue3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1b_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueue3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueue3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueue3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueue3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueue3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr, cursor: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarhandle: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueue4(pub ::windows::core::IUnknown);
impl IMSMQQueue4 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, Param0: ::windows::core::IntoParam<'a, IMSMQEvent3>>(&self, event: Param0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), event.into_param().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__: <IMSMQMessage as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), &mut result__).from_abi::<IMSMQMessage>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveNextByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNextByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekPreviousByLookupId<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupIdAllowPeek<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, lookupid: Param0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__: <IMSMQMessage4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), lookupid.into_param().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), &mut result__).from_abi::<IMSMQMessage4>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueue4 {
    type Vtable = IMSMQQueue4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b20_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueue4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueue4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueue4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueue4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueue4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueue4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, event: ::windows::core::RawPtr, cursor: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, receivetimeout: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarhandle: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisopen: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantbody: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantconnectortype: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfo(pub ::windows::core::IUnknown);
impl IMSMQQueueInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguidservicetype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpathname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue> {
        let mut result__: <IMSMQQueue as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), &mut result__).from_abi::<IMSMQQueue>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfo {
    type Vtable = IMSMQQueueInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07b_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQQueueInfo> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfo> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidqueue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidservicetype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcreatetime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmodifytime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, istransactional: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, isworldreadable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfo2(pub ::windows::core::IUnknown);
impl IMSMQQueueInfo2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguidservicetype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpathname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue2> {
        let mut result__: <IMSMQQueue2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), &mut result__).from_abi::<IMSMQQueue2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), varsecurity.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfo2 {
    type Vtable = IMSMQQueueInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd174a80_89cf_11d2_b0f2_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfo2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfo2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidqueue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidservicetype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcreatetime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmodifytime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, istransactional: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, isworldreadable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathnamedns: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsecurity: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfo3(pub ::windows::core::IUnknown);
impl IMSMQQueueInfo3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguidservicetype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpathname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue3> {
        let mut result__: <IMSMQQueue3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), &mut result__).from_abi::<IMSMQQueue3>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), varsecurity.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmulticastaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstrmulticastaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfo3 {
    type Vtable = IMSMQQueueInfo3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1d_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfo3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfo3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidqueue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidservicetype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcreatetime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmodifytime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, istransactional: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, isworldreadable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathnamedns: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsecurity: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmulticastaddress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstradspath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfo4(pub ::windows::core::IUnknown);
impl IMSMQQueueInfo4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrguidservicetype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrguidservicetype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrlabel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), bstrlabel.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpathname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrpathname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrformatname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), bstrformatname.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(lprivlevel)).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournal)).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquota)).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lbasepriority)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lauthenticate)).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(ljournalquota)).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue4> {
        let mut result__: <IMSMQQueue4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(access), ::core::mem::transmute(sharemode), &mut result__).from_abi::<IMSMQQueue4>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, varsecurity: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), varsecurity.into_param().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrmulticastaddress: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), bstrmulticastaddress.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfo4 {
    type Vtable = IMSMQQueueInfo4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b21_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfo4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfo4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfo4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidqueue: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrguidservicetype: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrlabel: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournal: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarcreatetime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarmodifytime: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, istransactional: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, isworldreadable: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathnamedns: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvarsecurity: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmulticastaddress: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstradspath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfos(pub ::windows::core::IUnknown);
impl IMSMQQueueInfos {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__: <IMSMQQueueInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfos {
    type Vtable = IMSMQQueueInfos_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07d_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQQueueInfos> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfos> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfos {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfos {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfos2(pub ::windows::core::IUnknown);
impl IMSMQQueueInfos2 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__: <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfos2 {
    type Vtable = IMSMQQueueInfos2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b0f_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfos2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfos2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfos2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfos3(pub ::windows::core::IUnknown);
impl IMSMQQueueInfos3 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__: <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfos3 {
    type Vtable = IMSMQQueueInfos3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1e_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfos3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfos3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfos3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueInfos4(pub ::windows::core::IUnknown);
impl IMSMQQueueInfos4 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__: <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueInfos4 {
    type Vtable = IMSMQQueueInfos4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b22_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQQueueInfos4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueInfos4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueInfos4 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQQueueManagement(pub ::windows::core::IUnknown);
impl IMSMQQueueManagement {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn JournalMessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInJournal(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EodGetReceiveInfo(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQQueueManagement {
    type Vtable = IMSMQQueueManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fbe7759_5760_444d_b8a5_5e7ab9a84cce);
}
impl ::core::convert::From<IMSMQQueueManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueManagement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQQueueManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueManagement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQManagement> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQManagement> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQManagement> for &IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQManagement> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQQueueManagement {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, machine: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, pathname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, formatname: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrformatname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrmachine: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmessagecount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plforeignstatus: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plqueuetype: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfislocal: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbytesinqueue: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbytesinjournal: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvcollection: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransaction(pub ::windows::core::IUnknown);
impl IMSMQTransaction {
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransaction {
    type Vtable = IMSMQTransaction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07f_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQTransaction> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransaction> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransaction {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransaction: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grftc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grfrm: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, fasync: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransaction2(pub ::windows::core::IUnknown);
impl IMSMQTransaction2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitNew<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vartransaction: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), vartransaction.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransaction2 {
    type Vtable = IMSMQTransaction2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce0c5b0_6e67_11d2_b0e6_00e02c074f6b);
}
impl ::core::convert::From<IMSMQTransaction2> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransaction2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction> for &IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransaction2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransaction: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grftc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grfrm: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, fasync: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransaction3(pub ::windows::core::IUnknown);
impl IMSMQTransaction3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitNew<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, vartransaction: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), vartransaction.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ITransaction(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransaction3 {
    type Vtable = IMSMQTransaction3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b13_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQTransaction3> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransaction3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction2> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction2> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSMQTransaction> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, IMSMQTransaction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransaction3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pltransaction: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grftc: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, grfrm: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fretaining: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>, fasync: *const ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvaritransaction: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransactionDispenser(pub ::windows::core::IUnknown);
impl IMSMQTransactionDispenser {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction> {
        let mut result__: <IMSMQTransaction as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser {
    type Vtable = IMSMQTransactionDispenser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e083_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<IMSMQTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransactionDispenser {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransactionDispenser2(pub ::windows::core::IUnknown);
impl IMSMQTransactionDispenser2 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2> {
        let mut result__: <IMSMQTransaction2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser2 {
    type Vtable = IMSMQTransactionDispenser2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b11_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransactionDispenser2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMSMQTransactionDispenser3(pub ::windows::core::IUnknown);
impl IMSMQTransactionDispenser3 {
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3> {
        let mut result__: <IMSMQTransaction3 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMSMQTransaction3>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__: <super::Com::IDispatch as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser3 {
    type Vtable = IMSMQTransactionDispenser3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b15_2168_11d3_898c_00e02c074f6b);
}
impl ::core::convert::From<IMSMQTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMSMQTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IMSMQTransactionDispenser3 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
pub const LONG_LIVED: u32 = 4294967294u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQACCESS(pub i32);
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
impl ::core::convert::From<i32> for MQACCESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQACCESS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQAUTHENTICATE(pub i32);
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
impl ::core::convert::From<i32> for MQAUTHENTICATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQAUTHENTICATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQCALG(pub i32);
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
impl ::core::convert::From<i32> for MQCALG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQCALG {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQCERT_REGISTER(pub i32);
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
impl ::core::convert::From<i32> for MQCERT_REGISTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQCERT_REGISTER {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQDEFAULT(pub i32);
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
impl ::core::convert::From<i32> for MQDEFAULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQDEFAULT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQERROR(pub i32);
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
impl ::core::convert::From<i32> for MQERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQERROR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQJOURNAL(pub i32);
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
impl ::core::convert::From<i32> for MQJOURNAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQJOURNAL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMAX(pub i32);
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
impl ::core::convert::From<i32> for MQMAX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMAX {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
impl ::core::convert::From<i32> for MQMSGACKNOWLEDGEMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGACKNOWLEDGEMENT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGAUTHENTICATION(pub i32);
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
impl ::core::convert::From<i32> for MQMSGAUTHENTICATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGAUTHENTICATION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGAUTHLEVEL(pub i32);
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
impl ::core::convert::From<i32> for MQMSGAUTHLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGAUTHLEVEL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGCLASS(pub i32);
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
impl ::core::convert::From<i32> for MQMSGCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGCLASS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGCURSOR(pub i32);
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
impl ::core::convert::From<i32> for MQMSGCURSOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGCURSOR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGDELIVERY(pub i32);
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
impl ::core::convert::From<i32> for MQMSGDELIVERY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGDELIVERY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGIDSIZE(pub i32);
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
impl ::core::convert::From<i32> for MQMSGIDSIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGIDSIZE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGJOURNAL(pub i32);
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
impl ::core::convert::From<i32> for MQMSGJOURNAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGJOURNAL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGMAX(pub i32);
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
impl ::core::convert::From<i32> for MQMSGMAX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGMAX {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGPRIVLEVEL(pub i32);
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
impl ::core::convert::From<i32> for MQMSGPRIVLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGPRIVLEVEL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGSENDERIDTYPE(pub i32);
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
impl ::core::convert::From<i32> for MQMSGSENDERIDTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGSENDERIDTYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQMSGTRACE(pub i32);
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
impl ::core::convert::From<i32> for MQMSGTRACE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQMSGTRACE {
    type Abi = Self;
}
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQPRIORITY(pub i32);
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
impl ::core::convert::From<i32> for MQPRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQPRIORITY {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQPRIVLEVEL(pub i32);
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
impl ::core::convert::From<i32> for MQPRIVLEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQPRIVLEVEL {
    type Abi = Self;
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQSHARE(pub i32);
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
impl ::core::convert::From<i32> for MQSHARE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQSHARE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQTRANSACTION(pub i32);
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
impl ::core::convert::From<i32> for MQTRANSACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQTRANSACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQTRANSACTIONAL(pub i32);
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
impl ::core::convert::From<i32> for MQTRANSACTIONAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQTRANSACTIONAL {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MQWARNING(pub i32);
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
impl ::core::convert::From<i32> for MQWARNING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MQWARNING {
    type Abi = Self;
}
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824164i32 as _);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824165i32 as _);
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824167i32 as _);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824166i32 as _);
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
pub const MQ_MOVE_ACCESS: u32 = 4u32;
pub const MQ_OK: ::windows::core::HRESULT = ::windows::core::HRESULT(0i32 as _);
pub const MSMQApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e086_dccd_11d0_aa4b_0060970debae);
pub const MSMQCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72b9031_2f0c_43e8_924e_e6052cdc493f);
pub const MSMQCoordinatedTransactionDispenser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e082_dccd_11d0_aa4b_0060970debae);
pub const MSMQDestination: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b18_2168_11d3_898c_00e02c074f6b);
pub const MSMQEvent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07a_dccd_11d0_aa4b_0060970debae);
pub const MSMQManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39ce96fe_f4c5_4484_a143_4c2d5d324229);
pub const MSMQMessage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e075_dccd_11d0_aa4b_0060970debae);
pub const MSMQOutgoingQueueManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0188401c_247a_4fed_99c6_bf14119d7055);
pub const MSMQQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e073_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e079_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07c_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfos: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07e_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33b6d07e_f27d_42fa_b2d7_bf82e11e9374);
pub const MSMQTransaction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e080_dccd_11d0_aa4b_0060970debae);
pub const MSMQTransactionDispenser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e084_dccd_11d0_aa4b_0060970debae);
pub const PREQ: u32 = 4u32;
pub const PRGE: u32 = 3u32;
pub const PRGT: u32 = 2u32;
pub const PRLE: u32 = 1u32;
pub const PRLT: u32 = 0u32;
pub const PRNE: u32 = 5u32;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
pub const PROPID_M_BASE: u32 = 0u32;
pub const PROPID_M_BODY: u32 = 9u32;
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
pub const PROPID_M_CLASS: u32 = 1u32;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
pub const PROPID_M_DELIVERY: u32 = 5u32;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
pub const PROPID_M_EXTENSION: u32 = 35u32;
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
pub const PROPID_M_HASH_ALG: u32 = 26u32;
pub const PROPID_M_JOURNAL: u32 = 7u32;
pub const PROPID_M_LABEL: u32 = 11u32;
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
pub const PROPID_M_LOOKUPID: u32 = 60u32;
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
pub const PROPID_M_MSGID: u32 = 2u32;
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
pub const PROPID_M_PRIORITY: u32 = 4u32;
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
pub const PROPID_M_PROV_NAME: u32 = 48u32;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
pub const PROPID_M_SENDERID: u32 = 20u32;
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
pub const PROPID_M_SENTTIME: u32 = 31u32;
pub const PROPID_M_SIGNATURE: u32 = 45u32;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
pub const PROPID_M_TRACE: u32 = 41u32;
pub const PROPID_M_VERSION: u32 = 19u32;
pub const PROPID_M_XACTID: u32 = 52u32;
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
pub const PROPID_PC_BASE: u32 = 5800u32;
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
pub const PROPID_PC_VERSION: u32 = 5801u32;
pub const PROPID_QM_BASE: u32 = 200u32;
pub const PROPID_QM_CONNECTION: u32 = 204u32;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
pub const PROPID_QM_PATHNAME: u32 = 203u32;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
pub const PROPID_QM_SITE_ID: u32 = 201u32;
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
pub const PROPID_Q_BASE: u32 = 100u32;
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
pub const PROPID_Q_INSTANCE: u32 = 101u32;
pub const PROPID_Q_JOURNAL: u32 = 104u32;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
pub const PROPID_Q_LABEL: u32 = 108u32;
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
pub const PROPID_Q_PATHNAME: u32 = 103u32;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
pub const PROPID_Q_QUOTA: u32 = 105u32;
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
pub const PROPID_Q_TYPE: u32 = 102u32;
pub const QUERY_SORTASCEND: u32 = 0u32;
pub const QUERY_SORTDESCEND: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QUEUE_STATE(pub i32);
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
impl ::core::convert::From<i32> for QUEUE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QUEUE_TYPE(pub i32);
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
impl ::core::convert::From<i32> for QUEUE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RELOPS(pub i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
impl ::core::convert::From<i32> for RELOPS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RELOPS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct XACT_STATUS(pub i32);
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
impl ::core::convert::From<i32> for XACT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for XACT_STATUS {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _DMSMQEventEvents(pub ::windows::core::IUnknown);
impl _DMSMQEventEvents {}
unsafe impl ::windows::core::Interface for _DMSMQEventEvents {
    type Vtable = _DMSMQEventEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e078_dccd_11d0_aa4b_0060970debae);
}
impl ::core::convert::From<_DMSMQEventEvents> for ::windows::core::IUnknown {
    fn from(value: _DMSMQEventEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_DMSMQEventEvents> for ::windows::core::IUnknown {
    fn from(value: &_DMSMQEventEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _DMSMQEventEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _DMSMQEventEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _DMSMQEventEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &_DMSMQEventEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _DMSMQEventEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
